## Contribuir a Rustlings

En primer lugar, ¡gracias por tomarte el tiempo para contribuir! ❤️

### Referencia Rápida

Quiero...

_agregar un ejercicio! ➡️ [lee esto](#addex) y luego [abre una Pull Request](#prs)_

_actualizar un ejercicio obsoleto! ➡️ [abre una Pull Request](#prs)_

_reportar un error! ➡️ [abre un problema (Issue)](#issues)_

_corregir un error! ➡️ [abre una Pull Request](#prs)_

_implementar una nueva característica! ➡️ [abre un problema para discutirlo primero, luego una Pull Request](#issues)_

<a name="#src"></a>
### Trabajando en el código fuente

`rustlings` es básicamente un envoltorio glorificado de `rustc`. Por lo tanto, el código fuente no es realmente complicado, ya que la mayor parte del trabajo la realiza `rustc`. `src/main.rs` contiene una simple CLI de `argh` que se conecta a la mayoría de los otros archivos fuente.

<a name="addex"></a>
### Agregando un ejercicio

¡El primer paso es agregar el ejercicio! Nombra el archivo `exercises/yourTopic/yourTopicN.rs`, asegúrate de
poner algunos enlaces útiles y vincula a secciones del libro en `exercises/yourTopic/README.md`.

Luego asegúrate de que se ejecute con `rustlings`. Los metadatos del ejercicio se almacenan en `info.toml`, bajo el array `exercises`. El orden del array `exercises` determina el orden en que se ejecutan los ejercicios con `rustlings verify` y `rustlings watch`.

Agrega los metadatos de tu ejercicio en el orden correcto en el array `exercises`. Si no estás seguro del orden correcto, agrégalo al final y consulta en tu Pull Request. Los metadatos del ejercicio deben contener lo siguiente:

```diff
  ...
+ [[exercises]]
+ name = "tuTemaN"
+ path = "exercises/tuTema/tuTemaN.rs"
+ mode = "compile"
+ hint = """
+ Alguna pista útil para tu ejercicio."""
  ...
```

El atributo `mode` decide si Rustlings solo compilará tu ejercicio o lo compilará y lo probará. Si tienes pruebas para verificar en tu ejercicio, elige `test`; de lo contrario, elige `compile`. Si estás trabajando en un ejercicio de Clippy, usa `mode = "clippy"`.

¡Eso es todo! Siéntete libre de presentar una Pull Request.

<a name="issues"></a>
### Problemas (Issues)

Puedes abrir un problema [aquí](https://github.com/RustLangEs/rustlings/issues/new).
Si estás reportando un error, por favor incluye la salida de los siguientes comandos:

- `rustc --version`
- `rustlings --version`
- `ls -la`
- El nombre y la versión de tu sistema operativo

<a name="prs"></a>
### Pull Requests

Abrir una Pull Request es tan fácil como hacer un fork del repositorio y comprometer tus
cambios. Hay algunas cosas a tener en cuenta:

#### Escribe mensajes de commit correctos

Seguimos la especificación de [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0-beta.4/).
Esto significa que debes formatear tus mensajes de commit de una manera específica. Supongamos
que estás trabajando en agregar un nuevo ejercicio llamado `foobar1.rs`. Podrías escribir
el siguiente mensaje de commit:

```
feat: agrega ejercicio foobar1.rs
```

Si solo estás corrigiendo un error, por favor utiliza el tipo `fix`:

```
fix(verify): asegura de que verify no se autodestruya
```

El scope dentro de los paréntesis es opcional, pero debería ser cualquiera de estos:

- `installation` (para el script de instalación)
- `cli` (para cambios generales en la CLI)
- `verify` (para el archivo fuente de verificación)
- `watch` (para la fuente de la funcionalidad de observación)
- `run` (para la fuente de la funcionalidad de ejecución)
- `NOMBREDELEJERCICIO` (si estás cambiando un ejercicio específico o un conjunto de ejercicios,
  sustitúyelos aquí)

Cuando el commit también cierra un problema existente, enlázalo en el cuerpo del mensaje:

```
fix: actualiza foobar

cierra #101029908
```

Si estás haciendo cambios simples, como actualizar un enlace del libro, usa `chore`:

```
chore: actualiza el enlace del libro en exercise1.rs
```

Si estás actualizando la documentación, usa `docs`:

```
docs: agrega más información a Readme
```

Si, y solo si, estás absolutamente seguro de que deseas realizar un cambio que rompa algo (¡discútelo antes!), agrega un signo de exclamación al tipo y explica el cambio rompedor en el cuerpo del mensaje:

```
fix!: cambia completamente la verificación

BREAKING CHANGE: Esto debe hacerse porque lorem ipsum dolor
```

#### Flujo de Pull Request

Una vez que abras una Pull Request, puede ser revisada o etiquetada (o ambas) hasta que
los mantainers acepten tus cambios. ¡Por favor ten paciencia, esto puede llevar algún tiempo!