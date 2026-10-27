---
title: "Rust — Nivel 3: Ownership, borrowing y lifetimes"
description: "Domina stack y heap, ownership, move y copy, referencias, borrowing, slices, lifetimes y el borrow checker con ejercicios y un proyecto integrador."
language: rust
level: 3
levelLabel: "Nivel 03"
category: "Memoria y seguridad"
duration: "Intermedio"
order: 3
slug: rust-nivel-3
projectSlug: "rust-nivel-3-auditoria-eventos"
tags: ["ownership", "borrowing", "lifetimes", "referencias", "memoria"]
---
# 🦀 Curso de Rust — Nivel 3: Ownership, borrowing y lifetimes

> **El nivel más importante.** Aquí está lo que hace único a Rust: **seguridad de memoria sin recolector de basura y sin errores en ejecución**. Todo lo que viste en los niveles 1 y 2 se apoya en estas ideas.
>
> **Objetivo:** que tus estudiantes puedan **explicar** (no solo recitar) por qué el compilador rechaza ciertos programas, **predecir** si un código compila y **corregirlo** con criterio, sin recurrir a `.clone()` por desesperación.
>
> **Requisitos:** Niveles 1 y 2 (en particular el Módulo 0 del Nivel 2, que presentó estas ideas de forma resumida).
>
> **Cómo leer este nivel:** cada módulo construye sobre el anterior. No conviene saltarse ninguno.

---

## Índice

