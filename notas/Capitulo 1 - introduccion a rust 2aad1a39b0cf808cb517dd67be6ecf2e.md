# Capitulo 1 - introduccion a rust

# Como ejecutar codigo de rust

para escribir codigo en rust necesitas crear un archivo con la extension `.rs` y para ejecutar  ese archivo utilizas los siguientes comando:

supongamos que escribes tu codigo en un archivo llamado **main.rs:**

1. primero te ubicas en la ruta del archivo y ejecutas el siguiente comando: `rustc [main.rs](http://main.rs)` . que hace este comando
    - utilizar el compilador de rust para leer el archivo y traducirlo a codigo binario
    - crear un nuevo archivo con el codigo binario llamado igual que el otro archivo, este nuevo archivo es un ejecutable, por lo que en windows sera un .exe y en linux no tendra extencion, en este caso se llamara (estoy en linux): **main**
2. Por ultimo paso, ejecutar el archivo con: ./main (en linux) o en windows .\main

A diferencia de rust otros lenguajes dinamicos como javascript o python no separan la compilacion y la ejecucion en 2 pasos

## anatomia de un programa en rust

```rust
fn main() {}
```

La funcion main es especial ya que es el primer bloque de codigo que corre cada programa ejecutable de rust

# Cargo

Es un sistema de compilacion y gestor de paquetes de rust. Es como el pnpm o npm de rust, al igual que estos crea un archivo para manejar las dependencias y configuraciones del proyecto, en este caso se llama: Cargo.toml.

## estructura de un proyecto de cargo

los proyectos de cargos se crean con `cargo new nombre_proyecto` 

```rust
/nombre_proyecto
Cargo.toml
	/.git
	/src
		main.rs
```

el comando para crear un nuevo proyecto, tambien inicializa un nuevo repositorio de git, excepto si se ejecuta dentro de un repositorio ya existente, aunque se puede inicializar de todas formas usando cargo new —vsc=git

cargo init: crea el archivo Cargo.toml

## Compilando y corriendo un proyecto de cargo

`cargo build`: se ejecuta en la carpeta raiz del proyecto, este cra una carpeta llamada target con archivo y una carpeta (debug), donde se encuentra el ejecutable

`cargo run`: podemos usarla para compilar el codigo y enseguida ejecutarlo

`cargo check`: se ocupa de asegurarse de que el codigo compile, pero no crea el ejecutable

### compilar para release

cuando ya el codigo esta listo para produccion puedes utilizar el siguiente comando: `cargo build --release`

### otros comandos de cargo

cargo update: hace una actualizacion de todas las librerias

cargo doc —open : genera una documentacion local del proyecto, incluyendo documentacion de las libreria utilizadas, por lo que seria interesante usarlo mientras se contruyen cosas para saber como funcionan ciertas librerias, aunque ya se pueda encontrar al documentacion de estas en internet

# Input

# Variables

Para crear variables en rust se utiliza la palabra reservada `let` . Podriamos dividir estas variables en 2 tipos las mutables y las inmutables, en rust las variables son inmutables por defecto y para volverlas inmutables debes utilizar la palabra `mut`, Ejemplo:

```rust
let number: number = 15 //inmutable
let mut number_mutable: number = 10 //mutable
```

## Notas:

- Rust utiliza ; al final de cada linea