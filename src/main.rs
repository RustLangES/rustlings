use crate::exercise::{Exercise, ExerciseList};
use crate::project::RustAnalyzerProject;
use crate::run::{reset, run};
use crate::verify::verify;
use clap::{Parser, Subcommand};
use console::Emoji;
use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::OsStr;
use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[macro_use]
mod ui;

mod exercise;
mod project;
mod run;
mod verify;

/// Rustlings is a collection of small exercises to get you used to writing and reading Rust code
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Show outputs from the test exercises
    #[arg(long)]
    nocapture: bool,
    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Verify all exercises according to the recommended order
    Verify,
    /// Rerun `verify` when files were edited
    Watch {
        /// Show hints on success
        #[arg(long)]
        success_hints: bool,
    },
    /// Run/Test a single exercise
    Run {
        /// The name of the exercise
        name: String,
    },
    /// Reset a single exercise using "git stash -- <filename>"
    Reset {
        /// The name of the exercise
        name: String,
    },
    /// Return a hint for the given exercise
    Hint {
        /// The name of the exercise
        name: String,
    },
    /// List the exercises available in Rustlings
    List {
        /// Show only the paths of the exercises
        #[arg(short, long)]
        paths: bool,
        /// Show only the names of the exercises
        #[arg(short, long)]
        names: bool,
        /// Provide a string to match exercise names.
        /// Comma separated patterns are accepted
        #[arg(short, long)]
        filter: Option<String>,
        /// Display only exercises not yet solved
        #[arg(short, long)]
        unsolved: bool,
        /// Display only exercises that have been solved
        #[arg(short, long)]
        solved: bool,
    },
    /// Enable rust-analyzer for exercises
    Lsp,
}

