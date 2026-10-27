---
title: "Rust — Nivel 2: Estructuras y organización del código"
description: "Modela tus propios tipos, organiza módulos, trabaja con colecciones y crea código reutilizable con genéricos y traits."
language: rust
level: 2
levelLabel: "Nivel 02"
category: "Estructuras y organización"
duration: "Intermedio"
order: 2
slug: rust-nivel-2
projectSlug: "rust-nivel-2-inventario-logistico"
tags: ["ownership", "structs", "enums", "módulos", "Cargo", "colecciones", "genéricos", "traits"]
---

# 🦀 Curso de Rust — Nivel 2: Estructuras y organización del código

> **Objetivo:** que tus estudiantes pasen de escribir programas de un solo archivo con tipos básicos a **modelar datos propios, organizar el código en módulos y archivos, usar colecciones y escribir código reutilizable** con genéricos y traits.
>
> **Requisitos:** haber completado el Nivel 1 (variables, tipos, `if`/`match`, bucles, funciones, scope, tuplas y arrays).
>
> **Versión recomendada:** Rust estable reciente. Todo el código funciona con las ediciones 2021 y 2024 (las versiones recientes de `cargo new` usan 2024 por defecto).

---

## Índice

0. [Puente: lo mínimo de ownership, referencias, slices, `Option` y `Result`](#módulo-0-puente-lo-mínimo-de-ownership-referencias-slices-option-y-result)
1. [Structs: crear tipos de datos propios](#módulo-1-structs-crear-tipos-de-datos-propios)
2. [Enums: representar estados y alternativas](#módulo-2-enums-representar-estados-y-alternativas)
3. [Métodos e `impl`](#módulo-3-métodos-e-impl)
4. [Módulos: `mod`, `pub` y organización de archivos](#módulo-4-módulos-mod-pub-y-organización-de-archivos)
5. [Crates, paquetes y Cargo](#módulo-5-crates-paquetes-y-cargo)
6. [Colecciones: `Vec<T>`, `HashMap<K, V>` y `HashSet<T>`](#módulo-6-colecciones-vect-hashmapk-v-y-hashsett)
7. [Genéricos y funciones reutilizables](#módulo-7-genéricos-y-funciones-reutilizables)
8. [Traits: comportamientos compartidos](#módulo-8-traits-comportamientos-compartidos)
9. [Proyecto integrador: inventario y abastecimiento logístico](#proyecto-integrador-inventario-y-abastecimiento-logistico)
10. [Ejercicios con soluciones](#ejercicios-con-soluciones)
11. [Hoja de referencia rápida](#hoja-de-referencia-rápida)
12. [Errores frecuentes del compilador](#errores-frecuentes-del-compilador)
13. [Plan de clases sugerido](#plan-de-clases-sugerido)

**Leyenda:**

- ✅ = se puede / es correcto
- ❌ = NO se puede / no compila
- ⚠️ = compila, pero cuidado (puede fallar al ejecutar o dar resultados inesperados)

---

# Módulo 0: Puente: lo mínimo de ownership, referencias, slices, `Option` y `Result`

> En el Nivel 1 marcamos estos temas como "adelanto". Aquí los vemos **solo hasta donde hacen falta** para entender structs, métodos y colecciones. No es un curso completo de ownership: es el **kit de supervivencia**.

## 0.1 Ownership (propiedad)

Tres reglas:

1. Todo valor tiene **un dueño** (una variable).
2. Solo puede haber **un dueño a la vez**.
3. Cuando el dueño sale de su ámbito, el valor **se libera** automáticamente.

### Tipos que se **copian** vs tipos que se **mueven**

```rust
fn main() {
    // Tipos simples (enteros, f64, bool, char): se COPIAN
    let a = 5;
    let b = a;
    println!("{a} {b}");                 // ✅ ambos válidos

    // String (y otros tipos con memoria dinámica): se MUEVEN
    let s1 = String::from("hola");
    let s2 = s1;                         // el dueño ahora es s2
    // println!("{s1}");                 // ❌ E0382: borrow of moved value
    println!("{s2}");                    // ✅

    // Si necesitas dos copias independientes: clone()
    let s3 = String::from("hola");
    let s4 = s3.clone();
    println!("{s3} {s4}");               // ✅
}
```

### Pasar un valor a una función también lo mueve

```rust
fn consumir(texto: String) {
    println!("Consumí: {texto}");
}   // aquí `texto` se libera

fn main() {
    let mensaje = String::from("hola");
    consumir(mensaje);
    // println!("{mensaje}");    // ❌ E0382: mensaje ya fue movido a la función
}
```

Solución más común: **prestar** el valor con una referencia (siguiente sección).

## 0.2 Referencias: prestar sin ceder la propiedad

- `&T` → referencia **compartida** (solo lectura). Puede haber **muchas**.
- `&mut T` → referencia **exclusiva** (lectura y escritura). Solo **una** a la vez.

```rust
fn longitud(texto: &String) -> usize {
    texto.len()                  // solo lee
}

fn agregar_signo(texto: &mut String) {
    texto.push('!');             // modifica el original
}

fn main() {
    let mut saludo = String::from("hola");

    let n = longitud(&saludo);           // presta de solo lectura
    println!("{n}");

    agregar_signo(&mut saludo);          // presta con permiso de escritura
    println!("{saludo}");                // hola!   (sigue siendo nuestro)
}
```

Para modificar un número a través de `&mut` se usa `*` (desreferenciar):

```rust
fn incrementar(n: &mut i32) {
    *n += 1;
}

fn main() {
    let mut x = 10;
    incrementar(&mut x);
    println!("{x}");        // 11
}
```

### Las reglas del préstamo

> **En un momento dado puedes tener: muchas `&T` ó una sola `&mut T`, pero nunca ambas a la vez.**

```rust
fn main() {
    let mut s = String::from("hola");

    let r1 = &s;
    let r2 = &s;                 // ✅ varias lecturas a la vez
    println!("{r1} {r2}");       // después de este uso, r1 y r2 "terminan"

    let r3 = &mut s;             // ✅ ahora sí: ya nadie más lee
    r3.push_str(" mundo");
    println!("{r3}");
}
```

```rust
fn main() {
    let mut s = String::from("hola");

    // ❌ E0502: no se puede prestar como mutable mientras hay una lectura activa
    // let lectura = &s;
    // let escritura = &mut s;
    // println!("{lectura}");

    // ❌ E0499: dos referencias mutables a la vez
    // let a = &mut s;
    // let b = &mut s;
    // a.push('x');

    // ❌ E0596: pedir &mut de una variable que no es mut
    // let fija = String::from("x");
    // let r = &mut fija;

    s.push('!');
}
```

**Por qué existen estas reglas:** evitan que alguien modifique un dato mientras otro lo está leyendo (condiciones de carrera y errores de memoria), todo **en tiempo de compilación**.

### Buenas prácticas rápidas

| Quiero... | Parámetro |
|---|---|
| Solo leer un texto | `&str` |
| Solo leer un valor grande | `&T` |
| Modificar el original | `&mut T` |
| Tomar la propiedad (consumirlo) | `T` |
| Tipos simples (`i32`, `f64`, `bool`) | `T` directo (se copian) |

## 0.3 Slices: una "ventana" a parte de una colección

Un **slice** es una referencia a una porción contigua: `&str` (texto) y `&[T]` (secuencia).

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    let parte: &[i32] = &a[1..4];       // posiciones 1, 2, 3
    println!("{:?}", parte);            // [20, 30, 40]
    println!("{:?}", &a[..2]);          // [10, 20]
    println!("{:?}", &a[3..]);          // [40, 50]

    let texto = String::from("hola mundo");
    let palabra: &str = &texto[0..4];   // "hola"
    println!("{palabra}");
}
```

**Su gran utilidad:** una función con `&[T]` acepta **arrays y `Vec` de cualquier tamaño**.

```rust
fn suma(numeros: &[i32]) -> i32 {
    let mut total = 0;
    for n in numeros {
        total += n;              // n es &i32; i32 += &i32 funciona
    }
    total
}

fn main() {
    let array = [1, 2, 3];
    let vector = vec![10, 20, 30, 40];   // (Vec: lo veremos en el Módulo 6)

    println!("{}", suma(&array));         // 6
    println!("{}", suma(&vector));        // 100
    println!("{}", suma(&vector[1..3]));  // 50
}
```

Con `&mut [T]` puedes modificar los elementos:

```rust
fn duplicar_todos(numeros: &mut [i32]) {
    for n in numeros.iter_mut() {
        *n *= 2;
    }
}

fn main() {
    let mut a = [1, 2, 3];
    duplicar_todos(&mut a);
    println!("{:?}", a);        // [2, 4, 6]
}
```

## 0.4 `Option<T>`: "puede haber un valor o no"

Rust **no tiene `null`**. Cuando algo puede no existir, se usa `Option`, que es un enum con dos variantes:

- `Some(valor)` → hay valor
- `None` → no hay

```rust
fn primer_par(numeros: &[i32]) -> Option<i32> {
    for n in numeros {
        if n % 2 == 0 {
            return Some(*n);
        }
    }
    None
}

fn main() {
    let a = [1, 3, 4, 7];
    let b = [1, 3, 5];

    // Forma 1: match (obliga a cubrir los dos casos)
    match primer_par(&a) {
        Some(n) => println!("Primer par: {n}"),
        None => println!("No hay pares"),
    }

    // Forma 2: if let (cuando solo te importa el caso Some)
    if let Some(n) = primer_par(&b) {
        println!("Primer par: {n}");
    } else {
        println!("No hay pares en b");
    }

    // Forma 3: valor por defecto
    let n = primer_par(&b).unwrap_or(0);
    println!("{n}");

    // Consultas
    println!("{}", primer_par(&a).is_some());   // true
    println!("{}", primer_par(&b).is_none());   // true

    // ⚠️ unwrap() hace panic si es None:
    // let x = primer_par(&b).unwrap();          // ❌ panic en ejecución
    // expect() es igual pero con tu mensaje:
    // let y = primer_par(&b).expect("debía haber un par");
}
```

⚠️ **Un `Option<i32>` NO es un `i32`**: no puedes sumarlo directamente. Debes "abrirlo" con `match`, `if let` o `unwrap_or`.

## 0.5 `Result<T, E>`: "salió bien o salió mal"

Para operaciones que **pueden fallar con una razón**: `Ok(valor)` o `Err(error)`.

```rust
fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("no se puede dividir entre cero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match dividir(10, 2) {
        Ok(r) => println!("Resultado: {r}"),
        Err(e) => println!("Error: {e}"),
    }

    match dividir(10, 0) {
        Ok(r) => println!("Resultado: {r}"),
        Err(e) => println!("Error: {e}"),
    }

    // Atajos
    let valor = dividir(8, 0).unwrap_or(-1);
    println!("{valor}");                        // -1

    if let Err(e) = dividir(1, 0) {
        println!("falló: {e}");
    }
}
```

### El operador `?`: propagar el error

Dentro de una función que devuelve `Result`, `?` significa: *"si es `Ok`, dame el valor; si es `Err`, sal de la función devolviendo ese error"*.

```rust
use std::num::ParseIntError;

fn doble_del_texto(texto: &str) -> Result<i32, ParseIntError> {
    let numero = texto.trim().parse::<i32>()?;    // si falla, retorna el Err
    Ok(numero * 2)
}

fn main() {
    println!("{:?}", doble_del_texto("21"));      // Ok(42)
    println!("{:?}", doble_del_texto("abc"));     // Err(ParseIntError { kind: InvalidDigit })
}
```

- ✅ `?` solo se usa en funciones que devuelven `Result` (o `Option`).
- ❌ No funciona dentro de `main()` si `main` no devuelve `Result`.

### `Option` vs `Result`

| | `Option<T>` | `Result<T, E>` |
|---|---|---|
| Significado | Hay valor o no | Éxito o error con motivo |
| Variantes | `Some(T)` / `None` | `Ok(T)` / `Err(E)` |
| Ejemplo | Buscar en una lista | Convertir texto a número, validar datos |

## 0.6 Resumen del puente

1. Los tipos con memoria dinámica (`String`, `Vec`, structs con ellos) **se mueven**; los simples **se copian**.
2. `&` presta para leer (muchos a la vez); `&mut` presta para escribir (uno solo).
3. Un slice `&[T]` / `&str` es una ventana de solo lectura (o `&mut [T]` para escribir).
4. `Option` = puede faltar; `Result` = puede fallar.
5. `match`, `if let`, `unwrap_or` y `?` son las herramientas para abrir ambos.

---

# Módulo 1: Structs: crear tipos de datos propios

Un **struct** agrupa varios datos relacionados bajo **un nombre**, y cada dato tiene su **nombre de campo**. Es la evolución natural de la tupla.

## 1.1 Definir y crear un struct

```rust
struct Usuario {
    nombre: String,
    email: String,
    edad: u32,
    activo: bool,
}

fn main() {
    let u1 = Usuario {
        nombre: String::from("Ana"),
        email: String::from("ana@mail.com"),
        edad: 25,
        activo: true,
    };

    println!("{} tiene {} años", u1.nombre, u1.edad);   // acceso con punto
}
```

- ✅ Nombre del struct en `PascalCase` (`Usuario`, `ProductoDeTienda`).
- ✅ Campos en `snake_case`.
- ✅ El orden al crear la instancia **no** importa.
- ❌ **Debes** dar valor a **todos** los campos (no hay valores por defecto en la definición).

```rust
struct Punto { x: f64, y: f64 }

fn main() {
    let p = Punto { y: 2.0, x: 1.0 };      // ✅ distinto orden, sin problema
    // let q = Punto { x: 1.0 };           // ❌ E0063: missing field `y`
    // let r = Punto { x: 1.0, y: 2.0, z: 3.0 };   // ❌ E0560: no existe el campo `z`
    println!("{} {}", p.x, p.y);
}
```

## 1.2 Mutabilidad: se decide por **instancia**, no por campo

```rust
struct Contador { valor: u32, nombre: String }

fn main() {
    let mut c = Contador { valor: 0, nombre: String::from("clics") };
    c.valor += 1;                    // ✅ la instancia completa es mut
    c.nombre.push_str("_total");
    println!("{} = {}", c.nombre, c.valor);

    let fijo = Contador { valor: 5, nombre: String::from("fijo") };
    // fijo.valor = 6;               // ❌ E0594: la instancia no es mut
    println!("{}", fijo.valor);

    // ❌ No existe marcar un solo campo como mut dentro de la definición:
    // struct Malo { mut valor: u32 }
}
```

## 1.3 Atajos útiles

### Inicialización abreviada (*field init shorthand*)

Si la variable se llama igual que el campo, no repitas el nombre:

```rust
struct Usuario { nombre: String, edad: u32 }

fn crear_usuario(nombre: String, edad: u32) -> Usuario {
    Usuario { nombre, edad }          // equivale a nombre: nombre, edad: edad
}

fn main() {
    let u = crear_usuario(String::from("Luis"), 30);
    println!("{} {}", u.nombre, u.edad);
}
```

### Actualización con `..` (*struct update syntax*)

Crea una instancia nueva tomando el resto de campos de otra:

```rust
#[derive(Debug, Clone)]
struct Config {
    host: String,
    puerto: u16,
    debug: bool,
}

fn main() {
    let base = Config {
        host: String::from("localhost"),
        puerto: 8080,
        debug: false,
    };

    // ✅ Copia con .clone() para seguir usando `base`
    let produccion = Config {
        host: String::from("miapp.com"),
        ..base.clone()
    };
    println!("{:?}", produccion);

    // ⚠️ Sin clone(), los campos String de `base` se MUEVEN a la nueva instancia
    //    y ya no podrías usar `base` completo.
    let otra = Config { puerto: 9000, ..base };
    println!("{:?}", otra);
    // println!("{:?}", base);          // ❌ base fue (parcialmente) movido
}
```

## 1.4 Imprimir structs: `#[derive(Debug)]`

Un struct **no se puede imprimir** por defecto. Para depurar, agrega `#[derive(Debug)]` y usa `{:?}` o `{:#?}`:

```rust
#[derive(Debug)]
struct Punto {
    x: f64,
    y: f64,
}

fn main() {
    let p = Punto { x: 1.5, y: -2.0 };

    println!("{:?}", p);       // Punto { x: 1.5, y: -2.0 }
    println!("{:#?}", p);      // versión multilínea, más legible
    // println!("{}", p);      // ❌ E0277: `Punto` no implementa Display
}
```

(En el Módulo 8 verás cómo implementar `Display` para controlar el formato de `{}`.)

## 1.5 Otros traits "derivables" muy útiles

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Punto {
    x: f64,
    y: f64,
}

fn main() {
    let a = Punto { x: 1.0, y: 2.0 };
    let b = a;                         // ✅ Copy: se copia, `a` sigue válido
    let c = a.clone();                 // ✅ Clone

    println!("{}", a == b);            // ✅ PartialEq permite ==  -> true
    println!("{:?} {:?}", a, c);
}
```

| `derive` | Qué te da |
|---|---|
| `Debug` | Imprimir con `{:?}` |
| `Clone` | Duplicar con `.clone()` |
| `Copy` | Que se copie solo al asignar/pasar (requiere `Clone`) |
| `PartialEq` | Comparar con `==` y `!=` |
| `Default` | Valor por defecto con `Tipo::default()` |

⚠️ **`Copy` solo se puede si TODOS los campos son `Copy`**:

```rust
// ❌ E0204: String no es Copy, así que el struct no puede serlo
// #[derive(Clone, Copy)]
// struct Persona { nombre: String, edad: u32 }

#[derive(Clone)]          // ✅ Clone sí funciona
struct Persona { nombre: String, edad: u32 }
```

### `Default`: valores por defecto

```rust
#[derive(Debug, Default)]
struct Ajustes {
    volumen: u8,          // 0
    silencio: bool,       // false
    usuario: String,      // ""
}

fn main() {
    let a = Ajustes::default();
    println!("{:?}", a);

    // Combinado con .. para cambiar solo lo que importa:
    let b = Ajustes { volumen: 70, ..Default::default() };
    println!("{:?}", b);
}
```

## 1.6 Structs dentro de structs

```rust
#[derive(Debug)]
struct Direccion {
    ciudad: String,
    calle: String,
}

#[derive(Debug)]
struct Cliente {
    nombre: String,
    direccion: Direccion,
}

fn main() {
    let c = Cliente {
        nombre: String::from("Ana"),
        direccion: Direccion {
            ciudad: String::from("Medellín"),
            calle: String::from("Calle 10"),
        },
    };

    println!("{}", c.direccion.ciudad);     // encadenar con punto
    println!("{:#?}", c);
}
```

## 1.7 Desestructurar un struct

```rust
struct Punto { x: i32, y: i32 }

struct Usuario { nombre: String, edad: u32, activo: bool }

fn main() {
    let p = Punto { x: 3, y: 7 };
    let Punto { x, y } = p;                  // crea variables x e y
    println!("{x} {y}");

    let u = Usuario { nombre: String::from("Ana"), edad: 25, activo: true };
    let Usuario { nombre, .. } = u;          // `..` ignora el resto de campos
    println!("{nombre}");

    // También en match:
    let q = Punto { x: 0, y: 5 };
    match q {
        Punto { x: 0, y } => println!("sobre el eje Y en {y}"),
        Punto { x, y: 0 } => println!("sobre el eje X en {x}"),
        Punto { x, y } => println!("({x}, {y})"),
    }
}
```

## 1.8 Tuple structs y unit structs

### Tuple struct: struct con campos sin nombre

```rust
struct Color(u8, u8, u8);
struct Metros(f64);
struct Kilometros(f64);

fn main() {
    let rojo = Color(255, 0, 0);
    println!("{} {} {}", rojo.0, rojo.1, rojo.2);

    let distancia = Metros(500.0);
    println!("{}", distancia.0);

    // ✅ Metros y Kilometros son tipos DISTINTOS aunque ambos guarden un f64:
    // let k: Kilometros = distancia;       // ❌ E0308: no se pueden mezclar
    let _k = Kilometros(0.5);
}
```

Útil para evitar errores de unidades ("patrón *newtype*").

### Unit struct: sin campos

```rust
struct Marcador;           // no guarda datos; sirve para implementar comportamiento

fn main() {
    let _m = Marcador;
}
```

## 1.9 Texto dentro de un struct: usa `String`

```rust
// ❌ E0106: missing lifetime specifier
// struct Usuario {
//     nombre: &str,
// }

// ✅ Por ahora, siempre String en los campos de texto:
struct Usuario {
    nombre: String,
}
```

> `&str` en un campo exige indicar **lifetimes** (tema posterior). Regla simple para clase: *"en structs, los campos de texto son `String`"*.

## 1.10 Cuándo usar tupla, array o struct

| Necesitas... | Usa |
|---|---|
| Agrupar 2-3 valores temporales | Tupla |
| Muchos elementos del mismo tipo | Array / `Vec` |
| Datos con significado, nombres de campo, o más de 3 valores | **Struct** |

---

# Módulo 2: Enums: representar estados y alternativas

Un **enum** representa un valor que puede ser **una de varias alternativas** (variantes). Mientras el struct dice *"esto Y esto Y esto"*, el enum dice *"esto O esto O esto"*.

## 2.1 Enum simple

```rust
#[derive(Debug)]
enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}

fn main() {
    let d = Direccion::Norte;            // se accede con ::
    println!("{:?}", d);                 // Norte
}
```

- ✅ Enum y variantes en `PascalCase`.
- ✅ Una variable de tipo `Direccion` **solo** puede ser una de esas 4 variantes. Nada más.

### `match` sobre un enum (exhaustivo)

```rust
enum Direccion { Norte, Sur, Este, Oeste }

fn describir(d: Direccion) -> &'static str {
    match d {
        Direccion::Norte => "arriba",
        Direccion::Sur => "abajo",
        Direccion::Este => "derecha",
        Direccion::Oeste => "izquierda",
    }
}

fn main() {
    println!("{}", describir(Direccion::Este));
}
```

Si olvidas una variante, el compilador **no te deja compilar** (`E0004: non-exhaustive patterns`). Si agregas una variante nueva más adelante, el compilador te señala **todos** los `match` que debes actualizar. Esa es la gran ventaja de los enums.

### Comparar enums

```rust
#[derive(Debug, PartialEq, Clone, Copy)]
enum Estado { Activo, Inactivo }

fn main() {
    let e = Estado::Activo;

    println!("{}", e == Estado::Activo);     // ✅ requiere derive(PartialEq)
    // Sin PartialEq: ❌ E0369 al usar ==

    // Alternativa sin PartialEq: la macro matches!
    println!("{}", matches!(e, Estado::Inactivo));   // false
}
```

### Valores numéricos asociados

```rust
enum Nivel {
    Bajo = 1,
    Medio = 5,
    Alto = 10,
}

fn main() {
    println!("{}", Nivel::Alto as i32);      // 10
    println!("{}", Nivel::Bajo as i32);      // 1
}
```

## 2.2 Enums con datos dentro

Aquí Rust se separa de enums de otros lenguajes: **cada variante puede guardar datos distintos**.

```rust
#[derive(Debug)]
enum Mensaje {
    Salir,                              // sin datos
    Mover { x: i32, y: i32 },           // campos con nombre (como un struct)
    Escribir(String),                   // un dato (como tuple struct)
    CambiarColor(u8, u8, u8),           // varios datos sin nombre
}

fn procesar(m: Mensaje) {
    match m {
        Mensaje::Salir => println!("Saliendo..."),
        Mensaje::Mover { x, y } => println!("Moviendo a ({x}, {y})"),
        Mensaje::Escribir(texto) => println!("Escribiendo: {texto}"),
        Mensaje::CambiarColor(r, g, b) => println!("Color RGB({r}, {g}, {b})"),
    }
}

fn main() {
    procesar(Mensaje::Mover { x: 10, y: -5 });
    procesar(Mensaje::Escribir(String::from("hola")));
    procesar(Mensaje::CambiarColor(255, 128, 0));
    procesar(Mensaje::Salir);
}
```

> **Sin enums con datos** tendrías que crear 4 structs distintos y no podrías guardarlos "como el mismo tipo". El enum los unifica y obliga a manejar cada caso.

### ⚠️ `match` sobre un enum con `String` dentro **mueve** el contenido

```rust
enum Estado {
    Cancelado(String),
    Activo,
}

fn main() {
    let e = Estado::Cancelado(String::from("sin stock"));

    // Usa una referencia para NO consumir el enum:
    match &e {
        Estado::Cancelado(motivo) => println!("Cancelado: {motivo}"),
        Estado::Activo => println!("Activo"),
    }

    println!("sigo pudiendo usar e");       // ✅ gracias al &
    let _ = e;
}
```

## 2.3 `Option` y `Result` **son enums**

Ya los usaste en el puente. Esta es su definición real en la librería estándar:

```rust
// (esto ya existe en Rust; lo mostramos solo para que veas que son enums normales)
enum Option<T> {
    None,
    Some(T),
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

(La `<T>` se explica en el Módulo 7 de genéricos.) Por eso `match` funciona igual con ellos que con tus enums, y por eso Rust no necesita `null`.

## 2.4 Modelar estados: ejemplo realista

```rust
#[derive(Debug)]
enum EstadoPedido {
    Pendiente,
    Pagado { monto: f64 },
    Enviado { guia: String },
    Entregado,
    Cancelado(String),               // motivo
}

fn describir(estado: &EstadoPedido) -> String {
    match estado {
        EstadoPedido::Pendiente => String::from("Esperando pago"),
        EstadoPedido::Pagado { monto } => format!("Pagado: ${monto:.0}"),
        EstadoPedido::Enviado { guia } => format!("En camino (guía {guia})"),
        EstadoPedido::Entregado => String::from("Entregado ✅"),
        EstadoPedido::Cancelado(motivo) => format!("Cancelado: {motivo}"),
    }
}

fn main() {
    let pedidos = [
        EstadoPedido::Pendiente,
        EstadoPedido::Pagado { monto: 45_000.0 },
        EstadoPedido::Enviado { guia: String::from("GU-123") },
        EstadoPedido::Entregado,
        EstadoPedido::Cancelado(String::from("sin stock")),
    ];

    for p in &pedidos {
        println!("{}", describir(p));
    }
}
```

**Ventaja clave:** el "estado imposible" no existe. Un pedido `Pendiente` **no puede** tener guía de envío porque esa variante no tiene ese campo.

## 2.5 `if let` con enums

Cuando solo te interesa **una** variante:

```rust
enum Mensaje {
    Salir,
    Escribir(String),
}

fn main() {
    let m = Mensaje::Escribir(String::from("hola"));

    if let Mensaje::Escribir(texto) = &m {
        println!("Texto: {texto}");
    } else {
        println!("Era otra cosa");
    }

    let _ = Mensaje::Salir;
}
```

## 2.6 Comparación

| | Struct | Enum |
|---|---|---|
| Idea | **Y**: todos los campos a la vez | **O**: una variante a la vez |
| Ejemplo | `Usuario { nombre, edad }` | `Pago::Efectivo` o `Pago::Tarjeta(..)` |
| Uso típico | Entidades con atributos | Estados, tipos de evento, resultados |
| `match` obligatorio | No | Sí (para actuar según variante) |

## 2.7 Lo que NO se puede con enums

```rust
#[derive(Debug)]
enum Color { Rojo, Verde }

// ❌ Una variante NO es un tipo:
// fn f(c: Color::Rojo) { }          // ERROR: usa `Color`, no la variante

fn main() {
    let c = Color::Rojo;
    // println!("{c}");              // ❌ sin Display (usa {:?} con derive(Debug))
    // let x = Rojo;                 // ❌ falta el prefijo: Color::Rojo
    println!("{:?}", c);
    let _ = Color::Verde;
}
```

---

# Módulo 3: Métodos e `impl`

Un **método** es una función asociada a un tipo. Se define dentro de un bloque `impl` (implementación) y se llama con punto: `valor.metodo()`.

## 3.1 Primer `impl`

```rust
struct Rectangulo {
    ancho: f64,
    alto: f64,
}

impl Rectangulo {
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }

    fn perimetro(&self) -> f64 {
        2.0 * (self.ancho + self.alto)
    }

    fn es_cuadrado(&self) -> bool {
        self.ancho == self.alto
    }
}

fn main() {
    let r = Rectangulo { ancho: 3.0, alto: 4.0 };

    println!("{}", r.area());          // 12
    println!("{}", r.perimetro());     // 14
    println!("{}", r.es_cuadrado());   // false
}
```

El primer parámetro **`self`** representa la instancia sobre la que se llama el método (equivale al `this` de otros lenguajes, pero se escribe explícito).

## 3.2 Las tres formas de `self`

| Forma | Significado | Cuándo usarla |
|---|---|---|
| `&self` | Presta **para leer** | Consultas, cálculos (la más común) |
| `&mut self` | Presta **para modificar** | Cambiar el estado del objeto |
| `self` | **Toma la propiedad** (consume el objeto) | Transformarlo en otra cosa, "cerrar" el objeto |

```rust
struct Contador {
    valor: u32,
}

impl Contador {
    fn leer(&self) -> u32 {              // solo lectura
        self.valor
    }

    fn incrementar(&mut self) {          // modifica
        self.valor += 1;
    }

    fn terminar(self) -> u32 {           // consume el Contador
        println!("Contador cerrado");
        self.valor
    }
}

fn main() {
    let mut c = Contador { valor: 0 };
    c.incrementar();
    c.incrementar();
    println!("{}", c.leer());            // 2

    let final_valor = c.terminar();      // c se consume aquí
    println!("{final_valor}");
    // println!("{}", c.leer());         // ❌ E0382: c fue movido por terminar(self)
}
```

```rust
struct Contador { valor: u32 }
impl Contador {
    fn incrementar(&mut self) { self.valor += 1; }
}

fn main() {
    let c = Contador { valor: 0 };
    // c.incrementar();     // ❌ E0596: c no es mut, no se puede llamar un método &mut self
    let mut d = c;
    d.incrementar();         // ✅
}
```

## 3.3 Funciones asociadas (sin `self`): constructores
Si una función dentro de `impl` **no** tiene `self`, es una **función asociada** y se llama con `Tipo::funcion()` (con `::`, no con punto). Es la forma estándar de crear constructores. Por convención el principal se llama `new`.

```rust
struct Rectangulo {
    ancho: f64,
    alto: f64,
}

impl Rectangulo {
    // Funciones asociadas (sin self)
    fn new(ancho: f64, alto: f64) -> Self {
        Self { ancho, alto }
    }

    fn cuadrado(lado: f64) -> Self {
        Self { ancho: lado, alto: lado }
    }

    // Métodos (con self)
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }
}

fn main() {
    let a = Rectangulo::new(3.0, 4.0);       // con ::
    let b = Rectangulo::cuadrado(5.0);

    println!("{}", a.area());                // 12
    println!("{}", b.area());                // 25

    // let c = a.new(1.0, 2.0);              // ❌ new no tiene self: no se llama con punto
}
```

- **`Self`** (con mayúscula) es un alias del tipo del `impl`. `Self { ... }` equivale a `Rectangulo { ... }`.
- Los constructores no son una palabra especial del lenguaje: `new` es solo una **convención**.

## 3.4 Llamar un método de dos formas

```rust
struct Rectangulo { ancho: f64, alto: f64 }
impl Rectangulo {
    fn area(&self) -> f64 { self.ancho * self.alto }
}

fn main() {
    let r = Rectangulo { ancho: 2.0, alto: 3.0 };
    println!("{}", r.area());                  // forma normal
    println!("{}", Rectangulo::area(&r));      // forma explícita (equivalente)
}
```

Rust agrega `&` o `&mut` automáticamente cuando usas la sintaxis con punto.

## 3.5 Métodos con parámetros, constantes asociadas y varios `impl`

```rust
struct Rectangulo {
    ancho: f64,
    alto: f64,
}

impl Rectangulo {
    const LADOS: u32 = 4;                                  // constante asociada

    fn escalar(&mut self, factor: f64) {
        self.ancho *= factor;
        self.alto *= factor;
    }

    fn cabe_en(&self, otro: &Rectangulo) -> bool {         // recibe otra instancia prestada
        self.ancho <= otro.ancho && self.alto <= otro.alto
    }
}

// ✅ Puedes tener varios bloques impl para el mismo tipo:
impl Rectangulo {
    fn describir(&self) -> String {
        format!("{}x{}", self.ancho, self.alto)
    }
}

fn main() {
    let mut chico = Rectangulo { ancho: 1.0, alto: 2.0 };
    let grande = Rectangulo { ancho: 10.0, alto: 10.0 };

    println!("{}", chico.cabe_en(&grande));     // true
    chico.escalar(3.0);
    println!("{}", chico.describir());          // 3x6
    println!("{}", Rectangulo::LADOS);          // 4
}
```

## 3.6 Constructores que validan: devolver `Result`

```rust
#[derive(Debug)]
struct Cuenta {
    titular: String,
    saldo: f64,
}

impl Cuenta {
    fn new(titular: &str, saldo_inicial: f64) -> Result<Self, String> {
        if saldo_inicial < 0.0 {
            return Err(String::from("El saldo inicial no puede ser negativo"));
        }
        Ok(Self {
            titular: titular.to_string(),
            saldo: saldo_inicial,
        })
    }

    fn depositar(&mut self, monto: f64) -> Result<(), String> {
        if monto <= 0.0 {
            return Err(String::from("El monto debe ser positivo"));
        }
        self.saldo += monto;
        Ok(())
    }

    fn retirar(&mut self, monto: f64) -> Result<(), String> {
        if monto > self.saldo {
            Err(format!(
                "Fondos insuficientes: saldo {:.2}, intento {:.2}",
                self.saldo, monto
            ))
        } else {
            self.saldo -= monto;
            Ok(())
        }
    }
}

fn main() {
    let mut c = Cuenta::new("Ana", 100.0).unwrap();

    match c.retirar(30.0) {
        Ok(()) => println!("Retiro exitoso, saldo: {}", c.saldo),
        Err(e) => println!("Error: {e}"),
    }

    if let Err(e) = c.retirar(500.0) {
        println!("Error: {e}");
    }

    match Cuenta::new("Luis", -5.0) {
        Ok(_) => println!("creada"),
        Err(e) => println!("Error: {e}"),
    }

    let _ = c.depositar(50.0);
    println!("{:?}", c);
}
```

> Ventaja: **es imposible crear una `Cuenta` inválida** si el único camino para crearla es `new`. (En el Módulo 4 veremos cómo hacer que ese sea realmente el único camino con campos privados.)

## 3.7 Encadenar métodos (patrón *builder*)

Con `mut self` que devuelve `Self`, puedes encadenar llamadas:

```rust
#[derive(Debug)]
struct Servidor {
    host: String,
    puerto: u16,
    seguro: bool,
}

impl Servidor {
    fn new() -> Self {
        Self { host: String::from("localhost"), puerto: 80, seguro: false }
    }

    fn con_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    fn con_puerto(mut self, puerto: u16) -> Self {
        self.puerto = puerto;
        self
    }

    fn seguro(mut self) -> Self {
        self.seguro = true;
        self
    }
}

fn main() {
    let s = Servidor::new()
        .con_host("miapp.com")
        .con_puerto(443)
        .seguro();

    println!("{:?}", s);
}
```

## 3.8 Métodos en enums

Los enums también tienen `impl`:

```rust
#[derive(Debug)]
enum Semaforo {
    Rojo,
    Amarillo,
    Verde,
}

impl Semaforo {
    fn siguiente(&self) -> Semaforo {
        match self {
            Semaforo::Rojo => Semaforo::Verde,
            Semaforo::Verde => Semaforo::Amarillo,
            Semaforo::Amarillo => Semaforo::Rojo,
        }
    }

    fn segundos(&self) -> u32 {
        match self {
            Semaforo::Rojo => 30,
            Semaforo::Amarillo => 5,
            Semaforo::Verde => 25,
        }
    }
}

enum EstadoPedido {
    Pendiente,
    Enviado { guia: String },
    Cancelado(String),
}

impl EstadoPedido {
    fn describir(&self) -> String {
        match self {
            EstadoPedido::Pendiente => String::from("Esperando pago"),
            EstadoPedido::Enviado { guia } => format!("En camino, guía {guia}"),
            EstadoPedido::Cancelado(motivo) => format!("Cancelado: {motivo}"),
        }
    }

    fn esta_cerrado(&self) -> bool {
        matches!(self, EstadoPedido::Cancelado(_))
    }
}

fn main() {
    let mut s = Semaforo::Rojo;
    for _ in 0..4 {
        println!("{:?} durante {} s", s, s.segundos());
        s = s.siguiente();
    }

    let p = EstadoPedido::Enviado { guia: String::from("GU-9") };
    println!("{} | cerrado: {}", p.describir(), p.esta_cerrado());
    let _ = EstadoPedido::Pendiente;
    let _ = EstadoPedido::Cancelado(String::new());
}
```

## 3.9 Lo que NO existe

| ❌ | ✅ Alternativa |
|---|---|
| Herencia de structs | Composición (struct dentro de struct) y traits (Módulo 8) |
| `this` implícito | `self` explícito |
| Sobrecarga de métodos (mismo nombre, otros parámetros) | Nombres distintos: `new`, `con_capacidad`... |
| Constructor especial | Funciones asociadas como `new` |
| Métodos con parámetros opcionales | Varias funciones o patrón builder |

---

# Módulo 4: Módulos: `mod`, `pub` y organización de archivos

Cuando el proyecto crece, un solo archivo es inmanejable. Los **módulos** permiten **agrupar código relacionado** y **controlar qué es visible desde fuera**.

## 4.1 Módulos en línea (`mod` con llaves)

```rust
mod geometria {
    pub const PI: f64 = 3.14159;

    pub fn area_circulo(radio: f64) -> f64 {
        PI * radio * radio
    }

    fn funcion_privada() {          // sin pub: solo visible dentro del módulo
        println!("solo para uso interno");
    }

    pub fn usar_privada() {
        funcion_privada();          // ✅ dentro del mismo módulo sí se puede
    }
}

fn main() {
    println!("{}", geometria::area_circulo(2.0));
    println!("{}", geometria::PI);
    geometria::usar_privada();

    // geometria::funcion_privada();   // ❌ E0603: function `funcion_privada` is private
}
```

**Regla fundamental: TODO es privado por defecto.** Lo que quieras exponer debe llevar `pub`.

## 4.2 Rutas: cómo nombrar cosas

| Ruta | Significado |
|---|---|
| `crate::a::b` | **Absoluta**, desde la raíz del proyecto |
| `self::b` | Relativa al módulo actual |
| `super::b` | Relativa al **módulo padre** |
| `a::b` | Relativa, desde donde estás |

```rust
fn saludo_global() {
    println!("hola desde la raíz");
}

mod padre {
    pub fn llamar() {
        super::saludo_global();             // sube un nivel
        crate::saludo_global();             // ruta absoluta
        self::hijo::detalle();              // baja al submódulo
    }

    pub mod hijo {
        pub fn detalle() {
            println!("detalle en el hijo");
        }
    }
}

fn main() {
    padre::llamar();
    padre::hijo::detalle();
}
```

## 4.3 `use`: acortar rutas

```rust
mod geometria {
    pub mod formas {
        pub fn cuadrado(lado: f64) -> f64 { lado * lado }
        pub fn triangulo(base: f64, altura: f64) -> f64 { base * altura / 2.0 }
    }
}

use geometria::formas::cuadrado;                       // importar una función
use geometria::formas::{triangulo};                    // forma con llaves
// use geometria::formas::{cuadrado, triangulo};       // varias a la vez

fn main() {
    println!("{}", cuadrado(3.0));
    println!("{}", triangulo(4.0, 5.0));
    println!("{}", geometria::formas::cuadrado(2.0));  // la ruta larga sigue siendo válida
}
```

Más variantes:

```rust
use std::collections::HashMap;                  // de la librería estándar
use std::fmt::{self, Display};                  // self = el propio módulo fmt
use std::io::Result as IoResult;                // `as` para renombrar y evitar choques

// use std::collections::*;                     // ⚠️ glob: trae todo, evítalo (confunde de dónde viene cada cosa)

fn main() {
    let _m: HashMap<i32, i32> = HashMap::new();
}
```

**Convención:** importa el **módulo** para funciones (`use geometria::formas; formas::cuadrado(..)`) e importa el **tipo** directamente para structs/enums (`use std::collections::HashMap;`).

## 4.4 `pub` en structs, enums y campos

Esto es clave para la **encapsulación**:

- `pub struct` hace público el **tipo**, pero **sus campos siguen privados** salvo que cada uno lleve `pub`.
- En un `pub enum`, **todas las variantes** son públicas automáticamente.

```rust
mod banco {
    pub struct Cuenta {
        pub titular: String,        // público: se puede leer/escribir desde fuera
        saldo: f64,                 // PRIVADO: solo accesible dentro del módulo
    }

    impl Cuenta {
        pub fn new(titular: &str) -> Self {
            Cuenta { titular: titular.to_string(), saldo: 0.0 }
        }

        pub fn depositar(&mut self, monto: f64) {
            if monto > 0.0 {
                self.saldo += monto;
            }
        }

        pub fn saldo(&self) -> f64 {       // "getter" público
            self.saldo
        }
    }

    pub enum Movimiento {               // variantes públicas automáticamente
        Deposito,
        Retiro,
    }
}

use banco::{Cuenta, Movimiento};

fn main() {
    let mut c = Cuenta::new("Ana");
    c.depositar(100.0);
    println!("{} tiene {}", c.titular, c.saldo());

    let _m = Movimiento::Deposito;
    let _n = Movimiento::Retiro;

    // c.saldo = 1_000_000.0;          // ❌ E0616: el campo `saldo` es privado
    // let trampa = Cuenta { titular: String::from("X"), saldo: 5.0 };
    //                                 // ❌ E0451: no se puede construir con un campo privado
}
```

> Con campos privados + constructor `new`, **nadie puede crear una cuenta con datos inválidos ni modificar el saldo saltándose tus reglas**. Es la base del diseño seguro en Rust.

### Otros niveles de visibilidad

```rust
mod a {
    pub fn publica() {}                 // visible para todos
    pub(crate) fn solo_en_el_crate() {} // visible en todo el proyecto, pero no para quien lo use como librería
    pub(super) fn solo_para_el_padre() {} // visible solo para el módulo padre
    fn privada() {}                     // solo dentro de `a`
}

fn main() {
    a::publica();
    a::solo_en_el_crate();
    a::solo_para_el_padre();
}
```

## 4.5 Organizar en **archivos**

El módulo en archivo equivale al módulo en línea. Si escribes `mod nombre;` (con punto y coma, sin llaves), Rust busca el código en:

- `src/nombre.rs`, o
- `src/nombre/mod.rs` (estilo antiguo)

### Ejemplo con submódulos

```
mi_app/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── geometria.rs
    └── geometria/
        └── formas.rs
```

**`src/main.rs`**

```rust
mod geometria;                          // declara el módulo -> busca src/geometria.rs

use geometria::formas::Cuadrado;

fn main() {
    let c = Cuadrado { lado: 3.0 };
    println!("Área: {}", c.area());
    println!("Perímetro: {}", geometria::perimetro_cuadrado(3.0));
}
```

**`src/geometria.rs`**

```rust
pub mod formas;                         // declara el submódulo -> busca src/geometria/formas.rs

pub fn perimetro_cuadrado(lado: f64) -> f64 {
    4.0 * lado
}
```

**`src/geometria/formas.rs`**

```rust
pub struct Cuadrado {
    pub lado: f64,
}

impl Cuadrado {
    pub fn area(&self) -> f64 {
        self.lado * self.lado
    }
}
```

### Reglas de oro de los módulos en archivos

1. **Crear el archivo no basta**: hay que declarar `mod nombre;` en el archivo padre. Sin esa línea, Rust ignora el archivo.
2. `mod nombre;` se escribe **una sola vez**, en el padre. Quien quiera usarlo luego, hace `use`.
3. `use` **no** declara módulos: solo acorta rutas.
4. Los submódulos de `src/a.rs` van en `src/a/` (por ejemplo `src/a/b.rs`).
5. Dentro de otros módulos se llega al resto con `crate::...` o `super::...`.

### Re-exportar con `pub use`

Para que una estructura interna se vea "más arriba":

```rust
// src/geometria.rs
pub mod formas;
pub use formas::Cuadrado;               // ahora se puede usar como geometria::Cuadrado
```

## 4.6 Errores típicos con módulos

| Código | Mensaje | Causa | Solución |
|---|---|---|---|
| `E0603` | `function/struct is private` | Falta `pub` | Agregar `pub` |
| `E0616` | `field is private` | Campo sin `pub` | Hacer `pub` o dar método getter |
| `E0451` | `field is private` (al construir) | Struct literal con campo privado | Usar `new` |
| `E0433` / `E0432` | `failed to resolve` / `unresolved import` | Falta `mod x;` en el padre, o ruta mal escrita | Declarar el módulo, revisar la ruta |
| `E0583` | `file not found for module` | `mod x;` pero no existe `x.rs` | Crear el archivo en la ruta correcta |

---

# Módulo 5: Crates, paquetes y Cargo

## 5.1 Vocabulario

| Término | Qué es |
|---|---|
| **Crate** | La unidad de compilación: un **binario** (programa ejecutable, con `main`) o una **biblioteca** (código reutilizable) |
| **Paquete (package)** | Una carpeta con `Cargo.toml` que contiene **uno o más crates** (máximo una biblioteca y cualquier número de binarios) |
| **Módulo** | Organización **dentro** de un crate (Módulo 4) |
| **Cargo** | La herramienta para crear, compilar, probar y gestionar dependencias |
| **crates.io** | El repositorio público de crates de la comunidad |

```
Paquete (Cargo.toml)
 └── Crate (binario o biblioteca)
      └── Módulos (mod)
```

## 5.2 Crear paquetes

```bash
cargo new mi_app            # paquete binario (con src/main.rs)
cargo new mi_lib --lib      # paquete biblioteca (con src/lib.rs)
cargo init                  # convierte la carpeta actual en un paquete
```

Estructura de un paquete binario:

```
mi_app/
├── Cargo.toml
├── Cargo.lock              # versiones exactas usadas (se genera solo)
├── .gitignore
├── src/
│   └── main.rs             # raíz del crate binario
└── target/                 # resultados de compilar (no se sube a git)
```

## 5.3 `Cargo.toml`

```toml
[package]
name = "mi_app"
version = "0.1.0"
edition = "2021"            # (los proyectos nuevos pueden traer "2024")

[dependencies]
# crates externos van aquí
```

- `name`: nombre del paquete.
- `version`: versión semántica `MAYOR.MENOR.PARCHE`.
- `edition`: la "edición" del lenguaje (2021, 2024...). Las ediciones **no rompen** el código anterior.

## 5.4 Comandos esenciales

| Comando | Qué hace |
|---|---|
| `cargo new nombre` | Crea un proyecto binario |
| `cargo new nombre --lib` | Crea una biblioteca |
| `cargo check` | Verifica que compila **sin generar el ejecutable** (rápido, úsalo muy seguido) |
| `cargo build` | Compila en modo *debug* (rápido de compilar, lento de ejecutar) |
| `cargo build --release` | Compila optimizado (resultado en `target/release/`) |
| `cargo run` | Compila y ejecuta |
| `cargo run --release` | Ejecuta la versión optimizada |
| `cargo test` | Ejecuta las pruebas |
| `cargo fmt` | Da formato estándar al código |
| `cargo clippy` | Sugerencias de mejora ("linter") |
| `cargo doc --open` | Genera y abre la documentación |
| `cargo add nombre` | Agrega una dependencia a `Cargo.toml` |
| `cargo update` | Actualiza dependencias dentro de lo permitido |
| `cargo tree` | Muestra el árbol de dependencias |
| `cargo clean` | Borra la carpeta `target/` |

**Flujo diario:** `cargo check` mientras escribes → `cargo run` para probar → `cargo fmt` y `cargo clippy` antes de entregar.

## 5.5 Usar dependencias externas

Agrega la dependencia de cualquiera de estas formas:

```bash
cargo add rand@0.8
```

o editando `Cargo.toml` a mano:

```toml
[dependencies]
rand = "0.8"
```

Y úsala en el código (este ejemplo corresponde a `rand` 0.8; en versiones nuevas algunos nombres cambian, **revisa siempre la documentación en <https://docs.rs>**):

```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let dado: u32 = rng.gen_range(1..=6);
    println!("Sacaste un {dado}");
}
```

Notas:

- `"0.8"` significa "compatible con 0.8.x" (versionado semántico).
- La primera vez, Cargo **descarga y compila** la dependencia automáticamente.
- `Cargo.lock` fija las versiones exactas que usaste, para que todos compilen igual.
- Dependencias solo para pruebas: sección `[dev-dependencies]`.
- Dependencia local: `mi_lib = { path = "../mi_lib" }`.
- Activar opciones de un crate: `nombre = { version = "1", features = ["algo"] }`.

**Dónde buscar crates:** <https://crates.io> (el catálogo) y <https://docs.rs> (la documentación de cada uno).

## 5.6 Un paquete con biblioteca **y** binario

Si una carpeta tiene `src/lib.rs` **y** `src/main.rs`, el paquete tiene **dos crates**: la biblioteca (con la lógica) y el binario (que la usa). Es la estructura recomendada: la lógica queda reutilizable y probable.

```
calculadora/
├── Cargo.toml
└── src/
    ├── lib.rs           # crate biblioteca: la lógica
    └── main.rs          # crate binario: el programa que la usa
```

**`src/lib.rs`**

```rust
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

pub fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("división entre cero"))
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_basica() {
        assert_eq!(sumar(2, 3), 5);
    }

    #[test]
    fn division_por_cero_falla() {
        assert!(dividir(1, 0).is_err());
    }
}
```

**`src/main.rs`**

```rust
use calculadora::{dividir, sumar};      // el nombre del paquete es la raíz de la ruta

fn main() {
    println!("{}", sumar(2, 3));
    println!("{:?}", dividir(10, 0));
}
```

Ejecuta:

```bash
cargo run      # corre el binario
cargo test     # corre las pruebas de lib.rs
```

Detalles útiles:

- Si el nombre del paquete lleva guiones (`mi-calculadora`), en el código se escribe con guion bajo (`mi_calculadora`).
- Varios ejecutables en un paquete: archivos en `src/bin/` (cada uno con su `main`) y se corren con `cargo run --bin nombre`.
- Para publicar tu biblioteca en crates.io: `cargo publish` (requiere cuenta y metadatos en `Cargo.toml`).

## 5.7 Pruebas mínimas

Una función con `#[test]` es una prueba. Macros típicas:

```rust
assert!(condicion);              // falla si es false
assert_eq!(a, b);                // falla si a != b
assert_ne!(a, b);                // falla si a == b
```

`cargo test` ejecuta todas e informa cuáles pasaron.

---

# Módulo 6: Colecciones: `Vec<T>`, `HashMap<K, V>` y `HashSet<T>`

A diferencia de los arrays (tamaño fijo), las **colecciones** guardan datos en memoria dinámica y **pueden crecer y decrecer en ejecución**.

| Colección | Qué es | Ejemplo de uso |
|---|---|---|
| `Vec<T>` | Lista ordenada que crece | Lista de productos, de notas |
| `HashMap<K, V>` | Diccionario clave → valor | Stock por producto, conteo de palabras |
| `HashSet<T>` | Conjunto de valores **únicos** | Etiquetas sin repetir, "ya visitados" |

## 6.1 `Vec<T>`

### Crear

```rust
fn main() {
    let mut a: Vec<i32> = Vec::new();         // vacío (hay que indicar el tipo)
    a.push(1);
    a.push(2);

    let b = vec![10, 20, 30];                 // macro vec! con valores iniciales
    let c = vec![0; 5];                       // cinco ceros
    let mut d = Vec::with_capacity(100);      // reserva espacio (optimización)
    d.push("hola");

    println!("{:?} {:?} {:?} {:?}", a, b, c, d);

    // let e = Vec::new();                    // ❌ E0282: ¿vector de qué tipo?
}
```

### Agregar y quitar

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    v.push(4);                      // agrega al final         [1,2,3,4]
    v.insert(1, 99);                // inserta en posición 1   [1,99,2,3,4]
    let ultimo = v.pop();           // quita el último         -> Some(4)
    let quitado = v.remove(0);      // quita posición 0        -> 1   (⚠️ panic si no existe)
    v.truncate(2);                  // deja solo 2 elementos
    println!("{:?} {:?} {}", v, ultimo, quitado);

    v.clear();                      // vacía
    println!("{}", v.is_empty());   // true
    println!("{:?}", v.pop());      // None (pop sobre vacío NO hace panic)
}
```

### Leer elementos: `[]` vs `.get()`

```rust
fn main() {
    let v = vec![10, 20, 30];

    println!("{}", v[0]);                 // 10
    // println!("{}", v[10]);             // ⚠️ panic: index out of bounds

    println!("{:?}", v.get(1));           // Some(20)
    println!("{:?}", v.get(10));          // None  ✅ seguro

    match v.get(5) {
        Some(x) => println!("Valor: {x}"),
        None => println!("Posición inexistente"),
    }

    println!("{:?} {:?}", v.first(), v.last());   // Some(10) Some(30)
    println!("{}", v.len());                       // 3
}
```

### Recorrer

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    for x in &v {                      // préstamo de lectura: v sigue disponible
        println!("{x}");
    }

    for x in &mut v {                  // préstamo mutable: puedes modificar
        *x *= 10;
    }
    println!("{:?}", v);               // [10, 20, 30]

    for (i, x) in v.iter().enumerate() {
        println!("{i}: {x}");
    }

    for x in v {                       // ⚠️ sin & el Vec se MUEVE y se consume
        println!("{x}");
    }
    // println!("{:?}", v);            // ❌ E0382: v ya fue consumido por el for
}
```

### Métodos útiles

```rust
fn main() {
    let mut v = vec![5, 3, 8, 1];

    println!("{}", v.contains(&8));                 // true
    println!("{}", v.iter().sum::<i32>());          // 17
    println!("{:?}", v.iter().max());               // Some(8)
    println!("{:?}", v.iter().min());               // Some(1)

    v.sort();                                       // [1, 3, 5, 8]
    v.reverse();                                    // [8, 5, 3, 1]
    v.swap(0, 3);                                   // intercambia posiciones
    println!("{:?}", v);                            // [1, 5, 3, 8]

    let mut repetidos = vec![1, 1, 2, 2, 2, 3, 1];
    repetidos.dedup();                              // quita CONSECUTIVOS repetidos
    println!("{:?}", repetidos);                    // [1, 2, 3, 1]

    let mut a = vec![1, 2];
    a.extend_from_slice(&[3, 4]);                   // agrega varios
    println!("{:?}", a);

    let palabras = vec![String::from("uno"), String::from("dos")];
    println!("{}", palabras.join(", "));            // uno, dos
}
```

⚠️ `sort()` **no funciona con `f64`** (los decimales pueden ser `NaN`, que no tiene orden). Receta para decimales (verás el porqué en niveles posteriores):

```rust
fn main() {
    let mut v = vec![2.5, 1.0, 3.7];
    // v.sort();                                    // ❌ E0277: f64 no implementa Ord
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());    // ✅ receta para flotantes
    println!("{:?}", v);                            // [1.0, 2.5, 3.7]
}
```

### Slices de un `Vec` y paso a funciones

```rust
fn promedio(valores: &[f64]) -> Option<f64> {         // acepta Vec y arrays
    if valores.is_empty() {
        return None;
    }
    let mut suma = 0.0;
    for v in valores {
        suma += v;
    }
    Some(suma / valores.len() as f64)
}

fn main() {
    let notas = vec![4.0, 3.5, 5.0];
    println!("{:?}", promedio(&notas));               // Some(4.166...)
    println!("{:?}", promedio(&notas[0..2]));         // Some(3.75)
    println!("{:?}", promedio(&[]));                  // None
}
```

### Vec de structs y de enums

```rust
#[derive(Debug)]
struct Producto {
    nombre: String,
    precio: f64,
}

#[derive(Debug)]
enum Celda {
    Entero(i32),
    Decimal(f64),
    Texto(String),
}

fn main() {
    let mut carrito: Vec<Producto> = Vec::new();
    carrito.push(Producto { nombre: String::from("Café"), precio: 12_000.0 });
    carrito.push(Producto { nombre: String::from("Pan"), precio: 3_500.0 });

    let mut total = 0.0;
    for p in &carrito {
        total += p.precio;
    }
    println!("Total: {total}");

    // Un Vec solo guarda UN tipo... pero un enum permite "tipos" variados:
    let fila = vec![
        Celda::Entero(5),
        Celda::Decimal(2.5),
        Celda::Texto(String::from("hola")),
    ];
    println!("{:?}", fila);
}
```

### Vec bidimensional

```rust
fn main() {
    let mut matriz: Vec<Vec<i32>> = Vec::new();
    for i in 0..3 {
        let mut fila = Vec::new();
        for j in 0..3 {
            fila.push(i * 3 + j);
        }
        matriz.push(fila);
    }
    println!("{:?}", matriz);       // [[0, 1, 2], [3, 4, 5], [6, 7, 8]]
    println!("{}", matriz[1][2]);   // 5
}
```

### ⚠️ Reglas de préstamo con `Vec`

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    // ❌ E0502: tienes una referencia a un elemento y modificas el Vec
    // let primero = &v[0];
    // v.push(4);              // push puede reubicar el Vec en memoria y dejar `primero` colgando
    // println!("{primero}");

    // ✅ Primero usa el valor, luego modifica:
    let primero = v[0];        // copia del i32
    v.push(4);
    println!("{primero} {:?}", v);

    // ❌ E0502: no puedes modificar el Vec mientras lo recorres
    // for x in &v { v.push(*x); }

    // ❌ E0507: no puedes sacar (mover) un String fuera de un índice
    let nombres = vec![String::from("Ana"), String::from("Luis")];
    // let n = nombres[0];                 // ERROR
    let n = &nombres[0];                   // ✅ préstamo
    let m = nombres[1].clone();            // ✅ copia
    println!("{n} {m}");
}
```

## 6.2 `HashMap<K, V>`

Un diccionario: asocia una **clave** a un **valor**. Búsqueda muy rápida por clave.

```rust
use std::collections::HashMap;           // ← obligatorio: no está en el prelude
```

### Crear e insertar

```rust
use std::collections::HashMap;

fn main() {
    let mut precios: HashMap<String, f64> = HashMap::new();
    precios.insert(String::from("café"), 12_000.0);
    precios.insert(String::from("pan"), 3_500.0);

    // Con &str como clave (literales):
    let mut stock: HashMap<&str, u32> = HashMap::new();
    stock.insert("agua", 40);
    stock.insert("arroz", 25);
    // Creación directa desde un array de pares:
    let edades = HashMap::from([("Ana", 25), ("Luis", 30)]);

    println!("{:?}", precios.len());     // 2
    println!("{:?}", stock.len());       // 2
    println!("{:?}", edades.len());      // 2
}
```

### Leer: `get` devuelve `Option`

```rust
use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<&str, u32> = HashMap::new();
    stock.insert("agua", 40);

    println!("{:?}", stock.get("agua"));       // Some(40)
    println!("{:?}", stock.get("leche"));      // None

    match stock.get("agua") {
        Some(cantidad) => println!("Hay {cantidad}"),
        None => println!("No existe"),
    }

    let cantidad = stock.get("leche").unwrap_or(&0);    // valor por defecto
    println!("{cantidad}");

    println!("{}", stock.contains_key("agua"));          // true
    println!("{}", stock["agua"]);                       // 40
    // println!("{}", stock["leche"]);                   // ⚠️ panic: la clave no existe
}
```

### Modificar y eliminar

```rust
use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<&str, u32> = HashMap::new();

    stock.insert("agua", 40);
    let anterior = stock.insert("agua", 35);       // misma clave: SOBRESCRIBE y devuelve el valor anterior
    println!("{:?}", anterior);                    // Some(40)

    if let Some(cantidad) = stock.get_mut("agua") {   // acceso mutable
        *cantidad += 10;
    }
    println!("{:?}", stock.get("agua"));           // Some(45)

    let quitado = stock.remove("agua");            // elimina y devuelve el valor
    println!("{:?} {}", quitado, stock.len());     // Some(45) 0
}
```

### El patrón `entry`: "inserta si no existe"

La forma idiomática de **contar** o **agrupar**:

```rust
use std::collections::HashMap;

fn main() {
    let texto = "el gato y el perro y el pez";
    let mut conteo: HashMap<&str, u32> = HashMap::new();

    for palabra in texto.split_whitespace() {
        let contador = conteo.entry(palabra).or_insert(0);   // si no existe, pone 0
        *contador += 1;                                      // y luego suma
    }

    // ⚠️ Un HashMap NO garantiza orden. Para mostrar en orden, ordena las claves:
    let mut claves: Vec<_> = conteo.keys().collect();
    claves.sort();
    for clave in claves {
        println!("{clave}: {}", conteo[clave]);
    }
    // el: 3 / gato: 1 / perro: 1 / pez: 1 / y: 2
}
```

Agrupar valores bajo una clave:

```rust
use std::collections::HashMap;

fn main() {
    let alumnos = [("A", "Ana"), ("B", "Beto"), ("A", "Alicia"), ("B", "Bruno")];
    let mut grupos: HashMap<&str, Vec<&str>> = HashMap::new();

    for (grupo, nombre) in alumnos {
        grupos.entry(grupo).or_insert(Vec::new()).push(nombre);
    }

    println!("{:?}", grupos.get("A"));     // Some(["Ana", "Alicia"])
    println!("{:?}", grupos.get("B"));     // Some(["Beto", "Bruno"])
}
```

### Recorrer

```rust
use std::collections::HashMap;

fn main() {
    let mut notas = HashMap::new();
    notas.insert("Ana", 4.5);
    notas.insert("Luis", 3.8);

    for (nombre, nota) in &notas {                 // ⚠️ el orden puede variar entre ejecuciones
        println!("{nombre}: {nota}");
    }

    for nombre in notas.keys() {
        println!("clave: {nombre}");
    }
    for nota in notas.values() {
        println!("valor: {nota}");
    }

    for nota in notas.values_mut() {              // modificar valores
        *nota += 0.1;
    }
}
```

### Qué se puede usar como clave

La clave debe poder **compararse con igualdad** y **calcularse un hash** (`Eq + Hash`):

| Clave | ¿Sirve? |
|---|---|
| `i32`, `u64`, `char`, `bool` | ✅ |
| `String`, `&str` | ✅ |
| Tuplas de tipos válidos `(i32, i32)` | ✅ |
| `f64` / `f32` | ❌ (`NaN` rompe la igualdad) |
| Tu struct/enum | ✅ con `#[derive(PartialEq, Eq, Hash)]` |

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Rol { Admin, Usuario }

fn main() {
    let mut permisos: HashMap<Rol, Vec<&str>> = HashMap::new();
    permisos.insert(Rol::Admin, vec!["leer", "escribir", "borrar"]);
    permisos.insert(Rol::Usuario, vec!["leer"]);

    println!("{:?}", permisos.get(&Rol::Admin));

    // let mal: HashMap<f64, i32> = HashMap::new();
    // mal.insert(1.5, 1);          // ❌ E0599/E0277: f64 no implementa Hash/Eq
}
```

### Propiedad en `HashMap`

Al insertar, los `String` **se mueven** al mapa:

```rust
use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    let clave = String::from("a");
    let valor = String::from("uno");
    m.insert(clave, valor);
    // println!("{clave}");          // ❌ E0382: clave fue movida al mapa
    println!("{:?}", m);
}
```

## 6.3 `HashSet<T>`

Un conjunto: **sin elementos repetidos** y **sin orden**.

```rust
use std::collections::HashSet;

fn main() {
    let mut etiquetas: HashSet<&str> = HashSet::new();

    println!("{}", etiquetas.insert("rust"));       // true  (era nuevo)
    println!("{}", etiquetas.insert("código"));     // true
    println!("{}", etiquetas.insert("rust"));       // false (ya estaba; no se duplica)

    println!("{}", etiquetas.len());                // 2
    println!("{}", etiquetas.contains("rust"));     // true
    println!("{}", etiquetas.remove("código"));     // true
    println!("{}", etiquetas.is_empty());           // false

    let numeros = HashSet::from([1, 2, 3, 3, 2]);
    println!("{}", numeros.len());                  // 3
}
```

### Quitar duplicados de un `Vec` conservando el orden

```rust
use std::collections::HashSet;

fn main() {
    let datos = vec![3, 1, 3, 2, 1, 5];
    let mut vistos = HashSet::new();
    let mut unicos = Vec::new();

    for n in datos {
        if vistos.insert(n) {          // insert devuelve true solo la primera vez
            unicos.push(n);
        }
    }
    println!("{:?}", unicos);          // [3, 1, 2, 5]
}
```

### Operaciones de conjuntos

```rust
use std::collections::HashSet;

fn main() {
    let a = HashSet::from([1, 2, 3, 4]);
    let b = HashSet::from([3, 4, 5, 6]);

    // Cada operación devuelve algo que se recorre con for (el orden no está garantizado)
    for x in a.intersection(&b) {              // en ambos: 3, 4
        println!("común: {x}");
    }
    for x in a.difference(&b) {                // en a pero no en b: 1, 2
        println!("solo en a: {x}");
    }
    for x in a.union(&b) {                     // todos, sin repetir
        println!("unión: {x}");
    }

    println!("{}", a.is_subset(&b));           // false
}
```

### ⚠️ Orden

Ni `HashMap` ni `HashSet` mantienen orden de inserción. Si necesitas mostrar en orden, copia los elementos a un `Vec` y ordénalo:

```rust
use std::collections::HashSet;

fn main() {
    let s = HashSet::from([3, 1, 2]);
    let mut v: Vec<_> = s.iter().collect();
    v.sort();
    println!("{:?}", v);        // [1, 2, 3]
}
```

## 6.4 ¿Cuál colección elegir?

| Necesito... | Usa |
|---|---|
| Lista ordenada, acceso por posición, puede repetir | `Vec<T>` |
| Cantidad fija conocida al compilar | Array `[T; N]` |
| Buscar rápido un dato por una clave | `HashMap<K, V>` |
| Contar o agrupar | `HashMap` con `entry` |
| Saber si un elemento "ya está" / quitar repetidos | `HashSet<T>` |
| Operaciones matemáticas de conjuntos | `HashSet<T>` |

---

# Módulo 7: Genéricos y funciones reutilizables

Los **genéricos** permiten escribir **una sola vez** código que funciona con **muchos tipos**, sin duplicarlo y sin perder la verificación de tipos.

## 7.1 El problema

```rust
fn mayor_i32(a: i32, b: i32) -> i32 { if a > b { a } else { b } }
fn mayor_f64(a: f64, b: f64) -> f64 { if a > b { a } else { b } }
fn mayor_char(a: char, b: char) -> char { if a > b { a } else { b } }
// ...una función por cada tipo: repetitivo
```

## 7.2 Funciones genéricas

`<T>` declara un **parámetro de tipo**: un "tipo por determinar":

```rust
fn identidad<T>(valor: T) -> T {
    valor
}

fn intercambiar<A, B>(par: (A, B)) -> (B, A) {
    (par.1, par.0)
}

fn main() {
    println!("{}", identidad(5));              // T = i32
    println!("{}", identidad("hola"));         // T = &str
    println!("{:?}", intercambiar((1, "uno")));   // ("uno", 1)

    // Indicar el tipo explícitamente (turbofish):
    let x = identidad::<f64>(2.5);
    println!("{x}");
}
```

## 7.3 Restricciones (*trait bounds*)

Con un `T` cualquiera **no puedes hacer nada** que no esté garantizado. Para usar `>` hay que **exigir** que `T` se pueda comparar:

```rust
// ❌ E0369: binary operation `>` cannot be applied to type `T`
// fn mayor<T>(a: T, b: T) -> T {
//     if a > b { a } else { b }
// }

// ✅ Exigimos que T implemente PartialOrd (comparable con > y <)
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", mayor(3, 9));           // 9
    println!("{}", mayor(2.5, 1.5));       // 2.5
    println!("{}", mayor('a', 'z'));       // z
    println!("{}", mayor("abc", "abd"));   // abd

    // mayor(1, 2.5);      // ❌ E0308: T debe ser el MISMO tipo en ambos parámetros
}
```

### Varias restricciones con `+`

```rust
fn el_mayor<T: PartialOrd + Copy>(lista: &[T]) -> T {
    let mut mayor = lista[0];                // Copy permite copiar el elemento
    for item in lista {
        if *item > mayor {
            mayor = *item;
        }
    }
    mayor
}

fn main() {
    println!("{}", el_mayor(&[3, 9, 2]));            // 9
    println!("{}", el_mayor(&[1.5, 0.2]));           // 1.5
    println!("{}", el_mayor(&['x', 'b', 'q']));      // x
    let v = vec![10, 50, 20];
    println!("{}", el_mayor(&v));                    // 50  (funciona también con Vec)
}
```

### Cláusula `where` (más legible cuando hay muchas restricciones)

```rust
use std::fmt::{Debug, Display};

fn mostrar<T, U>(a: T, b: U) -> String
where
    T: Display,
    U: Debug,
{
    format!("{a} / {b:?}")
}

fn main() {
    println!("{}", mostrar(5, "texto"));       // 5 / "texto"
}
```

### Restricciones frecuentes

| Restricción | Permite |
|---|---|
| `PartialEq` | `==` y `!=` |
| `PartialOrd` | `<`, `>`, `<=`, `>=` |
| `Copy` | Copiar el valor implícitamente |
| `Clone` | `.clone()` |
| `Debug` | `{:?}` |
| `Display` | `{}` |
| `Default` | `T::default()` |

### ❌ Usar un método que la restricción no garantiza

```rust
// fn mostrar<T>(x: T) {
//     println!("{}", x);        // ❌ E0277: T no implementa Display
// }

fn mostrar<T: std::fmt::Display>(x: T) {      // ✅
    println!("{x}");
}

fn main() {
    mostrar(42);
}
```

## 7.4 Structs genéricos

```rust
#[derive(Debug)]
struct Punto<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Par<A, B> {
    primero: A,
    segundo: B,
}

fn main() {
    let entero = Punto { x: 3, y: 4 };            // Punto<i32>
    let decimal = Punto { x: 1.5, y: 2.5 };       // Punto<f64>
    let mixto = Par { primero: "Ana", segundo: 25 };

    println!("{:?} {:?} {:?}", entero, decimal, mixto);

    // let malo = Punto { x: 1, y: 2.5 };         // ❌ E0308: x e y deben ser del MISMO T
}
```

### `impl` genérico

```rust
struct Punto<T> {
    x: T,
    y: T,
}

impl<T> Punto<T> {                          // <T> después de impl
    fn new(x: T, y: T) -> Self {
        Punto { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }
}

// ✅ Métodos solo para un tipo concreto:
impl Punto<f64> {
    fn distancia_al_origen(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let a = Punto::new(3.0, 4.0);
    println!("{}", a.distancia_al_origen());    // 5
    println!("{}", a.x());

    let b = Punto::new(1, 2);
    println!("{}", b.x());
    // b.distancia_al_origen();                 // ❌ E0599: solo existe para Punto<f64>
}
```

### Ejemplo completo: una `Pila<T>` genérica

```rust
#[derive(Debug)]
struct Pila<T> {
    elementos: Vec<T>,
}

impl<T> Pila<T> {
    fn new() -> Self {
        Pila { elementos: Vec::new() }
    }

    fn apilar(&mut self, valor: T) {
        self.elementos.push(valor);
    }

    fn desapilar(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    fn cima(&self) -> Option<&T> {
        self.elementos.last()
    }

    fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }

    fn tamano(&self) -> usize {
        self.elementos.len()
    }
}

fn main() {
    let mut numeros: Pila<i32> = Pila::new();
    numeros.apilar(1);
    numeros.apilar(2);
    numeros.apilar(3);
    println!("{:?}", numeros.cima());           // Some(3)
    println!("{:?}", numeros.desapilar());      // Some(3)
    println!("{}", numeros.tamano());           // 2

    let mut palabras = Pila::new();             // el tipo se infiere de lo que apilas
    palabras.apilar(String::from("hola"));
    palabras.apilar(String::from("mundo"));
    println!("{:?}", palabras);
    println!("{}", palabras.esta_vacia());      // false
}
```

## 7.5 Enums genéricos

```rust
#[derive(Debug)]
enum MiOpcion<T> {
    Nada,
    Algo(T),
}

#[derive(Debug)]
enum Respuesta<T, E> {
    Exito(T),
    Fallo(E),
}

fn main() {
    let a: MiOpcion<i32> = MiOpcion::Algo(5);
    let b: MiOpcion<String> = MiOpcion::Nada;
    let c: Respuesta<f64, String> = Respuesta::Exito(2.5);
    let d: Respuesta<f64, String> = Respuesta::Fallo(String::from("error"));
    println!("{:?} {:?} {:?} {:?}", a, b, c, d);
}
```

Así están hechos `Option<T>` y `Result<T, E>`, y también `Vec<T>` y `HashMap<K, V>`.

## 7.6 Funciones genéricas útiles

```rust
fn contiene<T: PartialEq>(lista: &[T], buscado: &T) -> bool {
    for item in lista {
        if item == buscado {
            return true;
        }
    }
    false
}

fn contar<T: PartialEq>(lista: &[T], buscado: &T) -> usize {
    let mut n = 0;
    for item in lista {
        if item == buscado {
            n += 1;
        }
    }
    n
}

fn primero_o_defecto<T: Clone + Default>(lista: &[T]) -> T {
    if lista.is_empty() {
        T::default()
    } else {
        lista[0].clone()
    }
}

fn main() {
    println!("{}", contiene(&[1, 2, 3], &2));                 // true
    println!("{}", contiene(&["a", "b"], &"z"));              // false
    println!("{}", contar(&[1, 2, 2, 3, 2], &2));             // 3
    println!("{}", primero_o_defecto::<i32>(&[]));            // 0
    println!("{}", primero_o_defecto(&[String::from("hola")]));   // hola
}
```

## 7.7 ¿Cuesta rendimiento?

**No.** Rust usa **monomorfización**: en compilación genera una versión específica de la función para cada tipo con el que la usas (`mayor::<i32>`, `mayor::<f64>`...). El resultado es tan rápido como si hubieras escrito cada versión a mano. A cambio, el binario puede crecer un poco.

## 7.8 Reglas de oro de los genéricos

1. Si dos funciones hacen lo mismo y solo cambia el tipo → **genérico**.
2. Declara `<T>` después del nombre de la función, struct, enum o `impl`.
3. Con un `T` "desnudo" no puedes usar operadores ni métodos: **agrega restricciones** (`T: PartialOrd`, etc.).
4. Un mismo `T` significa **un mismo tipo** en todos los lugares donde aparece; para tipos distintos usa `<A, B>`.
5. Pon solo las restricciones que realmente necesitas.

---

# Módulo 8: Traits: comportamientos compartidos

Un **trait** define un conjunto de **métodos que un tipo debe ofrecer**. Es el mecanismo de Rust para decir *"todo lo que sepa hacer X"*. Cumple un papel parecido a las interfaces de otros lenguajes.

## 8.1 Definir e implementar un trait

```rust
trait Describible {
    fn describir(&self) -> String;
}

struct Perro {
    nombre: String,
}

struct Libro {
    titulo: String,
    paginas: u32,
}

impl Describible for Perro {
    fn describir(&self) -> String {
        format!("Perro llamado {}", self.nombre)
    }
}

impl Describible for Libro {
    fn describir(&self) -> String {
        format!("Libro '{}' de {} páginas", self.titulo, self.paginas)
    }
}

fn main() {
    let p = Perro { nombre: String::from("Rex") };
    let l = Libro { titulo: String::from("Rust"), paginas: 500 };

    println!("{}", p.describir());
    println!("{}", l.describir());
}
```

Sintaxis: `impl NombreDelTrait for Tipo { ... }`.

**Regla:** al implementar un trait **debes** implementar **todos** sus métodos sin cuerpo (si no, `E0046: not all trait items implemented`).

### También se puede implementar para enums y tipos básicos

```rust
trait Describible {
    fn describir(&self) -> String;
}

enum Estado { Activo, Inactivo }

impl Describible for Estado {
    fn describir(&self) -> String {
        match self {
            Estado::Activo => String::from("activo"),
            Estado::Inactivo => String::from("inactivo"),
        }
    }
}

impl Describible for i32 {                 // ✅ trait tuyo, tipo de la librería
    fn describir(&self) -> String {
        format!("el número entero {self}")
    }
}

fn main() {
    println!("{}", Estado::Activo.describir());
    println!("{}", 42_i32.describir());       // el sufijo evita ambigüedad del literal
    let _ = Estado::Inactivo;
}
```

## 8.2 Métodos con implementación por defecto

El trait puede traer una versión "de fábrica" que los tipos pueden usar o reemplazar:

```rust
trait Saludador {
    fn nombre(&self) -> String;                    // obligatorio

    fn saludar(&self) -> String {                  // por defecto: usa nombre()
        format!("¡Hola, {}!", self.nombre())
    }
}

struct Estudiante { nombre: String }
struct Profesor { nombre: String }

impl Saludador for Estudiante {
    fn nombre(&self) -> String { self.nombre.clone() }
    // saludar() se hereda tal cual
}

impl Saludador for Profesor {
    fn nombre(&self) -> String { self.nombre.clone() }

    fn saludar(&self) -> String {                  // reemplaza el valor por defecto
        format!("Buenos días, profesor {}", self.nombre)
    }
}

fn main() {
    let e = Estudiante { nombre: String::from("Ana") };
    let p = Profesor { nombre: String::from("Pérez") };
    println!("{}", e.saludar());      // ¡Hola, Ana!
    println!("{}", p.saludar());      // Buenos días, profesor Pérez
}
```

## 8.3 Traits como restricciones: funciones que aceptan "cualquier cosa que sepa..."

Aquí se unen los **genéricos** (Módulo 7) con los **traits**:

```rust
trait Area {
    fn area(&self) -> f64;
}

struct Circulo { radio: f64 }
struct Rectangulo { ancho: f64, alto: f64 }

impl Area for Circulo {
    fn area(&self) -> f64 { 3.14159 * self.radio * self.radio }
}

impl Area for Rectangulo {
    fn area(&self) -> f64 { self.ancho * self.alto }
}

// Forma 1: genérico con restricción
fn imprimir_area<T: Area>(figura: &T) {
    println!("Área: {:.2}", figura.area());
}

// Forma 2: sintaxis abreviada impl Trait (equivalente, más corta)
fn imprimir_area_corto(figura: &impl Area) {
    println!("Área: {:.2}", figura.area());
}

// Forma 3: where, útil con varias restricciones
fn comparar<A, B>(a: &A, b: &B) -> bool
where
    A: Area,
    B: Area,
{
    a.area() > b.area()
}

fn main() {
    let c = Circulo { radio: 2.0 };
    let r = Rectangulo { ancho: 3.0, alto: 4.0 };

    imprimir_area(&c);                 // Área: 12.57
    imprimir_area_corto(&r);           // Área: 12.00
    println!("{}", comparar(&c, &r));  // true
}
```

```rust
// ❌ Si el tipo no implementa el trait, no compila (E0277)
// struct Triangulo;
// imprimir_area(&Triangulo);      // ERROR: Triangulo no implementa Area
```

### Varios traits a la vez

```rust
use std::fmt::Debug;

trait Describible {
    fn describir(&self) -> String;
}

fn mostrar<T: Describible + Debug>(x: &T) {
    println!("{} | {:?}", x.describir(), x);
}

fn mostrar2(x: &(impl Describible + Debug)) {
    println!("{} | {:?}", x.describir(), x);
}

#[derive(Debug)]
struct Gato;

impl Describible for Gato {
    fn describir(&self) -> String { String::from("un gato") }
}

fn main() {
    mostrar(&Gato);
    mostrar2(&Gato);
}
```

## 8.4 Funciones asociadas en traits (sin `self`)

```rust
trait Animal {
    fn nombre_especie() -> String;         // sin self
    fn sonido(&self) -> String;
}

struct Perro;
struct Gato;

impl Animal for Perro {
    fn nombre_especie() -> String { String::from("Perro") }
    fn sonido(&self) -> String { String::from("guau") }
}

impl Animal for Gato {
    fn nombre_especie() -> String { String::from("Gato") }
    fn sonido(&self) -> String { String::from("miau") }
}

fn presentar<T: Animal>(a: &T) {
    println!("{} dice {}", T::nombre_especie(), a.sonido());
}

fn main() {
    println!("{}", Perro::nombre_especie());
    presentar(&Perro);       // Perro dice guau
    presentar(&Gato);        // Gato dice miau
}
```

## 8.5 Traits de la librería estándar que debes conocer

### Con `#[derive(...)]` (los más comunes)

| Trait | Qué habilita |
|---|---|
| `Debug` | `{:?}` |
| `Clone` | `.clone()` |
| `Copy` | Copia implícita |
| `PartialEq` / `Eq` | `==`, `!=` |
| `PartialOrd` / `Ord` | `<`, `>`, `.sort()` |
| `Hash` | Usar como clave de `HashMap` / elemento de `HashSet` |
| `Default` | `Tipo::default()` |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Nivel {
    Bajo,
    Medio,
    Alto,
}

fn main() {
    println!("{}", Nivel::Bajo < Nivel::Alto);     // true (orden de declaración)

    let mut niveles = vec![Nivel::Alto, Nivel::Bajo, Nivel::Medio];
    niveles.sort();                                 // posible gracias a Ord
    println!("{:?}", niveles);                      // [Bajo, Medio, Alto]
}
```

### Implementar `Display` a mano (para que `{}` funcione)

```rust
use std::fmt;

struct Punto {
    x: f64,
    y: f64,
}

impl fmt::Display for Punto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Punto { x: 1.5, y: -2.0 };
    println!("{p}");                     // (1.5, -2)
    println!("{}", p);
    let texto: String = p.to_string();   // to_string() viene gratis al implementar Display
    println!("{texto}");
}
```

### Conversiones propias con `From`

```rust
struct Punto {
    x: f64,
    y: f64,
}

impl From<(f64, f64)> for Punto {
    fn from(par: (f64, f64)) -> Self {
        Punto { x: par.0, y: par.1 }
    }
}

fn main() {
    let a = Punto::from((1.0, 2.0));
    let b: Punto = (3.0, 4.0).into();         // into() se obtiene gratis con From
    println!("{} {} / {} {}", a.x, a.y, b.x, b.y);
}
```

## 8.6 Reglas importantes de los traits

### 1. El trait debe estar "a la vista" para llamar sus métodos

Si el trait vive en otro módulo, hay que importarlo con `use`:

```rust
mod animales {
    pub trait Sonido {
        fn hacer_sonido(&self) -> String;
    }

    pub struct Perro;

    impl Sonido for Perro {
        fn hacer_sonido(&self) -> String { String::from("guau") }
    }
}

use animales::{Perro, Sonido};      // ← sin importar `Sonido`, hacer_sonido() no se encuentra

fn main() {
    println!("{}", Perro.hacer_sonido());
}
```

### 2. Regla de la huérfana (*orphan rule*)

Puedes implementar un trait **si el trait o el tipo son tuyos** (o ambos):

| Trait | Tipo | ¿Permitido? |
|---|---|---|
| Tuyo | Tuyo | ✅ |
| Tuyo | Externo (`i32`, `Vec`...) | ✅ |
| Externo (`Display`) | Tuyo | ✅ |
| Externo (`Display`) | Externo (`Vec<i32>`) | ❌ E0117 |

### 3. Un tipo implementa un trait **una sola vez**

No puedes escribir dos `impl Area for Circulo`.

### 4. Los traits no guardan datos

Un trait define **comportamiento**, no campos. Los datos viven en el struct.

## 8.7 Cuándo usar cada herramienta

| Necesito... | Herramienta |
|---|---|
| Agrupar datos | `struct` |
| Representar alternativas / estados | `enum` |
| Añadir comportamiento a un tipo | `impl` |
| Código que funcione para muchos tipos | Genéricos `<T>` |
| Exigir que esos tipos sepan hacer algo | Traits como restricciones (`T: Trait`) |
| Comportamiento compartido entre tipos distintos | `trait` |

> **Límite del Nivel 2:** con genéricos + traits, una función acepta *un* tipo concreto a la vez. Para guardar **tipos distintos juntos en una misma lista** que comparten un trait se necesitan *trait objects* (`dyn Trait`), tema del siguiente nivel.

---

# Proyecto integrador: inventario y abastecimiento logístico

El encargo empresarial de este nivel consiste en construir un inventario modular con reglas de negocio, colecciones y tipos propios.

**Proyecto independiente:** [Abrir el enunciado completo](/proyectos/rust-nivel-2-inventario-logistico/).

**Módulos integrados:** revisa en el enunciado qué requisito empresarial pone en práctica cada módulo del nivel.

---


# Ejercicios con soluciones

> 💡 Que intenten antes de abrir la solución. Todas están plegadas.

## Módulo 0: Puente

**0.1** Escribe `fn agregar_sufijo(s: &mut String)` que añada `"_v2"`.
**0.2** Escribe `fn dividir_seguro(a: i32, b: i32) -> Result<i32, String>`.
**0.3** Escribe `fn primer_par(nums: &[i32]) -> Option<i32>`.

<details>
<summary>Ver soluciones</summary>

```rust
fn agregar_sufijo(s: &mut String) {
    s.push_str("_v2");
}

fn dividir_seguro(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("división entre cero"))
    } else {
        Ok(a / b)
    }
}

fn primer_par(nums: &[i32]) -> Option<i32> {
    for n in nums {
        if n % 2 == 0 {
            return Some(*n);
        }
    }
    None
}

fn main() {
    let mut nombre = String::from("archivo");
    agregar_sufijo(&mut nombre);
    println!("{nombre}");                       // archivo_v2

    println!("{:?}", dividir_seguro(10, 2));    // Ok(5)
    println!("{:?}", dividir_seguro(1, 0));     // Err("división entre cero")

    println!("{:?}", primer_par(&[1, 3, 6, 8])); // Some(6)
    println!("{:?}", primer_par(&[1, 3]));       // None
}
```
</details>

## Módulo 1: Structs

**1.1** Define `Libro { titulo, autor, paginas, leido }` con `Debug` y `Clone`. Crea uno y luego otro usando `..` para cambiar solo el título.

<details>
<summary>Ver solución</summary>

```rust
#[derive(Debug, Clone)]
struct Libro {
    titulo: String,
    autor: String,
    paginas: u32,
    leido: bool,
}

fn main() {
    let libro1 = Libro {
        titulo: String::from("Rust en la práctica"),
        autor: String::from("Ana Gómez"),
        paginas: 350,
        leido: false,
    };

    let libro2 = Libro {
        titulo: String::from("Rust avanzado"),
        ..libro1.clone()
    };

    println!("{:#?}", libro1);
    println!("{:#?}", libro2);
}
```
</details>

## Módulo 2: Enums

**2.1** Crea `enum Figura { Circulo(f64), Rectangulo { ancho: f64, alto: f64 }, Triangulo(f64, f64) }` y una función `area(&Figura) -> f64` con `match`.

<details>
<summary>Ver solución</summary>

```rust
enum Figura {
    Circulo(f64),
    Rectangulo { ancho: f64, alto: f64 },
    Triangulo(f64, f64),
}

fn area(f: &Figura) -> f64 {
    match f {
        Figura::Circulo(radio) => 3.14159 * radio * radio,
        Figura::Rectangulo { ancho, alto } => ancho * alto,
        Figura::Triangulo(base, altura) => base * altura / 2.0,
    }
}

fn main() {
    let figuras = [
        Figura::Circulo(2.0),
        Figura::Rectangulo { ancho: 3.0, alto: 4.0 },
        Figura::Triangulo(5.0, 2.0),
    ];
    for f in &figuras {
        println!("{:.2}", area(f));      // 12.57 / 12.00 / 5.00
    }
}
```
</details>

## Módulo 3: Métodos e `impl`

**3.1** Crea `struct Contador` con `new()`, `incrementar(&mut self)` y `valor(&self)`.
**3.2** Crea `CuentaBancaria` con `depositar` y `retirar` que devuelvan `Result<(), String>` (monto positivo; no retirar más del saldo).

<details>
<summary>Ver soluciones</summary>

```rust
struct Contador {
    valor: u32,
}

impl Contador {
    fn new() -> Self {
        Contador { valor: 0 }
    }
    fn incrementar(&mut self) {
        self.valor += 1;
    }
    fn valor(&self) -> u32 {
        self.valor
    }
}

struct CuentaBancaria {
    titular: String,
    saldo: f64,
}

impl CuentaBancaria {
    fn new(titular: &str) -> Self {
        CuentaBancaria { titular: titular.to_string(), saldo: 0.0 }
    }

    fn depositar(&mut self, monto: f64) -> Result<(), String> {
        if monto <= 0.0 {
            return Err(String::from("El monto debe ser positivo"));
        }
        self.saldo += monto;
        Ok(())
    }

    fn retirar(&mut self, monto: f64) -> Result<(), String> {
        if monto <= 0.0 {
            return Err(String::from("El monto debe ser positivo"));
        }
        if monto > self.saldo {
            return Err(String::from("Fondos insuficientes"));
        }
        self.saldo -= monto;
        Ok(())
    }
}

fn main() {
    let mut c = Contador::new();
    c.incrementar();
    c.incrementar();
    println!("{}", c.valor());                     // 2

    let mut cuenta = CuentaBancaria::new("Ana");
    println!("{:?}", cuenta.depositar(100.0));     // Ok(())
    println!("{:?}", cuenta.retirar(30.0));        // Ok(())
    println!("{:?}", cuenta.retirar(500.0));       // Err("Fondos insuficientes")
    println!("{} tiene {}", cuenta.titular, cuenta.saldo);   // Ana tiene 70
}
```
</details>

## Módulo 4: Módulos

**4.1** Crea un módulo `operaciones` con `sumar`, `restar` (públicas) y una función privada `validar`. Llama las públicas desde `main`.
**4.2** Mueve ese módulo a su propio archivo `src/operaciones.rs`.

<details>
<summary>Ver soluciones</summary>

```rust
// 4.1: todo en main.rs
mod operaciones {
    pub fn sumar(a: i32, b: i32) -> i32 {
        validar(a);
        a + b
    }

    pub fn restar(a: i32, b: i32) -> i32 {
        a - b
    }

    fn validar(n: i32) {                  // privada
        if n < 0 {
            println!("(aviso: número negativo)");
        }
    }
}

fn main() {
    println!("{}", operaciones::sumar(2, 3));
    println!("{}", operaciones::restar(9, 4));
    // operaciones::validar(1);           // ❌ E0603: privada
}
```

```rust
// 4.2: en src/main.rs
mod operaciones;                          // busca src/operaciones.rs

use operaciones::{restar, sumar};

fn main() {
    println!("{}", sumar(2, 3));
    println!("{}", restar(9, 4));
}
```

```rust
// src/operaciones.rs  (sin la línea `mod operaciones { }`: el archivo YA es el módulo)
pub fn sumar(a: i32, b: i32) -> i32 {
    validar(a);
    a + b
}

pub fn restar(a: i32, b: i32) -> i32 {
    a - b
}

fn validar(n: i32) {
    if n < 0 {
        println!("(aviso: número negativo)");
    }
}
```
</details>

## Módulo 5: Crates, paquetes y Cargo

**5.1** Crea con Cargo un paquete `utilidades` con biblioteca y binario. En `lib.rs` escribe `pub fn es_par(n: i32) -> bool` con una prueba. Úsala desde `main.rs`.
**5.2** Agrega la dependencia `rand` con `cargo add rand@0.8` y simula tirar un dado.

<details>
<summary>Ver solución</summary>

```bash
cargo new utilidades --lib
cd utilidades
# crea además src/main.rs a mano
```

```rust
// src/lib.rs
pub fn es_par(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detecta_pares() {
        assert!(es_par(4));
        assert!(!es_par(7));
    }
}
```

```rust
// src/main.rs
use utilidades::es_par;

fn main() {
    println!("{}", es_par(10));
}
```

Para la dependencia (ejercicio 5.2):

```bash
cargo add rand@0.8
```

```rust
// src/main.rs
use rand::Rng;

fn main() {
    let dado: u32 = rand::thread_rng().gen_range(1..=6);
    println!("Sacaste {dado}");
}
```

Verifica con `cargo test` y `cargo run`.
</details>

## Módulo 6: Colecciones

**6.1** Calcula el promedio de un `Vec<f64>` (devuelve `Option<f64>` si está vacío).
**6.2** Cuenta las palabras de un texto con `HashMap` y muéstralas ordenadas.
**6.3** Quita los duplicados de `vec![3, 1, 3, 2, 1, 5]` conservando el orden.

<details>
<summary>Ver soluciones</summary>

```rust
use std::collections::{HashMap, HashSet};

fn promedio(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut suma = 0.0;
    for x in v {
        suma += x;
    }
    Some(suma / v.len() as f64)
}

fn main() {
    // 6.1
    println!("{:?}", promedio(&vec![4.0, 3.5, 5.0]));    // Some(4.1666...)
    println!("{:?}", promedio(&[]));                      // None

    // 6.2
    let texto = "el gato y el perro y el pez";
    let mut conteo: HashMap<&str, u32> = HashMap::new();
    for palabra in texto.split_whitespace() {
        let c = conteo.entry(palabra).or_insert(0);
        *c += 1;
    }
    let mut claves: Vec<_> = conteo.keys().collect();
    claves.sort();
    for k in claves {
        println!("{k}: {}", conteo[k]);
    }
    // el: 3 / gato: 1 / perro: 1 / pez: 1 / y: 2

    // 6.3
    let datos = vec![3, 1, 3, 2, 1, 5];
    let mut vistos = HashSet::new();
    let mut unicos = Vec::new();
    for n in datos {
        if vistos.insert(n) {
            unicos.push(n);
        }
    }
    println!("{:?}", unicos);                             // [3, 1, 2, 5]
}
```
</details>

## Módulo 7: Genéricos

**7.1** Escribe `fn contiene<T: PartialEq>(lista: &[T], buscado: &T) -> bool`.
**7.2** Crea `struct Par<T> { a: T, b: T }` con un método `mayor(&self) -> &T`.

<details>
<summary>Ver soluciones</summary>

```rust
fn contiene<T: PartialEq>(lista: &[T], buscado: &T) -> bool {
    for item in lista {
        if item == buscado {
            return true;
        }
    }
    false
}

struct Par<T> {
    a: T,
    b: T,
}

impl<T: PartialOrd> Par<T> {
    fn mayor(&self) -> &T {
        if self.a > self.b {
            &self.a
        } else {
            &self.b
        }
    }
}

fn main() {
    println!("{}", contiene(&[1, 2, 3], &3));                   // true
    println!("{}", contiene(&["a", "b"], &"z"));                // false

    let p = Par { a: 10, b: 7 };
    println!("{}", p.mayor());                                  // 10
    let q = Par { a: String::from("abc"), b: String::from("xyz") };
    println!("{}", q.mayor());                                  // xyz
}
```
</details>

## Módulo 8: Traits

**8.1** Define `trait Area` con `area(&self)` y un método por defecto `describir(&self)`. Impleméntalo para `Circulo` y `Rectangulo`. Escribe una función genérica que reciba dos figuras y devuelva la mayor área.

<details>
<summary>Ver solución</summary>

```rust
trait Area {
    fn area(&self) -> f64;

    fn describir(&self) -> String {
        format!("Área = {:.2}", self.area())
    }
}

struct Circulo {
    radio: f64,
}

struct Rectangulo {
    ancho: f64,
    alto: f64,
}

impl Area for Circulo {
    fn area(&self) -> f64 {
        3.14159 * self.radio * self.radio
    }
}

impl Area for Rectangulo {
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }
}

fn mayor_area<A: Area, B: Area>(a: &A, b: &B) -> f64 {
    if a.area() > b.area() {
        a.area()
    } else {
        b.area()
    }
}

fn main() {
    let c = Circulo { radio: 2.0 };
    let r = Rectangulo { ancho: 3.0, alto: 4.0 };

    println!("{}", c.describir());              // Área = 12.57
    println!("{}", r.describir());              // Área = 12.00
    println!("{:.2}", mayor_area(&c, &r));      // 12.57
}
```
</details>

---

# Hoja de referencia rápida

```rust
// ---------- STRUCTS ----------
#[derive(Debug, Clone, PartialEq)]
struct Usuario { nombre: String, edad: u32 }
let u = Usuario { nombre: String::from("Ana"), edad: 25 };
let u2 = Usuario { edad: 30, ..u.clone() };
struct Color(u8, u8, u8);                 // tuple struct

// ---------- ENUMS ----------
enum Msg { Salir, Mover { x: i32, y: i32 }, Texto(String) }
match &m {
    Msg::Salir => ...,
    Msg::Mover { x, y } => ...,
    Msg::Texto(t) => ...,
}
if let Msg::Texto(t) = &m { ... }

// ---------- IMPL ----------
impl Usuario {
    fn new(nombre: &str, edad: u32) -> Self { ... }   // asociada: Usuario::new(..)
    fn saludar(&self) -> String { ... }               // lee
    fn cumplir(&mut self) { self.edad += 1; }         // modifica
    fn consumir(self) -> String { self.nombre }       // consume
}

// ---------- MÓDULOS ----------
mod nombre { pub fn f() {} }              // en línea
mod nombre;                               // en archivo: src/nombre.rs
use crate::nombre::f;                     // rutas: crate:: self:: super::
pub struct S { pub a: i32, b: i32 }       // b privado
pub use otro::Tipo;                       // re-exportar

// ---------- CARGO ----------
// cargo new x | check | build | run | test | fmt | clippy | add y | doc --open

// ---------- COLECCIONES ----------
let mut v = vec![1, 2, 3];  v.push(4);  v.pop();  v.get(0);  v.len();
for x in &v {}  for x in &mut v { *x += 1; }
use std::collections::{HashMap, HashSet};
let mut m = HashMap::new();  m.insert("a", 1);  m.get("a");
*m.entry("b").or_insert(0) += 1;
let mut s = HashSet::new();  s.insert(5);  s.contains(&5);

// ---------- GENÉRICOS ----------
fn mayor<T: PartialOrd>(a: T, b: T) -> T { if a > b { a } else { b } }
struct Caja<T> { valor: T }
impl<T> Caja<T> { fn new(valor: T) -> Self { Caja { valor } } }

// ---------- TRAITS ----------
trait Area { fn area(&self) -> f64; fn nombre(&self) -> String { String::from("figura") } }
impl Area for Circulo { fn area(&self) -> f64 { ... } }
fn f<T: Area>(x: &T) {}   fn g(x: &impl Area) {}
impl fmt::Display for Tipo { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "...") } }

// ---------- PUENTE ----------
fn f(x: &T) {}  fn g(x: &mut T) {}  fn h(x: T) {}      // leer / escribir / consumir
fn s(x: &[i32]) {}  fn t(x: &str) {}                   // slices
Option<T>: Some(x) / None        Result<T, E>: Ok(x) / Err(e)       let n = algo()?;
```

---

# Errores frecuentes del compilador

| Código | Mensaje (resumido) | Causa típica | Solución |
|---|---|---|---|
| `E0382` | borrow of moved value | Usar una variable después de moverla (a otra variable, función, `for`, mapa...) | Prestar con `&`, `.clone()` o reestructurar |
| `E0499` | cannot borrow as mutable more than once | Dos `&mut` simultáneas | Usar una a la vez |
| `E0502` | cannot borrow as mutable because it is also borrowed as immutable | Leer y modificar a la vez (ej. `&v[0]` + `v.push`) | Terminar de usar la lectura antes de modificar, o copiar el valor |
| `E0596` | cannot borrow as mutable | Falta `mut` en la variable | Declarar `let mut` |
| `E0507` | cannot move out of index/borrowed content | Sacar un `String` de `v[0]` o de una referencia | `&v[0]` o `.clone()` |
| `E0063` | missing field | Faltan campos al crear un struct | Dar todos los campos o usar `..` |
| `E0560` | struct has no field | Campo mal escrito | Revisar la definición |
| `E0106` | missing lifetime specifier | `&str` dentro de un struct | Usar `String` |
| `E0204` | the trait `Copy` cannot be implemented | `derive(Copy)` con un campo `String` | Quitar `Copy`, usar `Clone` |
| `E0004` | non-exhaustive patterns | `match` sin cubrir todas las variantes | Agregar variantes o `_` |
| `E0599` | no method named ... found | Método inexistente o no cumple restricción (ej. `distancia` en `Punto<i32>`) | Revisar el tipo o las restricciones |
| `E0603` | is private | Falta `pub` | Agregar `pub` |
| `E0616` / `E0451` | field is private | Acceso o construcción con campo privado | Getter / constructor `new` |
| `E0432` / `E0433` | unresolved import / failed to resolve | Falta `mod x;` en el padre o ruta incorrecta | Declarar el módulo, revisar la ruta |
| `E0583` | file not found for module | `mod x;` sin archivo `x.rs` | Crear el archivo en la ruta correcta |
| `E0282` | type annotations needed | `Vec::new()` sin indicar el tipo | `Vec<i32>` o usar el vector después |
| `E0369` | binary operation cannot be applied to type `T` | Usar `>`/`+` con un `T` sin restricción | `T: PartialOrd`, etc. |
| `E0277` | trait bound not satisfied / doesn't implement `Display` | Usar `{}` en un struct, o `f64` como clave de `HashMap` | Implementar el trait, o usar `{:?}` con `Debug` |
| `E0046` | not all trait items implemented | Faltan métodos al implementar un trait | Implementarlos |
| `E0117` | only traits defined in the current crate can be implemented | Trait externo para tipo externo | Envolver el tipo en tu propio struct |
| (panic) | index out of bounds / key not found | `v[10]` o `mapa["x"]` inexistente | `.get()` + `match` |
| (panic) | called `unwrap()` on `None` / `Err` | `unwrap` sobre un valor ausente | `unwrap_or`, `match`, `if let` |

💡 **Hábito:** leer el error completo. En el Nivel 2 los mensajes sobre propiedad y préstamos son **muy** explicativos y casi siempre sugieren la corrección.

---

# Plan de clases sugerido

Propuesta de 6 sesiones de ~90 minutos (ajustable):

| Sesión | Contenido | Actividad práctica |
|---|---|---|
| **1** | Módulo 0 (puente: ownership, `&`/`&mut`, slices, `Option`, `Result`) | Ejercicios 0.1–0.3; provocar a propósito `E0382`, `E0499` y `E0502` |
| **2** | Módulo 1 (structs) + Módulo 2 (enums) | Ejercicios 1.1 y 2.1; modelar el estado de un pedido |
| **3** | Módulo 3 (`impl`, métodos) | Ejercicios 3.1 y 3.2 (Contador y CuentaBancaria) |
| **4** | Módulo 4 (módulos) + Módulo 5 (Cargo) | Ejercicios 4.1, 4.2, 5.1, 5.2; dividir un proyecto en archivos |
| **5** | Módulo 6 (colecciones) | Ejercicios 6.1–6.3; contador de palabras |
| **6** | Módulo 7 (genéricos) + Módulo 8 (traits) + Proyecto integrador | Ejercicios 7.1, 7.2, 8.1; modificar el proyecto de inventario logístico |

### Recomendaciones de enseñanza

1. **El puente es la sesión más importante.** Dedícale tiempo: si los estudiantes entienden *mover vs. prestar*, el resto del nivel fluye. Muchos errores del compilador de este nivel salen de ahí.
2. **Muestra la ventaja de `match` exhaustivo:** agrega una variante a un enum y deja que el compilador "marque" cada `match` por actualizar.
3. **Usa el proyecto integrador como hilo conductor:** cada módulo agrega una pieza; al final lo arman.
4. **Insiste en tres ideas centrales del Nivel 2:**
   - **Modela con tipos:** que los estados imposibles no se puedan escribir (enums con datos, campos privados + `new`).
   - **Quien presta no pierde:** `&` para leer, `&mut` para modificar, valor directo para consumir.
   - **Genérico + trait = reutilización segura:** `T: Trait` dice exactamente qué se necesita.
5. **Pide predicciones:** "¿compila?", "¿qué error saldrá?". Es la mejor forma de fijar las reglas de propiedad.

### Qué sigue (Nivel 3, para referencia)

Lifetimes, closures e iteradores, `Box`, `Rc` y `RefCell`, trait objects (`dyn Trait`), manejo de errores avanzado, concurrencia, pruebas a fondo y macros.

---

*Fin del Nivel 2. ¡A modelar con Rust! 🦀*
