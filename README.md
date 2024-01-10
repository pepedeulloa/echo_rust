# echo_rust

Este repositorio contiene un programa en Rust que emula el comportamiento del comando `echo` de Unix. Utilizando la biblioteca `clap`, proporciona la capacidad de gestionar argumentos de línea de comandos y controlar la adición de saltos de línea entre los argumentos de salida.

## Funcionalidades

- Emulación del comando `echo` de Unix en Rust.
- Control de la adición de saltos de línea entre los argumentos de salida.

## Estructura del Código

### Dependencias

- `core::fmt`: Módulo estándar de formateo en Rust.
- `clap::Parser`: Atributo de `clap` para simplificar la creación de un parser de línea de comandos.

### `Cli` Struct

Estructura con campos que definen los comandos correspondiente a la adición de saltos de línea y almacenar una lista de cadenas de texto.

### `impl fmt::Display for Cli`

Implementa el trait `Display` para personalizar el formato de salida al imprimir el objeto.

### `main` Function

Función principal que instancia y parsea la estructura `Cli`. Se imprime el resultado, con la opción de agregar saltos de línea según la configuración.

## Instrucciones de Uso

1. Asegúrate de tener Rust instalado.
2. Clona el repositorio: `git clone [URL]`.
3. Navega al directorio del repositorio: `cd [nombre del directorio]`.
4. Ejecuta `cargo run -- [opciones]` para compilar y ejecutar el programa.

## Ejemplos de Uso

- Ejecutar sin opciones:
  ```bash
  cargo run
  ```

- Especificar texto de salida:
  ```bash
  cargo run -- Hello World
  ```

- Activar la opción de salto de línea:
  ```bash
  cargo run -- -n Hello World
  ```

## Notas

Calquier contribucion es bienvenidas. Reporta problemas o envía solicitudes de extracción para mejorar el código. ¡Gracias por contribuir!