fn main() {
    let args = Args::parse();

    if args.command.is_none() {
        println!("\n{WELCOME}\n");
    }

    if !Path::new("info.toml").exists() {
        println!(
            "{} debe ejecutarse desde el directorio rustlings"
            std::env::current_exe().unwrap().to_str().unwrap()
        );
        println!("Intente `cd rustlings/`!");
        std::process::exit(1);
    }

    if !rustc_exists() {
        println!("No podemos encontrar `rustc`.");
        println!("Intente ejecutar `rustc --version` para diagnosticar su problema.");
        println!("Para obtener instrucciones sobre cómo instalar Rust, consulte el README.");
        std::process::exit(1);
    }

    let toml_str = &fs::read_to_string("info.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;
    let verbose = args.nocapture;

    let command = args.command.unwrap_or_else(|| {
        println!("{DEFAULT_OUT}\n");
        std::process::exit(0);
    });

    match command {
        Subcommands::List {
            paths,
            names,
            filter,
            unsolved,
            solved,
        } => {
            if !paths && !names {
                println!("{:<17}\t{:<46}\t{:<7}", "Nombre", "Ruta", "Estado");
            }
            let mut exercises_done: u16 = 0;
            let filters = filter.clone().unwrap_or_default().to_lowercase();
            exercises.iter().for_each(|e| {
                let fname = format!("{}", e.path.display());
                let filter_cond = filters
                    .split(',')
                    .filter(|f| !f.trim().is_empty())
                    .any(|f| e.name.contains(f) || fname.contains(f));
                let status = if e.looks_done() {
                    exercises_done += 1;
                    "Hecho"
                } else {
                    "Pendiente"
                };
                let solve_cond = {
                    (e.looks_done() && solved)
                        || (!e.looks_done() && unsolved)
                        || (!solved && !unsolved)
                };
                if solve_cond && (filter_cond || filter.is_none()) {
                    let line = if paths {
                        format!("{fname}\n")
                    } else if names {
                        format!("{}\n", e.name)
                    } else {
                        format!("{:<17}\t{fname:<46}\t{status:<7}\n", e.name)
                    };
                    // Somehow using println! leads to the binary panicking
                    // when its output is piped.
                    // So, we're handling a Broken Pipe error and exiting with 0 anyway
                    let stdout = std::io::stdout();
                    {
                        let mut handle = stdout.lock();
                        handle.write_all(line.as_bytes()).unwrap_or_else(|e| {
                            match e.kind() {
                                std::io::ErrorKind::BrokenPipe => std::process::exit(0),
                                _ => std::process::exit(1),
                            };
                        });
                    }
                }
            });
            let percentage_progress = exercises_done as f32 / exercises.len() as f32 * 100.0;
            println!(
                "Progreso: Has completado {} / {} ejercicios ({:.1} %)."
                exercises_done,
                exercises.len(),
                percentage_progress
            );
            std::process::exit(0);
        }

        Subcommands::Run { name } => {
            let exercise = find_exercise(&name, &exercises);

            run(exercise, verbose).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Reset { name } => {
            let exercise = find_exercise(&name, &exercises);

            reset(exercise).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Hint { name } => {
            let exercise = find_exercise(&name, &exercises);

            println!("{}", exercise.hint);
        }

        Subcommands::Verify => {
            verify(&exercises, (0, exercises.len()), verbose, false)
                .unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Lsp => {
            let mut project = RustAnalyzerProject::new();
            project
                .get_sysroot_src()
                .expect("No se pudo encontrar la ruta de las herramientas, ¿tiene `rustc` instalado?");
            project
                .exercises_to_json()
                .expect("No se pudieron analizar los archivos de ejercicios de rustlings");

            if project.crates.is_empty() {
                println!("Falló al encontrar ejercicios, asegúrese de estar en la carpeta `rustlings`");
            } else if project.write_to_disk().is_err() {
                println!("Falló al escribir rust-project.json en el disco para rust-analyzer");
            } else {
                println!("Generado con éxito rust-project.json");
                println!("rust-analyzer ahora analizará los ejercicios, reinicie su servidor de lenguaje o editor")
            }
        }

        Subcommands::Watch { success_hints } => match watch(&exercises, verbose, success_hints) {
            Err(e) => {
                println!(
                    "Error: No se pudo observar su progreso. El mensaje de error fue {:?}."
                    e
                );
                println!("Lo más probable es que se haya quedado sin espacio en disco o se haya alcanzado su 'límite de inotify'.");
                std::process::exit(1);
            }
            Ok(WatchStatus::Finished) => {
                println!(
                    "{emoji} ¡Todos los ejercicios completados! {emoji}", 
                    emoji = Emoji("🎉", "★")
                );
                println!("\n{FENISH_LINE}\n");
            }
            Ok(WatchStatus::Unfinished) => {
                println!("Esperamos que estés disfrutando aprendiendo sobre Rust!");
                println!("Si quieres continuar trabajando en los ejercicios en otro momento, simplemente ejecuta `rustlings watch` de nuevo");
            }
        },
    }
}

fn spawn_watch_shell(
    failed_exercise_hint: &Arc<Mutex<Option<String>>>,
    should_quit: Arc<AtomicBool>,
) {
    let failed_exercise_hint = Arc::clone(failed_exercise_hint);
    println!("Bienvenid@ al modo de observación! Puedes escribir 'help' para obtener una descripción general de los comandos que puedes usar aquí.");
    thread::spawn(move || loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "hint" {
                    if let Some(hint) = &*failed_exercise_hint.lock().unwrap() {
                        println!("{hint}");
                    }
                } else if input == "clear" {
                    println!("\x1B[2J\x1B[1;1H");
                } else if input.eq("quit") {
                    should_quit.store(true, Ordering::SeqCst);
                    println!("¡Adiós!");
                } else if input.eq("help") {
                    println!("Comandos disponibles en el modo de observación:");
                    println!("  hint   - imprime la pista del ejercicio actual");
                    println!("  clear  - limpia la pantalla");
                    println!("  quit   - sale del modo de observación");
                    println!("  !<cmd> - ejecuta un comando, como `!rustc --explain E0381`");
                    println!("  help   - muestra este mensaje de ayuda");
                    println!();
                    println!("El modo de observación reevalúa automáticamente el ejercicio actual");
                    println!("cuando editas el contenido de un archivo.");
                } else if let Some(cmd) = input.strip_prefix('!') {
                    let parts: Vec<&str> = cmd.split_whitespace().collect();
                    if parts.is_empty() {
                        println!("No se proporcionó ningún comando");
                    } else if let Err(e) = Command::new(parts[0]).args(&parts[1..]).status() {
                        println!("Falló al ejecutar el comando `{}`: {}", cmd, e);
                    }
                } else {
                    println!("unknown command: {input}");
                }
            }
            Err(error) => println!("Error leyendo el comando: {error}"),
        }
    });
}

fn find_exercise<'a>(name: &str, exercises: &'a [Exercise]) -> &'a Exercise {
    if name.eq("next") {
        exercises
            .iter()
            .find(|e| !e.looks_done())
            .unwrap_or_else(|| {
                println!("🎉 ¡Felicidades! ¡Has completado todos los ejercicios!");
                println!("🔚 ¡No hay más ejercicios por hacer después de este!");
                std::process::exit(1)
            })
    } else {
        exercises
            .iter()
            .find(|e| e.name == name)
            .unwrap_or_else(|| {
                println!("No se encontró ningún ejercicio para '{name}'!");
                std::process::exit(1)
            })
    }
}

enum WatchStatus {
    Finished,
    Unfinished,
}

