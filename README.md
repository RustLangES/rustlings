<div class="oranda-hide">

# rustlings 🦀❤️

</div>

¡Saludos y bienvenidos a Rustlings! Este proyecto contiene pequeños ejercicios para que te acostumbres a leer y escribir código en Rust. ¡Esto incluye la lectura y respuesta a los mensajes del compilador!

_...¿Buscas la versión antigua basada en la web de Rustlings? Prueba [aquí](https://github.com/rust-lang/rustlings/tree/rustlings-1) (Sin traducir)_

Alternativamente, para aquellos que están aprendiendo Rust por primera vez, existen varios otros recursos:

- [El Libro](https://rustlanges.github.io/rust-book-es/index.html) - El recurso más completo para aprender Rust, aunque a veces es un poco teórico. ¡Usarás esto junto con Rustlings!
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - ¡Aprende Rust resolviendo pequeños ejercicios! Es casi como rustlings, pero en línea (Sin traducir).

## Empecemos

_Nota: Si estás en MacOS, asegúrate de haber instalado Xcode y sus herramientas de desarrollo escribiendo `xcode-select --install`._
_Nota: Si estás en Linux, asegúrate de haber instalado gcc. Deb: `sudo apt install gcc`. Yum: `sudo yum -y install gcc`._

Deberás tener Rust instalado. Puedes obtenerlo visitando <https://rustup.rs>. Esto también instalará Cargo, el gestor de paquetes/proyectos de Rust.

## MacOS/Linux

Simplemente ejecuta:

```bash
curl -L https://raw.githubusercontent.com/RustLangES/rustlings/main/install.sh | bash
```

O si deseas que se instale en una ubicación diferente:

```bash
curl -L https://raw.githubusercontent.com/RustLangES/rustlings/main/install.sh | bash -s mi_ruta/
```

Esto instalará Rustlings y te dará acceso al comando `rustlings`. ¡Ejecútalo para comenzar!

### Nix

Básicamente: Clona el repositorio en la última etiqueta (tag), y finalmente ejecuta `nix develop` o `nix-shell`.

```bash
# Descubre la versión más reciente en https://github.com/RustLangES/rustlings/releases/latest (en la edición 5.6.1)
git clone -b 5.6.1 --depth 1 https://github.com/RustLangES/rustlings
cd rustlings
# si la version de nix > 2.3
nix develop
# si la version de nix <= 2.3
nix-shell
```

## Windows

En PowerShell (Ejecutar como administrador), establece `ExecutionPolicy` en `RemoteSigned`:

```ps1
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Luego, puedes ejecutar:

```ps1
Start-BitsTransfer -Source https://raw.githubusercontent.com/RustLangES/rustlings/main/install.ps1 -Destination $env:TMP/install_rustlings.ps1; Unblock-File $env:TMP/install_rustlings.ps1; Invoke-Expression $env:TMP/install_rustlings.ps1
```

Para instalar Rustlings. Igual que en MacOS/Linux, tendrás acceso al comando `rustlings` después de hacerlo. Ten en cuenta que esto funciona mejor en PowerShell, y otros terminales pueden darte errores.

Si recibes un mensaje de 'permiso denegado', es posible que debas excluir el directorio donde clonaste Rustlings en tu antivirus.

## Desde el Navegador

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/RustLangES/rustlings)

[![Open Rustlings On Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new/?repo=RustLangES%2Frustlings&ref=main)

## Manualmente

Básicamente: Clona el repositorio en la última etiqueta y luego ejecuta  `cargo install --path .`.

```bash
# Descubre la versión más reciente en https://github.com/RustLangEs/rustlings/releases/latest (en la edición 5.6.1)
git clone -b 5.6.1 --depth 1 https://github.com/RustLangEs/rustlings
cd rustlings
cargo install --force --path .
```

Si hay errores de instalación, asegúrate de que tu cadena de herramientas esté actualizada. Para obtener la última versión, ejecuta:

```bash
rustup update
```

Luego, como se mencionó anteriormente, ejecuta `rustlings` para comenzar.

## Realizando ejercicios

Los ejercicios están ordenados por tema y se pueden encontrar en el subdirectorio `rustlings/exercises/<tema>`. Para cada tema, hay un archivo README adicional con algunos recursos para ayudarte a empezar en ese tema. Realmente recomendamos que los consultes antes de comenzar.

La tarea es simple. La mayoría de los ejercicios contienen un error que impide que se compilen, ¡y depende de ti corregirlo! Algunos ejercicios también se ejecutan como pruebas, pero rustlings los maneja de la misma manera. Para ejecutar los ejercicios en el orden recomendado, ejecuta:

```bash
rustlings watch
```

Esto intentará verificar la finalización de cada ejercicio en un orden preestablecido (lo que creemos que es lo mejor para los recién llegados). También volverá a ejecutarse automáticamente cada vez que cambies un archivo en el directorio `exercises/`. Si solo deseas ejecutarlo una vez, puedes usar:

```bash
rustlings verify
```

Esto hará lo mismo que watch, pero se cerrará después de ejecutar.

En caso de que desees seguir tu propio orden o solo quieras verificar un solo ejercicio, puedes ejecutar:

```bash
rustlings run miEjercicio1
```

Or simply use the following command to run the next unsolved exercise in the course:

```bash
rustlings run next
```

En caso de que te quedes atascado, puedes ejecutar el siguiente comando para obtener una pista para tu ejercicio:

```bash
rustlings hint miEjercicio1
```

También puedes obtener una pista para el próximo ejercicio no resuelto con el siguiente comando:

```bash
rustlings hint next
```

Para verificar tu progreso, puedes ejecutar el siguiente comando:

```bash
rustlings list
```

## Probándote a ti mismo

Después de cada par de secciones, habrá una prueba que pondrá a prueba tu conocimiento en varias secciones a la vez. Estas pruebas se encuentran en `exercises/quizN.rs`.

## Habilitando `rust-analyzer`

Ejecuta el comando `rustlings lsp`, el cual generará un `rust-project.json` en la raíz del proyecto. Esto permite que [rust-analyzer](https://rust-analyzer.github.io/) analice cada ejercicio.

## Continuando

Una vez que hayas completado Rustlings, ¡pon en práctica tus nuevos conocimientos! Continúa practicando tus habilidades en Rust construyendo tus propios proyectos, contribuyendo a Rustlings o encontrando otros proyectos de código abierto a los que puedas contribuir.

## Desinstalando Rustlings

Si deseas eliminar Rustlings de tu sistema, hay dos pasos. Primero, deberás eliminar la carpeta de ejercicios que el script de instalación creó para ti:

```bash
rm -rf rustlings # o el nombre de carpeta personalizado que hayas elegido o renombrado, si lo hiciste.
```

Segundo, ejecuta `cargo uninstall` para eliminar el binario `rustlings`:

```bash
cargo uninstall rustlings
```

¡Ahora deberías haber terminado!

## Contribuir

Consulta [CONTRIBUTING.md](https://github.com/RustLangEs/rustlings/blob/main/CONTRIBUTING.md).

## Colaboradores ✨

Agradecemos a las maravillosas personas que se encuentran en[AUTHORS.md](https://github.com/RustLangEs/rustlings/blob/main/AUTHORS.md) 🎉
