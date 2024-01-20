# echo_rust

**echo_rust** é unha ferramenta escrita en Rust que emula o comportamento do comando `echo` en sistemas Unix. Proporciona unha maneira sinxela de imprimir texto na saída estándar, coa opción adicional de controlar o formato de saída.

## Dependencias

Este programa foi desenvolvido utilizando a versión 1.74.0 de Rust. A xestión de argumentos da liña de comandos simplificouse grazas ao uso do crate [clap](https://crates.io/crates/clap) na súa versión 4.4.11. `clap` ofrece unha interfaz declarativa para definir e analizar os argumentos do programa, facilitando a creación dunha experiencia de usuario amigable e eficiente.

## Instalación

1. Clona o repositorio: `git clone https://github.com/pepedeulloa/echo_rust.git`
2. Navega ao directorio do proxecto: `cd echo_rust`
3. Compila o proxecto: `cargo build --release`

## Opcións

-n, --newline: Non engadir os argumentos nunha nova liña.
-h, --help: Mostra a axuda do programa.
-V, --version: Mostra a version do programa.

## Uso

```bash
# Exemplo de como executar o programa
cd target/release
./echo-rust --newline "Ola" "Mundo"
```