1. [Stack y heap: dónde viven los datos](#módulo-1-stack-y-heap-dónde-viven-los-datos)
2. [Ownership (propiedad)](#módulo-2-ownership-propiedad)
3. [Move y Copy](#módulo-3-move-y-copy)
4. [Borrowing: referencias `&T` y `&mut T`](#módulo-4-borrowing-referencias-t-y-mut-t)
5. [Reglas de aliasing y mutabilidad](#módulo-5-reglas-de-aliasing-y-mutabilidad)
6. [Slices: `&str` y `&[T]`](#módulo-6-slices-str-y-t)
7. [Lifetimes: `'a`, referencias y su validez](#módulo-7-lifetimes-a-referencias-y-su-validez)
8. [Cómo funciona el borrow checker](#módulo-8-cómo-funciona-el-borrow-checker)
9. [Proyecto integrador: auditoría de eventos sin copias innecesarias](#proyecto-integrador-auditoria-de-eventos-sin-copias-innecesarias)
10. [Ejercicios con soluciones](#ejercicios-con-soluciones)
11. [Hoja de referencia rápida](#hoja-de-referencia-rápida)
12. [Errores frecuentes del compilador](#errores-frecuentes-del-compilador)
13. [Plan de clases sugerido](#plan-de-clases-sugerido)

**Leyenda:**

- ✅ = se puede / es correcto
- ❌ = NO se puede / no compila
- ⚠️ = compila, pero cuidado

> **Nota sobre el orden:** el temario lista *stack y heap* casi al final, pero aquí va **primero**, porque explica *por qué* existen las reglas de ownership. Entender la memoria hace que todo lo demás tenga sentido.

---

# Módulo 1: Stack y heap: dónde viven los datos

## 1.1 Dos zonas de memoria

Cuando un programa corre, usa principalmente dos zonas de memoria:

| | **Stack (pila)** | **Heap (montón)** |
|---|---|---|
| Organización | Ordenada, como una pila de platos (LIFO) | Zona "libre" donde se reserva espacio a pedido |
| Tamaño de los datos | **Fijo y conocido al compilar** | **Dinámico**: puede crecer o decidirse en ejecución |
| Velocidad | Muy rápida (solo se mueve un puntero) | Más lenta (hay que buscar un hueco y acceder por puntero) |
| Quién la gestiona | Se libera sola al terminar cada función | Alguien debe **liberarla** |
| Ejemplos | `i32`, `f64`, `bool`, `char`, arrays `[T; N]`, tuplas, referencias | El **contenido** de un `String`, de un `Vec<T>` |

**Idea central:** el stack se limpia solo; el heap es el que causa los problemas (olvidar liberar, liberar dos veces, usar algo ya liberado). **Ownership existe para gestionar el heap de forma segura.**

## 1.2 El stack y las llamadas a funciones

Cada llamada a función crea un **marco (frame)** en el stack con sus variables locales. Al terminar la función, su marco desaparece.

```rust
fn suma(x: i32, y: i32) -> i32 {
    let resultado = x + y;
    resultado
}

fn main() {
    let a = 1;
    let b = suma(a, 2);
    println!("{b}");
}
```

```
Durante suma(a, 2):                  Después de suma():
┌──────────────────────┐             ┌──────────────────────┐
│ frame de suma        │             │ frame de main        │
│   x = 1              │             │   a = 1              │
│   y = 2              │             │   b = 3              │
│   resultado = 3      │             └──────────────────────┘
├──────────────────────┤   ← el frame de suma ya no existe
│ frame de main        │
│   a = 1              │
│   b = (pendiente)    │
└──────────────────────┘
```

Como el tamaño de cada frame se conoce al compilar, reservar y liberar es casi instantáneo.

### ⚠️ El stack es limitado

```rust
// ❌ Recursión infinita: llena el stack y el programa termina con
//    "thread 'main' has overflowed its stack"
// fn infinita(n: u64) -> u64 {
//     infinita(n + 1) + 1
// }

fn main() {
    // ⚠️ Arrays enormes en el stack también pueden desbordarlo:
    // let enorme = [0u8; 10_000_000];        // 10 MB: probable stack overflow (el stack suele ser de ~8 MB)
    // ✅ Para datos grandes usa Vec (el contenido va al heap):
    let enorme = vec![0u8; 10_000_000];
    println!("{}", enorme.len());
}
```

## 1.3 El heap: `String` y `Vec`

Un `String` puede crecer en ejecución, así que su texto **no cabe en un tamaño fijo del stack**. Rust lo divide en dos partes:

- En el **stack**: una pequeña "ficha" con 3 datos (puntero, longitud, capacidad). Siempre ocupa lo mismo.
- En el **heap**: los bytes del texto.

```rust
let s = String::from("hola");
```

```
STACK                               HEAP
┌─────────────────┐
│ s               │
│  ptr  ──────────┼───────────▶  ┌───┬───┬───┬───┐
│  len      = 4   │              │ h │ o │ l │ a │
│  capacity = 4   │              └───┴───┴───┴───┘
└─────────────────┘
```

- `ptr`: dirección donde empiezan los bytes en el heap.
- `len`: cuántos bytes usa el texto ahora.
- `capacity`: cuántos bytes hay reservados (puede ser mayor que `len`).

Un `Vec<i32>` es igual: ficha en el stack (ptr, len, capacity) y los números en el heap.

```rust
fn main() {
    let mut s = String::with_capacity(8);
    println!("len={} capacity>=8: {}", s.len(), s.capacity() >= 8);   // len=0 capacity>=8: true

    s.push_str("hola");
    println!("len={}", s.len());                                      // len=4

    // Cuando se supera la capacidad, Rust pide más espacio al heap
    // y puede MOVER el texto a otra dirección (dato clave para el Módulo 5).
    s.push_str(" mundo, esto ya no cabe en 8 bytes");
    println!("len={}", s.len());
}
```

## 1.4 ¿Y `&str`, `&[T]` y las referencias?

- Una **referencia** `&T` es un **puntero** (8 bytes en un PC de 64 bits) que vive en el stack y apunta a un dato que vive en otro lugar.
- Un **slice** (`&str`, `&[T]`) es una referencia "gorda": **puntero + longitud** (16 bytes).
- Un literal `"hola"` guarda sus bytes en la memoria de solo lectura del propio programa; la variable `&str` (puntero + longitud) vive en el stack.

```rust
use std::mem::size_of;

fn main() {
    // Valores típicos en un sistema de 64 bits:
    println!("i32:         {} bytes", size_of::<i32>());          // 4
    println!("f64:         {} bytes", size_of::<f64>());          // 8
    println!("char:        {} bytes", size_of::<char>());         // 4
    println!("[i32; 4]:    {} bytes", size_of::<[i32; 4]>());     // 16
    println!("(i32, f64):  {} bytes", size_of::<(i32, f64)>());   // 16 (con relleno de alineación)
    println!("&i32:        {} bytes", size_of::<&i32>());         // 8   puntero
    println!("&str:        {} bytes", size_of::<&str>());         // 16  puntero + longitud
    println!("&[i32]:      {} bytes", size_of::<&[i32]>());       // 16  puntero + longitud
    println!("String:      {} bytes", size_of::<String>());       // 24  ptr + len + capacity
    println!("Vec<i32>:    {} bytes", size_of::<Vec<i32>>());     // 24  ptr + len + capacity
}
```

> **Observación para clase:** un `String` de 5 letras y uno de 5 millones de letras ocupan **24 bytes en el stack**. La diferencia está en el heap.

## 1.5 ¿Dónde vive cada cosa?

| Código | Stack | Heap | Memoria estática (solo lectura) |
|---|---|---|---|
| `let x: i32 = 5;` | `x` | | |
| `let a = [0; 10];` | los 10 elementos | | |
| `let t = (1, 2.5, 'a');` | toda la tupla | | |
| `let s = String::from("hi");` | ptr, len, capacity | los bytes `hi` | |
| `let v = vec![1, 2, 3];` | ptr, len, capacity | `1, 2, 3` | |
| `let l = "hola";` | ptr + len | | los bytes `hola` |
| `let r = &x;` | el puntero `r` | | |
| `struct P { id: u32, nombre: String }` | `id` y la ficha de `nombre` | los bytes del nombre | |

## 1.6 El problema que Rust resuelve

El heap exige que alguien libere la memoria **exactamente una vez** y **en el momento correcto**. Los lenguajes resuelven esto de tres maneras:

| Enfoque | Ejemplo | Ventaja | Problema |
|---|---|---|---|
| **Manual** | C, C++ (`malloc`/`free`) | Control total, sin costo extra | Olvidar liberar (fuga), liberar dos veces, usar memoria liberada → fallos graves y vulnerabilidades |
| **Recolector de basura** | Java, Python, JavaScript, Go | Cómodo | Costo en ejecución, pausas impredecibles |
| **Ownership** | **Rust** | Seguro **y** sin costo en ejecución | Hay que aprender las reglas (este nivel) |

Los tres errores clásicos de memoria que Rust **elimina en compilación**:

1. **Fuga (leak):** nadie libera la memoria.
2. **Doble liberación (double free):** dos partes liberan lo mismo.
3. **Uso después de liberar / puntero colgante (use-after-free / dangling pointer):** se usa un dato que ya no existe.

## 1.7 Copiar en stack vs. copiar en heap

- Copiar un `i32` (stack): copiar 4 bytes. Barato.
- Copiar un `String` completo: pedir memoria nueva y duplicar todos los bytes. **Caro**.

Por eso Rust **nunca hace copias profundas escondidas**: si quieres duplicar datos del heap, lo escribes explícitamente con `.clone()`. Y por eso `let b = a;` no significa lo mismo para un `i32` que para un `String` (Módulo 3).

---

# Módulo 2: Ownership (propiedad)

## 2.1 Las tres reglas

> 1. **Cada valor tiene un único dueño** (una variable).
> 2. **Solo puede haber un dueño a la vez.**
> 3. **Cuando el dueño sale de su ámbito, el valor se libera** (Rust llama automáticamente a `drop`).

Con esto, el compilador sabe **exactamente cuándo** liberar cada dato del heap: al final del ámbito de su dueño. Sin recolector, sin `free` manual.

```rust
fn main() {
    {
        let s = String::from("hola");   // s es dueño del texto en el heap
        println!("{s}");
    }                                    // ← s sale de ámbito: el heap se libera aquí

    // println!("{s}");                  // ❌ E0425: s ya no existe
}
```

## 2.2 Ver el `drop` en acción

Para **ver** cuándo se libera un valor, usaremos una herramienta de visualización: el trait `Drop`. Su método `drop` se ejecuta automáticamente cuando el dueño sale de ámbito. (Por ahora, solo úsalo como "linterna": no necesitas implementarlo en tus programas.)

```rust
struct Aviso {
    nombre: String,
}

impl Aviso {
    fn new(nombre: &str) -> Self {
        Aviso { nombre: nombre.to_string() }
    }
}

impl Drop for Aviso {
    fn drop(&mut self) {
        println!("   [drop] liberando {}", self.nombre);
    }
}

fn main() {
    let _a = Aviso::new("A");
    {
        let _b = Aviso::new("B");
        println!("dentro del bloque");
    }                                    // B se libera aquí
    let _c = Aviso::new("C");
    println!("fin de main");
}                                        // C y luego A se liberan aquí
```

Salida:

```
dentro del bloque
   [drop] liberando B
fin de main
   [drop] liberando C
   [drop] liberando A
```

**Regla:** al terminar un ámbito, las variables se liberan en **orden inverso** al que se declararon (la última en nacer es la primera en morir).

*(En los ejemplos de este módulo usa el mismo `struct Aviso` con su `impl`; solo cambia el `main`.)*

### Detalles que sorprenden

```rust
fn main() {
    // 1) Reasignar libera el valor ANTERIOR en ese mismo momento
    let mut a = Aviso::new("vieja");
    println!("antes de reasignar");
    a = Aviso::new("nueva");             // aquí se libera "vieja"
    println!("después de reasignar");

    // 2) Shadowing NO libera el valor anterior: sigue vivo hasta el final del ámbito
    let x = Aviso::new("primera");
    let x = Aviso::new("segunda");
    println!("x ahora es {}", x.nombre);

    // 3) `_` descarta inmediatamente; `_nombre` vive hasta el final del ámbito
    let _ = Aviso::new("inmediato");     // se libera en esta misma línea
    let _guardado = Aviso::new("guardado");
    println!("fin de main");
}
```

Salida:

```
antes de reasignar
   [drop] liberando vieja
después de reasignar
x ahora es segunda
   [drop] liberando inmediato
fin de main
   [drop] liberando guardado
   [drop] liberando segunda
   [drop] liberando primera
   [drop] liberando nueva
```

## 2.3 Ownership y funciones

**Pasar un valor a una función transfiere la propiedad** (el valor se *mueve*):

```rust
fn consumir(a: Aviso) {
    println!("consumiendo {}", a.nombre);
}                                        // `a` sale de ámbito: se libera aquí

fn main() {
    let a = Aviso::new("A");
    consumir(a);                         // la propiedad pasa a la función
    println!("de vuelta en main");
    // println!("{}", a.nombre);         // ❌ E0382: `a` ya fue movido
}
```

Salida:

```
consumiendo A
   [drop] liberando A
de vuelta en main
```

**Devolver un valor transfiere la propiedad de vuelta:**

```rust
fn crear() -> String {
    let s = String::from("creado dentro");
    s                                    // se MUEVE al llamador (no se libera aquí)
}

fn tomar_y_devolver(s: String) -> String {
    println!("recibí: {s}");
    s                                    // devuelve la propiedad
}

fn main() {
    let a = crear();
    let b = tomar_y_devolver(a);         // a se movió; ahora b es el dueño
    println!("{b}");
}
```

Funciona, pero es **incómodo**: si cada función debe devolver lo que recibió para que no se pierda, el código se vuelve torpe. La solución elegante son los **préstamos** (Módulo 4).

## 2.4 Todas las operaciones y qué hacen con la propiedad

| Operación | Qué ocurre |
|---|---|
| `let b = a;` | **Move** (o copia si el tipo es `Copy`) |
| `f(a)` | `a` se mueve a la función |
| `a = nuevo;` | Se libera el valor viejo; el nuevo pasa a `a` |
| `v.push(a)` | `a` se mueve **dentro** del `Vec` |
| `return a` / último valor sin `;` | `a` se mueve al llamador |
| `&a` / `&mut a` | **Préstamo**: no mueve nada |
| `a.clone()` | Crea una copia **independiente** (nuevo dueño) |
| `for x in v` | El `Vec` se mueve y se consume; cada `x` es dueño de su elemento |
| `for x in &v` | Préstamo: `v` sigue siendo dueño |

## 2.5 Ownership en structs y colecciones

Un struct **es dueño** de sus campos; cuando el struct se libera, se liberan sus campos. Un `Vec` es dueño de sus elementos.

```rust
struct Persona {
    nombre: String,
    apellido: String,
    edad: u32,
}

fn main() {
    let p = Persona {
        nombre: String::from("Ana"),
        apellido: String::from("Gómez"),
        edad: 25,
    };

    // Movimiento parcial: sacamos SOLO un campo
    let n = p.nombre;
    println!("{} {}", p.apellido, p.edad);       // ✅ los demás campos siguen usables
    println!("{n}");

    // println!("{}", p.nombre);                 // ❌ E0382: p.nombre ya fue movido
    // let q = p;                                // ❌ E0382: p fue movido parcialmente
}
```

```rust
fn main() {
    let mut nombres: Vec<String> = Vec::new();

    let ana = String::from("Ana");
    nombres.push(ana);                            // `ana` se mueve al Vec
    // println!("{ana}");                         // ❌ E0382

    nombres.push(String::from("Luis"));

    let ultimo = nombres.pop();                   // Option<String>: AHORA nosotros somos dueños
    println!("{:?}", ultimo);                     // Some("Luis")

    let primero = nombres.remove(0);              // String: sacado del Vec, con propiedad
    println!("{primero}");

    // let x = nombres[0];                        // ❌ E0507: no se puede mover fuera de un índice
    // El Vec debe seguir siendo dueño de lo que contiene: usa & o .clone()
}
```

## 2.6 Mover dentro de bucles y condiciones

El compilador analiza **todos los caminos posibles**:

```rust
fn consumir(s: String) {
    println!("{s}");
}

fn main() {
    let s = String::from("hola");

    // ❌ E0382: value moved here, in previous iteration of loop
    // for _ in 0..2 {
    //     consumir(s);
    // }

    // ❌ E0382: movido en una rama; después podría estar ya movido
    // let condicion = true;
    // if condicion {
    //     consumir(s);
    // }
    // println!("{s}");

    // ✅ Re-inicializar una variable `mut` después de moverla es válido:
    let mut t = String::from("uno");
    consumir(t);
    t = String::from("dos");
    println!("{t}");

    let _ = s;
}
```

## 2.7 Lo que ownership NO permite (resumen)

| ❌ | Por qué |
|---|---|
| Usar un valor después de moverlo | El dueño anterior ya no es válido |
| Dos dueños del mismo dato del heap | Doble liberación |
| Sacar (mover) un elemento de un `Vec` por índice | El `Vec` quedaría con un "hueco" |
| Devolver una referencia a una variable local | La variable se libera al terminar la función (Módulo 7) |

---

# Módulo 3: Move y Copy

## 3.1 Move: la propiedad se transfiere

```rust
fn main() {
    let s1 = String::from("hola");
    let s2 = s1;                     // MOVE: s2 es ahora el único dueño

    // println!("{s1}");             // ❌ E0382: borrow of moved value: `s1`
    println!("{s2}");
}
```

¿Por qué `s1` queda inválida? Mira la memoria:

```
Después de  let s2 = s1;

   STACK                              HEAP
   ┌──────────────────┐
   │ s1  (INVÁLIDA ✗) │
   └──────────────────┘
   ┌──────────────────┐
   │ s2               │
   │  ptr ────────────┼──────────▶  ┌───┬───┬───┬───┐
   │  len = 4         │             │ h │ o │ l │ a │
   │  cap = 4         │             └───┴───┴───┴───┘
   └──────────────────┘
```

- Solo se copia la **ficha** (ptr, len, cap), que es barata. **El texto del heap no se duplica.**
- Si `s1` siguiera siendo válida, al terminar el ámbito **las dos** intentarían liberar el mismo texto → *double free*.
- Rust invalida `s1` y así **exactamente una** variable libera el heap.

> **Frase para clase:** *"Un move es copiar la ficha del stack e invalidar la original."*

## 3.2 Copy: tipos que se duplican solos

Si el dato vive **solo en el stack** y es barato de duplicar, copiarlo es seguro: ambas variables siguen siendo válidas e independientes.

```rust
fn main() {
    let a = 5;
    let b = a;                       // COPY: ambas válidas
    println!("{a} {b}");

    let p = (1, 2.5, 'x');
    let q = p;                       // la tupla entera es Copy
    println!("{:?} {:?}", p, q);

    let arr = [1, 2, 3];
    let arr2 = arr;
    println!("{:?} {:?}", arr, arr2);
}
```

### ¿Qué tipos son `Copy`?

| ✅ `Copy` | ❌ NO `Copy` (se mueven) |
|---|---|
| Enteros (`i32`, `u64`, `usize`...) | `String` |
| `f32`, `f64` | `Vec<T>` |
| `bool`, `char` | `HashMap`, `HashSet` |
| Referencias compartidas `&T` | Referencias mutables `&mut T` |
| Tuplas y arrays **si todos sus elementos son `Copy`** | Tuplas y arrays con algún elemento no `Copy` |
| `Option<T>` si `T` es `Copy` | `Option<String>` |
| Tus structs/enums con `#[derive(Clone, Copy)]` (si todos sus campos son `Copy`) | Structs con un `String`, `Vec`... |

```rust
fn main() {
    // Tupla y array con elemento no-Copy: se MUEVEN
    let t = (1, String::from("hola"));
    let u = t;
    // println!("{:?}", t);          // ❌ E0382
    println!("{:?}", u);

    let a = [String::from("a"), String::from("b")];
    let b = a;
    // println!("{:?}", a);          // ❌ E0382
    println!("{:?}", b);

    // Option
    let o1: Option<i32> = Some(5);
    let o2 = o1;
    println!("{:?} {:?}", o1, o2);   // ✅ Option<i32> es Copy

    let o3: Option<String> = Some(String::from("x"));
    let o4 = o3;
    // println!("{:?}", o3);         // ❌ Option<String> NO es Copy
    println!("{:?}", o4);
}
```

## 3.3 Copy en tus propios tipos

```rust
#[derive(Debug, Clone, Copy)]
struct Punto {
    x: f64,
    y: f64,
}

fn mover_a_origen(p: Punto) -> Punto {       // recibe una COPIA
    Punto { x: 0.0, y: p.y }
}

fn main() {
    let a = Punto { x: 1.0, y: 2.0 };
    let b = a;                               // copia
    let c = mover_a_origen(a);               // otra copia
    println!("{:?} {:?} {:?}", a, b, c);     // las tres válidas
}
```

Reglas para que un tipo sea `Copy`:

```rust
// ✅ Todos los campos son Copy
#[derive(Clone, Copy)]
struct A { x: i32, y: f64 }

// ✅ Una referencia compartida también es Copy
#[derive(Clone, Copy)]
struct B { texto: &'static str, n: u8 }

// ❌ E0204: `String` no es Copy, el struct no puede serlo
// #[derive(Clone, Copy)]
// struct C { nombre: String }

// ❌ E0184: un tipo con `Drop` (destructor) nunca puede ser Copy
// #[derive(Clone, Copy)]
// struct D;
// impl Drop for D { fn drop(&mut self) {} }

fn main() {
    let _ = A { x: 1, y: 2.0 };
    let _ = B { texto: "x", n: 1 };
}
```

## 3.4 Clone: copia profunda explícita

`.clone()` crea una **copia independiente**, incluyendo los datos del heap. Puede ser costosa, por eso es **explícita**.

```rust
fn main() {
    let s1 = String::from("hola");
    let s2 = s1.clone();             // nuevo bloque en el heap con el mismo texto

    println!("{s1} {s2}");           // ✅ las dos válidas e independientes
}
```

```
   STACK                              HEAP
   ┌──────────────────┐
   │ s1               │
   │  ptr ────────────┼──────────▶  ┌───┬───┬───┬───┐
   └──────────────────┘             │ h │ o │ l │ a │
   ┌──────────────────┐             └───┴───┴───┴───┘
   │ s2               │
   │  ptr ────────────┼──────────▶  ┌───┬───┬───┬───┐
   └──────────────────┘             │ h │ o │ l │ a │
                                    └───┴───┴───┴───┘
```

### Copy vs Clone

| | `Copy` | `Clone` |
|---|---|---|
| Se invoca | **Implícito** (al asignar o pasar) | **Explícito**: `.clone()` |
| Costo | Siempre barato (bytes del stack) | Puede ser caro (memoria del heap) |
| Qué tipos | Solo datos simples | Casi cualquier tipo |
| Relación | Todo `Copy` también es `Clone` | No todo `Clone` es `Copy` |

### ⚠️ `.clone()` como "parche" es un mal hábito

Cuando el compilador dice `E0382`, la tentación es poner `.clone()` hasta que compile. A veces es correcto, pero **muchas veces indica un diseño mejorable**. Orden de preferencia para corregir un error de move:

1. **Prestar** con `&` (casi siempre la mejor opción si solo lees).
2. **Reestructurar** (devolver el valor, cambiar el orden, acortar el ámbito).
3. **`.clone()`** si realmente necesitas dos dueños independientes.

```rust
fn longitud(s: &String) -> usize { s.len() }

fn main() {
    let s = String::from("hola");

    // ❌ Parche innecesario:  longitud_que_consume(s.clone())
    // ✅ Mejor: prestar
    let n = longitud(&s);
    println!("{s} tiene {n} bytes");
}
```

## 3.5 Move y Copy con referencias

```rust
fn main() {
    let s = String::from("hola");

    // &T es Copy: puedes tener muchas copias de la referencia
    let r1 = &s;
    let r2 = r1;                     // copia de la referencia
    println!("{r1} {r2} {s}");       // ✅ todas válidas

    // &mut T NO es Copy: se mueve
    let mut t = String::from("adiós");
    let m1 = &mut t;
    let m2 = m1;                     // m1 se MUEVE a m2
    // m1.push('!');                 // ❌ E0382: borrow of moved value: `m1`
    m2.push('!');
    println!("{t}");
}
```

## 3.6 Sacar valores a través de una referencia

Con una referencia **no eres dueño**, así que no puedes mover el contenido fuera:

```rust
fn main() {
    let numero = 5;
    let r = &numero;
    let copia = *r;                  // ✅ i32 es Copy: se copia el valor
    println!("{copia}");

    let texto = String::from("hola");
    let rt = &texto;
    // let robado = *rt;             // ❌ E0507: cannot move out of `*rt` which is behind a shared reference
    let duplicado = rt.clone();      // ✅ copia explícita
    println!("{duplicado}");
}
```

## 3.7 Resumen visual: ¿qué pasa con `let b = a;`?

```
¿El tipo de `a` es Copy?
        │
   ┌────┴────┐
  Sí         No
   │          │
 COPIA      MOVE
 a y b      b es el nuevo dueño
 válidas    a queda inválida
```

---

# Módulo 4: Borrowing: referencias `&T` y `&mut T`

## 4.1 Qué es una referencia

Una **referencia** es un puntero seguro: te permite **usar un valor sin ser su dueño**. Crear una referencia se llama **tomar prestado** (*borrowing*).

- `&x` → referencia **inmutable** (compartida): solo lectura.
- `&mut x` → referencia **mutable** (exclusiva): lectura y escritura.

```
   STACK                              HEAP
   ┌──────────────────┐
   │ s (dueño)        │
   │  ptr ────────────┼──────────▶  ┌───┬───┬───┬───┐
   │  len, cap        │             │ h │ o │ l │ a │
   └──────────────────┘             └───┴───┴───┴───┘
   ┌──────────────────┐
   │ r = &s           │
   │  ──▶ (apunta a s)│        r NO es dueña: no libera nada
   └──────────────────┘
```

Garantías de Rust sobre toda referencia:

- ✅ **Nunca es nula.**
- ✅ **Siempre apunta a un dato válido** (nunca colgante).
- ✅ **Respeta las reglas de aliasing** (Módulo 5).

## 4.2 Referencias inmutables `&T`

```rust
fn longitud(texto: &String) -> usize {
    texto.len()                        // solo lee
}

fn main() {
    let s = String::from("hola mundo");

    let n = longitud(&s);              // presta s, no la entrega
    println!("'{s}' tiene {n} bytes"); // ✅ s sigue siendo nuestra
}
```

### Muchas lecturas a la vez

```rust
fn main() {
    let s = String::from("hola");

    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{r1} {r2} {r3} {s}");    // ✅ todas leen; incluso el dueño puede leer
}
```

### ❌ No se puede modificar a través de `&T`

```rust
// fn agregar_signo(texto: &String) {
//     texto.push('!');       // ❌ E0596: cannot borrow `*texto` as mutable,
//                            //    as it is behind a `&` reference
// }
```

## 4.3 Referencias mutables `&mut T`

```rust
fn agregar_signo(texto: &mut String) {
    texto.push('!');                   // modifica el original
}

fn main() {
    let mut s = String::from("hola");  // la variable DEBE ser mut
    agregar_signo(&mut s);             // se presta con permiso de escritura
    println!("{s}");                   // hola!
}
```

Requisitos para `&mut`:

```rust
fn main() {
    // 1) El original debe declararse `mut`:
    let fija = String::from("x");
    // let r = &mut fija;             // ❌ E0596: cannot borrow `fija` as mutable

    // 2) Para asignar un valor completo a través de la referencia, usa *
    let mut n = 10;
    let r = &mut n;
    *r += 5;                          // modifica el valor apuntado
    *r = *r * 2;
    println!("{n}");                  // 30

    let _ = fija;
}
```

## 4.4 Desreferenciar con `*` y desreferencia automática

`*r` significa "el valor al que apunta `r`". Muchas veces Rust lo hace por ti:

```rust
fn main() {
    let s = String::from("hola");
    let r = &s;
    let rr = &r;                       // referencia a una referencia

    println!("{}", r.len());           // ✅ auto-deref: no necesitas (*r).len()
    println!("{}", rr.len());          // ✅ también funciona con &&String
    println!("{r} {rr}");              // println! también desreferencia
}
```

Pero con **operadores** hay que tener cuidado:

```rust
fn main() {
    let x = 5;
    let r = &x;

    println!("{}", *r == 5);           // ✅ comparar el VALOR
    println!("{}", r == &5);           // ✅ comparar referencia con referencia
    // println!("{}", r == 5);         // ❌ E0277: can't compare `&{integer}` with `{integer}`

    println!("{}", *r + 1);            // ✅ 6
    println!("{}", r + 1);             // ✅ 6 (i32 implementa suma con &i32)
}
```

**Regla práctica:** con **métodos** (`.len()`, `.push()`) y **campos** (`.nombre`), Rust desreferencia solo. Con **asignación** (`*r = ...`) y **comparaciones con un valor**, usa `*`.

## 4.5 Reasignar vs. modificar a través de la referencia

Son dos cosas distintas (ver también el Módulo 5):

```rust
fn main() {
    let a = 1;
    let b = 2;

    let mut r = &a;                    // `mut` aquí permite cambiar A QUÉ apunta r
    println!("{r}");                   // 1
    r = &b;                            // ahora apunta a b
    println!("{r}");                   // 2
}
```

## 4.6 Pasar referencias a funciones: el patrón diario

| Quiero... | Parámetro | Al llamar |
|---|---|---|
| Solo **leer** un texto | `&str` | `f(&s)` o `f("literal")` |
| Solo **leer** una secuencia | `&[T]` | `f(&v)` o `f(&array)` |
| Solo **leer** un valor grande | `&T` | `f(&x)` |
| **Modificar** el original | `&mut T` | `f(&mut x)` |
| **Consumir** el valor | `T` | `f(x)` |
| Valores pequeños (`i32`, `bool`...) | `T` | `f(x)` (se copian) |

```rust
fn saludar(nombre: &str) {                    // &str acepta String prestado Y literales
    println!("Hola, {nombre}");
}

fn promedio(valores: &[f64]) -> f64 {          // &[f64] acepta Vec y arrays
    let mut suma = 0.0;
    for v in valores {
        suma += v;
    }
    suma / valores.len() as f64
}

fn duplicar_todos(valores: &mut Vec<i32>) {   // &mut para modificar
    for v in valores.iter_mut() {
        *v *= 2;
    }
}

fn main() {
    let nombre = String::from("Ana");
    saludar(&nombre);                          // &String → &str automáticamente
    saludar("Luis");

    let notas = vec![4.0, 3.5, 5.0];
    println!("{:.2}", promedio(&notas));

    let mut numeros = vec![1, 2, 3];
    duplicar_todos(&mut numeros);
    println!("{:?}", numeros);                 // [2, 4, 6]
}
```

> **Consejo:** prefiere `&str` a `&String` y `&[T]` a `&Vec<T>` en los parámetros: aceptan más tipos y son igual de eficientes.

## 4.7 Referencias en bucles y métodos

```rust
fn main() {
    let mut v = vec![10, 20, 30];

    for x in &v {                      // préstamo inmutable: v sigue viva
        println!("{x}");
    }

    for x in &mut v {                  // préstamo mutable: se pueden modificar
        *x += 1;
    }

    for x in v.iter() {                // equivalente a &v
        print!("{x} ");
    }
    println!();

    println!("{:?}", v.first());       // Option<&i32>: una referencia dentro del Option
    println!("{:?}", v.get(1));        // Option<&i32>
}
```

Los métodos de structs usan las mismas formas (Nivel 2): `&self` presta para leer, `&mut self` presta para modificar y `self` consume.

## 4.8 Una referencia no puede sobrevivir a su dueño

```rust
// ❌ E0106: missing lifetime specifier
// fn colgante() -> &String {
//     let s = String::from("hola");
//     &s                              // s muere al terminar la función → referencia colgante
// }

// ✅ Devuelve el valor (transfiere la propiedad)
fn no_colgante() -> String {
    let s = String::from("hola");
    s
}

fn main() {
    println!("{}", no_colgante());
}
```

En el Módulo 7 verás cómo el compilador razona sobre esto con **lifetimes**.

---

# Módulo 5: Reglas de aliasing y mutabilidad

## 5.1 El problema: aliasing + mutación

**Aliasing** significa tener **más de un nombre (o puntero) para el mismo dato**. Mientras solo se **lee**, no hay problema. El peligro aparece cuando **alguien modifica** mientras **otro lee o escribe** el mismo dato:

- Lees un dato mientras otro lo cambia a la mitad → valores inconsistentes.
- Guardas un puntero a un elemento y otro código reubica la colección → puntero colgante.
- En programas con hilos → *data races*.

Rust resuelve todo esto con **una sola regla de fondo**:

> ## Aliasing **XOR** mutabilidad
> En un momento dado un dato puede tener **muchos lectores** (`&T`) **o** **un único escritor** (`&mut T`). **Nunca ambos.**

**Analogía útil para clase: un documento compartido.**
Muchas personas pueden **leerlo** a la vez, **o** una sola persona puede **editarlo**; pero nadie lee mientras alguien edita.

## 5.2 Las reglas del préstamo

1. **Muchas `&T` o una sola `&mut T`**, nunca las dos a la vez.
2. **Toda referencia debe ser siempre válida** (no puede sobrevivir al dueño: Módulo 7).
3. **Mientras un dato está prestado, el dueño queda "congelado"** según el tipo de préstamo:

| Situación | ¿El dueño puede **leer**? | ¿El dueño puede **escribir / mover**? |
|---|---|---|
| Sin préstamos activos | ✅ | ✅ |
| Con una o más `&T` activas | ✅ | ❌ |
| Con una `&mut T` activa | ❌ | ❌ |

*"Activa" significa: la referencia **se volverá a usar más adelante** (Módulo 8).*

## 5.3 Las infracciones, una por una

### E0499: dos `&mut` a la vez

```rust
fn main() {
    let mut s = String::from("hola");

    // ❌ E0499: cannot borrow `s` as mutable more than once at a time
    // let a = &mut s;
    // let b = &mut s;
    // a.push('1');
    // b.push('2');

    // ✅ Una después de otra (la primera termina de usarse antes de crear la segunda)
    let a = &mut s;
    a.push('1');
    let b = &mut s;
    b.push('2');
    println!("{s}");              // hola12
}
```

### E0502: `&` y `&mut` a la vez

```rust
fn main() {
    let mut s = String::from("hola");

    // ❌ E0502: cannot borrow `s` as mutable because it is also borrowed as immutable
    // let lectura = &s;
    // let escritura = &mut s;
    // println!("{lectura}");

    // ❌ También si el dueño intenta LEER mientras hay una &mut activa:
    // let w = &mut s;
    // println!("{s}");           // E0502
    // w.push('!');

    let lectura = &s;
    println!("{lectura}");        // la lectura termina aquí
    s.push('!');                  // ✅ ahora sí
    println!("{s}");
}
```

### E0506: asignar al dueño mientras está prestado

```rust
fn main() {
    let mut x = 10;

    // ❌ E0506: cannot assign to `x` because it is borrowed
    // let r = &x;
    // x = 20;
    // println!("{r}");

    let r = &x;
    println!("{r}");
    x = 20;                       // ✅ r ya no se usa
    println!("{x}");
}
```

### E0505: mover al dueño mientras está prestado

```rust
fn consumir(s: String) {
    println!("{s}");
}

fn main() {
    let s = String::from("hola");

    // ❌ E0505: cannot move out of `s` because it is borrowed
    // let r = &s;
    // consumir(s);
    // println!("{r}");

    let r = &s;
    println!("{r}");
    consumir(s);                  // ✅ el préstamo ya terminó
}
```

## 5.4 ¿Por qué existe? El caso clásico: el `Vec` que se muda

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    // ❌ E0502
    // let primero = &v[0];
    // v.push(4);                 // ← push puede necesitar MÁS espacio...
    // println!("{primero}");
}
```

```
Antes del push:                        Después del push (si no cabe):
STACK            HEAP                  STACK            HEAP
v ──▶ [1,2,3]  (cap=3)                 v ──▶ [1,2,3,4,_,_] (nuevo bloque)
primero ──▶ [1]                        primero ──▶ ??? ← apunta al bloque VIEJO,
                                                   ya liberado (puntero colgante)
```

`push` puede **reubicar todo el contenido** a otra zona del heap. Si `primero` siguiera apuntando al bloque viejo, leerlo sería **uso después de liberar**. Rust lo evita **prohibiendo** que exista esa referencia al mismo tiempo que la modificación. Sin hilos, sin ejecución: **lo detecta al compilar**.

> Cuando escribes C++ y esto ocurre, el programa compila y falla (o es explotable) en ejecución. En Rust, **ni compila**.

## 5.5 Aliasing en argumentos de funciones

La regla también evita sorpresas con dos parámetros que apuntan a lo mismo:

```rust
fn copiar(origen: &String, destino: &mut String) {
    destino.clear();
    destino.push_str(origen);        // si origen y destino fueran LA MISMA cadena, sería un desastre
}

fn main() {
    let mut a = String::from("hola");
    let mut b = String::from("otro");

    copiar(&a, &mut b);              // ✅ son datos distintos
    println!("{a} {b}");

    // ❌ E0502: la misma variable como lectura y escritura a la vez
    // copiar(&a, &mut a);

    // ❌ E0499: dos &mut a la misma variable
    // fn dos(x: &mut i32, y: &mut i32) {}
    // let mut n = 0;
    // dos(&mut n, &mut n);

    a.push('!');
    b.push('!');
}
```

> **Bonus técnico:** como Rust garantiza que un `&mut` no tiene alias, el compilador puede **optimizar con más agresividad**. La regla no solo da seguridad: también da velocidad.

## 5.6 Mutabilidad: `mut` en la variable vs. `&mut` en la referencia

Son **tres ideas distintas** que conviene separar:

| Declaración | ¿Se puede reasignar la variable? | ¿Se puede modificar el dato a través de ella? |
|---|---|---|
| `let x = 5` | ❌ | ❌ |
| `let mut x = 5` | ✅ | ✅ |
| `let r = &x` | ❌ | ❌ (es `&`) |
| `let mut r = &x` | ✅ (puede apuntar a otro dato) | ❌ (sigue siendo `&`) |
| `let r = &mut x` | ❌ (no puede apuntar a otro) | ✅ |
| `let mut r = &mut x` | ✅ | ✅ |

```rust
fn main() {
    let mut a = 1;
    let mut b = 2;

    let r = &mut a;
    *r += 10;                        // ✅ modificar a través de r
    // r = &mut b;                   // ❌ E0384: r no es `mut`: no puede apuntar a otro

    let mut s = &mut b;
    *s += 1;                         // ✅
    s = &mut a;                      // ✅ s es `mut`: puede cambiar de objetivo
    *s += 1;

    println!("{a} {b}");
}
```

### La inmutabilidad "se hereda" hacia abajo

```rust
struct Usuario {
    nombre: String,
    edad: u32,
}

fn main() {
    let u = Usuario { nombre: String::from("Ana"), edad: 25 };
    // u.edad += 1;                  // ❌ E0594: `u` no es mut → NINGÚN campo lo es
    // u.nombre.push('!');           // ❌ E0596

    let mut v = Usuario { nombre: String::from("Luis"), edad: 30 };
    v.edad += 1;                     // ✅ toda la instancia es mut
    v.nombre.push('!');

    let r = &v;
    // r.edad += 1;                  // ❌ E0594: a través de & nada se modifica
    println!("{} {}", r.nombre, r.edad);
}
```

## 5.7 Préstamos de partes distintas (*splitting borrows*)

La regla es **por dato**: si tomas prestadas **partes disjuntas**, no hay conflicto.

### Campos distintos de un struct

```rust
struct Equipo {
    nombres: Vec<String>,
    puntos: Vec<u32>,
}

fn main() {
    let mut e = Equipo { nombres: vec![], puntos: vec![] };

    let n = &mut e.nombres;          // préstamo mutable SOLO de nombres
    let p = &mut e.puntos;           // préstamo mutable SOLO de puntos
    n.push(String::from("Ana"));
    p.push(10);                      // ✅ campos distintos: sin conflicto

    println!("{:?} {:?}", e.nombres, e.puntos);
}
```

⚠️ Esto funciona con **acceso directo a campos**. Si pasas por un **método** que recibe `&mut self`, el compilador ve que se presta **todo** el struct:

```rust
impl Equipo {
    fn nombres_mut(&mut self) -> &mut Vec<String> {
        &mut self.nombres
    }
}

fn demo() {
    let mut e = Equipo { nombres: vec![], puntos: vec![] };

    // ❌ E0499: nombres_mut() presta TODO `e` mutablemente (el compilador solo mira la firma)
    // let n = e.nombres_mut();
    // e.puntos.push(1);
    // n.push(String::from("x"));

    e.puntos.push(1);                // ✅ si no hay préstamo vivo, no hay problema
    e.nombres_mut().push(String::from("x"));
}
```

### Partes distintas de un array o slice: `split_at_mut`

```rust
fn main() {
    let mut a = [1, 2, 3, 4];

    // ❌ E0499: el compilador no sabe que los índices 0 y 1 son distintos
    // let x = &mut a[0];
    // let y = &mut a[1];
    // *x += 1;
    // *y += 1;

    // ✅ split_at_mut divide en dos mitades disjuntas, garantizadas por la librería
    let (izq, der) = a.split_at_mut(2);
    izq[0] += 10;
    der[0] += 10;
    println!("{:?}", a);             // [11, 2, 13, 4]
}
```

## 5.8 Cómo resolver un conflicto de préstamos

Antes de rendirte con `.clone()`, prueba en este orden:

| Estrategia | Ejemplo |
|---|---|
| 1. **Acortar el primer préstamo**: usar el valor antes de modificar | `println!("{lectura}"); s.push('!');` |
| 2. **Copiar el valor simple** (si es `Copy`) en lugar de guardar una referencia | `let primero = v[0];` en vez de `let primero = &v[0];` |
| 3. **Reordenar** las instrucciones | Mover la escritura después de la última lectura |
| 4. **Dividir** el préstamo (campos / `split_at_mut`) | Sección 5.7 |
| 5. **Usar índices** en lugar de referencias persistentes | `for i in 0..v.len() { v[i] *= 2; }` |
| 6. **Recolectar cambios y aplicarlos después** | Guardar en otro `Vec` y luego `extend` |
| 7. **`.clone()`** si de verdad hacen falta dos dueños | Último recurso |

```rust
fn main() {
    // ❌ E0502: modificar el Vec mientras se recorre con préstamo
    // let mut v = vec![1, 2, 3];
    // for x in &v {
    //     v.push(*x);
    // }

    // ✅ Estrategia 5: índices (la longitud se calcula UNA vez, antes del bucle)
    let mut v = vec![1, 2, 3];
    for i in 0..v.len() {
        let x = v[i];
        v.push(x * 10);
    }
    println!("{:?}", v);             // [1, 2, 3, 10, 20, 30]

    // ✅ Estrategia 6: recolectar y aplicar después
    let mut w = vec![1, 2, 3];
    let mut nuevos = Vec::new();
    for x in &w {
        nuevos.push(x * 100);
    }
    w.extend_from_slice(&nuevos);
    println!("{:?}", w);             // [1, 2, 3, 100, 200, 300]
}
```

## 5.9 Resumen del módulo

1. **Muchos lectores o un escritor.** Nunca los dos.
2. Mientras hay préstamos activos, **el dueño queda restringido**.
3. `mut` en la variable ≠ `&mut` en la referencia.
4. Los préstamos de **partes disjuntas** no entran en conflicto (campos directos, `split_at_mut`).
5. Existen herramientas para casos que estas reglas no permiten (mutabilidad interior), pero son de niveles posteriores.

---

# Módulo 6: Slices: `&str` y `&[T]`

## 6.1 Qué es un slice

Un **slice** es una **referencia a una porción contigua** de una secuencia. **No es dueño** de los datos: es una "ventana".

| Tipo de slice | Mira... | Ejemplo |
|---|---|---|
| `&str` | Una porción de texto | `&s[0..4]` |
| `&[T]` | Una porción de un array o `Vec` | `&v[1..3]` |
| `&mut [T]` | Una porción que se puede modificar | `&mut v[1..3]` |

Por dentro, un slice es un **puntero + longitud** (16 bytes, "puntero gordo"):

```
let a = [10, 20, 30, 40, 50];
let parte = &a[1..4];

 STACK
 a  : [10][20][30][40][50]
            ▲
 parte: ptr ┘  len = 3      → ve [20, 30, 40]
```

## 6.2 Crear slices con rangos

| Sintaxis | Significado |
|---|---|
| `&a[1..4]` | posiciones 1, 2 y 3 (el final **no** se incluye) |
| `&a[1..=3]` | posiciones 1, 2 y 3 (el final **sí** se incluye) |
| `&a[..2]` | desde el inicio hasta la 1 |
| `&a[2..]` | desde la 2 hasta el final |
| `&a[..]` | todo |

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    println!("{:?}", &a[1..4]);      // [20, 30, 40]
    println!("{:?}", &a[1..=3]);     // [20, 30, 40]
    println!("{:?}", &a[..2]);       // [10, 20]
    println!("{:?}", &a[2..]);       // [30, 40, 50]
    println!("{:?}", &a[..]);        // [10, 20, 30, 40, 50]

    let v = vec![1, 2, 3, 4];        // también funciona con Vec
    println!("{:?}", &v[1..3]);      // [2, 3]

    let s = String::from("hola mundo");
    println!("{}", &s[0..4]);        // hola
    println!("{}", &s[5..]);         // mundo
}
```

### ⚠️ Rangos inválidos hacen `panic`

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    // ⚠️ panic: range end index 10 out of range for slice of length 5
    // let x = &a[2..10];

    // ⚠️ panic: slice index starts at 3 but ends at 2
    // let y = &a[3..2];

    // ✅ Versión segura: .get() con rango devuelve Option
    println!("{:?}", a.get(2..10));  // None
    println!("{:?}", a.get(1..3));   // Some([2, 3])
}
```

## 6.3 `&str`: el slice de texto

- Un **literal** `"hola"` es de tipo `&'static str`: un slice a texto guardado en el propio programa.
- `&String` se convierte automáticamente en `&str` (*deref coercion*).

| | `String` | `&str` |
|---|---|---|
| ¿Es dueño del texto? | ✅ | ❌ (solo mira) |
| ¿Se puede modificar? | ✅ (si es `mut`) | ❌ |
| ¿Dónde vive el texto? | Heap | Heap, stack o memoria del programa (según de dónde venga) |
| Tamaño en stack | 24 bytes | 16 bytes |
| Uso típico | Construir/modificar texto | **Parámetros de solo lectura**, literales |

```rust
fn es_saludo(texto: &str) -> bool {
    texto.starts_with("hola")
}

fn main() {
    let literal = "hola mundo";                   // &'static str
    let propio = String::from("hola a todos");    // String
    let parte = &propio[0..4];                    // &str que mira dentro de `propio`

    println!("{}", es_saludo(literal));           // ✅
    println!("{}", es_saludo(&propio));           // ✅ &String → &str
    println!("{}", es_saludo(parte));             // ✅
}
```

### ⚠️ Los índices de texto son **bytes**, no letras

UTF-8: una letra puede ocupar 1 a 4 bytes. Cortar en medio de una letra hace `panic`.

```rust
fn main() {
    let palabra = "ñandú";
    println!("{}", palabra.len());               // 7 bytes
    println!("{}", &palabra[0..2]);              // ñ  (la ñ ocupa 2 bytes)

    // ⚠️ panic: byte index 1 is not a char boundary
    // println!("{}", &palabra[0..1]);

    // ✅ Seguro: get devuelve Option
    println!("{:?}", palabra.get(0..1));         // None
    println!("{:?}", palabra.get(0..2));         // Some("ñ")

    // Para trabajar por letras, usa .chars():
    println!("{:?}", palabra.chars().nth(0));    // Some('ñ')
}
```

## 6.4 El slice **depende** de su dueño

Como un slice es un préstamo, **aplican las reglas del Módulo 5**. Es el ejemplo clásico que Rust previene:

```rust
fn primera_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, byte) in bytes.iter().enumerate() {
        if *byte == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let mut frase = String::from("hola mundo");

    let palabra = primera_palabra(&frase);       // préstamo inmutable de `frase`

    // ❌ E0502: cannot borrow `frase` as mutable because it is also borrowed as immutable
    // frase.clear();

    println!("La primera palabra es: {palabra}");  // `palabra` se usa aquí

    frase.clear();                               // ✅ ahora sí: `palabra` ya no se usa
    println!("[{frase}]");
}
```

Si `clear()` fuera permitido antes del `println!`, `palabra` quedaría apuntando a texto que ya no existe. **El slice mantiene "congelado" a su dueño mientras se use.** Ese es exactamente el tipo de error que en C produce fallos en ejecución.

## 6.5 `&[T]`: slices de arrays y vectores

Una función que recibe `&[T]` acepta **arrays, `Vec` y porciones de ambos**:

```rust
fn suma(numeros: &[i32]) -> i32 {
    let mut total = 0;
    for n in numeros {
        total += n;
    }
    total
}

fn mayor(numeros: &[i32]) -> Option<&i32> {
    numeros.iter().max()
}

fn segunda_mitad(numeros: &[i32]) -> &[i32] {
    let mitad = numeros.len() / 2;
    &numeros[mitad..]
}

fn main() {
    let array = [1, 2, 3, 4, 5, 6];
    let vector = vec![10, 20, 30];

    println!("{}", suma(&array));                // 21
    println!("{}", suma(&vector));               // 60
    println!("{}", suma(&array[1..4]));          // 9
    println!("{:?}", mayor(&vector));            // Some(30)
    println!("{:?}", segunda_mitad(&array));     // [4, 5, 6]
}
```

## 6.6 Slices mutables: `&mut [T]`

```rust
fn poner_ceros(valores: &mut [i32]) {
    for v in valores.iter_mut() {
        *v = 0;
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    poner_ceros(&mut v[1..3]);       // solo modifica las posiciones 1 y 2
    println!("{:?}", v);             // [1, 0, 0, 4, 5]

    let mut a = [5, 3, 8, 1];
    a.sort();                        // sort() actúa sobre el slice completo
    println!("{:?}", a);             // [1, 3, 5, 8]
    a[1..].reverse();                // invierte solo desde la posición 1
    println!("{:?}", a);             // [1, 8, 5, 3]
}
```

Las reglas de aliasing aplican igual: **un solo `&mut` sobre una región** (de ahí `split_at_mut` en el Módulo 5).

## 6.7 Métodos útiles de slices

```rust
fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    let s = &a[..];

    println!("{} {}", s.len(), s.is_empty());          // 6 false
    println!("{:?} {:?}", s.first(), s.last());        // Some(1) Some(6)
    println!("{:?}", s.get(10));                       // None
    println!("{}", s.contains(&4));                    // true

    let (izq, der) = s.split_at(2);                    // divide en dos slices (solo lectura)
    println!("{:?} {:?}", izq, der);                   // [1, 2] [3, 4, 5, 6]

    for par in s.windows(2) {                          // ventanas deslizantes de tamaño 2
        print!("{:?} ", par);                          // [1, 2] [2, 3] [3, 4] [4, 5] [5, 6]
    }
    println!();

    for grupo in s.chunks(4) {                         // trozos de hasta 4
        print!("{:?} ", grupo);                        // [1, 2, 3, 4] [5, 6]
    }
    println!();

    let propio: Vec<i32> = s[..3].to_vec();            // copia a un Vec con dueño propio
    println!("{:?}", propio);                          // [1, 2, 3]
}
```

`to_vec()` es la forma de pasar de "ventana prestada" a "datos propios".

## 6.8 Array, `Vec` y slice: la tabla que lo ordena todo

| | Array `[T; N]` | `Vec<T>` | Slice `&[T]` |
|---|---|---|---|
| ¿Es dueño de los datos? | ✅ | ✅ | ❌ (los mira) |
| Tamaño | Fijo, conocido al compilar | Variable | Se conoce en ejecución |
| Datos en | Stack | Heap | Donde estén (stack/heap) |
| ¿Crece? | ❌ | ✅ | ❌ |
| Como parámetro de solo lectura | Rara vez | Rara vez | ✅ **la mejor opción** |

## 6.9 Resumen del módulo

1. Un slice es una **ventana prestada** (puntero + longitud), nunca dueña.
2. `&str` para texto, `&[T]` para secuencias; `&mut [T]` para modificar.
3. Usa **slices en los parámetros**: aceptan más tipos y no copian nada.
4. Índices de texto = **bytes**; cuidado con letras de varios bytes.
5. Un slice **congela a su dueño** mientras se use.
6. `.get(rango)` es la versión que no hace `panic`.

---

# Módulo 7: Lifetimes: `'a`, referencias y su validez

## 7.1 El problema: referencias colgantes

Una referencia es válida **solo mientras vive el dato al que apunta**. Rust debe asegurarse de que **ninguna referencia sobreviva a su dato**:

⚠️ **Este programa NO compila** (es el error que queremos entender):

```rust
fn main() {
    let r;                           // r existe en TODO main
    {
        let x = 5;                   // x solo existe en este bloque
        r = &x;                      // ❌ E0597: `x` does not live long enough
    }                                // x se libera aquí
    println!("{r}");                 // r apuntaría a algo que ya no existe
}
```

```
fn main() {
    let r;                           // ───────────┐ 'a  (vida de r)
    {                                //            │
        let x = 5;                   // ───┐ 'b    │  (vida de x, más corta que 'a)
        r = &x;                      //    │       │
    }                                // ───┘       │
    println!("{r}");                 //            │  ← aquí 'b ya terminó: ERROR
}                                    // ───────────┘
```

> **Un lifetime es el tramo de código durante el cual una referencia es válida.**

El compilador **compara** la vida de la referencia con la vida del dato. Si el dato muere antes de que la referencia termine de usarse → error.

**Buena noticia:** casi siempre el compilador lo deduce solo. Las anotaciones (`'a`) solo se piden cuando hay **ambigüedad**.

## 7.2 Anotaciones de lifetime: la sintaxis

```rust
&i32            // una referencia (lifetime deducido)
&'a i32         // una referencia con lifetime llamado 'a
&'a mut i32     // referencia mutable con lifetime 'a
```

Se **declaran** entre `< >`, igual que los parámetros genéricos (Nivel 2, Módulo 7):

```rust
fn ejemplo<'a>(x: &'a str) -> &'a str {
    x
}
```

### ⚠️ Idea clave: las anotaciones **describen**, no **cambian** nada

Un lifetime anotado **no alarga ni acorta** la vida de ningún dato. Solo le dice al compilador **cómo se relacionan** las referencias entre sí, para que pueda verificar que todo es consistente.

> `'a` significa: *"esta referencia vive al menos tanto como `'a`"*, y la misma `'a` en dos lugares significa *"están ligadas: la más corta manda"*.

## 7.3 Cuándo hace falta: devolver una referencia sin saber de cuál

```rust
// ❌ E0106: missing lifetime specifier
// fn mas_larga(a: &str, b: &str) -> &str {
//     if a.len() > b.len() { a } else { b }
// }
```

El compilador pregunta: *"La referencia que devuelves, ¿viene de `a` o de `b`? ¿Cuánto debe vivir?"* No puede saberlo (depende de valores en ejecución). Tú se lo explicas:

```rust
fn mas_larga<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    let x = String::from("rust");
    let y = String::from("ownership");
    println!("{}", mas_larga(&x, &y));    // ownership
}
```

**Cómo leerlo:**
*"`a` y `b` viven al menos `'a`; el resultado vive como mucho `'a`. En la práctica, `'a` será **la más corta** de las dos."*

### Comprobar que funciona (el resultado depende del más corto)

```rust
fn mas_larga<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    let larga = String::from("una cadena larga");
    let resultado;
    {
        let corta = String::from("corta");
        resultado = mas_larga(larga.as_str(), corta.as_str());
        println!("{resultado}");              // ✅ dentro del bloque: `corta` aún vive
    }
    // println!("{resultado}");               // ❌ E0597: `corta` does not live long enough
    //                                        //    resultado podría apuntar a `corta`, ya liberada
}
```

El compilador no sabe si `resultado` apunta a `larga` o a `corta` → asume lo más estricto: **que podría apuntar a la corta**.

## 7.4 Lifetimes distintos: cuando una entrada no importa

Si el resultado **solo** puede venir de uno de los parámetros, ligas **solo ese**:

```rust
fn primero<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str {
    a                                // el resultado siempre viene de `a`
}

fn main() {
    let a = String::from("siempre vivo");
    let resultado;
    {
        let b = String::from("temporal");
        resultado = primero(&a, &b);   // ✅ `b` no condiciona al resultado
    }
    println!("{resultado}");           // ✅ válido: depende solo de `a`
}
```

Y el compilador detecta mentiras en la firma:

```rust
// ❌ error: lifetime may not live long enough
// fn elegir<'a, 'b>(a: &'a str, b: &'b str) -> &'a str {
//     b                              // la firma promete devolver algo ligado a 'a, pero devuelve 'b
// }
```

## 7.5 Devolver referencias a variables locales (siempre inválido)

```rust
// ❌ E0515: cannot return reference to local variable `s`
// fn crear<'a>() -> &'a String {
//     let s = String::from("hola");
//     &s                              // s muere al terminar la función
// }

// ✅ Soluciones:
fn crear_propio() -> String {          // 1) devolver el valor (con propiedad)
    String::from("hola")
}

fn crear_literal() -> &'static str {   // 2) devolver un literal (vive todo el programa)
    "hola"
}

fn main() {
    println!("{} {}", crear_propio(), crear_literal());
}
```

> **Regla:** una función **no puede devolver una referencia a algo creado dentro de ella**. Si lo creaste adentro, devuelve el valor.

También ocurre con **temporales**:

```rust
fn main() {
    // ❌ E0716: temporary value dropped while borrowed
    // let r: &str = String::from("hola").as_str();
    // println!("{r}");

    // ✅ Guarda el String en una variable para que viva lo suficiente
    let dueño = String::from("hola");
    let r: &str = dueño.as_str();
    println!("{r}");
}
```

## 7.6 Elisión de lifetimes: por qué casi nunca los escribes

Rust aplica **3 reglas** para completar los lifetimes que omitiste. Si tras aplicarlas todo queda determinado, no necesitas anotar:

1. **Cada** referencia de entrada sin anotar recibe su **propio** lifetime.
2. Si hay **exactamente un** lifetime de entrada, se asigna a **todas** las salidas.
3. Si hay `&self` o `&mut self`, el lifetime de `self` se asigna a **todas** las salidas.

```text
// Lo que escribes                       // Lo que el compilador entiende
fn largo(s: &str) -> usize               // fn largo<'a>(s: &'a str) -> usize
fn primera(s: &str) -> &str              // fn primera<'a>(s: &'a str) -> &'a str      (regla 2)
fn dos(a: &str, b: &str) -> usize        // fn dos<'a, 'b>(a: &'a str, b: &'b str) -> usize
fn mas_larga(a: &str, b: &str) -> &str   // ❌ dos entradas, ninguna regla decide → hay que anotar
fn nombre(&self) -> &str                 // fn nombre<'a>(&'a self) -> &'a str         (regla 3)
```

Por eso **todas las funciones con `&str` que escribiste hasta ahora compilaron sin anotaciones.**

## 7.7 Lifetimes en structs que guardan referencias

En el Nivel 2 dijimos *"en los structs usa `String`"*. Ahora sabemos por qué: un struct con `&str` necesita un lifetime que garantice que **el dato no muera antes que el struct**.

```rust
// ❌ E0106: missing lifetime specifier
// struct Extracto {
//     texto: &str,
// }

// ✅ El struct "depende" de que el texto viva al menos tanto como él
struct Extracto<'a> {
    texto: &'a str,
}

impl<'a> Extracto<'a> {                      // 'a se declara en el impl Y en el tipo
    fn new(texto: &'a str) -> Self {
        Extracto { texto }
    }

    fn longitud(&self) -> usize {
        self.texto.len()
    }

    // Devuelve algo con lifetime 'a (vive tanto como el TEXTO ORIGINAL)
    fn contenido(&self) -> &'a str {
        self.texto
    }

    // Aquí rige la regla 3: el resultado queda ligado a &self
    fn anunciar(&self, aviso: &str) -> &str {
        println!("Atención: {aviso}");
        self.texto
    }
}

fn main() {
    let novela = String::from("Llamadme Ismael. Hace algunos años...");
    let e = Extracto::new(&novela[..15]);

    println!("{}", e.longitud());              // 15
    println!("{}", e.contenido());             // Llamadme Ismael
    println!("{}", e.anunciar("importante"));
}
```

El compilador **impide** que el struct sobreviva al texto. ⚠️ **Este programa NO compila:**

```rust
fn main() {
    let e;
    {
        let texto = String::from("temporal");
        e = Extracto::new(&texto);        // ❌ E0597: `texto` does not live long enough
    }                                     // texto se libera aquí...
    println!("{}", e.contenido());        // ...pero `e` todavía la usa
}
```

### ¿Struct con `String` o con `&'a str`?

| | `String` (dueño) | `&'a str` (préstamo) |
|---|---|---|
| Complejidad | Simple, sin anotaciones | Requiere `<'a>` y entender su alcance |
| Copia datos | Sí (una vez al crear) | No (cero copias) |
| Independencia | Vive solo | Depende del texto original |
| Cuándo | **Por defecto**, datos que se guardan | Vistas temporales: analizadores, *parsers*, lectura de textos grandes |

> **Consejo pedagógico:** empieza siempre con tipos que **poseen** sus datos (`String`). Pasa a referencias con lifetimes solo cuando haya una razón concreta (rendimiento o evitar copias).

### Varios lifetimes en un struct

```rust
struct Par<'a, 'b> {
    primero: &'a str,
    segundo: &'b str,
}

fn main() {
    let a = String::from("uno");
    let b = String::from("dos");
    let p = Par { primero: &a, segundo: &b };
    println!("{} {}", p.primero, p.segundo);
}
```

## 7.8 Lifetimes junto a genéricos

Los lifetimes se listan **primero**, luego los tipos genéricos:

```rust
fn mayor<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> &'a T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", mayor(&3, &9));                          // 9
    println!("{}", mayor(&String::from("a"), &String::from("b")));   // b
}
```

## 7.9 `'static`

`'static` es el lifetime **más largo**: el dato vive **durante todo el programa**.

```rust
fn saludo() -> &'static str {
    "hola"                           // los literales de texto son 'static
}

fn main() {
    let s: &'static str = "texto fijo";
    println!("{s} {}", saludo());
}
```

### ⚠️ Mala práctica: usar `'static` para "callar" al compilador

Cuando un error de lifetime aparece, el mensaje a veces sugiere algo con `'static`. **Casi nunca es la solución correcta.** Casi siempre el problema es de diseño: el dato no vive lo suficiente. Opciones correctas: devolver un valor con dueño (`String`), reordenar ámbitos o clonar el dato.

## 7.10 Guía para resolver errores de lifetime

```
¿Necesito devolver una referencia?
   │
   ├─ No → devuelve el valor (String, Vec, struct...)  ← lo más simple
   │
   └─ Sí → ¿de dónde sale esa referencia?
          ├─ Es un literal             → &'static str
          ├─ Sale de UN parámetro      → ligar solo ese: fn f<'a>(x: &'a T, y: &U) -> &'a V
          ├─ Sale de VARIOS parámetros → fn f<'a>(x: &'a T, y: &'a T) -> &'a T
          └─ Sale de algo creado dentro → ❌ imposible: devuelve el valor
```

| Código | Mensaje | Qué significa |
|---|---|---|
| `E0106` | missing lifetime specifier | Rust no puede deducir el lifetime; anótalo |
| `E0597` | `x` does not live long enough | La referencia sobrevive a su dato |
| `E0515` | cannot return reference to local variable | Estás devolviendo una referencia a algo local |
| `E0716` | temporary value dropped while borrowed | Referencia a un temporal que muere en esa línea |
| (sin código) | lifetime may not live long enough | La firma promete más de lo que el cuerpo cumple |

## 7.11 Resumen del módulo

1. Un **lifetime** es el tramo en que una referencia es válida; el compilador lo compara con la vida del dato.
2. **Casi siempre se deduce solo** (reglas de elisión).
3. Se anota cuando una función devuelve una referencia y hay **varias** posibles fuentes, o cuando un **struct guarda referencias**.
4. Anotar **describe** relaciones; **no cambia** cuánto vive nada.
5. No se puede devolver una referencia a algo local: **devuelve el valor**.
6. Ante la duda, **usa tipos con dueño** (`String`, `Vec`).

---

# Módulo 8: Cómo funciona el borrow checker

El **borrow checker** es la parte del compilador que verifica las reglas de ownership, borrowing y lifetimes. Es **análisis estático**: se ejecuta al compilar y **no tiene costo en ejecución**.

## 8.1 Qué verifica, en tres preguntas

Para **cada uso** de una variable o referencia, el borrow checker pregunta:

1. **¿Está inicializada y no fue movida?** (si no → `E0382`, `E0381`)
2. **¿Hay algún préstamo vivo que entre en conflicto con este uso?** (si sí → `E0499`, `E0502`, `E0505`, `E0506`)
3. **¿Todas las referencias viven menos que su dato?** (si no → `E0597`, `E0515`, `E0106`)

> El compilador no ejecuta tu programa: **razona sobre todos los caminos posibles** que podría seguir.

## 8.2 Idea central: los préstamos tienen un alcance **calculado**

Cada vez que haces `&x` o `&mut x`, el compilador crea un **préstamo** y calcula **hasta dónde vive**: desde que se crea hasta **su último uso**.

## 8.3 NLL: *Non-Lexical Lifetimes* (la regla moderna)

Antiguamente un préstamo duraba hasta el **final del bloque `{}`**. Hoy dura **hasta su último uso**. Eso hace que muchísimos programas "razonables" compilen:

```rust
fn main() {
    let mut s = String::from("hola");

    let r = &s;                      // préstamo inmutable empieza
    println!("{r}");                 // ← ÚLTIMO USO de r: el préstamo termina aquí

    s.push('!');                     // ✅ no hay préstamo vivo
    println!("{s}");
}
```

Comparado con el caso que falla:

```rust
fn main() {
    let mut s = String::from("hola");

    let r = &s;                      // préstamo inmutable empieza...
    // s.push('!');                  // ❌ E0502: r todavía se usará abajo
    println!("{r}");                 // ...y termina aquí
}
```

> **La clave:** el error depende de si la referencia **se vuelve a usar después**, no de cuántas líneas haya en medio.

### El análisis considera el flujo de control

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let primero = &v[0];
    let hay_muchos = v.len() > 5;

    if hay_muchos {
        println!("{primero}");       // aquí `primero` se usa...
    } else {
        v.push(4);                   // ✅ ...pero en esta rama ya no: no hay conflicto
    }
}
```

## 8.4 Simular el borrow checker a mano (ejercicio estrella)

Recorre este programa línea por línea anotando **qué préstamos están vivos**:

```rust
fn main() {
    let mut s = String::from("hola");   // L1: s es dueña
    let r1 = &s;                        // L2: nace préstamo compartido #1
    let r2 = &s;                        // L3: nace préstamo compartido #2
    println!("{r1} {r2}");              // L4: último uso de r1 y r2 → mueren ambos
    let r3 = &mut s;                    // L5: nace préstamo mutable (no hay otros vivos) ✅
    r3.push('!');                       // L6
    println!("{r3}");                   // L7: último uso de r3 → muere
    println!("{s}");                    // L8: s libre otra vez ✅
}
```

| Línea | Préstamos vivos **después** de la línea | ¿Conflicto? |
|---|---|---|
| L1 | ninguno | — |
| L2 | `r1` (compartido) | no |
| L3 | `r1`, `r2` (compartidos) | no: muchos lectores |
| L4 | ninguno (último uso de ambos) | no |
| L5 | `r3` (mutable) | no: no había otros vivos |
| L6 | `r3` | no |
| L7 | ninguno (último uso) | no |
| L8 | ninguno | no: `s` puede leerse |

**Variante:** si agregas `println!("{r1}");` **después de L5**, `r1` ya no muere en L4: vive hasta ahí → en L5 coexiste con la `&mut` → **E0502**.

## 8.5 Leer un error del compilador

El formato es siempre parecido (el texto exacto puede variar un poco entre versiones):

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let primero = &v[0];
  |                    - immutable borrow occurs here
4 |     v.push(4);
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("{primero}");
  |               --------- immutable borrow later used here
```

**Receta de lectura en 3 pasos** (casi todos los errores de borrowing se leen igual):

| # | Pregunta | Dónde la responde el mensaje |
|---|---|---|
| 1 | ¿Dónde **nació** el primer préstamo? | "…borrow occurs here" (primera marca) |
| 2 | ¿Qué uso **entra en conflicto**? | La marca con `^^^^` |
| 3 | ¿Dónde **se vuelve a usar** el primer préstamo? | "…later used here" (la que mantiene vivo el préstamo) |

**La solución casi siempre está en la tercera:** acortar ese uso, o hacerlo antes del conflicto.

> **Tips de depuración:**
> - `cargo check` da estos errores sin generar el ejecutable (más rápido).
> - `rustc --explain E0502` imprime una explicación detallada del código de error, con ejemplos.

## 8.6 Reborrowing: por qué `&mut` no se "gasta" al pasarla

Una `&mut T` no es `Copy`, pero al pasarla a una función Rust hace un **reborrow** automático (un préstamo temporal más corto), así que sigues pudiendo usarla:

```rust
fn tocar(v: &mut Vec<i32>) {
    v.push(1);
}

fn main() {
    let mut datos = vec![];

    let r = &mut datos;
    tocar(r);                        // reborrow implícito: r sigue viva
    tocar(r);                        // ✅ otra vez
    r.push(3);                       // ✅

    println!("{:?}", datos);         // ✅ r ya no se usa: datos disponible. [1, 1, 3]
}
```

## 8.7 Préstamos de dos fases (*two-phase borrows*)

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    v.push(v.len());                 // ✅ compila
}
```

Parece un conflicto: `v.push(...)` toma `&mut v`, y `v.len()` toma `&v` a la vez. Rust lo permite porque el `&mut` se **reserva** primero y solo se **activa** cuando `push` realmente se ejecuta, **después** de evaluar los argumentos. Es una comodidad del compilador para casos evidentemente seguros.

## 8.8 Seguimiento de *moves*

El compilador también rastrea qué variables están **inicializadas**:

```rust
fn consumir(s: String) {
    println!("{s}");
}

fn main() {
    let mut s = String::from("a");
    consumir(s);                     // s queda "no inicializada"
    // println!("{s}");              // ❌ E0382

    s = String::from("b");           // ✅ asignar de nuevo la "revive" (requiere `mut`)
    println!("{s}");
}
```

## 8.9 El borrow checker es **conservador**

> Garantiza que **ningún programa inseguro compile**, pero a cambio **puede rechazar** algunos programas que en realidad son seguros.

Es el caso del método `nombres_mut()` del Módulo 5: el compilador solo mira **la firma**, no el cuerpo, así que asume lo peor (que se presta todo `self`). Soluciones habituales: acceder a los campos directamente, reestructurar, o dividir el struct en partes.

Cuando el compilador rechaza algo que "debería" ser válido:

1. **Reformula** el código (cambia orden, ámbitos, usa índices).
2. **Divide** el dato en partes independientes.
3. Si es un patrón realmente legítimo, existen herramientas más avanzadas (mutabilidad interior, código `unsafe`), pero pertenecen a niveles posteriores y son la **excepción**, no la regla.

## 8.10 Estrategia general ante un error de borrowing

```
El compilador se queja
        │
        ▼
1. Leer el error completo (3 pasos de 8.5)
        │
        ▼
2. ¿Puedo ACORTAR el préstamo? (usar la referencia antes, soltarla antes)
        │ no
        ▼
3. ¿Puedo COPIAR un valor simple en vez de guardar la referencia?
        │ no
        ▼
4. ¿Puedo REORDENAR o separar en partes (campos, split_at_mut, índices)?
        │ no
        ▼
5. ¿Debería cambiar quién ES DUEÑO del dato? (devolver el valor, mover)
        │ no
        ▼
6. .clone() — y preguntarme por qué hacía falta
```

## 8.11 Resumen del módulo

1. El borrow checker **razona sobre todos los caminos** del programa, sin ejecutarlo.
2. Un préstamo vive desde que se crea **hasta su último uso** (NLL), no hasta el final del bloque.
3. Todo error de borrowing se lee con **3 preguntas**: ¿dónde nace?, ¿qué choca?, ¿dónde se vuelve a usar?
4. Es **conservador**: prefiere rechazar un programa seguro a aceptar uno peligroso.
5. **Costo en ejecución: cero.**

---

# Proyecto integrador: auditoría de eventos sin copias innecesarias

El encargo empresarial de este nivel consiste en analizar logs de acceso usando ownership, borrowing, slices y lifetimes para evitar copias y referencias inválidas.

**Proyecto independiente:** [Abrir el enunciado completo](/proyectos/rust-nivel-3-auditoria-eventos/).

**Módulos integrados:** revisa en el enunciado qué requisito empresarial pone en práctica cada módulo del nivel.

---


# Ejercicios con soluciones

> 💡 **Formato recomendado:** en este nivel, los mejores ejercicios son **"¿compila o no?"** y **"arregla el error"**. Pide que expliquen **por qué** antes de mirar la solución.

## Módulo 1: Stack y heap

**1.1** Indica para cada variable qué parte vive en el stack y qué parte en el heap:

```rust
let a = 42;
let b = [1, 2, 3];
let c = String::from("rust");
let d = vec![1.5, 2.5];
let e = "literal";
let f = &a;
```

**1.2** ¿Por qué un `String` ocupa siempre 24 bytes en el stack, sin importar su contenido?

<details>
<summary>Ver soluciones</summary>

**1.1**

| Variable | Stack | Heap |
|---|---|---|
| `a` | el `i32` | — |
| `b` | los 3 `i32` (12 bytes) | — |
| `c` | ptr, len, capacity (24 bytes) | los bytes `rust` |
| `d` | ptr, len, capacity (24 bytes) | los 2 `f64` |
| `e` | ptr + len (16 bytes) | — (los bytes están en la memoria estática del programa) |
| `f` | el puntero (8 bytes) | — (apunta a `a`, que está en el stack) |

**1.2** Porque en el stack solo guarda la "ficha" (puntero, longitud y capacidad: 3 valores de 8 bytes). El texto en sí vive en el heap y puede ser de cualquier tamaño.
</details>

## Módulo 2: Ownership

**2.1** Predice la salida (en orden) y explica cuándo se libera cada valor:

```rust
fn main() {
    let _a = Aviso::new("A");
    let b = Aviso::new("B");
    let _c = Aviso::new("C");
    drop(b);                       // libera b manualmente (función de la librería estándar)
    println!("fin");
}
```

**2.2** Este código no compila. Arréglalo de **tres** formas distintas:

```rust
fn contar_letras(s: String) -> usize {
    s.len()
}

fn main() {
    let nombre = String::from("Ana");
    let n = contar_letras(nombre);
    println!("{nombre} tiene {n} letras");
}
```

<details>
<summary>Ver soluciones</summary>

**2.1** Salida:

```
   [drop] liberando B        ← drop(b) lo libera inmediatamente
fin
   [drop] liberando C        ← al final, orden inverso: C...
   [drop] liberando A        ← ...luego A
```

**2.2** Tres arreglos:

```rust
// Opción A (la mejor): prestar con &str
fn contar_letras(s: &str) -> usize {
    s.len()
}

fn main() {
    let nombre = String::from("Ana");
    let n = contar_letras(&nombre);
    println!("{nombre} tiene {n} letras");
}
```

```rust
// Opción B: clonar (funciona, pero copia el heap sin necesidad)
fn contar_letras(s: String) -> usize {
    s.len()
}

fn main() {
    let nombre = String::from("Ana");
    let n = contar_letras(nombre.clone());
    println!("{nombre} tiene {n} letras");
}
```

```rust
// Opción C: devolver la propiedad junto con el resultado
fn contar_letras(s: String) -> (String, usize) {
    let n = s.len();
    (s, n)
}

fn main() {
    let nombre = String::from("Ana");
    let (nombre, n) = contar_letras(nombre);
    println!("{nombre} tiene {n} letras");
}
```
</details>

## Módulo 3: Move y Copy

**3.1** ¿Qué líneas **no** compilan y por qué?

```rust
fn main() {
    let a = String::from("uno");        // L1
    let b = a;                          // L2
    let c = b.clone();                  // L3
    println!("{a}");                    // L4
    println!("{b} {c}");                // L5
    let n = 5;                          // L6
    let m = n;                          // L7
    println!("{n} {m}");                // L8
    let v = vec![String::from("x")];    // L9
    let s = v[0];                       // L10
}
```

**3.2** ¿Cuáles de estos tipos pueden derivar `Copy`?

```rust
struct A { x: i32, y: f64 }
struct B { nombre: String }
struct C { texto: &'static str, n: u8 }
struct D { datos: Vec<i32> }
struct E { par: (i32, [u8; 4]) }
```

<details>
<summary>Ver soluciones</summary>

**3.1**
- **L4** ❌ `E0382`: `a` fue movida a `b` en L2.
- **L10** ❌ `E0507`: no se puede mover un `String` fuera de un `Vec` por índice (usa `&v[0]` o `v[0].clone()`).
- L5, L8 ✅. L8 funciona porque `i32` es `Copy`.

**3.2**

| Tipo | ¿`Copy`? | Motivo |
|---|---|---|
| `A` | ✅ | `i32` y `f64` son `Copy` |
| `B` | ❌ | `String` no es `Copy` |
| `C` | ✅ | `&'static str` (referencia compartida) es `Copy`, `u8` también |
| `D` | ❌ | `Vec` no es `Copy` |
| `E` | ✅ | tupla y array de tipos `Copy` |
</details>

## Módulo 4 y 5: Borrowing y aliasing

**4.1** Arregla cada uno de estos tres programas (son programas **independientes**, cada uno con su `main`) **sin usar `.clone()`**:

```rust
// A
fn main() {
    let mut v = vec![1, 2, 3];
    let primero = &v[0];
    v.push(4);
    println!("{primero}");
}

// B
fn main() {
    let mut s = String::from("hola");
    let a = &mut s;
    let b = &mut s;
    a.push('1');
    b.push('2');
}

// C
fn main() {
    let mut x = 10;
    let r = &x;
    x += 1;
    println!("{r}");
}
```

**4.2** ¿Cuál es el error y por qué existe la regla que lo provoca?

```rust
fn agregar_copia(lista: &mut Vec<i32>, nuevo: &i32) {
    lista.push(*nuevo);
}

fn main() {
    let mut v = vec![1, 2, 3];
    agregar_copia(&mut v, &v[0]);
}
```

<details>
<summary>Ver soluciones</summary>

**4.1**

```rust
// A: copiar el valor simple (i32 es Copy) en lugar de guardar la referencia
fn main() {
    let mut v = vec![1, 2, 3];
    let primero = v[0];            // copia el valor
    v.push(4);
    println!("{primero}");
}

// B: usar cada &mut por separado (la primera termina antes de crear la segunda)
fn main() {
    let mut s = String::from("hola");
    let a = &mut s;
    a.push('1');
    let b = &mut s;
    b.push('2');
    println!("{s}");
}

// C: usar la lectura ANTES de modificar
fn main() {
    let mut x = 10;
    let r = &x;
    println!("{r}");
    x += 1;
    println!("{x}");
}
```

**4.2** Error `E0502`: se pide `&mut v` y `&v[0]` a la vez. Es exactamente el caso del `Vec` que se reubica: `push` podría mover el contenido y dejar `nuevo` apuntando a memoria liberada. Solución: copiar el valor primero.

```rust
fn agregar_copia(lista: &mut Vec<i32>, nuevo: &i32) {
    lista.push(*nuevo);
}

fn main() {
    let mut v = vec![1, 2, 3];
    let valor = v[0];                 // copia del i32: ya no hay préstamo sobre v
    agregar_copia(&mut v, &valor);
    println!("{:?}", v);              // [1, 2, 3, 1]
}
```
</details>

## Módulo 6: Slices

**6.1** Implementa estas tres funciones:

- `fn segunda_mitad(v: &[i32]) -> &[i32]`
- `fn poner_ceros(v: &mut [i32])`
- `fn ultima_palabra(s: &str) -> &str` (devuelve `""` si no hay palabras)

**6.2** ¿Por qué falla este código y cómo se arregla?

```rust
fn main() {
    let mut frase = String::from("hola mundo");
    let primera = &frase[0..4];
    frase.push_str(" cruel");
    println!("{primera}");
}
```

<details>
<summary>Ver soluciones</summary>

**6.1**

```rust
fn segunda_mitad(v: &[i32]) -> &[i32] {
    let mitad = v.len() / 2;
    &v[mitad..]
}

fn poner_ceros(v: &mut [i32]) {
    for x in v.iter_mut() {
        *x = 0;
    }
}

fn ultima_palabra(s: &str) -> &str {
    match s.split_whitespace().last() {
        Some(p) => p,
        None => "",
    }
}

fn main() {
    println!("{:?}", segunda_mitad(&[1, 2, 3, 4, 5]));      // [3, 4, 5]

    let mut v = vec![1, 2, 3, 4];
    poner_ceros(&mut v[1..3]);
    println!("{:?}", v);                                     // [1, 0, 0, 4]

    println!("{}", ultima_palabra("hola mundo cruel"));      // cruel
    println!("[{}]", ultima_palabra("   "));                 // []
}
```

**6.2** `E0502`: `primera` es un slice que **mira dentro de `frase`**; `push_str` puede reubicar el texto en el heap y dejar el slice colgante. Arreglos:

```rust
fn main() {
    // Opción 1: usar el slice antes de modificar
    let mut frase = String::from("hola mundo");
    let primera = &frase[0..4];
    println!("{primera}");
    frase.push_str(" cruel");

    // Opción 2: si necesitas conservar la palabra después, copiarla a un String propio
    let propia = frase[0..4].to_string();
    frase.push_str("!");
    println!("{propia} {frase}");
}
```
</details>

## Módulo 7: Lifetimes

**7.1** Añade las anotaciones necesarias para que compile:

```rust
fn elegir(a: &str, b: &str, usar_a: bool) -> &str {
    if usar_a { a } else { b }
}
```

**7.2** Arregla el struct:

```rust
struct Etiqueta {
    texto: &str,
}
```

**7.3** ¿Por qué no compila? Arréglalo de dos formas.

```rust
fn main() {
    let r;
    {
        let s = String::from("hola");
        r = &s;
    }
    println!("{r}");
}
```

**7.4** Arregla esta función:

```rust
fn crear() -> &String {
    let s = String::from("hola");
    &s
}
```

<details>
<summary>Ver soluciones</summary>

**7.1**

```rust
fn elegir<'a>(a: &'a str, b: &'a str, usar_a: bool) -> &'a str {
    if usar_a { a } else { b }
}

fn main() {
    println!("{}", elegir("uno", "dos", false));     // dos
}
```

**7.2**

```rust
struct Etiqueta<'a> {
    texto: &'a str,
}

impl<'a> Etiqueta<'a> {
    fn mostrar(&self) -> &str {
        self.texto
    }
}

fn main() {
    let e = Etiqueta { texto: "oferta" };
    println!("{}", e.mostrar());
}
```

**7.3** `E0597`: `s` muere al cerrar el bloque, pero `r` se usa después. Dos arreglos:

```rust
// Arreglo 1: que el dato viva tanto como la referencia
fn main() {
    let s = String::from("hola");
    let r = &s;
    println!("{r}");
}

// Arreglo 2: usar la referencia mientras el dato aún vive
fn main() {
    {
        let s = String::from("hola");
        let r = &s;
        println!("{r}");
    }
}
```

**7.4** No se puede devolver una referencia a una variable local. Devuelve el valor:

```rust
fn crear() -> String {
    String::from("hola")
}

fn main() {
    println!("{}", crear());
}
```
</details>

## Módulo 8: Borrow checker

**8.1** Para el siguiente programa, indica en qué línea ocurre el error, cuál es y por qué. Luego indica **qué cambio mínimo** lo arregla.

```rust
fn main() {
    let mut datos = vec![1, 2, 3];   // L1
    let a = &datos;                  // L2
    let b = &datos;                  // L3
    println!("{:?} {:?}", a, b);     // L4
    let c = &mut datos;              // L5
    c.push(4);                       // L6
    println!("{:?}", c);             // L7
    println!("{:?}", a);             // L8
}
```

**8.2** ¿Compila? Explica por qué.

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    v.push(v.len());
    let r = &mut v;
    r.push(10);
    r.push(20);
    println!("{:?}", v);
}
```

<details>
<summary>Ver soluciones</summary>

**8.1** El error está en **L5**: `E0502` (no se puede pedir `&mut datos` porque ya hay préstamos inmutables). `a` se usa por última vez en **L8**, así que su préstamo sigue **vivo** durante L5, L6 y L7, y choca con la `&mut`.

Cambio mínimo: **eliminar L8 o moverlo antes de L5** (o fusionarlo con L4). Así `a` y `b` mueren en L4 y L5 es válido.

**8.2** ✅ **Sí compila.**
- `v.push(v.len())`: funciona gracias a los *préstamos de dos fases*.
- `r.push(10)` y `r.push(20)`: `r` sigue siendo usable (los métodos hacen *reborrow* implícito).
- `println!("{:?}", v)`: `r` ya no se usa después, así que su préstamo mutable terminó (NLL).

Salida: `[1, 2, 3, 3, 10, 20]` (`v.push(v.len())` agrega un `3`, porque `v` tenía 3 elementos).
</details>

---

# Hoja de referencia rápida

```rust
// ---------- OWNERSHIP ----------
// 1) un dueño por valor  2) un solo dueño a la vez  3) el valor se libera al salir el dueño
let s2 = s1;                // MOVE (String, Vec...)  → s1 inválida
let b = a;                  // COPY (i32, f64, bool, char, &T...) → ambas válidas
let c = s.clone();          // copia profunda explícita
f(s);                       // mueve s a f
v.push(s);                  // mueve s al Vec

// ---------- BORROWING ----------
let r = &x;                 // préstamo inmutable (muchos a la vez)
let m = &mut x;             // préstamo mutable   (uno solo; x debe ser mut)
*m += 1;                    // modificar a través de &mut
*r == 5                     // comparar valor (con *)
r.len()                     // métodos y campos: auto-deref

// ---------- REGLAS ----------
// Muchos &T  XOR  un único &mut T      (nunca ambos a la vez)
// Mientras hay &T:   el dueño puede leer, NO escribir/mover
// Mientras hay &mut: el dueño NO puede leer ni escribir
// Un préstamo vive hasta su ÚLTIMO USO (NLL)

// ---------- SLICES ----------
&s[0..4]   &s[..]           // &str
&v[1..3]   &mut v[1..3]     // &[T], &mut [T]
fn f(x: &str) {}   fn g(x: &[i32]) {}      // parámetros recomendados
a.split_at_mut(2)           // dos &mut disjuntos
.get(0..2)                  // slice sin panic (Option)

// ---------- LIFETIMES ----------
fn f<'a>(x: &'a str, y: &'a str) -> &'a str { ... }   // resultado ligado a ambas
fn g(s: &str) -> &str { ... }                          // elisión: una entrada → misma vida
struct S<'a> { r: &'a str }                            // struct con referencia
impl<'a> S<'a> { fn m(&self) -> &'a str { self.r } }
&'static str                                           // vive todo el programa
// No se puede devolver &local → devuelve el valor

// ---------- STACK Y HEAP ----------
// stack: tamaño fijo, rápido, se libera solo (i32, [T;N], tuplas, punteros, fichas)
// heap : tamaño dinámico (contenido de String y Vec)
// String / Vec = 24 bytes en stack (ptr, len, cap);  &str / &[T] = 16 bytes (ptr, len);  &T = 8 bytes
```

### ¿Qué pongo en el parámetro?

```
¿Necesito CONSUMIR el valor (guardarlo, transformarlo)?   → T
¿Necesito MODIFICARLO?                                     → &mut T
¿Solo LEERLO?                                              → &T   (texto: &str, secuencias: &[T])
¿Es un tipo pequeño y Copy (i32, f64, bool, char)?         → T
```

---

# Errores frecuentes del compilador

| Código | Mensaje (resumido) | Causa típica | Solución |
|---|---|---|---|
| `E0382` | borrow of moved value | Usar un valor después de moverlo (a otra variable, función, `Vec`, bucle...) | Prestar con `&`, reestructurar, o `.clone()` |
| `E0381` | used binding isn't initialized | Variable declarada sin valor y usada | Inicializarla antes |
| `E0499` | cannot borrow as mutable more than once | Dos `&mut` vivas a la vez | Usar una a la vez; dividir el dato |
| `E0502` | cannot borrow as mutable because it is also borrowed as immutable | Leer y modificar a la vez | Usar la lectura antes; copiar el valor; reordenar |
| `E0505` | cannot move out of ... because it is borrowed | Mover al dueño mientras hay préstamo vivo | Terminar de usar el préstamo antes |
| `E0506` | cannot assign to ... because it is borrowed | Reasignar al dueño con préstamo vivo | Terminar de usar el préstamo antes |
| `E0507` | cannot move out of ... behind a reference / index | Sacar un `String` de `&T` o de `v[i]` | `&`, `.clone()`, `remove`/`pop` |
| `E0596` | cannot borrow as mutable | Falta `mut` en la variable, o se intenta mutar a través de `&` | `let mut`, o cambiar a `&mut` |
| `E0594` | cannot assign to ... as it is not declared as mutable | Asignar a campo/variable no `mut` | Declarar `mut` |
| `E0384` | cannot assign twice to immutable variable | Reasignar sin `mut` | Declarar `mut` |
| `E0106` | missing lifetime specifier | Función o struct con referencias sin lifetime deducible | Anotar `'a`, o devolver un valor propio |
| `E0597` | does not live long enough | La referencia sobrevive a su dato | Mover el dato a un ámbito más amplio, o usar la referencia antes |
| `E0515` | cannot return reference to local variable | Devolver `&local` | Devolver el valor |
| `E0716` | temporary value dropped while borrowed | Referencia a un temporal | Guardar el temporal en una variable |
| `E0204` | Copy cannot be implemented | `derive(Copy)` con un campo `String`/`Vec` | Usar `Clone` |
| `E0184` | Copy cannot be implemented for a type with a destructor | `Copy` + `Drop` | Elegir uno |
| (panic) | byte index is not a char boundary | Cortar un `&str` en medio de una letra | `.get(rango)` o `.chars()` |
| (panic) | range end index out of range | Slice fuera de rango | `.get(rango)` |

💡 **Herramientas:** `cargo check` (rápido, solo errores) y `rustc --explain E0502` (explicación del código de error con ejemplos).

---

# Plan de clases sugerido

Propuesta de 6 sesiones de ~90 minutos. Este nivel es **el más importante**: conviene no apurarlo.

| Sesión | Contenido | Actividad práctica |
|---|---|---|
| **1** | Módulo 1 (stack y heap) + Módulo 2 (ownership) | Dibujar en la pizarra el stack y el heap de un `String`; ejercicios 1.1, 1.2, 2.1 con `Aviso` y `Drop` |
| **2** | Módulo 3 (move y Copy) | Ejercicios 2.2, 3.1, 3.2; "¿compila o no?" en vivo |
| **3** | Módulo 4 (referencias `&` y `&mut`) | Reescribir funciones que consumen para que presten; práctica de `&str` y `&[T]` |
| **4** | Módulo 5 (aliasing y mutabilidad) | Ejercicios 4.1 y 4.2; provocar E0499, E0502, E0505 y E0506 a propósito |
| **5** | Módulo 6 (slices) + Módulo 7 (lifetimes, primera parte) | Ejercicios 6.1, 6.2, 7.1, 7.2 |
| **6** | Módulo 7 (lifetimes, segunda parte) + Módulo 8 (borrow checker) + Proyecto integrador | Ejercicios 7.3, 7.4, 8.1, 8.2; simulación a mano del borrow checker; ampliar el analizador de auditoría de eventos |

### Recomendaciones de enseñanza

1. **Dibuja memoria.** Antes de escribir código, que los estudiantes dibujen stack y heap. Casi todo error de ownership se entiende mirando el dibujo.
2. **Usa la analogía del documento compartido** para aliasing: muchos lectores **o** un editor, nunca ambos.
3. **Enseña a leer el error antes de corregirlo.** La receta de 3 preguntas del Módulo 8 (¿dónde nace el préstamo?, ¿qué choca?, ¿dónde se vuelve a usar?) resuelve casi todos los casos.
4. **Prohíbe el `.clone()` reflejo.** Exige justificar cada `.clone()`: "¿por qué necesito dos dueños?". La mayoría de las veces la respuesta es "no los necesito, me basta prestar".
5. **Ejercicios de predicción:** "¿compila?", "¿qué línea falla?", "¿qué se imprime y en qué orden?". Son más valiosos que escribir código nuevo en este nivel.
6. **Normaliza pelear con el compilador.** Es parte del aprendizaje. Con práctica, los errores pasan de frustrantes a informativos.
7. **Ideas centrales para repetir hasta que salgan solas:**
   - *Cada valor tiene un dueño; cuando el dueño se va, el valor se libera.*
   - *Prestar no es regalar: `&` para leer, `&mut` para escribir.*
   - *Muchos lectores o un escritor, nunca los dos.*
   - *Una referencia nunca puede vivir más que su dato.*

### Qué sigue

Con este nivel dominado, el resto de Rust (iteradores y closures, punteros inteligentes, trait objects, concurrencia) se vuelve mucho más accesible, porque casi todo se apoya en lo que aprendiste aquí.

---

*Fin del Nivel 3. Quien entiende ownership, entiende Rust. 🦀*
