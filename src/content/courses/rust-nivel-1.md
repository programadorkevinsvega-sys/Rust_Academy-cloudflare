---
title: "Rust — Nivel 1: Fundamentos del lenguaje"
description: "Aprende los fundamentos de Rust: variables, tipos, funciones, control de flujo y estructuras básicas, con ejercicios y un proyecto práctico."
language: rust
level: 1
levelLabel: "Nivel 01"
category: "Fundamentos"
duration: "Principiante"
order: 1
slug: rust-nivel-1
tags: ["variables", "tipos", "funciones", "control de flujo", "arrays", "tuplas"]
---

# 🦀 Curso de Rust — Nivel 1: Fundamentos del lenguaje

> **Objetivo:** que al terminar este nivel tus estudiantes puedan leer y escribir programas pequeños en Rust usando variables, tipos, control de flujo, funciones, tuplas y arrays, y que entiendan **por qué** el compilador les dice "no" cuando se lo dice.
>
> **Requisitos:** saber qué es una variable y una función en cualquier lenguaje. No se necesita experiencia previa en Rust.
>
> **Versión recomendada:** Rust 1.70 o superior, edición 2021 (todo el código de este curso funciona con la última estable).

---

## Índice

0. [Preparación del entorno](#0-preparación-del-entorno)
1. [Variables: `let`, `mut` y constantes](#módulo-1-variables-let-mut-y-constantes)
2. [Tipos de datos: `i32`, `u64`, `f64`, `bool`, `char` y `String`](#módulo-2-tipos-de-datos)
3. [Operadores y conversiones de tipos](#módulo-3-operadores-y-conversiones-de-tipos)
4. [Condicionales: `if`, `else` y `match`](#módulo-4-condicionales-if-else-y-match)
5. [Bucles: `loop`, `while` y `for`](#módulo-5-bucles-loop-while-y-for)
6. [Funciones, parámetros y valores de retorno](#módulo-6-funciones-parámetros-y-valores-de-retorno)
7. [Ámbitos de variables (scope)](#módulo-7-ámbitos-de-variables-scope)
8. [Tuplas y arrays](#módulo-8-tuplas-y-arrays)
9. [Proyecto integrador: reporte de notas](#proyecto-integrador-reporte-de-notas)
10. [Ejercicios con soluciones](#ejercicios-con-soluciones)
11. [Hoja de referencia rápida](#hoja-de-referencia-rápida)
12. [Errores frecuentes del compilador](#errores-frecuentes-del-compilador)
13. [Plan de clases sugerido](#plan-de-clases-sugerido)

**Leyenda usada en todo el curso:**

- ✅ = se puede / es correcto
- ❌ = NO se puede / no compila
- ⚠️ = compila, pero cuidado (puede fallar en ejecución o dar resultados inesperados)

---

## 0. Preparación del entorno

### Opción A: sin instalar nada
Usa el **Rust Playground** en el navegador: <https://play.rust-lang.org>. Pega el código y presiona *Run*.

### Opción B: instalación local
1. Instala Rust con `rustup` desde <https://rustup.rs>.
2. Verifica la instalación:

```bash
rustc --version
cargo --version
```

3. Crea tu primer proyecto con **Cargo** (el gestor de proyectos de Rust):

```bash
cargo new curso_rust
cd curso_rust
cargo run
```

Esto crea `src/main.rs` con un "Hola, mundo". Todo el código de este curso se escribe dentro de `src/main.rs`.

### Anatomía del primer programa

```rust
fn main() {
    println!("¡Hola, Rust!");
}
```

- `fn main()` es la **función de entrada**: el programa empieza aquí.
- `println!` termina en `!` porque es una **macro**, no una función normal. Imprime y agrega salto de línea.
- Cada sentencia termina con `;`.

### Imprimir en pantalla (lo usaremos todo el curso)

```rust
fn main() {
    let nombre = "Ana";
    let edad = 20;
    let altura = 1.6543;

    println!("Hola");                        // texto simple
    print!("Sin salto de línea. ");          // print! no agrega salto
    println!();                              // línea vacía

    println!("Me llamo {}", nombre);         // {} = Display
    println!("Me llamo {nombre}");           // ✅ variable dentro de las llaves
    println!("{} tiene {} años", nombre, edad);
    println!("{0} {1} {0}", "a", "b");       // posiciones: a b a

    println!("{:.2}", altura);               // 2 decimales -> 1.65
    println!("{:>8}", edad);                 // alineado a la derecha, ancho 8
    println!("{:<8}|", edad);                // alineado a la izquierda
    println!("{:05}", edad);                 // relleno con ceros -> 00020
    println!("{:b}", edad);                  // binario -> 10100
    println!("{:x}", 255);                   // hexadecimal -> ff

    println!("{:?}", nombre);                // {:?} = Debug -> "Ana" (con comillas)
    println!("{:?}", (1, 2, 3));             // útil para tuplas y arrays
}
```

⚠️ Dentro de `{}` solo puedes poner **nombres de variables**, no expresiones:

```rust
let a = 2;
let b = 3;
// println!("{a + b}");      // ❌ error: no se permiten expresiones
println!("{}", a + b);       // ✅
```

### Comentarios

```rust
// Comentario de una línea

/* Comentario
   de varias líneas */

/// Comentario de documentación (se usa antes de funciones; lo verás en niveles siguientes)
```

---

# Módulo 1: Variables: `let`, `mut` y constantes

## 1.1 Variables inmutables con `let`

En Rust, **por defecto, las variables NO se pueden modificar**. Esto no es una limitación: es una decisión de diseño para evitar errores.

```rust
fn main() {
    let x = 5;
    println!("x vale {x}");

    // x = 6;   // ❌ ERROR: cannot assign twice to immutable variable `x`
}
```

El mensaje del compilador (error `E0384`) incluso sugiere la solución: *"consider making this binding mutable: `mut x`"*.

## 1.2 Variables mutables con `mut`

Si necesitas cambiar el valor, lo declaras explícitamente con `mut`:

```rust
fn main() {
    let mut contador = 0;
    println!("Inicio: {contador}");

    contador = 10;       // ✅ reasignar
    contador += 1;       // ✅ operadores compuestos
    println!("Ahora: {contador}");   // 11
}
```

**Regla de oro:** usa `let` normal siempre y solo agrega `mut` cuando el compilador te lo pida o realmente necesites cambiar el valor.

### ❌ Lo que `mut` NO permite: cambiar el tipo

El **valor** puede cambiar, pero el **tipo** de la variable se queda fijo:

```rust
fn main() {
    let mut dato = 5;     // dato es i32
    dato = 7;             // ✅ mismo tipo
    // dato = "hola";     // ❌ ERROR E0308: mismatched types (se esperaba entero)
}
```

## 1.3 Inferencia de tipos y anotaciones

Rust deduce el tipo a partir del valor. También puedes escribirlo tú con `: tipo`.

```rust
fn main() {
    let a = 10;            // inferido: i32
    let b: i64 = 10;       // anotado explícitamente
    let c = 3.14;          // inferido: f64
    let d: f32 = 3.14;     // anotado
    let e = true;          // bool
    let f = 'R';           // char
    let g = "texto";       // &str

    println!("{a} {b} {c} {d} {e} {f} {g}");
}
```

Cuándo **debes** anotar el tipo: cuando Rust no puede adivinarlo.

```rust
fn main() {
    // let numero = "42".parse().unwrap();     // ❌ error: ¿parsear a qué tipo?
    let numero: i32 = "42".parse().unwrap();   // ✅
    let otro = "42".parse::<i32>().unwrap();   // ✅ (sintaxis "turbofish")
    println!("{numero} {otro}");
}
```

## 1.4 Declarar ahora, asignar después

Puedes declarar una variable sin valor y asignarla **una sola vez** más adelante (el compilador verifica que se inicialice antes de usarla):

```rust
fn main() {
    let mensaje;                 // declarada, sin valor todavía
    let hace_calor = true;

    if hace_calor {
        mensaje = "Toma agua";
    } else {
        mensaje = "Ponte una chaqueta";
    }
    println!("{mensaje}");       // ✅ en todos los caminos se asignó
}
```

```rust
fn main() {
    let x: i32;
    // println!("{x}");   // ❌ ERROR E0381: usado sin inicializar
    x = 5;
    println!("{x}");      // ✅
}
```

## 1.5 Shadowing (sombreado)

Puedes volver a declarar una variable con el **mismo nombre** usando `let`. La nueva "tapa" a la anterior.

```rust
fn main() {
    let x = 5;
    let x = x + 1;        // nueva variable x = 6
    let x = x * 2;        // nueva variable x = 12
    println!("x = {x}");  // 12
}
```

**Diferencia clave con `mut`:** el shadowing **sí permite cambiar el tipo**.

```rust
fn main() {
    let espacios = "     ";           // &str
    let espacios = espacios.len();    // ✅ ahora es usize (5)
    println!("{espacios}");

    // Con mut NO se puede:
    // let mut e = "     ";
    // e = e.len();                   // ❌ ERROR E0308
}
```

| | `mut` | shadowing (`let` otra vez) |
|---|---|---|
| Cambia el valor | ✅ | ✅ |
| Cambia el tipo | ❌ | ✅ |
| Crea una variable nueva | No | Sí |
| La variable queda inmutable después | No (sigue `mut`) | Sí (si no escribes `mut`) |

## 1.6 Constantes con `const`

Las constantes son valores que **nunca** cambian y se conocen **al compilar**.

```rust
const MAX_INTENTOS: u32 = 3;
const SEGUNDOS_POR_DIA: u32 = 60 * 60 * 24;    // ✅ expresión calculable al compilar
const PI_APROX: f64 = 3.14159;
const SALUDO: &str = "Bienvenido";

fn main() {
    println!("{MAX_INTENTOS} {SEGUNDOS_POR_DIA} {PI_APROX} {SALUDO}");
}
```

Reglas de `const`:

- ✅ El **tipo es obligatorio**: `const X: u32 = 5;`
- ✅ Nombre en `MAYUSCULAS_CON_GUION_BAJO` (convención).
- ✅ Se puede declarar **fuera de `main`** (global) o dentro de una función.
- ❌ **No** puede ser `mut`: `const mut X` no existe.
- ❌ **No** puede asignarse con un valor calculado en ejecución:

```rust
fn main() {
    // let entrada = "5";
    // const N: u32 = entrada.parse().unwrap();   // ❌ no se conoce al compilar
}
```

### `let` vs `const`

| | `let` | `const` |
|---|---|---|
| Mutable con `mut` | ✅ | ❌ nunca |
| Tipo obligatorio | No (se infiere) | **Sí** |
| Valor | Puede calcularse en ejecución | Debe conocerse al compilar |
| Dónde se declara | Solo dentro de funciones | En cualquier lugar (incluso global) |
| Convención de nombre | `snake_case` | `MAYUSCULAS_SNAKE` |

## 1.7 Variables no usadas

El compilador avisa (warning) si declaras algo que no usas. Para silenciarlo intencionalmente, empieza el nombre con `_`:

```rust
fn main() {
    let _no_la_uso = 42;    // ✅ sin advertencia
    let _ = 10;             // ✅ descarta el valor por completo
}
```

## 1.8 Nombres de variables

- Se escriben en **`snake_case`**: `edad_usuario`, `total_a_pagar`.
- Pueden tener letras, dígitos y `_`, pero no empezar con dígito.
- No pueden ser palabras reservadas (`let`, `fn`, `match`, `loop`...).

```rust
fn main() {
    let edad_usuario = 25;       // ✅
    let _temp = 1;               // ✅
    // let 2cosas = 5;           // ❌ no puede empezar con número
    // let fn = 5;               // ❌ palabra reservada
    // let edadUsuario = 25;     // ⚠️ compila, pero da warning (no es snake_case)
    println!("{edad_usuario}");
}
```

---

# Módulo 2: Tipos de datos

Rust es **estáticamente tipado**: cada variable tiene un tipo conocido al compilar. Los tipos de este módulo son los **escalares** (`i32`, `u64`, `f64`, `bool`, `char`) más `String`.

## 2.1 Enteros

Los enteros se nombran con una letra y los **bits** que ocupan:

- `i` = *integer con signo* (admite negativos): `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- `u` = *unsigned, sin signo* (solo ≥ 0): `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

| Tipo | Rango aproximado |
|---|---|
| `i8` | -128 a 127 |
| `u8` | 0 a 255 |
| `i16` | -32 768 a 32 767 |
| `u16` | 0 a 65 535 |
| **`i32`** (por defecto) | -2 147 483 648 a 2 147 483 647 |
| `u32` | 0 a 4 294 967 295 |
| `i64` | ≈ ±9,22 × 10¹⁸ |
| **`u64`** | 0 a 18 446 744 073 709 551 615 |
| `isize` / `usize` | depende de la arquitectura (64 bits en PCs modernos) |

```rust
fn main() {
    let a = 42;               // i32 por defecto
    let b: u64 = 5_000_000_000;   // ✅ no cabe en i32, sí en u64
    let c: u8 = 255;
    let d: i8 = -128;

    println!("{a} {b} {c} {d}");
    println!("Máximo i32: {}", i32::MAX);
    println!("Mínimo i32: {}", i32::MIN);
    println!("Máximo u8: {}", u8::MAX);
    println!("Máximo u64: {}", u64::MAX);
}
```

**¿Cuál elegir?**

- **`i32`** → opción por defecto para enteros.
- **`u64`** / **`i64`** → números grandes (factoriales, dinero en centavos, timestamps).
- **`usize`** → índices y tamaños (los arrays se indexan **solo** con `usize`).
- **`u8`** → bytes, valores 0-255.

### Formas de escribir literales numéricos

```rust
fn main() {
    let decimal = 98_222;        // el _ es solo visual
    let hex = 0xff;              // 255
    let octal = 0o77;            // 63
    let binario = 0b1111_0000;   // 240
    let byte = b'A';             // 65 (u8)
    let con_sufijo = 57u8;       // tipo en el literal
    let otro_sufijo = 1_000_i64;

    println!("{decimal} {hex} {octal} {binario} {byte} {con_sufijo} {otro_sufijo}");
}
```

### ❌ Qué no se puede con enteros

```rust
fn main() {
    // let x: u32 = -5;        // ❌ un tipo sin signo no admite negativos
    // let y: u8 = 256;        // ❌ fuera de rango (error en compilación)
    // let a: i32 = 5;
    // let b: i64 = 10;
    // let c = a + b;          // ❌ no se mezclan tipos distintos (ver Módulo 3)
}
```

### ⚠️ Desbordamiento (overflow)

Si una operación se pasa del rango del tipo:

- En modo **debug** (`cargo run`): el programa **hace panic** (se detiene con error).
- En modo **release** (`cargo run --release`): el valor "da la vuelta" silenciosamente.

Rust te da herramientas para decidir **explícitamente** qué hacer:

```rust
fn main() {
    let a: u8 = 255;

    println!("{:?}", a.checked_add(1));      // None  -> avisa que hubo overflow
    println!("{}", a.wrapping_add(1));       // 0     -> da la vuelta
    println!("{}", a.saturating_add(1));     // 255   -> se queda en el máximo
    println!("{:?}", a.overflowing_add(1));  // (0, true) -> valor y bandera
}
```

## 2.2 Números de punto flotante: `f64` y `f32`

```rust
fn main() {
    let x = 2.5;          // f64 por defecto
    let y: f32 = 1.5;     // menos precisión, la mitad de memoria
    let z = 1e3;          // 1000.0 (notación científica)
    let w = 5.0_f64;      // sufijo de tipo

    println!("{x} {y} {z} {w}");
}
```

Usa **`f64`** salvo que tengas una razón concreta para `f32`.

Operaciones comunes:

```rust
fn main() {
    let n = 9.0_f64;
    println!("{}", n.sqrt());          // 3     raíz cuadrada
    println!("{}", n.powi(2));         // 81    potencia entera
    println!("{}", n.powf(0.5));       // 3     potencia decimal
    println!("{}", 3.7_f64.floor());   // 3     hacia abajo
    println!("{}", 3.2_f64.ceil());    // 4     hacia arriba
    println!("{}", 3.5_f64.round());   // 4     redondear
    println!("{}", (-4.2_f64).abs());  // 4.2   valor absoluto
    println!("{}", 3.0_f64.max(7.0));  // 7
    println!("{}", 3.0_f64.min(7.0));  // 3
}
```

### ⚠️ Cuidados con los decimales

```rust
fn main() {
    // 1) Los flotantes son aproximados:
    println!("{}", 0.1 + 0.2);                       // 0.30000000000000004
    println!("{}", 0.1 + 0.2 == 0.3);                // false  ⚠️

    // Para comparar, usa una tolerancia:
    let iguales = (0.1_f64 + 0.2 - 0.3).abs() < 1e-10;
    println!("{iguales}");                           // true

    // 2) Dividir un flotante entre cero NO hace panic:
    println!("{}", 1.0 / 0.0);                       // inf
    println!("{}", 0.0_f64 / 0.0);                   // NaN (no es un número)

    // 3) Un entero entre cero SÍ hace panic:
    // let cero = 0;
    // println!("{}", 10 / cero);                    // ❌ panic: divide by zero
}
```

Y recuerda: **`5` y `5.0` son tipos distintos**.

```rust
fn main() {
    let a: f64 = 5.0;      // ✅
    // let b: f64 = 5;     // ❌ ERROR: un entero no es un flotante
    println!("{a}");
}
```

## 2.3 Booleanos: `bool`

Solo tiene dos valores: `true` y `false`.

```rust
fn main() {
    let activo = true;
    let es_mayor: bool = 18 > 21;      // false

    println!("{activo} {es_mayor}");
    println!("{}", !activo);           // false (negación)
}
```

❌ **No existe "verdadero/falso" para otros tipos** (como en JavaScript o Python):

```rust
fn main() {
    // if 1 { }              // ❌ el 1 no es bool
    // if "hola" { }         // ❌ un texto no es bool
    // if 0 == 0 { }         // ✅ esto sí: la comparación produce un bool
}
```

## 2.4 Caracteres: `char`

Un `char` es **un solo carácter Unicode**, se escribe con **comillas simples** y ocupa **4 bytes**.

```rust
fn main() {
    let letra = 'a';
    let enie = 'ñ';
    let corazon = '❤';
    let emoji = '😀';
    let chino = '中';
    let digito = '7';        // el CARÁCTER '7', no el número 7

    println!("{letra} {enie} {corazon} {emoji} {chino} {digito}");
}
```

Comillas simples vs. dobles:

```rust
fn main() {
    let c = 'A';       // ✅ char (un carácter)
    let s = "A";       // ✅ &str (texto, aunque sea de 1 carácter)
    // let malo = 'AB';   // ❌ un char admite UN solo carácter
    println!("{c} {s}");
}
```

Métodos útiles de `char`:

```rust
fn main() {
    let c = 'g';
    println!("{}", c.is_alphabetic());        // true
    println!("{}", '5'.is_numeric());         // true
    println!("{}", 'G'.is_uppercase());       // true
    println!("{}", c.is_whitespace());        // false
    println!("{}", c.to_ascii_uppercase());   // G
    println!("{}", 'Z'.to_ascii_lowercase()); // z
    println!("{:?}", '7'.to_digit(10));       // Some(7)  (Option, lo verás luego)
    println!("{}", 'ñ'.len_utf8());           // 2 bytes en UTF-8
}
```

## 2.5 Texto: `String` y `&str`

Rust tiene **dos** tipos principales de texto:

| | `&str` | `String` |
|---|---|---|
| Se escribe | `"hola"` | `String::from("hola")` |
| ¿Se puede modificar? | ❌ | ✅ (si es `mut`) |
| ¿Crece/decrece? | ❌ | ✅ |
| Dónde vive | Texto fijo dentro del programa | Memoria dinámica (heap) |
| Cuándo usarlo | Texto constante, parámetros de solo lectura | Texto que construyes o modificas |

```rust
fn main() {
    let literal: &str = "Hola";                      // texto fijo
    let mut propio: String = String::from("Hola");   // texto modificable

    propio.push_str(" mundo");    // agregar un texto
    propio.push('!');             // agregar un char
    println!("{literal}");        // Hola
    println!("{propio}");         // Hola mundo!

    // literal.push_str("x");     // ❌ &str no se puede modificar
}
```

Formas de crear un `String`:

```rust
fn main() {
    let a = String::from("uno");
    let b = "dos".to_string();
    let c: String = "tres".into();
    let d = String::new();            // vacío
    println!("[{a}] [{b}] [{c}] [{d}]");
}
```

### Métodos esenciales

```rust
fn main() {
    let mut s = String::from("  Hola Mundo  ");

    println!("{}", s.len());               // 14 -> BYTES, no caracteres
    println!("{}", s.is_empty());          // false
    println!("{}", s.trim());              // "Hola Mundo" (quita espacios)
    println!("{}", s.to_uppercase());      // "  HOLA MUNDO  "
    println!("{}", s.to_lowercase());      // "  hola mundo  "
    println!("{}", s.contains("Mundo"));   // true
    println!("{}", s.replace("Mundo", "Rust"));   // "  Hola Rust  "
    println!("{}", s.trim().starts_with("Hola")); // true
    println!("{}", s.trim().ends_with("do"));     // true

    s.clear();                             // lo deja vacío
    println!("[{s}]");                     // []

    // Recorrer palabras
    let frase = "uno dos tres";
    for palabra in frase.split_whitespace() {
        println!("{palabra}");
    }
}
```

### Concatenar texto

```rust
fn main() {
    let nombre = String::from("Ana");
    let apellido = String::from("Gómez");

    // Opción 1: format! (recomendada, no consume las variables)
    let completo = format!("{nombre} {apellido}");
    println!("{completo}");
    println!("{nombre}");                  // ✅ sigue disponible

    // Opción 2: operador +  (el de la izquierda DEBE ser String y se consume)
    let saludo = String::from("Hola, ") + &nombre + "!";
    println!("{saludo}");

    // ❌ No se pueden sumar dos &str:
    // let malo = "Hola" + " mundo";       // ERROR
    let bien = "Hola".to_string() + " mundo";   // ✅
    println!("{bien}");
}
```

### ⚠️ `len()` cuenta bytes y no se puede indexar con `[]`

Rust usa **UTF-8**: una letra puede ocupar 1, 2, 3 o 4 bytes.

```rust
fn main() {
    let palabra = String::from("ñandú");

    println!("{}", palabra.len());               // 7  -> bytes (ñ y ú ocupan 2 cada una)
    println!("{}", palabra.chars().count());     // 5  -> caracteres reales

    // println!("{}", palabra[0]);    // ❌ ERROR E0277: String no se puede indexar con enteros

    println!("{:?}", palabra.chars().next());    // Some('ñ')  primer carácter
    println!("{:?}", palabra.chars().nth(2));    // Some('n')  tercer carácter
    println!("{}", &palabra[0..2]);              // "ñ" -> rango en BYTES (0..2 es justo la ñ)
    // println!("{}", &palabra[0..1]);           // ❌ panic: corta la ñ por la mitad
}
```

Recorrer un `String`:

```rust
fn main() {
    let texto = "Hola";
    for c in texto.chars() {                 // carácter por carácter
        println!("{c}");
    }
    for (i, c) in texto.chars().enumerate() {   // con posición
        println!("{i}: {c}");
    }
}
```

## 2.6 Resumen de tamaños y valores por defecto

```rust
use std::mem::size_of;

fn main() {
    println!("i32  -> {} bytes", size_of::<i32>());    // 4
    println!("u64  -> {} bytes", size_of::<u64>());    // 8
    println!("f64  -> {} bytes", size_of::<f64>());    // 8
    println!("bool -> {} byte",  size_of::<bool>());   // 1
    println!("char -> {} bytes", size_of::<char>());   // 4
}
```

| Valor literal | Tipo por defecto |
|---|---|
| `42` | `i32` |
| `3.14` | `f64` |
| `true` | `bool` |
| `'x'` | `char` |
| `"hola"` | `&str` |

---

# Módulo 3: Operadores y conversiones de tipos

## 3.1 Operadores aritméticos

| Operador | Significado | Ejemplo | Resultado |
|---|---|---|---|
| `+` | suma | `7 + 2` | `9` |
| `-` | resta | `7 - 2` | `5` |
| `*` | multiplicación | `7 * 2` | `14` |
| `/` | división | `7 / 2` | `3` (¡entera!) |
| `%` | residuo (módulo) | `7 % 3` | `1` |

```rust
fn main() {
    println!("{}", 7 + 2);        // 9
    println!("{}", 7 - 2);        // 5
    println!("{}", 7 * 2);        // 14
    println!("{}", 7 / 2);        // 3    ⚠️ entre enteros, la división descarta decimales
    println!("{}", 7.0 / 2.0);    // 3.5  entre flotantes sí hay decimales
    println!("{}", 7 % 3);        // 1

    // Con negativos:
    println!("{}", -7 / 2);       // -3   (trunca hacia cero)
    println!("{}", -7 % 3);       // -1   (el residuo conserva el signo del dividendo)
    println!("{}", (-7_i32).rem_euclid(3));   // 2  (módulo "matemático", siempre ≥ 0)
}
```

### ❌ Qué NO se puede

```rust
fn main() {
    // 1) No existe ** para potencias:
    // let p = 2 ** 3;                 // ❌
    let p = 2_i32.pow(3);              // ✅ 8  (enteros)
    let q = 2.0_f64.powi(3);           // ✅ 8  (flotantes)
    println!("{p} {q}");

    // 2) No existen ++ ni --:
    let mut n = 5;
    // n++;                            // ❌
    n += 1;                            // ✅
    println!("{n}");

    // 3) No se mezclan tipos distintos:
    let entero: i32 = 5;
    let decimal: f64 = 2.5;
    // let r = entero * decimal;       // ❌ ERROR E0277: no se puede multiplicar i32 por f64
    let r = entero as f64 * decimal;   // ✅ convierte primero
    println!("{r}");                   // 12.5
}
```

## 3.2 Operadores de asignación compuesta

```rust
fn main() {
    let mut x = 10;
    x += 5;    // x = x + 5  -> 15
    x -= 3;    // x = x - 3  -> 12
    x *= 2;    // x = x * 2  -> 24
    x /= 4;    // x = x / 4  -> 6
    x %= 4;    // x = x % 4  -> 2
    println!("{x}");
}
```

## 3.3 Operadores de comparación

Todos devuelven `bool`. **Los dos lados deben ser del mismo tipo.**

| Operador | Significado |
|---|---|
| `==` | igual |
| `!=` | distinto |
| `<` `>` | menor / mayor |
| `<=` `>=` | menor o igual / mayor o igual |

```rust
fn main() {
    println!("{}", 5 == 5);              // true
    println!("{}", 5 != 3);              // true
    println!("{}", 3 < 5);               // true
    println!("{}", 'a' < 'b');           // true  (compara códigos Unicode)
    println!("{}", "hola" == "hola");    // true  (compara contenido)

    let nombre = String::from("Ana");
    println!("{}", nombre == "Ana");     // ✅ String vs &str se pueden comparar

    // println!("{}", 5 == 5.0);         // ❌ ERROR: i32 vs f64
    // println!("{}", 1 < 2 < 3);        // ❌ no se pueden encadenar comparaciones
    println!("{}", 1 < 2 && 2 < 3);      // ✅ así se encadena
}
```

## 3.4 Operadores lógicos

| Operador | Significado | Nota |
|---|---|---|
| `&&` | Y (AND) | Si el izquierdo es `false`, **no evalúa** el derecho |
| `\|\|` | O (OR) | Si el izquierdo es `true`, **no evalúa** el derecho |
| `!` | NO (NOT) | Invierte un `bool` |

```rust
fn main() {
    let edad = 25;
    let tiene_licencia = true;

    let puede_conducir = edad >= 18 && tiene_licencia;
    let es_extremo = edad < 12 || edad > 65;
    let no_es_menor = !(edad < 18);

    println!("{puede_conducir} {es_extremo} {no_es_menor}");   // true false true

    // ❌ No existen "and", "or", "not" como palabras:
    // let x = edad >= 18 and tiene_licencia;
}
```

Tabla de verdad rápida:

| a | b | `a && b` | `a \|\| b` |
|---|---|---|---|
| true | true | true | true |
| true | false | false | true |
| false | true | false | true |
| false | false | false | false |

## 3.5 Operadores a nivel de bits

Solo para enteros. Útiles para banderas, máscaras y protocolos.

```rust
fn main() {
    let a = 0b1100;   // 12
    let b = 0b1010;   // 10

    println!("{:04b}", a & b);   // 1000  AND
    println!("{:04b}", a | b);   // 1110  OR
    println!("{:04b}", a ^ b);   // 0110  XOR
    println!("{}", 1 << 4);      // 16    desplaza 4 bits a la izquierda (×16)
    println!("{}", 256 >> 2);    // 64    desplaza 2 bits a la derecha (÷4)
    println!("{}", !0b1111_0000_u8);   // 15  NOT bit a bit (en enteros se usa ! también)
}
```

## 3.6 Precedencia (de mayor a menor, resumido)

1. `!` y `-` (unarios)
2. `as` (conversión)
3. `*` `/` `%`
4. `+` `-`
5. `<<` `>>`
6. `&`
7. `^`
8. `|`
9. `==` `!=` `<` `>` `<=` `>=`
10. `&&`
11. `||`
12. `=` `+=` `-=`...

💡 **Consejo para clase:** ante la duda, usa paréntesis.

```rust
fn main() {
    println!("{}", 2 + 3 * 4);       // 14
    println!("{}", (2 + 3) * 4);     // 20
}
```

## 3.7 Conversiones de tipos

**Rust NUNCA convierte tipos numéricos automáticamente.** Siempre debes pedirlo. Hay cuatro herramientas principales:

### a) `as`: conversión directa (con cuidado)

```rust
fn main() {
    // Entero -> flotante
    let a: i32 = 7;
    let b = a as f64;
    println!("{b}");                       // 7

    // Flotante -> entero: TRUNCA (descarta decimales, no redondea)
    println!("{}", 3.99_f64 as i32);       // 3
    println!("{}", -3.99_f64 as i32);      // -3

    // Entre enteros: puede perder información
    let grande: i32 = 300;
    println!("{}", grande as u8);          // 44  ⚠️ 300 - 256 (se "corta")
    let negativo: i32 = -1;
    println!("{}", negativo as u32);       // 4294967295 ⚠️ reinterpreta los bits
    println!("{}", 255_u8 as i8);          // -1 ⚠️

    // Flotante -> entero: "satura" en los límites
    println!("{}", 300.7_f64 as u8);       // 255
    println!("{}", -5.0_f64 as u8);        // 0
    println!("{}", f64::NAN as i32);       // 0

    // bool -> entero (✅), entero -> bool (❌)
    println!("{}", true as i32);           // 1
    // let x = 1 as bool;                  // ❌ no existe

    // char <-> números
    println!("{}", 'A' as u32);            // 65
    println!("{}", 'A' as u8);             // 65
    println!("{}", 65_u8 as char);         // 'A'
    println!("{}", (b'a' + 1) as char);    // 'b'
    // let c = 65_u32 as char;             // ❌ solo u8 se puede convertir a char con as
}
```

⚠️ **`as` nunca falla**, pero puede **perder datos sin avisar**. Úsalo cuando sepas que el valor cabe, o cuando quieras ese truncamiento a propósito.

### b) `From` / `.into()`: conversiones que nunca pierden datos

Solo existen cuando es **seguro** (por ejemplo, de un tipo pequeño a uno más grande).

```rust
fn main() {
    let pequeno: i32 = 100;

    let grande = i64::from(pequeno);        // ✅ i32 -> i64 siempre cabe
    let decimal: f64 = pequeno.into();      // ✅ i32 -> f64 siempre cabe
    let codigo = u32::from('A');            // ✅ char -> u32 = 65
    println!("{grande} {decimal} {codigo}");

    // let mal = i32::from(5_i64);          // ❌ i64 -> i32 podría perder datos
    // let mal2: f32 = pequeno.into();      // ❌ i32 -> f32 podría perder precisión
}
```

### c) `TryFrom` / `.try_into()`: conversiones que pueden fallar

Devuelven un `Result`: `Ok(valor)` si cabe o `Err(...)` si no.

```rust
fn main() {
    let a: i32 = 200;
    let b: i32 = 300;

    println!("{:?}", u8::try_from(a));       // Ok(200)
    println!("{:?}", u8::try_from(b));       // Err(TryFromIntError(()))
    println!("{:?}", u8::try_from(-1_i32));  // Err(...)

    // Con valor por defecto si falla:
    let seguro = u8::try_from(b).unwrap_or(u8::MAX);
    println!("{seguro}");                    // 255

    // Con .try_into() (hay que anotar el tipo destino):
    let c: Result<u8, _> = a.try_into();
    println!("{:?}", c);                     // Ok(200)
}
```

> **Cuándo usar cada una:** `From`/`into` si es seguro → `try_from` si puede fallar y quieres controlarlo → `as` si sabes lo que haces o quieres truncar.

### d) Texto ↔ números

**De número a texto:** `.to_string()` o `format!`.

```rust
fn main() {
    let n = 42;
    let s1 = n.to_string();           // "42"
    let s2 = format!("{n}");          // "42"
    let s3 = 3.5.to_string();         // "3.5"
    let s4 = true.to_string();        // "true"
    let s5 = 'x'.to_string();         // "x"
    println!("{s1} {s2} {s3} {s4} {s5}");
}
```

**De texto a número:** `.parse::<Tipo>()`, que devuelve un `Result` porque el texto puede no ser válido.

```rust
fn main() {
    // 1) Con valor por defecto si falla (recomendado al principio):
    let a: i32 = "42".parse().unwrap_or(0);
    let b: i32 = "abc".parse().unwrap_or(0);
    println!("{a} {b}");                          // 42 0

    // 2) Con .expect(): si falla, el programa se detiene con TU mensaje
    let c: f64 = "3.14".parse().expect("no es un número válido");
    println!("{c}");

    // 3) Quitando espacios/saltos de línea antes de parsear:
    let d: i32 = "  17\n".trim().parse().unwrap();
    println!("{d}");

    // 4) Manejando el error con match (lo veremos en el Módulo 4):
    match "12x".parse::<i32>() {
        Ok(n) => println!("Número: {n}"),
        Err(e) => println!("Error: {e}"),         // Error: invalid digit found in string
    }

    // ⚠️ .unwrap() hace panic si falla:
    // let x: i32 = "abc".parse().unwrap();       // ❌ panic en ejecución
}
```
**Entre `char` y dígitos:**

```rust
fn main() {
    println!("{:?}", '7'.to_digit(10));          // Some(7)
    println!("{:?}", 'x'.to_digit(10));          // None
    println!("{:?}", char::from_digit(7, 10));   // Some('7')
    println!("{:?}", char::from_u32(65));        // Some('A')
}
```

### Resumen de conversiones

| Necesito... | Uso |
|---|---|
| Entero pequeño → grande (seguro) | `i64::from(x)` / `x.into()` |
| Entero grande → pequeño (puede fallar) | `u8::try_from(x)` |
| Entero ↔ flotante, truncar a propósito | `x as f64` / `x as i32` |
| Número → texto | `x.to_string()` |
| Texto → número | `s.parse::<i32>()` |
| `char` ↔ número | `'A' as u32`, `65u8 as char` |

---

# Módulo 4: Condicionales: `if`, `else` y `match`

## 4.1 `if`, `else if` y `else`

```rust
fn main() {
    let temperatura = 28;

    if temperatura > 30 {
        println!("Hace mucho calor");
    } else if temperatura > 20 {
        println!("Clima agradable");
    } else if temperatura > 10 {
        println!("Hace fresco");
    } else {
        println!("Hace frío");
    }
}
```

Reglas:

- ✅ **No** se ponen paréntesis alrededor de la condición (se permiten, pero el compilador avisa).
- ✅ Las llaves `{}` son **obligatorias**, aunque sea una sola línea.
- ✅ La condición **debe ser un `bool`**.
- ✅ Se evalúan en orden: se ejecuta **solo el primer bloque** cuya condición sea verdadera.

```rust
fn main() {
    let n = 5;

    // if n { println!("x"); }          // ❌ ERROR E0308: se esperaba bool, hay un entero
    // if n > 3 println!("x");          // ❌ faltan las llaves
    if n > 3 { println!("n es mayor que 3"); }   // ✅

    // Combinar condiciones:
    if n > 0 && n % 2 == 1 {
        println!("positivo e impar");
    }
}
```

## 4.2 `if` es una **expresión** (devuelve un valor)

A diferencia de otros lenguajes, en Rust un `if` produce un valor. Esto reemplaza al operador ternario (`? :`), que **no existe**.

```rust
fn main() {
    let edad = 20;

    // ❌ let categoria = edad >= 18 ? "adulto" : "menor";   // no existe el ternario

    let categoria = if edad >= 18 { "adulto" } else { "menor" };   // ✅
    println!("{categoria}");

    let descuento = if edad < 12 {
        50
    } else if edad >= 65 {
        30
    } else {
        0
    };
    println!("Descuento: {descuento}%");
}
```

Reglas del `if` como expresión:

```rust
fn main() {
    let c = true;

    // ✅ Todas las ramas deben devolver el MISMO tipo:
    let ok = if c { 1 } else { 2 };

    // ❌ ERROR E0308: if y else con tipos distintos
    // let mal = if c { 1 } else { "dos" };

    // ❌ Si usas el valor, el else es obligatorio:
    // let sin_else = if c { 1 };

    // ⚠️ El valor es la última línea SIN punto y coma.
    // Con ; devuelve () y no el número:
    // let v = if c { 1; } else { 2; };      // ❌ tipo ()

    println!("{ok}");
}
```

## 4.3 `match`: elegir entre muchos patrones

`match` compara un valor contra una lista de **patrones** y ejecuta el **primero** que coincide. Es como un `switch` mucho más poderoso.

```rust
fn main() {
    let dia = 3;

    match dia {
        1 => println!("Lunes"),
        2 => println!("Martes"),
        3 => println!("Miércoles"),
        4 => println!("Jueves"),
        5 => println!("Viernes"),
        6 | 7 => println!("Fin de semana"),   // | = "o"
        _ => println!("Día inválido"),        // _ = cualquier otro valor
    }
}
```

### Regla fundamental: **debe ser exhaustivo**

El compilador exige cubrir **todos los casos posibles**. Si olvidas alguno, no compila:

```rust
fn main() {
    let n: u8 = 3;

    // ❌ ERROR E0004: non-exhaustive patterns: `0_u8` and `4_u8..=u8::MAX` not covered
    // match n {
    //     1 => println!("uno"),
    //     2 => println!("dos"),
    //     3 => println!("tres"),
    // }

    // ✅ Se agrega un comodín:
    match n {
        1 => println!("uno"),
        2 => println!("dos"),
        3 => println!("tres"),
        _ => println!("otro"),
    }
}
```

💡 Esto es una **ventaja enorme**: Rust te obliga a pensar en todos los casos.

### `match` también es una expresión

```rust
fn main() {
    let nota = 85;

    let letra = match nota {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',            // fuera de 0..=100
    };

    println!("Nota {nota} -> {letra}");
}
```

- ✅ Todas las ramas deben devolver el mismo tipo.
- ✅ Cada rama termina con `,` (o llaves `{}` si tiene varias líneas).

### Patrones que puedes usar

**1. Valor exacto, varios valores y rangos**

```rust
fn main() {
    let n = 7;
    let descripcion = match n {
        0 => "cero",
        1 | 2 | 3 => "pequeño",         // varios valores con |
        4..=9 => "un dígito",           // rango inclusivo ..=
        10..=99 => "dos dígitos",
        _ => "grande",
    };
    println!("{descripcion}");
}
```

**2. Caracteres**

```rust
fn main() {
    let c = 'e';
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("vocal"),
        'a'..='z' => println!("consonante"),
        '0'..='9' => println!("dígito"),
        _ => println!("otro símbolo"),
    }
}
```

**3. Textos (`&str`)**

```rust
fn main() {
    let comando = "salir";
    match comando {
        "iniciar" => println!("Iniciando..."),
        "salir" | "exit" => println!("Adiós"),
        _ => println!("Comando desconocido"),
    }

    // Si tienes un String, conviértelo con .as_str():
    let entrada = String::from("iniciar");
    match entrada.as_str() {
        "iniciar" => println!("OK"),
        _ => println!("?"),
    }
}
```

**4. Guardas (`if` dentro de una rama)**

```rust
fn main() {
    let n = -5;
    match n {
        x if x < 0 => println!("{x} es negativo"),
        0 => println!("cero"),
        x if x % 2 == 0 => println!("{x} es positivo y par"),
        x => println!("{x} es positivo e impar"),   // x captura el valor
    }
}
```

**5. Capturar el valor con un nombre (`@`)**

```rust
fn main() {
    let edad = 15;
    match edad {
        n @ 0..=12 => println!("{n} años: niño"),
        n @ 13..=17 => println!("{n} años: adolescente"),
        n @ 18..=64 => println!("{n} años: adulto"),
        n => println!("{n} años: adulto mayor"),
    }
}
```

**6. Combinar varios valores (tuplas)**

```rust
fn main() {
    // FizzBuzz con match
    for n in 1..=15 {
        match (n % 3, n % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{n}"),
        }
    }

    // Con booleanos
    let (llueve, tengo_paraguas) = (true, false);
    match (llueve, tengo_paraguas) {
        (true, true) => println!("Salgo con paraguas"),
        (true, false) => println!("Me quedo en casa"),
        (false, _) => println!("Salgo tranquilo"),
    }
}
```

**7. Cubrir todos los enteros sin `_`**

```rust
fn main() {
    let n: i32 = -4;
    let signo = match n {
        i32::MIN..=-1 => "negativo",
        0 => "cero",
        1..=i32::MAX => "positivo",
    };   // ✅ exhaustivo: el compilador verifica que cubre TODO i32
    println!("{signo}");
}
```

### Orden de las ramas

Se evalúa de arriba hacia abajo y **gana la primera que coincide**:

```rust
fn main() {
    let n = 5;
    match n {
        _ => println!("siempre entra aquí"),   // ⚠️ el comodín primero tapa todo lo demás
        // 5 => println!("cinco"),             // nunca se alcanzaría (warning: unreachable)
    }
}
```

**El `_` va siempre al final.**

### `match` con `Result` (adelanto muy útil)

Muchas funciones devuelven `Result` (`Ok` o `Err`). `match` es la forma natural de manejarlos:

```rust
fn main() {
    let texto = "123";
    match texto.parse::<i32>() {
        Ok(numero) => println!("Es el número {numero}"),
        Err(_) => println!("No es un número válido"),
    }
}
```

## 4.4 ¿`if` o `match`?

| Situación | Usa |
|---|---|
| Una o dos condiciones simples | `if` / `else` |
| Condiciones con operadores lógicos complejos | `if` / `else if` |
| Comparar **un valor** contra muchos casos, rangos o patrones | `match` |
| Combinar varios valores a la vez | `match` sobre una tupla |
| Necesitas que el compilador verifique que cubres todos los casos | `match` |
| Condiciones con decimales (`f64`) | `if` (en `match` no se puede verificar exhaustividad con flotantes) |

---

# Módulo 5: Bucles: `loop`, `while` y `for`

Rust tiene tres formas de repetir código.

## 5.1 `loop`: bucle infinito

Se repite **para siempre** hasta que uses `break`.

```rust
fn main() {
    let mut contador = 0;

    loop {
        contador += 1;
        println!("Vuelta {contador}");

        if contador == 3 {
            break;            // sale del bucle
        }
    }
    println!("Fin");
}
```

### `break` puede devolver un valor (solo en `loop`)

```rust
fn main() {
    let mut n = 0;

    let resultado = loop {
        n += 1;
        if n * n > 200 {
            break n;          // el loop "vale" n
        }
    };

    println!("Primer n con n² > 200: {resultado}");   // 15
}
```

### `continue`: saltar a la siguiente vuelta

```rust
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 10 { break; }
        if i % 2 == 0 { continue; }    // si es par, salta el resto
        println!("{i}");               // 1 3 5 7 9
    }
}
```

**¿Cuándo usar `loop`?** Cuando no sabes cuántas vueltas habrá y la condición de salida está "en medio": menús, esperar entrada válida, reintentar operaciones.

## 5.2 `while`: repetir mientras se cumpla una condición

```rust
fn main() {
    let mut cuenta = 5;

    while cuenta > 0 {
        println!("{cuenta}...");
        cuenta -= 1;
    }
    println!("¡Despegue!");
}
```

- ✅ La condición se evalúa **antes** de cada vuelta (si empieza en `false`, no entra nunca).
- ✅ Debe ser `bool`.
- ✅ También puedes usar `break` y `continue`.
- ❌ **No** existe `do ... while`. Se imita con `loop` + `break`.

```rust
fn main() {
    // Simular do-while: se ejecuta al menos una vez
    let mut n = 10;
    loop {
        println!("n = {n}");
        n += 1;
        if n >= 5 { break; }     // condición de salida al final
    }
}
```

Ejemplo clásico: **suma de dígitos**.

```rust
fn main() {
    let mut n = 2024;
    let mut suma = 0;

    while n > 0 {
        suma += n % 10;    // último dígito
        n /= 10;           // quitar el último dígito
    }
    println!("Suma de dígitos: {suma}");   // 8
}
```

## 5.3 `for`: recorrer una secuencia

Es el bucle **más usado** en Rust. Recorre los elementos de algo "iterable".

### Rangos

```rust
fn main() {
    // 0..5  -> 0,1,2,3,4   (el final NO se incluye)
    for i in 0..5 {
        print!("{i} ");
    }
    println!();

    // 1..=5 -> 1,2,3,4,5   (el final SÍ se incluye)
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();

    // Al revés: .rev()
    for i in (1..=5).rev() {
        print!("{i} ");          // 5 4 3 2 1
    }
    println!();

    // Con saltos: .step_by()
    for i in (0..20).step_by(5) {
        print!("{i} ");          // 0 5 10 15
    }
    println!();

    // Si no usas la variable, usa _
    for _ in 0..3 {
        println!("¡Hola!");
    }
}
```

### ❌ Lo que NO existe

```rust
fn main() {
    // for (let i = 0; i < 10; i++) { }      // ❌ no existe el for estilo C

    // Rango "al revés" no funciona (queda vacío):
    for i in 10..0 {
        println!("{i}");                     // ⚠️ nunca se ejecuta
    }
    // Se hace así:
    for i in (0..10).rev() {
        print!("{i} ");
    }
    println!();
}
```

### Recorrer arrays y textos

```rust
fn main() {
    let numeros = [10, 20, 30, 40];

    // Solo los valores
    for n in numeros {
        println!("{n}");
    }

    // Valores con su posición
    for (indice, valor) in numeros.iter().enumerate() {
        println!("posición {indice} -> {valor}");
    }

    // Recorrer por índice
    for i in 0..numeros.len() {
        println!("numeros[{i}] = {}", numeros[i]);
    }

    // Caracteres de un texto
    for c in "Rust".chars() {
        println!("{c}");
    }
}
```

### Modificar los elementos mientras se recorren

```rust
fn main() {
    let mut precios = [100, 200, 300];

    for p in precios.iter_mut() {   // iter_mut da acceso para modificar
        *p += 10;                   // * = "el valor al que apunta p"
    }

    println!("{:?}", precios);      // [110, 210, 310]
}
```

> Por ahora basta recordar la receta: `iter_mut()` + `*elemento`. El porqué se explica en el nivel de *ownership y referencias*.

## 5.4 `break` y `continue` con etiquetas (bucles anidados)

Por defecto, `break` y `continue` afectan al bucle **más interno**. Con una **etiqueta** (`'nombre`) puedes apuntar a otro:

```rust
fn main() {
    'externo: for i in 1..=3 {
        for j in 1..=3 {
            if j == 2 {
                continue 'externo;     // salta a la siguiente vuelta del bucle externo
            }
            if i == 3 {
                break 'externo;        // sale de ambos bucles
            }
            println!("i={i}, j={j}");
        }
    }
}
```

Salida:

```
i=1, j=1
i=2, j=1
```

## 5.5 Ejemplos completos

**Tabla de multiplicar**

```rust
fn main() {
    let n = 7;
    for i in 1..=10 {
        println!("{n} x {i:2} = {}", n * i);
    }
}
```

**Factorial** (usamos `u64` porque crece muy rápido)

```rust
fn main() {
    let n: u64 = 10;
    let mut resultado: u64 = 1;

    for i in 1..=n {
        resultado *= i;
    }
    println!("{n}! = {resultado}");     // 3628800
    // Con n = 21 desborda u64 y hace panic en modo debug.
}
```

**Fibonacci (primeros 10 términos)**

```rust
fn main() {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..10 {
        print!("{a} ");
        let siguiente = a + b;
        a = b;
        b = siguiente;
    }
    println!();     // 0 1 1 2 3 5 8 13 21 34
}
```

**Triángulo de asteriscos (bucles anidados)**

```rust
fn main() {
    for fila in 1..=5 {
        for _ in 0..fila {
            print!("*");
        }
        println!();
    }
}
```

**Adivinar un número (patrón `loop` + `match`)**

```rust
fn main() {
    let secreto = 7;
    let intentos = [3, 9, 7];       // simulamos las respuestas del usuario
    let mut i = 0;

    loop {
        let intento = intentos[i];
        i += 1;

        match intento {
            n if n < secreto => println!("{n}: muy bajo"),
            n if n > secreto => println!("{n}: muy alto"),
            _ => {
                println!("¡Acertaste en {i} intentos!");
                break;
            }
        }
    }
}
```

## 5.6 ¿Cuál bucle elegir?

| Situación | Usa |
|---|---|
| Sé cuántas veces o recorro una colección | `for` |
| Repetir mientras una condición sea cierta | `while` |
| Repetir "para siempre" con salida manual, o devolver un valor con `break` | `loop` |

> **Regla práctica:** si puedes usar `for`, úsalo. Es más corto y evita errores de índices.

---

# Módulo 6: Funciones, parámetros y valores de retorno

## 6.1 Definir y llamar funciones

```rust
fn saludar() {
    println!("¡Hola desde una función!");
}

fn main() {
    saludar();
    saludar();
}
```

- Se declaran con `fn`.
- Los nombres van en `snake_case`.
- ✅ Puedes definir la función **antes o después** de `main`; el orden no importa.

## 6.2 Parámetros

**Los tipos de los parámetros son obligatorios.**

```rust
fn presentar(nombre: &str, edad: u32) {
    println!("{nombre} tiene {edad} años");
}

fn main() {
    presentar("Ana", 20);
    presentar("Luis", 25);

    // presentar("Ana");            // ❌ faltan argumentos (se exige exactamente 2)
    // presentar(20, "Ana");        // ❌ orden/tipos incorrectos
    // presentar("Ana", -5);        // ❌ u32 no admite negativos
}
```

## 6.3 Valores de retorno

Se declaran con `->` y el tipo. La función devuelve **la última expresión, sin punto y coma**.

```rust
fn sumar(a: i32, b: i32) -> i32 {
    a + b                  // ✅ sin ; -> es el valor de retorno
}

fn main() {
    let resultado = sumar(3, 4);
    println!("{resultado}");            // 7
    println!("{}", sumar(10, 20));      // se puede usar directo
}
```

### ⚠️ El error más común: el punto y coma

```rust
// ❌ ERROR E0308: mismatched types (se esperaba i32, se obtuvo ())
// fn sumar(a: i32, b: i32) -> i32 {
//     a + b;             // con ; se convierte en sentencia y no devuelve nada
// }
```

### Diferencia entre **expresión** y **sentencia**

- **Expresión:** produce un valor → `5`, `a + b`, `if ... {} else {}`, un bloque `{ ... }`
- **Sentencia:** realiza una acción y no produce valor → `let x = 5;`, cualquier cosa terminada en `;`

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1            // expresión: el bloque vale 4
    };
    println!("{y}");     // 4

    // let z = (let w = 5);   // ❌ let es una sentencia, no produce valor
}
```

### `return` para salir antes

```rust
fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        println!("No se puede dividir entre cero");
        return 0;            // ✅ salida anticipada (con ;)
    }
    a / b                    // retorno normal al final
}

fn main() {
    println!("{}", dividir(10, 2));   // 5
    println!("{}", dividir(10, 0));   // mensaje + 0
}
```

Una función **sin `->`** devuelve `()` (llamado *unit*, "nada"):

```rust
fn avisar(mensaje: &str) {          // equivale a -> ()
    println!("⚠ {mensaje}");
}

fn main() {
    avisar("Cuidado");
    let nada = avisar("otra vez");  // nada vale ()
    println!("{:?}", nada);         // ()
}
```

## 6.4 Devolver varios valores con tuplas

Rust no tiene "múltiples retornos", pero se logra con una **tupla**:

```rust
fn dividir_con_residuo(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn minimo_y_maximo(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}

fn main() {
    let (cociente, residuo) = dividir_con_residuo(17, 5);
    println!("17 / 5 = {cociente} resto {residuo}");        // 3 resto 2

    let (menor, mayor) = minimo_y_maximo(9, 4);
    println!("menor={menor}, mayor={mayor}");

    let resultado = dividir_con_residuo(20, 6);
    println!("{} {}", resultado.0, resultado.1);            // 3 2
}
```

## 6.5 Devolver texto

Hay dos caminos seguros para principiantes:

```rust
// 1) Devolver un String (nuevo, propio): siempre funciona
fn crear_saludo(nombre: &str) -> String {
    format!("Hola, {nombre}!")
}

// 2) Devolver un texto fijo: se escribe &'static str
fn nombre_del_dia(n: u32) -> &'static str {
    match n {
        1 => "Lunes",
        2 => "Martes",
        3 => "Miércoles",
        _ => "Otro día",
    }
}

fn main() {
    println!("{}", crear_saludo("Ana"));
    println!("{}", nombre_del_dia(2));
}
```

> `'static` significa "este texto vive durante todo el programa" (son los literales `"..."`). Para clase: *"si devuelvo un literal, uso `&'static str`; si construyo el texto, devuelvo `String`"*.
>
> ❌ Escribir `fn f() -> &str` sin más da error (`E0106: missing lifetime specifier`).

## 6.6 Funciones que devuelven `bool`

Muy comunes para validaciones:

```rust
fn es_par(n: i32) -> bool {
    n % 2 == 0
}

fn es_mayor_de_edad(edad: u32) -> bool {
    edad >= 18
}

fn main() {
    println!("{}", es_par(10));              // true
    println!("{}", es_mayor_de_edad(16));    // false

    if es_par(7) {
        println!("par");
    } else {
        println!("impar");
    }
}
```

## 6.7 Pasar argumentos: por valor y (adelanto) por referencia

### Por valor (tipos simples)

Los tipos simples (`i32`, `f64`, `bool`, `char`...) se **copian** al llamar la función. La función trabaja con su propia copia:

```rust
fn duplicar(mut n: i32) -> i32 {
    n *= 2;          // modifica la COPIA local (por eso el parámetro es mut)
    n
}

fn main() {
    let x = 5;
    let y = duplicar(x);
    println!("x = {x}, y = {y}");     // x = 5, y = 10  (x no cambió)
}
```

### Por referencia mutable (adelanto del Nivel 2)

Si quieres que la función modifique la variable **original**, pasas una referencia mutable con `&mut`:

```rust
fn incrementar(n: &mut i32) {
    *n += 1;                // * accede al valor original
}

fn main() {
    let mut contador = 0;
    incrementar(&mut contador);
    incrementar(&mut contador);
    println!("{contador}");    // 2
}
```

### Texto como parámetro: usa `&str`

```rust
fn longitud(texto: &str) -> usize {
    texto.len()
}

fn main() {
    let a = "literal";
    let b = String::from("un String");

    println!("{}", longitud(a));        // ✅
    println!("{}", longitud(&b));       // ✅ &String se acepta donde se espera &str
}
```

## 6.8 Lo que Rust NO tiene en funciones

| ❌ No existe | ✅ Alternativa |
|---|---|
| Sobrecarga (dos funciones con el mismo nombre) | Nombres distintos: `area_circulo`, `area_cuadrado` |
| Parámetros con valor por defecto | Pasar siempre todos los valores (o usar `Option`, nivel posterior) |
| Argumentos con nombre (`f(a=1, b=2)`) | Respetar el orden; usar *structs* (nivel posterior) |
| Número variable de argumentos | Pasar un array |
| Múltiples valores de retorno directos | Devolver una tupla |

## 6.9 Funciones dentro de funciones y recursión

```rust
fn main() {
    // Función anidada: solo existe dentro de main
    fn al_cuadrado(n: i32) -> i32 {
        n * n
    }
    println!("{}", al_cuadrado(6));      // 36
}
```

**Recursión:** una función que se llama a sí misma. Necesita un **caso base** para detenerse.

```rust
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1                       // caso base
    } else {
        n * factorial(n - 1)    // llamada recursiva
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("{}", factorial(5));     // 120
    println!("{}", factorial(20));    // 2432902008176640000
    println!("{}", fibonacci(10));    // 55
}
```

⚠️ Sin caso base, la recursión no termina y el programa se cae por *stack overflow*.

## 6.10 Funciones y constantes

```rust
const IVA: f64 = 0.19;

fn precio_con_iva(precio: f64) -> f64 {
    precio * (1.0 + IVA)       // ✅ las constantes globales se ven en cualquier función
}

fn main() {
    println!("{:.2}", precio_con_iva(100_000.0));   // 119000.00
}
```

---

# Módulo 7: Ámbitos de variables (scope)

El **ámbito** (*scope*) es la zona del código donde una variable **existe y puede usarse**. En Rust, el ámbito empieza donde declaras la variable y **termina en la llave `}` que cierra su bloque**.

## 7.1 Bloques `{ }`

```rust
fn main() {
    let externa = 1;                 // existe hasta el final de main

    {
        let interna = 2;             // existe solo dentro de este bloque
        println!("{externa} {interna}");   // ✅ aquí ve las dos
    }                                // <- interna desaparece aquí

    println!("{externa}");           // ✅
    // println!("{interna}");        // ❌ ERROR E0425: cannot find value `interna`
}
```

**Regla:** un bloque interno ve las variables del bloque externo, pero **el externo no ve las del interno**.

## 7.2 Una variable no existe antes de su declaración

```rustfn main() {
    // println!("{x}");     // ❌ ERROR E0425: todavía no existe
    let x = 5;
    println!("{x}");        // ✅
}
```

## 7.3 Ámbito en estructuras de control

Todo lo declarado dentro de un `if`, `match`, `while`, `for` o `loop` **muere al terminar ese bloque**.

```rust
fn main() {
    if true {
        let dentro_del_if = "hola";
        println!("{dentro_del_if}");
    }
    // println!("{dentro_del_if}");     // ❌ fuera de su ámbito

    for i in 0..3 {
        let doble = i * 2;
        println!("{i} -> {doble}");
    }
    // println!("{i}");                 // ❌ i solo existe dentro del for
    // println!("{doble}");             // ❌

    // ✅ Para usar un valor después, declara la variable ANTES del bloque:
    let mut suma = 0;
    for i in 1..=4 {
        suma += i;
    }
    println!("suma = {suma}");          // 10
}
```

## 7.4 Ámbito de funciones

Cada función tiene su **propio ámbito**: no ve las variables locales de quien la llama.

```rust
fn mostrar() {
    // println!("{numero}");     // ❌ ERROR E0425: numero es local de main
}

fn main() {
    let numero = 10;
    mostrar();
    println!("{numero}");
}
```

Si una función necesita un dato, se lo pasas como **parámetro**; si quieres un valor global inmutable, usa `const`:

```rust
const LIMITE: i32 = 100;          // global: visible en todo el archivo

fn comprobar(valor: i32) -> bool {
    valor <= LIMITE               // ✅ ve la constante global y su propio parámetro
}

fn main() {
    println!("{}", comprobar(50));    // true
    println!("{}", comprobar(150));   // false
}
```

❌ **No existen variables `let` globales.** Solo `const` (y `static`, de niveles avanzados).

## 7.5 Shadowing y ámbitos

El shadowing dentro de un bloque interno **solo dura mientras el bloque exista**:

```rust
fn main() {
    let x = 1;

    {
        let x = x + 1;            // nueva x = 2, solo en este bloque
        {
            let x = x * 10;       // otra x = 20, solo aquí
            println!("{x}");      // 20
        }
        println!("{x}");          // 2
    }

    println!("{x}");              // 1  (la original nunca cambió)
}
```

Compara con modificar con `mut` desde un bloque interno:

```rust
fn main() {
    let mut x = 1;
    {
        x = x + 1;                // ✅ modifica la MISMA x (no crea una nueva)
    }
    println!("{x}");              // 2
}
```

| | Shadowing (`let x = ...` interno) | Mutación (`x = ...` con `mut`) |
|---|---|---|
| Crea variable nueva | Sí | No |
| El cambio sobrevive al bloque | ❌ No | ✅ Sí |

## 7.6 Los bloques son expresiones

La última línea sin `;` es el valor del bloque. Así puedes "llevarte" un resultado y dejar atrás las variables auxiliares:

```rust
fn main() {
    let area = {
        let base = 5.0;
        let altura = 3.0;
        base * altura / 2.0         // valor del bloque
    };
    // base y altura ya no existen, pero area sí
    println!("{area}");             // 7.5
}
```

Esto mantiene limpio el código: las variables temporales no "contaminan" el resto de la función.

## 7.7 Fin del ámbito = liberación automática de memoria

Cuando una variable sale de su ámbito, Rust **libera su memoria automáticamente** (sin recolector de basura y sin `free` manual):

```rust
fn main() {
    {
        let texto = String::from("temporal");
        println!("{texto}");
    }   // <- aquí `texto` se libera automáticamente

    println!("fin");
}
```

Este es el corazón de la gestión de memoria de Rust. Se profundiza en el siguiente nivel (*ownership*).

### Adelanto: mover vs. copiar

```rust
fn main() {
    // Tipos simples: se COPIAN, ambas variables siguen vivas
    let a = 5;
    let b = a;
    println!("{a} {b}");               // ✅

    // String: se MUEVE, la original deja de ser válida
    let s1 = String::from("hola");
    let s2 = s1;
    // println!("{s1}");               // ❌ ERROR E0382: borrow of moved value: `s1`
    println!("{s2}");                  // ✅

    // Si quieres dos copias independientes:
    let s3 = String::from("hola");
    let s4 = s3.clone();               // copia explícita
    println!("{s3} {s4}");             // ✅
}
```

Resumen para clase: *"los números, `bool` y `char` se copian solos; los `String` se mueven, y si quieres copiar, usas `.clone()`"*.

## 7.8 Resumen de reglas de scope

1. Una variable existe **desde su `let` hasta la `}` de su bloque**.
2. Los bloques internos **ven** lo de afuera; los externos **no ven** lo de adentro.
3. Las funciones **no ven** las variables locales de otras funciones.
4. Las `const` globales se ven en todo el archivo.
5. El shadowing en un bloque interno **termina** con el bloque; `mut` cambia la variable original.
6. Al terminar el ámbito, la memoria se libera sola.

---

# Módulo 8: Tuplas y arrays

Ambos agrupan varios valores, pero con filosofías distintas:

| | **Tupla** | **Array** |
|---|---|---|
| Tipos de los elementos | Pueden ser **distintos** | Todos **iguales** |
| Tamaño | Fijo | Fijo |
| Acceso | `t.0`, `t.1`... | `a[0]`, `a[1]`... |
| Se puede recorrer con `for` | ❌ | ✅ |
| Ejemplo de tipo | `(i32, f64, &str)` | `[i32; 5]` |

## 8.1 Tuplas

### Crear y acceder

```rust
fn main() {
    let persona: (&str, u32, f64) = ("Ana", 25, 1.65);

    println!("{}", persona.0);      // Ana    (primer elemento)
    println!("{}", persona.1);      // 25
    println!("{}", persona.2);      // 1.65

    println!("{:?}", persona);      // ("Ana", 25, 1.65)
}
```

- El acceso es con **punto y número fijo**: `.0`, `.1`, `.2`...
- El tipo se infiere normalmente; la anotación es opcional.

### Desestructurar

Separa la tupla en variables individuales:

```rust
fn main() {
    let persona = ("Ana", 25, 1.65);

    let (nombre, edad, altura) = persona;
    println!("{nombre} tiene {edad} años y mide {altura} m");

    // Ignorar elementos con _
    let (nombre, _, _) = persona;
    println!("{nombre}");
}
```

### Modificar (necesita `mut`)

```rust
fn main() {
    let mut punto = (3, 4);
    punto.0 = 10;
    punto.1 += 1;
    println!("{:?}", punto);        // (10, 5)

    // punto.0 = "diez";            // ❌ ERROR E0308: el tipo del elemento no cambia
}
```

### Intercambiar valores

```rust
fn main() {
    let mut a = 1;
    let mut b = 2;
    (a, b) = (b, a);                // ✅ intercambio con tuplas
    println!("{a} {b}");            // 2 1
}
```

### Tuplas anidadas, comparación y unit

```rust
fn main() {
    // Anidadas
    let anidada = ((1, 2), "x");
    println!("{}", (anidada.0).1);  // 2
    println!("{}", anidada.0.1);    // 2 (también válido)

    // Se comparan elemento por elemento, en orden
    println!("{}", (1, 2) == (1, 2));     // true
    println!("{}", (1, 5) < (2, 0));      // true  (1 < 2 decide)
    println!("{}", (1, 2) < (1, 3));      // true

    // Tupla vacía: "unit" (lo que devuelve una función sin retorno)
    let nada = ();
    println!("{:?}", nada);               // ()
}
```

### ❌ Lo que NO se puede con tuplas

```rust
fn main() {
    let t = (10, 20, 30);

    // let x = t[0];          // ❌ ERROR E0608: no se indexa con []
    // let i = 1;
    // let y = t.i;           // ❌ el índice debe ser un literal fijo
    // let z = t.5;           // ❌ ERROR E0609: no existe el elemento 5
    // for v in t { }         // ❌ una tupla no se puede recorrer con for
    // t.0 = 99;              // ❌ t no es mut

    println!("{:?}", t);
}
```

### Usos típicos

```rust
// 1) Devolver varios valores
fn estadisticas(a: f64, b: f64) -> (f64, f64) {
    (a + b, (a + b) / 2.0)           // (suma, promedio)
}

fn main() {
    let (suma, promedio) = estadisticas(4.0, 6.0);
    println!("suma={suma}, promedio={promedio}");

    // 2) Agrupar datos relacionados de forma rápida
    let coordenada = (4.5, -3.2);
    let producto = ("Café", 12_500, true);     // nombre, precio, disponible
    println!("{:?} {:?}", coordenada, producto);

    // 3) Pares en un array de tuplas
    let alumnos = [("Ana", 4.5), ("Luis", 3.2), ("Sofía", 4.8)];
    for (nombre, nota) in alumnos {
        println!("{nombre}: {nota}");
    }
}
```

> **Cuándo NO usar tuplas:** si tienes más de 3 o 4 campos, o si necesitas recordar qué significa cada posición, conviene usar una `struct` (siguiente nivel).

## 8.2 Arrays

Un **array** guarda un número **fijo** de elementos del **mismo tipo**, en memoria contigua. El tamaño forma parte del tipo.

### Crear arrays

```rust
fn main() {
    let numeros = [1, 2, 3, 4, 5];                   // [i32; 5] inferido
    let notas: [f64; 3] = [4.5, 3.8, 5.0];           // tipo explícito: [tipo; cantidad]
    let ceros = [0; 10];                             // diez ceros
    let puntos = [1.5; 4];                           // [1.5, 1.5, 1.5, 1.5]
    let dias = ["Lun", "Mar", "Mié", "Jue", "Vie"];  // [&str; 5]
    let vocales = ['a', 'e', 'i', 'o', 'u'];         // [char; 5]

    println!("{:?}", numeros);
    println!("{:?}", notas);
    println!("{:?}", ceros);
    println!("{:?}", puntos);
    println!("{:?}", dias);
    println!("{:?}", vocales);
}
```

### Acceder por índice

- Los índices **empiezan en 0**.
- El índice debe ser de tipo **`usize`**.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    println!("{}", a[0]);              // 10  primero
    println!("{}", a[4]);              // 50  último
    println!("{}", a[a.len() - 1]);    // 50  último, sin saber el tamaño
    println!("{}", a.len());           // 5   cantidad de elementos

    // ❌ El índice no puede ser i32:
    // let i: i32 = 2;
    // println!("{}", a[i]);           // ERROR E0277
    let i: i32 = 2;
    println!("{}", a[i as usize]);     // ✅ conviertes a usize
}
```

### Modificar elementos

```rust
fn main() {
    let mut a = [1, 2, 3];
    a[0] = 100;
    a[2] += 5;
    println!("{:?}", a);         // [100, 2, 8]

    // a[1] = "dos";             // ❌ todos los elementos deben ser del mismo tipo
    // let b = [1, 2, 3];
    // b[0] = 9;                 // ❌ b no es mut
}
```

### ⚠️ Salirse del rango

Rust **comprueba los límites** y detiene el programa antes de leer memoria que no es tuya:

```rust
fn main() {
    let a = [1, 2, 3];

    // let x = a[10];                // ❌ error al COMPILAR: índice constante fuera de rango

    let indice = 10;
    // println!("{}", a[indice]);    // ⚠️ si el índice se calcula en ejecución:
                                     //    panic: index out of bounds: the len is 3 but the index is 10

    // ✅ Forma segura: .get() devuelve Option (Some o None)
    println!("{:?}", a.get(1));      // Some(2)
    println!("{:?}", a.get(10));     // None
}
```

Esto evita todo un tipo de errores graves de seguridad (acceso fuera de memoria) que existen en C/C++.

### El tamaño es fijo y parte del tipo

```rust
fn main() {
    let a: [i32; 3] = [1, 2, 3];
    // let b: [i32; 4] = a;          // ❌ ERROR E0308: [i32; 3] y [i32; 4] son tipos distintos

    // ❌ El tamaño debe conocerse al compilar:
    // let n = 5;
    // let c = [0; n];               // ERROR E0435: n no es constante

    const N: usize = 5;
    let d = [0; N];                  // ✅ con const sí
    println!("{:?} {:?}", a, d);

    // ❌ No se puede agregar ni quitar elementos de un array.
    // a.push(4);                    // ERROR: no existe push en arrays
    // Para colecciones que crecen se usa Vec<T> (siguiente nivel).
}
```

### Recorrer arrays

```rust
fn main() {
    let a = [10, 20, 30];

    // Forma 1: for directo (la más sencilla)
    for x in a {
        println!("{x}");
    }

    // Forma 2: con índice
    for (i, x) in a.iter().enumerate() {
        println!("a[{i}] = {x}");
    }

    // Forma 3: por posiciones
    for i in 0..a.len() {
        println!("{}", a[i]);
    }

    // Forma 4: con while
    let mut i = 0;
    while i < a.len() {
        println!("{}", a[i]);
        i += 1;
    }
}
```

### Métodos útiles

```rust
fn main() {
    let mut a = [5, 3, 8, 1, 9, 2];

    println!("{}", a.len());                    // 6
    println!("{}", a.is_empty());               // false
    println!("{}", a.contains(&8));             // true  (se pasa con &)
    println!("{:?}", a.first());                // Some(5)
    println!("{:?}", a.last());                 // Some(2)
    println!("{}", a.iter().sum::<i32>());      // 28
    println!("{:?}", a.iter().max());           // Some(9)
    println!("{:?}", a.iter().min());           // Some(1)

    a.sort();                                   // ordenar (requiere mut)
    println!("{:?}", a);                        // [1, 2, 3, 5, 8, 9]
    a.reverse();                                // invertir
    println!("{:?}", a);                        // [9, 8, 5, 3, 2, 1]

    let maximo = a.iter().max().unwrap();       // sacar el valor del Option
    println!("{maximo}");                       // 9
}
```

### Copiar y comparar arrays

```rust
fn main() {
    let a = [1, 2, 3];
    let b = a;                          // ✅ se COPIA (los elementos son tipos simples)
    println!("{:?} {:?}", a, b);        // las dos siguen disponibles

    println!("{}", a == b);             // true  (se comparan elemento a elemento)
    println!("{}", [1, 2] == [1, 3]);   // false
}
```

### Arrays bidimensionales (matrices)

```rust
fn main() {
    let mut matriz = [[0; 3]; 3];       // 3 filas x 3 columnas
    matriz[1][1] = 5;
    matriz[0][2] = 7;

    println!("{:?}", matriz);           // [[0, 0, 7], [0, 5, 0], [0, 0, 0]]

    for fila in matriz {
        for valor in fila {
            print!("{valor} ");
        }
        println!();
    }

    let tabla = [[1, 2, 3], [4, 5, 6]]; // 2 filas, 3 columnas
    println!("{}", tabla[1][2]);        // 6 (fila 1, columna 2)
}
```

### Adelanto: slices (porciones de un array)

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    let parte = &a[1..4];               // elementos en posiciones 1, 2, 3
    println!("{:?}", parte);            // [20, 30, 40]
    println!("{:?}", &a[..2]);          // [10, 20]
    println!("{:?}", &a[3..]);          // [40, 50]
    println!("{:?}", &a[..]);           // todo
}
```

### Arrays en funciones

```rust
fn suma(numeros: [i32; 5]) -> i32 {
    let mut total = 0;
    for n in numeros {
        total += n;
    }
    total
}

fn main() {
    let datos = [1, 2, 3, 4, 5];
    println!("{}", suma(datos));      // 15
    // println!("{}", suma([1, 2]));  // ❌ la función exige exactamente [i32; 5]
}
```

> En el siguiente nivel verás cómo escribir funciones que acepten arrays de **cualquier** tamaño usando slices (`&[i32]`) y colecciones dinámicas (`Vec`).

### Cuándo usar array y cuándo no

| Usa un array cuando... | NO uses un array cuando... |
|---|---|
| Sabes la cantidad exacta de elementos | La cantidad cambia durante el programa (usa `Vec`) |
| Los datos son pocos y de tamaño fijo (días de la semana, meses, RGB) | Necesitas agregar o quitar elementos |
| Todos los elementos son del mismo tipo | Los elementos son de tipos distintos (usa tupla o struct) |

---

# Proyecto integrador: reporte de notas

Este programa une **todos** los temas del Nivel 1: constantes, tipos, operadores, conversiones, `if`, `match`, bucles, funciones, tuplas y arrays.

```rust
const NOTA_MINIMA: f64 = 3.0;
const CANTIDAD: usize = 5;

/// Devuelve el promedio de las notas.
fn promedio(notas: [f64; CANTIDAD]) -> f64 {
    let mut suma = 0.0;
    for nota in notas {
        suma += nota;
    }
    suma / CANTIDAD as f64          // conversión: usize -> f64
}

/// Devuelve (máximo, mínimo) en una tupla.
fn maximo_y_minimo(notas: [f64; CANTIDAD]) -> (f64, f64) {
    let mut maximo = notas[0];
    let mut minimo = notas[0];

    for nota in notas {
        if nota > maximo {
            maximo = nota;
        }
        if nota < minimo {
            minimo = nota;
        }
    }
    (maximo, minimo)
}

/// Cuenta cuántas notas son aprobatorias.
fn contar_aprobadas(notas: [f64; CANTIDAD]) -> u32 {
    let mut contador = 0;
    for nota in notas {
        if nota >= NOTA_MINIMA {
            contador += 1;
        }
    }
    contador
}

/// Clasifica el desempeño según el promedio.
fn clasificar(prom: f64) -> &'static str {
    if prom >= 4.5 {
        "Excelente"
    } else if prom >= 4.0 {
        "Bueno"
    } else if prom >= NOTA_MINIMA {
        "Aceptable"
    } else {
        "Insuficiente"
    }
}

fn main() {
    let notas: [f64; CANTIDAD] = [4.5, 3.2, 2.8, 5.0, 3.9];

    let prom = promedio(notas);
    let (maxima, minima) = maximo_y_minimo(notas);
    let aprobadas = contar_aprobadas(notas);

    println!("===== REPORTE DE NOTAS =====");
    for (i, nota) in notas.iter().enumerate() {
        let estado = match *nota >= NOTA_MINIMA {
            true => "aprobada",
            false => "reprobada",
        };
        println!("Nota {}: {:.1} -> {}", i + 1, nota, estado);
    }
    println!("----------------------------");
    println!("Promedio:   {:.2}", prom);
    println!("Máxima:     {:.1}", maxima);
    println!("Mínima:     {:.1}", minima);
    println!("Aprobadas:  {} de {}", aprobadas, CANTIDAD);
    println!("Resultado:  {}", clasificar(prom));
}
```

**Salida esperada:**

```
===== REPORTE DE NOTAS =====
Nota 1: 4.5 -> aprobada
Nota 2: 3.2 -> aprobada
Nota 3: 2.8 -> reprobada
Nota 4: 5.0 -> aprobada
Nota 5: 3.9 -> aprobada
----------------------------
Promedio:   3.88
Máxima:     5.0
Mínima:     2.8
Aprobadas:  4 de 5
Resultado:  Aceptable
```

**Preguntas para discutir en clase:**

1. ¿Por qué `CANTIDAD` es una `const` y no un `let`? (Pista: el tamaño de un array debe conocerse al compilar.)
2. ¿Por qué hay que escribir `CANTIDAD as f64`?
3. ¿Qué pasaría si `clasificar` no tuviera el `else` final?
4. ¿Por qué se puede usar `notas` varias veces si se pasa a funciones por valor?  (Pista: los arrays de tipos simples se copian.)

---

# Ejercicios con soluciones

> 💡 **Sugerencia pedagógica:** que los estudiantes intenten resolver antes de abrir la solución. Las soluciones están plegadas.

## Módulo 1: Variables

**1.1** Declara una variable inmutable `nombre` y una mutable `puntos` que empiece en 0 y suba 3 veces en 10. Imprime ambas.

**1.2** Declara una constante `IVA` con valor `0.19` y calcula el total de un producto de `100000.0`.

<details>
<summary>Ver soluciones</summary>

```rust
const IVA: f64 = 0.19;

fn main() {
    // 1.1
    let nombre = "Jugador1";
    let mut puntos = 0;
    puntos += 10;
    puntos += 10;
    puntos += 10;
    println!("{nombre}: {puntos} puntos");      // Jugador1: 30 puntos

    // 1.2
    let precio = 100_000.0;
    let total = precio * (1.0 + IVA);
    println!("Total: {total:.2}");              // Total: 119000.00
}
```
</details>

## Módulo 2: Tipos de datos

**2.1** Declara una variable de cada tipo: `i32`, `u64`, `f64`, `bool`, `char` y `String`. Imprímelas con `{:?}`.

**2.2** Compara `.len()` y `.chars().count()` del texto `"canción"`. Explica la diferencia.

<details>
<summary>Ver soluciones</summary>

```rust
fn main() {
    // 2.1
    let a: i32 = -42;
    let b: u64 = 10_000_000_000;
    let c: f64 = 3.14;
    let d: bool = true;
    let e: char = 'Ñ';
    let f: String = String::from("Hola");
    println!("{:?} {:?} {:?} {:?} {:?} {:?}", a, b, c, d, e, f);

    // 2.2
    let texto = "canción";
    println!("bytes: {}", texto.len());                 // 8
    println!("caracteres: {}", texto.chars().count());  // 7
    // La 'ó' ocupa 2 bytes en UTF-8, por eso len() da 8 y hay 7 caracteres.
}
```
</details>

## Módulo 3: Operadores y conversiones

**3.1** Convierte 36.6 °C a Fahrenheit (`F = C × 9/5 + 32`).

**3.2** Dado el texto `"42"`, conviértelo a número y súmale `8.5`. Pista: ¿qué tipo necesitas?

**3.3** Convierte `300_i32` a `u8` de dos formas: con `as` y con `try_from`. Compara los resultados.

<details>
<summary>Ver soluciones</summary>

```rust
fn main() {
    // 3.1
    let celsius = 36.6_f64;
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("{celsius} °C = {fahrenheit:.1} °F");      // 97.9 °F

    // 3.2
    let texto = "42";
    let numero: f64 = texto.parse().unwrap_or(0.0);     // parseamos directo a f64
    println!("{}", numero + 8.5);                       // 50.5

    // 3.3
    let n: i32 = 300;
    println!("{}", n as u8);                            // 44 (se corta)
    println!("{:?}", u8::try_from(n));                  // Err(...) (avisa del problema)
}
```
</details>

## Módulo 4: Condicionales

**4.1** Escribe un programa que diga si un año es bisiesto (divisible por 4, excepto los divisibles por 100, salvo que lo sean por 400).

**4.2** Dada una nota entera 0-100, usa `match` con rangos para imprimir la letra A, B, C, D o F.

<details>
<summary>Ver soluciones</summary>

```rust
fn main() {
    // 4.1
    let anio = 2024;
    let bisiesto = (anio % 4 == 0 && anio % 100 != 0) || anio % 400 == 0;
    if bisiesto {
        println!("{anio} es bisiesto");
    } else {
        println!("{anio} no es bisiesto");
    }

    // 4.2
    let nota = 74;
    let letra = match nota {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',
    };
    println!("Nota {nota} -> {letra}");     // C
}
```
</details>

## Módulo 5: Bucles

**5.1** Imprime la tabla de multiplicar del 7 (del 1 al 10).

**5.2** Calcula la suma de los números pares del 1 al 100.

**5.3** Imprime los primeros 10 términos de Fibonacci.

**5.4** Imprime un triángulo de asteriscos de 5 filas.

<details>
<summary>Ver soluciones</summary>

```rust
fn main() {
    // 5.1
    for i in 1..=10 {
        println!("7 x {i} = {}", 7 * i);
    }

    // 5.2
    let mut suma = 0;
    for n in 1..=100 {
        if n % 2 == 0 {
            suma += n;
        }
    }
    println!("Suma de pares: {suma}");          // 2550
    // Variante más corta:  (2..=100).step_by(2).sum::<i32>()

    // 5.3
    let (mut a, mut b) = (0u32, 1u32);
    for _ in 0..10 {
        print!("{a} ");
        (a, b) = (b, a + b);
    }
    println!();

    // 5.4
    for fila in 1..=5 {
        for _ in 0..fila {
            print!("*");
        }
        println!();
    }
}
```
</details>

## Módulo 6: Funciones

**6.1** Escribe `fn es_primo(n: u32) -> bool`.

**6.2** Escribe `fn min_max(a: i32, b: i32, c: i32) -> (i32, i32)` que devuelva el menor y el mayor de tres números.

**6.3** Escribe una función recursiva `potencia(base: u64, exp: u32) -> u64`.

<details>
<summary>Ver soluciones</summary>

```rust
fn es_primo(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn min_max(a: i32, b: i32, c: i32) -> (i32, i32) {
    let mut menor = a;
    let mut mayor = a;
    if b < menor { menor = b; }
    if c < menor { menor = c; }
    if b > mayor { mayor = b; }
    if c > mayor { mayor = c; }
    (menor, mayor)
}

fn potencia(base: u64, exp: u32) -> u64 {
    if exp == 0 {
        1                              // caso base
    } else {
        base * potencia(base, exp - 1)
    }
}

fn main() {
    for n in 1..=20 {
        if es_primo(n) {
            print!("{n} ");             // 2 3 5 7 11 13 17 19
        }
    }
    println!();

    let (menor, mayor) = min_max(7, -2, 15);
    println!("menor={menor}, mayor={mayor}");   // -2 y 15

    println!("{}", potencia(2, 10));    // 1024
}
```
</details>

## Módulo 7: Scope

**7.1** Sin ejecutar el código, ¿qué imprime? Luego verifícalo.

```rust
fn main() {
    let x = 1;
    {
        let x = x + 1;
        {
            let x = x * 10;
            println!("{x}");
        }
        println!("{x}");
    }
    println!("{x}");
}
```

**7.2** ¿Por qué este código no compila? Corrígelo.

```rust
fn main() {
    for i in 1..=3 {
        let doble = i * 2;
    }
    println!("{doble}");
}
```

<details>
<summary>Ver soluciones</summary>

**7.1** Imprime `20`, luego `2`, luego `1`. Cada `let x` crea una variable nueva que solo vive dentro de su bloque.

**7.2** `doble` solo existe dentro del `for`. Solución: declararla fuera.

```rust
fn main() {
    let mut doble = 0;
    for i in 1..=3 {
        doble = i * 2;
    }
    println!("{doble}");     // 6 (el último valor asignado)
}
```
</details>

## Módulo 8: Tuplas y arrays

**8.1** Dado `[7, 3, 9, 1, 5]`, encuentra el valor máximo **y su posición** usando un bucle y devuélvelos en una tupla desde una función.

**8.2** Crea una matriz 3×3 con la tabla identidad (1 en la diagonal, 0 en el resto) usando bucles.

**8.3** Dado `let fechas = [("Ana", 25), ("Luis", 17), ("Sofía", 30)];`, imprime solo los mayores de edad.

<details>
<summary>Ver soluciones</summary>

```rust
fn maximo_con_posicion(datos: [i32; 5]) -> (i32, usize) {
    let mut max = datos[0];
    let mut pos = 0;
    for i in 1..datos.len() {
        if datos[i] > max {
            max = datos[i];
            pos = i;
        }
    }
    (max, pos)
}

fn main() {
    // 8.1
    let (max, pos) = maximo_con_posicion([7, 3, 9, 1, 5]);
    println!("Máximo {max} en la posición {pos}");     // 9 en la posición 2

    // 8.2
    let mut identidad = [[0; 3]; 3];
    for i in 0..3 {
        identidad[i][i] = 1;
    }
    println!("{:?}", identidad);    // [[1, 0, 0], [0, 1, 0], [0, 0, 1]]

    // 8.3
    let personas = [("Ana", 25), ("Luis", 17), ("Sofía", 30)];
    for (nombre, edad) in personas {
        if edad >= 18 {
            println!("{nombre} ({edad})");
        }
    }
}
```
</details>

---

# Hoja de referencia rápida

```rust
// ---------- VARIABLES ----------
let x = 5;                    // inmutable
let mut y = 5;                // mutable
let x = x + 1;                // shadowing
const MAX: u32 = 100;         // constante (tipo obligatorio)
// ---------- TIPOS ----------
let a: i32 = -5;      let b: u64 = 10;     let c: f64 = 2.5;
let d: bool = true;   let e: char = 'ñ';   let f: String = String::from("hi");
let g: &str = "hi";

// ---------- OPERADORES ----------
+  -  *  /  %                 // aritméticos
==  !=  <  >  <=  >=         // comparación
&&  ||  !                     // lógicos
&  |  ^  <<  >>              // bits
+=  -=  *=  /=  %=            // compuestos
2_i32.pow(3)   2.0_f64.powi(3)   // potencias

// ---------- CONVERSIONES ----------
x as f64                      // as: directo (puede truncar)
i64::from(x)   x.into()       // sin pérdida
u8::try_from(x)               // puede fallar -> Result
n.to_string()                 // número -> texto
"42".parse::<i32>()           // texto -> número (Result)

// ---------- CONDICIONALES ----------
if cond { } else if cond2 { } else { }
let v = if cond { 1 } else { 2 };
match valor {
    1 => ...,
    2 | 3 => ...,
    4..=9 => ...,
    n if n < 0 => ...,
    _ => ...,
}

// ---------- BUCLES ----------
loop { break; }               // infinito
let r = loop { break 5; };    // devuelve valor
while cond { }
for i in 0..5 { }             // 0..4
for i in 1..=5 { }            // 1..5
for i in (0..5).rev() { }
for i in (0..10).step_by(2) { }
for (i, x) in array.iter().enumerate() { }
'etiqueta: for ... { break 'etiqueta; }

// ---------- FUNCIONES ----------
fn nombre(p1: i32, p2: &str) -> i32 {
    p1 * 2                    // sin ; = retorno
}
fn par(n: i32) -> (i32, i32) { (n, n * 2) }   // varios valores

// ---------- TUPLAS Y ARRAYS ----------
let t = (1, "a", 2.5);  t.0  t.1  t.2
let (a, b, c) = t;
let arr = [1, 2, 3];    arr[0]   arr.len()
let ceros = [0; 5];
let matriz = [[0; 3]; 3];
```

---

# Errores frecuentes del compilador

| Código | Mensaje (resumido) | Causa típica | Solución |
|---|---|---|---|
| `E0384` | cannot assign twice to immutable variable | Reasignar una variable sin `mut` | Agregar `mut` |
| `E0308` | mismatched types | Tipos distintos (ej. `i32` vs `f64`, `if` con ramas distintas, `;` extra al final de una función) | Convertir con `as`/`into`, igualar tipos o quitar el `;` |
| `E0277` | cannot add/multiply `i32` by `f64` / cannot be indexed by | Operar tipos distintos, o indexar `String` / indexar array con `i32` | Convertir el tipo, usar `.chars()`, usar `usize` |
| `E0381` | used binding isn't initialized | Usar una variable declarada sin valor | Asignarla antes de usarla |
| `E0425` | cannot find value | Variable fuera de su scope o no declarada | Revisar dónde se declaró |
| `E0382` | borrow of moved value | Usar un `String` después de moverlo | `.clone()` o pasar referencia `&` |
| `E0004` | non-exhaustive patterns | `match` que no cubre todos los casos | Agregar `_ => ...` |
| `E0106` | missing lifetime specifier | Función que devuelve `&str` sin más | Devolver `String` o `&'static str` |
| `E0435` | attempt to use a non-constant value in a constant | Tamaño de array con una variable | Usar `const` |
| `E0608` / `E0609` | cannot index into a tuple / no field | `t[0]` o `t.5` en tuplas | Usar `t.0`, `t.1`... dentro del rango |
| (panic) | attempt to add with overflow | Pasarse del rango de un entero | Usar un tipo más grande o `checked_add` |
| (panic) | index out of bounds | Índice fuera del array | Validar con `.len()` o usar `.get()` |
| (panic) | attempt to divide by zero | Entero entre cero | Verificar el divisor antes |
| (panic) | called `unwrap()` on an `Err` / `None` | `parse().unwrap()` con texto inválido | Usar `unwrap_or`, `match` o `expect` |

💡 **Hábito para tus estudiantes:** *leer el error completo de arriba hacia abajo*. El compilador de Rust suele explicar el problema, señalar la línea exacta y sugerir la corrección.

---

# Plan de clases sugerido

Propuesta de 5 sesiones de ~90 minutos (ajustable):

| Sesión | Contenido | Actividad práctica |
|---|---|---|
| **1** | Preparación del entorno + Módulo 1 (variables, `mut`, `const`, shadowing) + inicio del Módulo 2 (enteros y flotantes) | Ejercicios 1.1, 1.2 y declarar variables de distintos tipos |
| **2** | Módulo 2 (bool, char, String) + Módulo 3 (operadores y conversiones) | Ejercicios 2.1, 2.2, 3.1, 3.2, 3.3 |
| **3** | Módulo 4 (`if`, `match`) + Módulo 5 (`loop`, `while`, `for`) | Ejercicios 4.1, 4.2, 5.1–5.4 |
| **4** | Módulo 6 (funciones) + Módulo 7 (scope) | Ejercicios 6.1–6.3, 7.1, 7.2 |
| **5** | Módulo 8 (tuplas y arrays) + Proyecto integrador | Ejercicios 8.1–8.3 y modificar el proyecto de notas |

### Recomendaciones de enseñanza

1. **Deja que el compilador sea el segundo profesor.** Provoca errores a propósito (descomenta las líneas ❌) y pide que expliquen el mensaje antes de corregirlo.
2. **Pide predicciones antes de ejecutar:** "¿qué imprime?", "¿compila o no?". Es la forma más rápida de fijar conceptos como scope, shadowing y conversiones.
3. **Usa el Playground** durante la clase para que nadie se atrase por problemas de instalación.
4. **Insiste en tres ideas centrales del Nivel 1:**
   - Todo es **inmutable por defecto** (`mut` es explícito).
   - **Los tipos nunca se mezclan solos** (la conversión es explícita).
   - **Casi todo es una expresión** (`if`, `match`, bloques, último valor de una función).
5. **Ejercicio de cierre de cada sesión:** que cada estudiante modifique el código de ejemplo (cambiar tipos, agregar casos al `match`, cambiar rangos) y observe qué pasa.

### Qué sigue (Nivel 2, para referencia)

Ownership y borrowing, referencias, slices, `Vec`, `struct`, `enum`, `Option` y `Result`, manejo de errores, módulos y `cargo`.

---

*Fin del Nivel 1. ¡A programar en Rust! 🦀*