fn watch(
    exercises: &[Exercise],
    verbose: bool,
    success_hints: bool,
) -> notify::Result<WatchStatus> {
    /* Clears the terminal with an ANSI escape code.
    Works in UNIX and newer Windows terminals. */
    fn clear_screen() {
        println!("\x1Bc");
    }

    let (tx, rx) = channel();
    let should_quit = Arc::new(AtomicBool::new(false));

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;
    watcher.watch(Path::new("./exercises"), RecursiveMode::Recursive)?;

    clear_screen();

    let to_owned_hint = |t: &Exercise| t.hint.to_owned();
    let failed_exercise_hint = match verify(
        exercises.iter(),
        (0, exercises.len()),
        verbose,
        success_hints,
    ) {
        Ok(_) => return Ok(WatchStatus::Finished),
        Err(exercise) => Arc::new(Mutex::new(Some(to_owned_hint(exercise)))),
    };
    spawn_watch_shell(&failed_exercise_hint, Arc::clone(&should_quit));
    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => match event {
                DebouncedEvent::Create(b) | DebouncedEvent::Chmod(b) | DebouncedEvent::Write(b) => {
                    if b.extension() == Some(OsStr::new("rs")) && b.exists() {
                        let filepath = b.as_path().canonicalize().unwrap();
                        let pending_exercises = exercises
                            .iter()
                            .find(|e| filepath.ends_with(&e.path))
                            .into_iter()
                            .chain(
                                exercises
                                    .iter()
                                    .filter(|e| !e.looks_done() && !filepath.ends_with(&e.path)),
                            );
                        let num_done = exercises.iter().filter(|e| e.looks_done()).count();
                        clear_screen();
                        match verify(
                            pending_exercises,
                            (num_done, exercises.len()),
                            verbose,
                            success_hints,
                        ) {
                            Ok(_) => return Ok(WatchStatus::Finished),
                            Err(exercise) => {
                                let mut failed_exercise_hint = failed_exercise_hint.lock().unwrap();
                                *failed_exercise_hint = Some(to_owned_hint(exercise));
                            }
                        }
                    }
                }
                _ => {}
            },
            Err(RecvTimeoutError::Timeout) => {
                // the timeout expired, just check the `should_quit` variable below then loop again
            }
            Err(e) => println!("Error observando: {e:?}"),
        }
        // Check if we need to exit
        if should_quit.load(Ordering::SeqCst) {
            return Ok(WatchStatus::Unfinished);
        }
    }
}

fn rustc_exists() -> bool {
    Command::new("rustc")
        .args(["--version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}

const DEFAULT_OUT: &str = r#"¡Gracias por instalar Rustlings!

¿Es la primera vez que lo usas? ¡No te preocupes, Rustlings fue creado para
principiantes! Vamos a enseñarte muchas cosas sobre Rust, pero antes de que 
podamos empezar, aquí tienes algunas notas sobre cómo opera Rustlings:

1. El concepto central detrás de Rustlings es que resuelvas ejercicios. Estos 
   ejercicios suelen tener algún tipo de error de sintaxis en ellos, lo que 
   causará que fallen en la compilación o en las pruebas. A veces, en lugar de 
   un error de sintaxis, hay un error lógico. Sin importar el tipo de error, tu 
   trabajo es encontrarlo y corregirlo. Sabrás cuando lo hayas corregido porque 
   entonces el ejercicio se compilará y Rustlings podrá pasar al siguiente 
   ejercicio.
2. Si ejecutas Rustlings en modo de observación (que recomendamos), comenzará 
   automáticamente con el primer ejercicio. ¡No te confundas si ves un mensaje 
   de error apareciendo tan pronto como ejecutes Rustlings! Esto es parte del 
   ejercicio que debes resolver, así que abre el archivo del ejercicio en un 
   editor y comienza tu trabajo de detective.
3. Si te quedas atascado en un ejercicio, hay una pista útil que puedes ver 
   escribiendo 'hint' (en modo de observación) o ejecutando 
   `rustlings hint nombre_del_ejercicio`.
4. Si un ejercicio no tiene sentido para ti, ¡siéntete libre de abrir un problema
   en GitHub! (https://github.com/RustLangEs/rustlings/issues/new). Revisamos 
   cada problema y a veces, otros aprendices también lo hacen, ¡así que pueden 
   ayudarse mutuamente!
5. Si deseas utilizar `rust-analyzer` con los ejercicios, que proporciona 
   características como el autocompletado, ejecuta el comando `rustlings lsp`.

¿Lo tienes todo claro? ¡Genial! Para empezar, ejecuta `rustlings watch` para 
obtener el primer ejercicio. ¡Asegúrate de tener tu editor abierto!"#;

const FENISH_LINE: &str = r"+----------------------------------------------------+
|          You made it to the Fe-nish line!          |
|          You made it to the Fe-nish line!          |
| Lo hiciste, ¡te sumergiste en Rustlings y ganaste! |
+--------------------------  ------------------------+
                          \\/
     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
 ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▒▒▓▓▒▒▒▒▒▒▒▒
           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒
       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒
           ▒▒  ▒▒                      ▒▒  ▒▒

¡Esperamos que hayas disfrutado aprendiendo sobre los diversos aspectos de Rust!
Si notaste algún problema, no dudes en informarlo en nuestro repositorio.
¡También puedes contribuir con tus propios ejercicios para ayudar a la comunidad en general!

Antes de informar un problema o contribuir, por favor, lee nuestras guías:
https://github.com/RustLangES/rustlings/blob/main/CONTRIBUTING.md";

const WELCOME: &str = r"       Bienvenid@ a...
                 _   _ _
  _ __ _   _ ___| |_| (_)_ __   __ _ ___
 | '__| | | / __| __| | | '_ \ / _` / __|
 | |  | |_| \__ \ |_| | | | | | (_| \__ \
 |_|   \__,_|___/\__|_|_|_| |_|\__, |___/
                               |___/";
