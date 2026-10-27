---
title: "Rust — Nivel 5: Concurrencia y programación asíncrona"
description: "Explora threads, Send y Sync, Arc, Mutex, canales, async/await y Tokio para construir programas concurrentes y asíncronos."
language: rust
level: 5
levelLabel: "Nivel 05"
category: "Concurrencia y async"
duration: "Avanzado"
order: 5
slug: rust-nivel-5
projectSlug: "rust-nivel-5-ingesta-eventos"
tags: ["threads", "Send", "Sync", "async", "await", "Tokio"]
---
# 🦀 Curso de Rust — Nivel 5: Concurrencia y programación asíncrona

> **Objetivo:** que tus estudiantes puedan escribir programas que **hacen varias cosas a la vez** (con hilos y con `async`), entiendan **por qué el compilador de Rust les impide las carreras de datos**, y sepan **controlar** esa concurrencia: cancelarla, ponerle plazos, frenarla cuando hay demasiada carga y limitarla.
>
> **Requisitos:** Niveles 1 a 4. Este nivel se apoya mucho en **ownership, borrowing y lifetimes** (Nivel 3), en **traits** (Nivel 2) y en **`Result`/`Option`** (Nivel 4).
>
> **Idea central del nivel:**
> *En Rust, el compilador convierte los errores de concurrencia más peligrosos (las carreras de datos) en **errores de compilación**. Si compila, no tiene carreras de datos.*

---

## Índice

0. [Puente: closures, lo mínimo](#módulo-0-puente-closures-lo-mínimo)
1. [Mapa mental: concurrencia, paralelismo y async](#módulo-1-mapa-mental-concurrencia-paralelismo-y-async)
2. [Threads y `std::thread`](#módulo-2-threads-y-stdthread)
3. [`Send` y `Sync`](#módulo-3-send-y-sync)
4. [`Arc`, `Mutex` y `RwLock`](#módulo-4-arc-mutex-y-rwlock)
5. [Canales para comunicar tareas (`std::sync::mpsc`)](#módulo-5-canales-para-comunicar-tareas-stdsyncmpsc)
6. [`async` y `.await`](#módulo-6-async-y-await)
7. [Futures y runtime asíncrono](#módulo-7-futures-y-runtime-asíncrono)
8. [Tokio: tareas](#módulo-8-tokio-tareas)
9. [Tokio: canales y sincronización](#módulo-9-tokio-canales-y-sincronización)
10. [Tokio: cancelación](#módulo-10-tokio-cancelación)
11. [Timeouts, backpressure y límites de concurrencia](#módulo-11-timeouts-backpressure-y-límites-de-concurrencia)
9. [Proyecto integrador: ingesta de eventos de pedidos en tiempo real](#proyectointegradoringestadeeventosdepedidosentiemporeal)
13. [Ejercicios con soluciones](#ejercicios-con-soluciones)
14. [Hoja de referencia rápida](#hoja-de-referencia-rápida)
15. [Errores frecuentes](#errores-frecuentes)
16. [Plan de clases sugerido](#plan-de-clases-sugerido)

**Leyenda:**

- ✅ = se puede / es correcto
- ❌ = NO se puede / no compila
- ⚠️ = compila, pero cuidado (puede fallar, bloquearse o dar resultados engañosos)

> **Aviso sobre la salida de los programas concurrentes:** cuando varios hilos o tareas imprimen a la vez, **el orden de las líneas puede cambiar de una ejecución a otra**. En este curso indicamos cuándo una salida es **estable** y cuándo es solo **"típica"**. Es una buena oportunidad para que tus estudiantes vean la no-determinación con sus propios ojos: que ejecuten cada ejemplo varias veces.

> **Aviso sobre el alcance:** los hilos y Tokio **exigen closures** (`|| ...`), que no habíamos visto. Por eso el Módulo 0 es un puente mínimo, como el que hicimos con ownership en el Nivel 2. Los iteradores y los combinadores (`map`, `and_then`...) siguen **fuera** de este nivel.

---

# Módulo 0: Puente: closures, lo mínimo

Un **closure** es una función **anónima** que puede **capturar** variables de su entorno. Se escribe con barras verticales: `|parámetros| cuerpo`.

## 0.1 Sintaxis

```rust
fn main() {
    let sumar = |a: i32, b: i32| a + b;          // con tipos
    let doble = |x| x * 2;                       // tipos inferidos por el primer uso
    let saludar = || println!("hola");           // sin parámetros
    let compleja = |x: i32| {                    // con bloque
        let y = x * 10;
        y + 1
    };

    println!("{} {}", sumar(2, 3), doble(4));    // 5 8
    saludar();
    println!("{}", compleja(2));                 // 21
}
```

## 0.2 Capturar variables (lo que una función normal no puede)

```rust
fn main() {
    // 1) Captura por referencia compartida (&): solo lee
    let factor = 3;
    let multiplicar = |x| x * factor;
    println!("{}", multiplicar(5));              // 15

    // 2) Captura por referencia mutable (&mut): modifica
    let mut contador = 0;
    let mut incrementar = || contador += 1;      // el closure debe declararse `mut`
    incrementar();
    incrementar();
    println!("{contador}");                      // 2 (usable porque el closure ya no se usa)

    // 3) Captura por valor con `move`: se lleva la propiedad
    let nombre = String::from("Ana");
    let saludar = move || println!("hola, {nombre}");
    saludar();
    // println!("{nombre}");                     // ❌ E0382: `nombre` se movió al closure
}
```

> **`move` es la palabra más importante de este nivel.** Hace que el closure **sea dueño** de lo que captura. Los hilos y las tareas la necesitan porque **pueden vivir más que la función que las creó**.

## 0.3 `move` con tipos `Copy`: se copia

```rust
fn main() {
    let mut n = 10;
    let mut sumar_uno = move || {
        n += 1;                                   // modifica SU copia
        n
    };
    println!("{}", sumar_uno());                  // 11
    println!("{}", sumar_uno());                  // 12
    println!("{n}");                              // 10  ← el original no cambió
}
```

(Esto vuelve a aparecer en el Módulo 2 como una trampa con hilos.)

## 0.4 Los tres traits de los closures

El compilador clasifica cada closure según **cómo usa** lo que captura:

| Trait | El closure... | Se puede llamar... |
|---|---|---|
| `Fn` | solo **lee** lo capturado | muchas veces, incluso a la vez |
| `FnMut` | **modifica** lo capturado | muchas veces, pero de una en una |
| `FnOnce` | **consume** (mueve) lo capturado | **una sola vez** |

Los parámetros que reciben closures usan estos traits como restricciones (Nivel 2):

```rust
fn aplicar<F: Fn(i32) -> i32>(f: F, valor: i32) -> i32 {
    f(valor)
}

fn ejecutar_una_vez<F: FnOnce() -> String>(f: F) -> String {
    f()
}

fn main() {
    let sumar_diez = |x| x + 10;
    println!("{}", aplicar(sumar_diez, 5));                 // 15

    let texto = String::from("consumido");
    let consumir = move || texto;                           // devuelve (mueve) el String
    println!("{}", ejecutar_una_vez(consumir));
}
```

**Para este nivel basta recordar:** `thread::spawn` y `tokio::spawn` piden un closure (o bloque `async`) que **sea dueño de sus datos** (`move`) y que se pueda **enviar a otro hilo** (`Send`).

## 0.5 Closures y errores comunes

| Error | Causa | Solución |
|---|---|---|
| `E0373` closure may outlive the current function | El closure toma prestado algo local y se envía a un hilo | Agregar `move` |
| `E0382` use of moved value (tras un `move`) | Usaste la variable después de llevársela el closure | Clonar antes (`Arc::clone`, `.clone()`) |
| `E0596` cannot borrow as mutable | Llamar un closure `FnMut` sin declararlo `mut` | `let mut f = || ...` |

---

# Módulo 1: Mapa mental: concurrencia, paralelismo y async

## 1.1 Tres palabras que se confunden

| Término | Significado | Analogía (cocina) |
|---|---|---|
| **Concurrencia** | Manejar **varias tareas en progreso** a la vez (aunque solo una avance en cada instante) | Un cocinero que mientras hierve el agua, pica las verduras |
| **Paralelismo** | Ejecutar **varias tareas simultáneamente** en varios núcleos | Dos cocineros trabajando a la vez |
| **Asincronía (`async`)** | Un estilo de programación para **esperar sin bloquear**, ideal con mucha E/S | Un cocinero que, en vez de mirar fijamente el horno, sigue con otra receta y vuelve cuando suena el timbre |

## 1.2 ¿Qué problema se resuelve con cada herramienta?

| Tipo de trabajo | Cuello de botella | Herramienta |
|---|---|---|
| **CPU-bound** (cálculos pesados: procesar imágenes, comprimir, simular) | El procesador | **Hilos** (paralelismo real en varios núcleos) |
| **I/O-bound** (red, discos, bases de datos: casi todo es **esperar**) | La espera | **`async`** (miles de tareas esperando con muy poca memoria) |
| Mezcla | Ambos | `async` para la E/S + hilos / `spawn_blocking` para el cálculo |

## 1.3 Hilos vs. tareas async

| | Hilo del sistema (`std::thread`) | Tarea async (Tokio) |
|---|---|---|
| Quién lo planifica | El **sistema operativo** | El **runtime** (en espacio de usuario) |
| Memoria por unidad | Una pila propia (típicamente ~2 MiB reservados en Rust) | Un pequeño objeto en el heap (del orden de cientos de bytes a pocos KB) |
| Cantidad práctica | Cientos o pocos miles | **Cientos de miles** |
| Cambio de contexto | Costoso (kernel) | Barato (solo un salto en el runtime) |
| Puede bloquearse | ✅ (el SO sigue planificando a los demás) | ❌ **No debe** bloquear el hilo del runtime |
| Ideal para | Cálculo pesado, tareas largas | Muchas esperas de red/disco |

## 1.4 Qué garantiza Rust (y qué no)

Rust llama a esto **"concurrencia sin miedo"** (*fearless concurrency*). Es importante ser **honestos** sobre su alcance:

| Problema | ¿Lo previene el compilador? |
|---|---|
| **Carrera de datos** (*data race*: dos hilos acceden al mismo dato, uno escribe, sin sincronizar) | ✅ **Sí**, en código seguro. Es un error de compilación |
| Uso después de liberar, punteros colgantes entre hilos | ✅ Sí (ownership y lifetimes) |
| **Condición de carrera lógica** (el resultado depende del orden de ejecución) | ❌ No |
| **Deadlock** (interbloqueo: dos hilos se esperan mutuamente para siempre) | ❌ No |
| Inanición, bloqueo del runtime async | ❌ No |
| Mal diseño (demasiado `Mutex`, demasiadas tareas) | ❌ No |

> **Frase para clase:** *"Rust te garantiza que no corromperás la memoria con varios hilos. Que el programa haga lo correcto sigue siendo trabajo tuyo."*

## 1.5 El error que Rust evita: una carrera de datos

```rust
use std::thread;

fn main() {
    let mut contador = 0;

    // ❌ E0373: closure may outlive the current function, but it borrows `contador`
    // let h = thread::spawn(|| {
    //     contador += 1;
    // });
    // h.join().unwrap();

    // ⚠️ Con `move` compila... pero el hilo modifica SU COPIA (contador es Copy):
    let h = thread::spawn(move || {
        contador += 1;
        println!("[hilo] contador = {contador}");     // 1
    });
    h.join().expect("el hilo no debe hacer panic");
    println!("[main] contador = {contador}");         // 0  ← no cambió
}
```

En C o C++, esto compilaría y fallaría de formas impredecibles. En Rust te obliga a **elegir explícitamente** cómo compartir (módulo 4) o comunicar (módulo 5).

---

# Módulo 2: Threads y `std::thread`

## 2.1 Crear un hilo con `thread::spawn`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let manejador = thread::spawn(|| {
        for i in 1..=3 {
            println!("[hilo] vuelta {i}");
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=3 {
        println!("[main] vuelta {i}");
        thread::sleep(Duration::from_millis(50));
    }

    manejador.join().expect("el hilo no debe hacer panic");
}
```

Salida **típica** (el orden entre `[hilo]` y `[main]` puede variar):

```
[main] vuelta 1
[hilo] vuelta 1
[main] vuelta 2
[hilo] vuelta 2
[main] vuelta 3
[hilo] vuelta 3
```

- `thread::spawn` recibe un closure y devuelve un **`JoinHandle<T>`**.
- **`join()`** espera a que el hilo termine y devuelve `Result<T, _>`: `Ok(valor)` con lo que devolvió el closure, o `Err(_)` si el hilo hizo **panic**.

### ⚠️ Si `main` termina, los hilos mueren

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        println!("esto probablemente NUNCA se imprime");
    });
    println!("main termina ya");
    // Sin join(), al terminar main el programa se cierra y mata el hilo.
}
```

**Regla:** si necesitas que un hilo termine, **guarda su `JoinHandle` y llama a `join()`**.

## 2.2 `move` y la propiedad de los datos

El closure del hilo puede vivir más que la función que lo creó, así que **no puede tomar prestadas** variables locales:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // ❌ E0373: closure may outlive the current function, but it borrows `v`
    // let h = thread::spawn(|| {
    //     println!("{:?}", v);
    // });

    // ✅ `move`: el hilo se vuelve dueño de `v`
    let h = thread::spawn(move || {
        println!("{:?}", v);
    });

    // println!("{:?}", v);         // ❌ E0382: `v` se movió al hilo
    h.join().expect("el hilo no debe hacer panic");
}
```

La firma de `spawn` lo explica todo (la leerás mejor tras el Módulo 3):

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,    // closure que se ejecuta una vez, enviable a otro hilo, sin préstamos locales
    T: Send + 'static,                    // el resultado también debe poder viajar y no depender de préstamos
```

- `FnOnce`: el closure se llama **una vez**.
- `Send`: se puede **mover** a otro hilo.
- `'static`: **no contiene referencias** a datos locales que puedan desaparecer (Nivel 3: lifetimes).

## 2.3 Devolver valores desde un hilo

```rust
use std::thread;

fn main() {
    let h = thread::spawn(|| {
        let mut suma: u64 = 0;
        for i in 1..=1_000_000u64 {
            suma += i;
        }
        suma                                   // el valor de retorno del closure
    });

    let resultado = h.join().expect("el hilo no debe hacer panic");
    println!("{resultado}");                   // 500000500000
}
```

## 2.4 Varios hilos

```rust
use std::thread;

fn main() {
    let mut manejadores = Vec::new();

    for id in 0..4 {
        manejadores.push(thread::spawn(move || {
            id * id                            // cada hilo calcula su propio resultado
        }));
    }

    let mut resultados = Vec::new();
    for h in manejadores {
        resultados.push(h.join().expect("el hilo no debe hacer panic"));
    }
    println!("{:?}", resultados);              // [0, 1, 4, 9]   (en el orden de los manejadores)
}
```

> Aunque los hilos corran en cualquier orden, **recoger los resultados en el orden de los `JoinHandle`** da una salida estable.

## 2.5 Si un hilo hace `panic`

El `panic` **mata solo a ese hilo**; los demás siguen. `join()` te avisa con `Err`:

```rust
use std::thread;

fn main() {
    let h = thread::spawn(|| {
        panic!("fallo dentro del hilo");
    });

    match h.join() {
        Ok(_) => println!("el hilo terminó bien"),
        Err(_) => println!("el hilo hizo panic, pero main sigue vivo"),
    }
}
```

## 2.6 Nombrar hilos y consultar el entorno

```rust
use std::thread;

fn main() {
    let h = thread::Builder::new()
        .name(String::from("trabajador-1"))
        .spawn(|| {
            println!("soy {:?}", thread::current().name());      // Some("trabajador-1")
        })
        .expect("no se pudo crear el hilo");          // Builder::spawn devuelve io::Result
    h.join().expect("el hilo no debe hacer panic");

    println!("{:?}", thread::current().name());        // Some("main")

    match thread::available_parallelism() {
        Ok(n) => println!("núcleos disponibles: {n}"),
        Err(e) => println!("no se pudo consultar: {e}"),
    }
}
```

Un hilo con nombre aparece en los mensajes de `panic` y en depuradores: muy útil.

## 2.7 Hilos con alcance: `thread::scope`

Los hilos normales exigen `'static` (por eso `move` y `Arc`). Con **`thread::scope`**, los hilos **garantizan terminar antes de que salga el bloque**, así que **pueden tomar prestados datos locales**:

```rust
use std::thread;

fn main() {
    let datos = vec![1, 2, 3, 4, 5, 6];

    thread::scope(|s| {
        s.spawn(|| {
            println!("hilo A ve {} elementos", datos.len());      // préstamo & de `datos`
        });
        s.spawn(|| {
            println!("hilo B suma {}", datos.iter().sum::<i32>());   // otro préstamo &
        });
    });                                                           // aquí se hace join automático de A y B

    println!("{:?}", datos);                                      // ✅ `datos` sigue siendo nuestro
}
```

### Modificar partes distintas a la vez (aliasing XOR mutabilidad, del Nivel 3)

```rust
use std::thread;

fn main() {
    let mut datos = vec![1, 2, 3, 4, 5, 6];
    let (izq, der) = datos.split_at_mut(3);          // dos &mut DISJUNTOS

    thread::scope(|s| {
        s.spawn(move || {
            for x in izq.iter_mut() {
                *x *= 2;
            }
        });
        s.spawn(move || {
            for x in der.iter_mut() {
                *x *= 10;
            }
        });
    });

    println!("{:?}", datos);                         // [2, 4, 6, 40, 50, 60]
}
```

> Aquí **no hace falta `Arc` ni `Mutex`**: como cada hilo tiene su parte exclusiva, **no hay nada que sincronizar**. Esta es la forma más barata de paralelismo.

### Dividir un cálculo entre hilos

```rust
use std::thread;

fn main() {
    let datos: Vec<u64> = (1..=1_000_000).collect();
    let hilos: usize = 4;
    let tamano = datos.len().div_ceil(hilos);

    let total: u64 = thread::scope(|s| {
        let mut manejadores = Vec::new();
        for trozo in datos.chunks(tamano) {                       // porciones (slices) de `datos`
            manejadores.push(s.spawn(move || trozo.iter().sum::<u64>()));
        }

        let mut suma = 0;
        for m in manejadores {
            suma += m.join().expect("el hilo no debe hacer panic");
        }
        suma
    });

    println!("{total}");                                           // 500000500000
}
```

### Lo que `scope` NO permite

```rust
use std::thread;

fn main() {
    let mut x = 0;
    thread::scope(|s| {
        // ❌ E0499: dos hilos con &mut a la misma variable
        // s.spawn(|| x += 1);
        // s.spawn(|| x += 1);
        s.spawn(|| x += 1);                 // ✅ uno solo
    });
    println!("{x}");                        // 1
}
```

## 2.8 Cuándo usar cada forma

| Situación | Usa |
|---|---|
| Datos locales, el trabajo termina dentro de la función | **`thread::scope`** |
| El hilo debe seguir vivo después (servicio en segundo plano) | `thread::spawn` + `move` (+ `Arc`, canales) |
| Reparto de un cálculo grande en porciones | `scope` + `chunks` / `split_at_mut` |
| Muchísimas tareas pequeñas | No hilos: **async** (Módulos 6–11) |

## 2.9 Coste de los hilos

- Cada hilo reserva una **pila** (en Rust, 2 MiB por defecto para hilos creados con `spawn`) y consume recursos del SO.
- Crear miles de hilos es **mala idea**: el rendimiento se degrada.
- No es gratis repartir trabajo: crear hilos, sincronizar y juntar resultados cuesta. **Mide antes de asumir que paralelizar acelera.**

---

# Módulo 3: `Send` y `Sync`

Estos dos traits son **el mecanismo** con el que el compilador sabe qué es seguro usar entre hilos.

## 3.1 Definiciones

| Trait | Significa | Pregunta que responde |
|---|---|---|
| **`Send`** | Un valor de este tipo se puede **mover** a otro hilo | ¿Puedo **enviar** esto a otro hilo? |
| **`Sync`** | Se puede **compartir una referencia** `&T` entre hilos | ¿Puedo **prestar** esto a varios hilos a la vez? |

Relación: **`T` es `Sync` si y solo si `&T` es `Send`.**

Propiedades importantes:

- Son **traits marcadores**: no tienen métodos.
- Son **traits automáticos** (*auto traits*): el compilador los deriva solo. Un struct es `Send` si **todos sus campos** lo son; si **un** campo no lo es, el struct tampoco.
- Los tipos básicos (`i32`, `String`, `Vec<T>`...) son `Send` y `Sync` (si lo es su contenido).

## 3.2 Comprobarlo con el compilador

```rust
use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn exige_send<T: Send>() {}
fn exige_sync<T: Sync>() {}

fn main() {
    exige_send::<i32>();
    exige_send::<String>();
    exige_send::<Vec<i32>>();
    exige_send::<Arc<Mutex<i32>>>();
    exige_sync::<Arc<Mutex<i32>>>();
    exige_send::<Cell<i32>>();                      // ✅ Cell es Send...

    // exige_sync::<Cell<i32>>();                    // ❌ E0277: `Cell<i32>` cannot be shared between threads safely
    // exige_send::<Rc<i32>>();                      // ❌ E0277: `Rc<i32>` cannot be sent between threads safely
    // exige_sync::<Rc<i32>>();                      // ❌
    // exige_send::<*const i32>();                   // ❌ los punteros crudos no son Send
    // exige_send::<std::sync::MutexGuard<'static, i32>>();   // ❌ un guardia de Mutex no es Send
}
```

Es un buen ejercicio de clase: que prueben tipos y descubran cuáles pasan.

## 3.3 Tabla de tipos comunes

| Tipo | `Send` | `Sync` | Por qué |
|---|---|---|---|
| `i32`, `bool`, `char`, `f64` | ✅ | ✅ | Datos simples |
| `String`, `Vec<T>`, `HashMap<K, V>` | ✅ (si `T`/`K`/`V` lo son) | ✅ (idem) | Dueños de su contenido |
| `&T` | ✅ si `T: Sync` | ✅ si `T: Sync` | Una lectura compartida es segura si `T` lo permite |
| `&mut T` | ✅ si `T: Send` | ✅ si `T: Sync` | Acceso exclusivo |
| **`Rc<T>`** | ❌ | ❌ | Su contador de referencias **no es atómico** |
| **`Arc<T>`** | ✅ si `T: Send + Sync` | ✅ si `T: Send + Sync` | Contador **atómico** |
| `Cell<T>`, `RefCell<T>` | ✅ si `T: Send` | ❌ | Mutabilidad interior **sin** sincronización |
| **`Mutex<T>`** | ✅ si `T: Send` | ✅ si `T: Send` | Sincroniza el acceso |
| **`RwLock<T>`** | ✅ si `T: Send` | ✅ si `T: Send + Sync` | Idem |
| `MutexGuard<T>` | ❌ | ✅ si `T: Sync` | El guardia debe liberarse en el mismo hilo que lo tomó |
| Punteros crudos `*const T` / `*mut T` | ❌ | ❌ | Sin garantías |
| `AtomicUsize` y demás atómicos | ✅ | ✅ | Operaciones atómicas por hardware |
| `mpsc::Sender<T>`, `mpsc::Receiver<T>` | ✅ si `T: Send` | `Sender` ✅; `Receiver` ❌ | El receptor es único |

## 3.4 Por qué `Rc` no es `Send`

```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let datos = Rc::new(5);

    // ❌ E0277: `Rc<i32>` cannot be sent between threads safely
    // let h = thread::spawn(move || {
    //     println!("{datos}");
    // });
    // h.join().unwrap();

    println!("{datos}");
}
```

`Rc` actualiza su contador de referencias con **operaciones normales**. Si dos hilos hicieran `clone` a la vez, el contador se corrompería y podría liberarse memoria **dos veces o demasiado pronto**. Por eso el compilador **no permite** enviarlo. La versión segura es `Arc` (Módulo 4).

> El mensaje de error del compilador nombra exactamente el trait: *"the trait `Send` is not implemented for `Rc<i32>`"*. **Los errores de `Send`/`Sync` son pistas, no obstáculos**: te dicen qué tipo no sirve entre hilos.

## 3.5 Un struct hereda la propiedad de sus campos

```rust
use std::rc::Rc;

fn exige_send<T: Send>() {}

#[allow(dead_code)]
struct Bueno {
    nombre: String,
    edad: u32,
}

#[allow(dead_code)]
struct Malo {
    nombre: String,
    compartido: Rc<i32>,       // un solo campo !Send contagia al struct entero
}

fn main() {
    exige_send::<Bueno>();                 // ✅
    // exige_send::<Malo>();               // ❌ E0277: `Rc<i32>` cannot be sent between threads safely
}
```

## 3.6 `unsafe impl Send/Sync`: una promesa

Es posible escribir `unsafe impl Send for MiTipo {}`: le **prometes** al compilador que es seguro. Si te equivocas, **tú** eres responsable de la carrera de datos. Está fuera del alcance de este curso: la inmensa mayoría de los programas **nunca** lo necesitan.

## 3.7 Resumen del módulo

1. **`Send`** = se puede *mover* a otro hilo. **`Sync`** = se puede *compartir por referencia* entre hilos.
2. Son automáticos: se derivan de los campos.
3. `Rc`, `Cell`/`RefCell` (para `Sync`), punteros crudos y `MutexGuard` (para `Send`) **no** pasan.
4. Los errores de `Send`/`Sync` te señalan **qué tipo** rompe la regla.

---

# Módulo 4: `Arc`, `Mutex` y `RwLock`

Hay dos grandes formas de coordinar hilos:

1. **Compartir memoria** (este módulo): varios hilos acceden al mismo dato, de forma sincronizada.
2. **Pasar mensajes** (módulo 5): los hilos se envían valores.

## 4.1 `Arc<T>`: propiedad compartida entre hilos

`Arc` = **A**tomic **R**eference **C**ounted. Es como `Rc`, pero con contador **atómico**: puede cruzar hilos.

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let datos = Arc::new(vec![1, 2, 3]);
    let mut manejadores = Vec::new();

    for i in 0..3 {
        let datos = Arc::clone(&datos);               // nueva referencia (solo incrementa el contador)
        manejadores.push(thread::spawn(move || {
            let suma: i32 = datos.iter().sum();
            println!("hilo {i}: suma = {suma}");
        }));
    }

    for h in manejadores {
        h.join().expect("el hilo no debe hacer panic");
    }
    println!("referencias vivas: {}", Arc::strong_count(&datos));   // 1
}
```

- `Arc::clone(&a)` **no copia los datos**: solo suma 1 al contador. Los datos se liberan cuando el contador llega a 0.
- Se prefiere escribir `Arc::clone(&a)` en lugar de `a.clone()` para dejar claro que es barato.
- **Un `Arc<T>` da acceso de solo lectura** (`&T`). Para modificar, hace falta algo más.

### ❌ No se puede modificar el contenido de un `Arc` directamente

```rust
use std::sync::Arc;

fn main() {
    let a = Arc::new(vec![1, 2, 3]);
    // a.push(4);        // ❌ E0596: cannot borrow data in an `Arc` as mutable
    println!("{:?}", a);
}
```

Esto es **aliasing XOR mutabilidad** (Nivel 3): un `Arc` implica **muchos dueños**, así que no puede dar `&mut`. Para mutar, usamos **mutabilidad interior** con un `Mutex`.

## 4.2 `Mutex<T>`: exclusión mutua

Un **Mutex** (*mutual exclusion*) permite que **un solo hilo a la vez** acceda al dato protegido. Para usarlo, se pide el **lock** (cerrojo) y se obtiene un **guardia** (`MutexGuard`) que da acceso al dato.

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut guardia = m.lock().expect("mutex envenenado");
        *guardia += 1;                         // el guardia se comporta como un &mut i32
    }                                          // ← el guardia se libera AQUÍ (Drop) y el lock se abre

    println!("{}", *m.lock().expect("mutex envenenado"));    // 6
}
```

Puntos clave:

- **`lock()` bloquea** el hilo hasta conseguir el cerrojo.
- Devuelve un `Result` (por el *envenenamiento*, ver 4.4).
- El cerrojo se libera **automáticamente** cuando el guardia sale de ámbito (ownership + `Drop`, Nivel 3).
- Un `Mutex` **protege datos**, no código: el dato vive **dentro** del `Mutex`, y es **imposible** acceder a él sin pedir el lock. (En C/C++, nada te obliga a ello.)

## 4.3 El patrón fundamental: `Arc<Mutex<T>>`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let contador = Arc::new(Mutex::new(0));
    let mut manejadores = Vec::new();

    for _ in 0..10 {
        let contador = Arc::clone(&contador);
        manejadores.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut guardia = contador.lock().expect("mutex envenenado");
                *guardia += 1;
            }                                              // cada guardia se libera en cada vuelta
        }));
    }

    for h in manejadores {
        h.join().expect("el hilo no debe hacer panic");
    }

    println!("total = {}", *contador.lock().expect("mutex envenenado"));   // total = 10000
}
```

**Siempre 10000.** Sin el `Mutex` sería una carrera de datos; y Rust ni siquiera te deja compilarlo.

### Qué pasa si te olvidas del `Arc`

```rust
use std::sync::Mutex;

fn main() {
    let contador = Mutex::new(0);

    for _ in 0..2 {
        // ❌ E0382: value moved into closure here, in previous iteration of loop
        // std::thread::spawn(move || {
        //     *contador.lock().unwrap() += 1;
        // });
    }

    println!("{:?}", contador.lock().expect("mutex envenenado"));
}
```

Con `move`, el primer hilo **se lleva** el `Mutex`; no queda nada para el segundo. **`Arc` da un dueño por hilo.**

### Sacar el valor al final

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let total = Arc::new(Mutex::new(0));
    let t = Arc::clone(&total);
    thread::spawn(move || {
        *t.lock().expect("mutex envenenado") += 5;
    })
    .join()
    .expect("el hilo no debe hacer panic");

    // Cuando solo queda UN dueño, se puede desenvolver todo:
    let mutex = Arc::try_unwrap(total).expect("todavía hay otros dueños");
    let valor = mutex.into_inner().expect("mutex envenenado");
    println!("{valor}");                                      // 5
}
```

## 4.4 Envenenamiento (*poisoning*)

Si un hilo hace **panic mientras tiene el lock**, el dato podría haber quedado a medias. Por eso Rust marca el `Mutex` como **envenenado**, y los siguientes `lock()` devuelven `Err`:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(0));
    let m2 = Arc::clone(&m);

    let h = thread::spawn(move || {
        let _guardia = m2.lock().expect("mutex envenenado");
        panic!("fallo mientras se tenía el lock");
    });
    let _ = h.join();                                   // Err: el hilo hizo panic

    println!("envenenado: {}", m.is_poisoned());        // true

    match m.lock() {
        Ok(g) => println!("ok: {}", *g),
        Err(envenenado) => {
            let g = envenenado.into_inner();            // recuperar el guardia a pesar de todo
            println!("recuperado con valor {}", *g);    // recuperado con valor 0
        }
    }
}
```

**Qué hacer en la práctica:**

- Si no esperas que ocurra: `.expect("mutex envenenado")` (un panic aquí es razonable: el estado puede estar corrupto).
- Si el dato es simple y se puede recuperar: `into_inner()` como arriba.
- Desde Rust 1.77 existe `clear_poison()` para "limpiar" la marca.

## 4.5 Deadlocks: lo que Rust **no** previene

```rust
use std::sync::Mutex;

fn main() {
    let a = Mutex::new(1);
    let _g1 = a.lock().expect("mutex envenenado");

    // ⚠️ Pedir el MISMO lock otra vez en el mismo hilo: se queda esperando para siempre
    // (o hace panic; el comportamiento no está especificado).
    // let _g2 = a.lock().expect("mutex envenenado");
}
```

El deadlock clásico con **dos** locks:

```text
Hilo 1: toma A ... y quiere B  ──┐
                                 ├── ninguno puede avanzar: esperan eternamente
Hilo 2: toma B ... y quiere A  ──┘
```

**Reglas para evitar deadlocks:**

1. **Mantén las secciones críticas cortas**: toma el lock, haz lo mínimo, suéltalo.
2. **No hagas trabajo lento con el lock tomado** (E/S, `sleep`, cálculos largos).
3. **Orden global de locks**: si necesitas dos, tómalos **siempre en el mismo orden** (por ejemplo por id).
4. **No llames a código desconocido** (callbacks) con un lock tomado.
5. Considera **canales** (módulo 5) en vez de varios locks.
6. Para soltar antes de tiempo: `drop(guardia);` o un bloque `{ ... }` más pequeño.

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(vec![1, 2, 3]);

    // ✅ Copia lo necesario y suelta el lock antes de trabajar
    let copia = {
        let guardia = m.lock().expect("mutex envenenado");
        guardia.clone()
    };                                                       // lock liberado aquí
    println!("{}", copia.iter().sum::<i32>());               // trabajo "largo" SIN lock
}
```

### `try_lock`: intentar sin bloquear

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(0);
    let _g = m.lock().expect("mutex envenenado");

    match m.try_lock() {
        Ok(_) => println!("lo conseguí"),
        Err(_) => println!("está ocupado, hago otra cosa"),      // este camino
    }
}
```

## 4.6 `RwLock<T>`: muchos lectores o un escritor

Es el principio **aliasing XOR mutabilidad** del Nivel 3, **aplicado en ejecución**:

- `read()` → **muchos lectores** a la vez.
- `write()` → **un único escritor**, sin lectores.

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let configuracion = Arc::new(RwLock::new(String::from("v1")));

    // Varios lectores a la vez
    let mut lectores = Vec::new();
    for i in 0..3 {
        let c = Arc::clone(&configuracion);
        lectores.push(thread::spawn(move || {
            let lectura = c.read().expect("rwlock envenenado");
            println!("lector {i} ve {}", *lectura);                  // lector N ve v1
        }));
    }
    for h in lectores {
        h.join().expect("el hilo no debe hacer panic");
    }

    // Un escritor
    {
        let mut escritura = configuracion.write().expect("rwlock envenenado");
        *escritura = String::from("v2");
    }

    println!("ahora: {}", *configuracion.read().expect("rwlock envenenado"));   // ahora: v2
}
```

Cuidados:

- ⚠️ **No** pidas `write()` teniendo ya un `read()` en el mismo hilo (deadlock). Suelta el `read` primero.
- Un `RwLock` es **más costoso** que un `Mutex` por operación: **solo vale la pena** con muchas lecturas y pocas escrituras.
- Admite envenenamiento igual que `Mutex`.

### ¿`Mutex` o `RwLock`?

| Situación | Elige |
|---|---|
| Escrituras frecuentes, secciones críticas cortas | `Mutex` |
| Muchísimas lecturas simultáneas, pocas escrituras (configuración, cachés) | `RwLock` |
| Duda | `Mutex` (más simple) |

## 4.7 Atómicos: contadores sin lock (mención)

Para contadores y banderas simples existen los **tipos atómicos**: operaciones indivisibles, sin `Mutex`.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let contador = Arc::new(AtomicUsize::new(0));
    let mut manejadores = Vec::new();

    for _ in 0..8 {
        let c = Arc::clone(&contador);
        manejadores.push(thread::spawn(move || {
            for _ in 0..1000 {
                c.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    for h in manejadores {
        h.join().expect("el hilo no debe hacer panic");
    }
    println!("{}", contador.load(Ordering::SeqCst));            // 8000
}
```

`Ordering` controla qué garantías de orden de memoria se dan. Para empezar, **usa siempre `SeqCst`** (la más estricta y simple de razonar).

## 4.8 Guía para elegir cómo compartir

| Necesito... | Herramienta |
|---|---|
| Que varios hilos **lean** un dato inmutable | `Arc<T>` |
| Que varios hilos **modifiquen** un dato | `Arc<Mutex<T>>` |
| Muchas lecturas, pocas escrituras | `Arc<RwLock<T>>` |
| Un contador o bandera simple | `Atomic*` |
| Trabajar sobre datos locales por partes | `thread::scope` + `split_at_mut` / `chunks` |
| Que cada hilo trabaje sobre **su propia** información y la envíe | **Canales** (módulo 5) |

## 4.9 Resumen del módulo

1. `Arc<T>` comparte **propiedad** entre hilos (solo lectura).
2. `Mutex<T>` / `RwLock<T>` aportan **mutabilidad interior sincronizada**.
3. `Arc<Mutex<T>>` es el patrón estándar para estado compartido modificable.
4. El **guardia** libera el lock al salir de ámbito: mantén ese ámbito pequeño.
5. Rust previene carreras de datos, pero **no deadlocks**.

---

# Módulo 5: Canales para comunicar tareas (`std::sync::mpsc`)

> *"No te comuniques compartiendo memoria; comparte memoria comunicándote."* (lema popularizado por Go)

Un **canal** es una tubería: unos hilos **envían** mensajes y otros los **reciben**. El nombre `mpsc` significa **m**ultiple **p**roducer, **s**ingle **c**onsumer: **muchos productores, un solo consumidor**.

## 5.1 Primer canal

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();           // tx = transmisor (Sender), rx = receptor (Receiver)

    thread::spawn(move || {
        for i in 1..=3 {
            tx.send(i).expect("el receptor sigue vivo");
            thread::sleep(Duration::from_millis(50));
        }
    });                                       // al terminar el hilo, `tx` se destruye

    for recibido in rx {                      // el bucle termina cuando TODOS los transmisores se destruyen
        println!("recibí {recibido}");
    }
    println!("canal cerrado");
}
```

Salida **estable**:

```
recibí 1
recibí 2
recibí 3
canal cerrado
```

## 5.2 Cómo funciona

| Operación | Qué hace | Resultado |
|---|---|---|
| `tx.send(v)` | Envía `v` (**mueve** el valor) | `Result<(), SendError<T>>`: `Err` si el receptor ya no existe |
| `rx.recv()` | **Bloquea** hasta recibir | `Result<T, RecvError>`: `Err` cuando **no quedan transmisores** |
| `rx.try_recv()` | No bloquea | `Err(TryRecvError::Empty)` o `Err(TryRecvError::Disconnected)` |
| `rx.recv_timeout(d)` | Espera como máximo `d` | `Err(RecvTimeoutError::Timeout)` o `Disconnected` |
| `for x in rx` / `rx.iter()` | Recibe hasta que se cierre el canal | Cada `x: T` |
| `tx.clone()` | Nuevo transmisor (**muchos productores**) | `Sender<T>` |

```rust
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    println!("{:?}", rx.try_recv());                              // Err(Empty)
    println!("{:?}", rx.recv_timeout(Duration::from_millis(50))); // Err(Timeout)

    tx.send(7).expect("receptor vivo");
    println!("{:?}", rx.recv());                                  // Ok(7)

    drop(tx);                                                    // se cierra el lado emisor
    println!("{:?}", rx.recv());                                  // Err(RecvError)
    println!("{:?}", rx.try_recv());                              // Err(Disconnected)

    // El otro lado: enviar cuando ya no hay receptor
    let (tx2, rx2) = mpsc::channel::<i32>();
    drop(rx2);
    match tx2.send(1) {
        Ok(()) => println!("enviado"),
        Err(e) => println!("falló: {e}"),                        // falló: sending on a closed channel
    }
}
```

## 5.3 `send` **mueve** el valor (ownership)

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let texto = String::from("hola");

    tx.send(texto).expect("receptor vivo");
    // println!("{texto}");               // ❌ E0382: `texto` se movió al canal

    println!("{}", rx.recv().expect("emisor vivo"));
}
```

Es la forma más segura de pasar datos entre hilos: **después de enviar, el emisor ya no puede tocarlos**. No hay dos hilos con acceso al mismo dato: **no hay carrera posible**.

**Requisito:** el tipo enviado debe ser `Send` (Módulo 3). Enviar un `Rc` por un canal **no compila**.

## 5.4 Varios productores

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for id in 0..3 {
        let tx = tx.clone();                              // un transmisor por hilo
        thread::spawn(move || {
            tx.send((id, id * 10)).expect("receptor vivo");
        });
    }
    drop(tx);                                             // ⚠️ IMPRESCINDIBLE: suelta el transmisor original

    let mut recibidos: Vec<(i32, i32)> = rx.iter().collect();
    recibidos.sort();                                     // el orden de llegada varía: ordenamos
    println!("{:?}", recibidos);                          // [(0, 0), (1, 10), (2, 20)]
}
```

### ⚠️ El error clásico: olvidar `drop(tx)`

Si conservas el `tx` original vivo, **el canal nunca se cierra** y `for x in rx` **espera para siempre**: queda un transmisor (el tuyo) que podría enviar más.

> **Regla:** el canal se cierra cuando **se destruyen todos** los transmisores. Si repartes clones, **destruye el original**.

## 5.5 Mensajes tipados: `enum` como protocolo

Un `enum` permite definir **qué órdenes entiende** un hilo (patrón *actor*). Incluso puedes enviar **dentro del mensaje** un `Sender` para recibir la respuesta:

```rust
use std::sync::mpsc;
use std::thread;

enum Orden {
    Sumar(i32),
    Reportar(mpsc::Sender<i32>),       // el mensaje trae el canal de respuesta
    Terminar,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Orden>();

    let actor = thread::spawn(move || {
        let mut total = 0;
        for orden in rx {
            match orden {
                Orden::Sumar(n) => total += n,
                Orden::Reportar(respuesta) => {
                    respuesta.send(total).expect("quien preguntó sigue esperando");
                }
                Orden::Terminar => break,
            }
        }
        total
    });

    tx.send(Orden::Sumar(5)).expect("actor vivo");
    tx.send(Orden::Sumar(7)).expect("actor vivo");

    let (tx_resp, rx_resp) = mpsc::channel();
    tx.send(Orden::Reportar(tx_resp)).expect("actor vivo");
    println!("total parcial: {}", rx_resp.recv().expect("el actor respondió"));   // 12

    tx.send(Orden::Terminar).expect("actor vivo");
    let final_ = actor.join().expect("el actor no debe hacer panic");
    println!("total final: {final_}");                                           // 12
}
```

**Ventaja enorme:** el estado (`total`) **pertenece a un solo hilo**: no necesita `Mutex`.

## 5.6 Canales **acotados**: `sync_channel` y *backpressure*

`mpsc::channel()` es **no acotado**: si el productor es más rápido que el consumidor, la cola **crece sin límite** (y puede agotar la memoria). `mpsc::sync_channel(n)` crea un canal con **capacidad `n`**: cuando está lleno, **`send` bloquea** hasta que haya espacio.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::sync_channel::<u32>(2);          // capacidad 2

    let productor = thread::spawn(move || {
        for i in 1..=5 {
            println!("[prod] enviando {i}");
            tx.send(i).expect("receptor vivo");           // se BLOQUEA cuando la cola está llena
            println!("[prod] enviado   {i}");
        }
    });

    thread::sleep(Duration::from_millis(300));            // consumidor "lento": tarda en empezar
    for v in rx {
        println!("            [cons] recibí {v}");
        thread::sleep(Duration::from_millis(100));
    }
    productor.join().expect("el productor no debe hacer panic");
}
```

Salida **típica**: el productor envía 1 y 2, **se queda esperando** en el 3 hasta que el consumidor empieza a vaciar la cola; a partir de ahí avanzan **al ritmo del consumidor**. Esto es **backpressure** (contrapresión): el consumidor lento **frena** al productor rápido. Lo retomamos a fondo en el Módulo 11.

`sync_channel(0)` es un **encuentro** (*rendezvous*): cada `send` espera a que alguien haga `recv`.

## 5.7 Varios consumidores: un *pool* de trabajadores

`Receiver` **no** es `Clone` (es "single consumer"). Para repartir trabajo entre varios hilos, se comparte el receptor con `Arc<Mutex<_>>`:

```rust
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let (tx_trabajo, rx_trabajo) = mpsc::channel::<u32>();
    let rx_trabajo = Arc::new(Mutex::new(rx_trabajo));
    let (tx_resultado, rx_resultado) = mpsc::channel::<(u32, u32)>();

    let mut trabajadores = Vec::new();
    for _ in 0..3 {
        let rx = Arc::clone(&rx_trabajo);
        let tx_res = tx_resultado.clone();
        trabajadores.push(thread::spawn(move || loop {
            // El guardia temporal se libera al final de ESTA sentencia, antes de procesar:
            let mensaje = rx.lock().expect("mutex envenenado").recv();
            match mensaje {
                Ok(n) => tx_res.send((n, n * n)).expect("recolector vivo"),
                Err(_) => break,                          // el canal se cerró: no hay más trabajo
            }
        }));
    }
    drop(tx_resultado);                                   // soltamos nuestro transmisor de resultados

    for n in 1..=6 {
        tx_trabajo.send(n).expect("hay trabajadores vivos");
    }
    drop(tx_trabajo);                                     // señal de "no hay más trabajo"

    for t in trabajadores {
        t.join().expect("el trabajador no debe hacer panic");
    }

    let mut resultados: Vec<(u32, u32)> = rx_resultado.iter().collect();
    resultados.sort();
    for (n, cuadrado) in resultados {
        println!("{n} → {cuadrado}");
    }
}
```

Salida **estable** (porque ordenamos): `1 → 1`, `2 → 4`, `3 → 9`, `4 → 16`, `5 → 25`, `6 → 36`.

### ⚠️ Trampa famosa: `while let` y el guardia

```rust
// ❌ El guardia temporal vive durante TODO el cuerpo del `while let`:
// while let Ok(n) = rx.lock().expect("...").recv() {
//     procesar(n);          // ← los demás trabajadores esperan el lock mientras éste procesa
// }
//
// ✅ Toma el mensaje en una sentencia aparte (como arriba), de modo que el lock se libere antes de procesar.
```

## 5.8 Cierre ordenado: cerrar el canal

La forma más limpia de decirle a los trabajadores "se acabó" es **cerrar el canal** (destruir los `Sender`): los `recv()` devuelven `Err` y los hilos salen del bucle. Es más simple y robusto que mandar un mensaje especial `Terminar` a cada hilo.

## 5.9 ¿Canales o `Mutex`?

| Usa **canales** cuando... | Usa **`Arc<Mutex<T>>`** cuando... |
|---|---|
| Hay un **flujo** de trabajo (productor → consumidor) | Muchos hilos **leen y escriben** un mismo estado pequeño |
| Quieres que **un solo hilo sea dueño** del estado (actor) | El estado es un contador, una caché, un mapa compartido |
| Necesitas **backpressure** | No hay flujo, solo un dato común |
| Quieres evitar deadlocks entre locks | La sección crítica es muy corta |

## 5.10 Resumen del módulo

1. `mpsc::channel()` → `(Sender, Receiver)`: **muchos productores, un consumidor**.
2. `send` **mueve** el valor; el tipo debe ser `Send`.
3. El canal se **cierra** cuando se destruyen **todos** los `Sender`: ¡recuerda `drop` del original!
4. `sync_channel(n)` acota la cola y aplica **backpressure**.
5. Un `enum` de mensajes + un hilo "actor" = estado sin locks.

---

# Módulo 6: `async` y `.await`

## 6.1 Por qué existe `async`

Un servidor web con 10 000 conexiones simultáneas casi siempre está **esperando** (a la red, al disco, a la base de datos). Con hilos, serían 10 000 hilos dormidos y **mucha memoria**. Con `async`, son **10 000 tareas pequeñas** que un **pequeño número de hilos** va atendiendo **solo cuando tienen algo que hacer**.

> **`async` no hace las cosas más rápidas.** Permite **esperar muchas cosas a la vez** sin gastar un hilo por espera.

## 6.2 `async fn` y `.await`

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn saludar(nombre: &str) -> String {
    sleep(Duration::from_millis(100)).await;      // "espero 100 ms SIN bloquear el hilo"
    format!("hola, {nombre}")
}

#[tokio::main]
async fn main() {
    let futuro = saludar("Ana");                  // ⚠️ no ejecuta NADA todavía
    println!("el futuro está creado, pero aún no corrió");

    let mensaje = futuro.await;                   // aquí SÍ se ejecuta
    println!("{mensaje}");
}
```

*(Para ejecutar este ejemplo necesitas Tokio: ver Módulo 8.1. Antes, en el Módulo 7, verás cómo funciona todo esto por dentro sin Tokio.)*

Reglas básicas:

1. **`async fn`** no devuelve el valor directamente: devuelve un **futuro** (`impl Future<Output = T>`).
2. Un futuro es **perezoso** (*lazy*): **no hace nada hasta que se le hace `.await`** (o se le da a un runtime). Crear el futuro **solo describe** el trabajo.
3. **`.await`** significa: *"suspende esta función hasta que el futuro termine; mientras tanto, el hilo puede atender otras tareas"*.
4. `.await` solo se puede usar **dentro de un contexto `async`**.

```rust
async fn calcular() -> i32 {
    42
}

fn funcion_normal() {
    // let x = calcular().await;
    // ❌ E0728: `await` is only allowed inside `async` functions and blocks
}

// ❌ E0752: `main` function is not allowed to be `async`
// async fn main() { }
//    → main necesita un runtime: por eso se usa #[tokio::main]

fn main() {
    funcion_normal();
}
```

## 6.3 Olvidar `.await`: el error más común

```rust
use tokio::time::{sleep, Duration};

async fn tarea() {
    sleep(Duration::from_millis(10)).await;
    println!("tarea ejecutada");
}

#[tokio::main]
async fn main() {
    tarea();             // ⚠️ warning: unused implementer of `Future` that must be used → NO se ejecuta
    tarea().await;       // ✅ ahora sí
}
```

## 6.4 Bloques `async`

También puedes crear un futuro **anónimo** con un bloque:

```rust
#[tokio::main]
async fn main() {
    let nombre = String::from("Ana");

    let futuro = async {
        1 + 1
    };

    let futuro_con_datos = async move {            // `move` se lleva `nombre` al futuro
        format!("hola, {nombre}")
    };

    println!("{}", futuro.await);                  // 2
    println!("{}", futuro_con_datos.await);        // hola, Ana
}
```

Igual que con los closures, **`async move`** es lo que usarás para **tareas** que viven más que la función que las crea.

## 6.5 Secuencial vs. concurrente

`await` una cosa **tras otra** es **secuencial**: las esperas se **suman**. Para esperar **varias a la vez**, usa `join!`:

```rust
use std::time::Instant;
use tokio::time::{sleep, Duration};

async fn tarea(nombre: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{nombre} listo")
}

#[tokio::main]
async fn main() {
    // SECUENCIAL: 300 + 200 = ~500 ms
    let inicio = Instant::now();
    let a = tarea("A", 300).await;
    let b = tarea("B", 200).await;
    println!("{a}, {b} en ~{} ms", inicio.elapsed().as_millis());       // ~500 ms

    // CONCURRENTE: el máximo de los dos = ~300 ms
    let inicio = Instant::now();
    let (a, b) = tokio::join!(tarea("A", 300), tarea("B", 200));
    println!("{a}, {b} en ~{} ms", inicio.elapsed().as_millis());       // ~300 ms
}
```

> **Lección clave:** `.await` **no** significa "en paralelo". Dos `.await` seguidos son **secuenciales**. La concurrencia aparece al **combinar futuros** (`join!`, `select!`) o al **crear tareas** (`spawn`).

## 6.6 Lo que `async` NO es

| Idea errónea | Realidad |
|---|---|
| "`async` usa más hilos" | Todo lo contrario: usa **menos** |
| "`async` acelera el cálculo" | No: para CPU pesada, usa hilos |
| "`await` ejecuta en paralelo" | `await` es secuencial; la concurrencia viene de `join!`/`spawn` |
| "Crear el futuro lo ejecuta" | Es **perezoso**: nada ocurre hasta el `.await` / `spawn` |
| "Puedo usar cualquier función bloqueante dentro" | ⚠️ **No**: bloquea el hilo del runtime y frena a **todas** las tareas |

## 6.7 El pecado capital: bloquear dentro de `async`

```rust
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let t1 = tokio::spawn(async {
        // ❌ BLOQUEA el único hilo del runtime: ninguna otra tarea puede avanzar mientras tanto
        std::thread::sleep(Duration::from_millis(500));
        println!("t1 terminó");
    });

    let t2 = tokio::spawn(async {
        // ✅ Cede el hilo mientras espera
        sleep(Duration::from_millis(100)).await;
        println!("t2 terminó");
    });

    t1.await.expect("t1 no debe hacer panic");
    t2.await.expect("t2 no debe hacer panic");
}
```

Con el `std::thread::sleep`, `t2` **no puede avanzar** hasta que `t1` termine, aunque solo necesitara 100 ms. Cambia el `std::thread::sleep` por `tokio::time::sleep(...).await` y `t2` termina primero.

| En código `async`, en vez de... | Usa... |
|---|---|
| `std::thread::sleep` | `tokio::time::sleep(..).await` |
| `std::fs::read_to_string` (sync) | `tokio::fs::read_to_string(..).await` |
| `std::net::TcpStream` | `tokio::net::TcpStream` |
| Cálculo pesado (CPU) | `tokio::task::spawn_blocking(|| ...)` |
| `std::sync::mpsc::Receiver::recv` | `tokio::sync::mpsc::Receiver::recv().await` |

## 6.8 `async` y los tipos `Send`

Cuando un futuro se envía a otro hilo (con `tokio::spawn`), **el futuro entero debe ser `Send`**. Eso incluye **todo lo que viva a través de un `.await`**:

```rust
use std::sync::Mutex;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let m = std::sync::Arc::new(Mutex::new(0));

    // ❌ error: future cannot be sent between threads safely
    //    (`MutexGuard<'_, i32>` no es Send y vive a través del .await)
    // tokio::spawn(async move {
    //     let mut g = m.lock().unwrap();
    //     *g += 1;
    //     sleep(Duration::from_millis(10)).await;      // ← el guardia sigue vivo aquí
    // });

    // ✅ Suelta el guardia ANTES del await
    let m2 = std::sync::Arc::clone(&m);
    tokio::spawn(async move {
        {
            let mut g = m2.lock().expect("mutex envenenado");
            *g += 1;
        }                                                  // guardia liberado
        sleep(Duration::from_millis(10)).await;
    })
    .await
    .expect("la tarea no debe hacer panic");

    println!("{}", *m.lock().expect("mutex envenenado"));  // 1
}
```

Los errores de "*future cannot be sent between threads safely*" casi siempre se resuelven **acortando la vida** de algo (`Rc`, un `MutexGuard`...) para que **no cruce un `.await`**. (Tokio ofrece además un `Mutex` async: Módulo 9.)

## 6.9 Resumen del módulo

1. `async fn` devuelve un **futuro perezoso**; `.await` lo ejecuta y cede el hilo mientras espera.
2. `.await` solo funciona en contextos `async`.
3. `.await` consecutivos son **secuenciales**; `join!`/`spawn` dan **concurrencia**.
4. **Nunca bloquees** dentro de `async`.
5. Lo que vive a través de un `.await` debe ser `Send` si la tarea se envía a otro hilo.

---

# Módulo 7: Futures y runtime asíncrono

Este módulo **abre la caja negra**. No lo necesitas para el día a día, pero **entenderlo** elimina el 80 % de la confusión sobre `async`.

## 7.1 El trait `Future`

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),       // terminé, aquí está el resultado
    Pending,        // aún no puedo avanzar; te aviso cuando pueda
}
```

Un **futuro** es un objeto con una única operación, **`poll`**: *"intenta avanzar; dime si ya terminaste o si todavía no"*.

- `Poll::Ready(valor)` → el futuro terminó.
- `Poll::Pending` → todavía no; **antes de devolverlo**, el futuro debe haber dejado un **aviso** (el `Waker`) para que el runtime sepa **cuándo volver a llamar a `poll`**.
- `Pin` garantiza que el futuro **no se mueve** de lugar en memoria mientras se evalúa (necesario porque `async` puede crear referencias internas). Para este curso: *"`Pin` = no me muevas"*.

## 7.2 Un `async fn` es una **máquina de estados**

El compilador transforma cada `async fn` en una **máquina de estados**: cada `.await` es un punto donde puede **pausarse y reanudarse**.

```rust
async fn ejemplo() -> u32 {
    let a = paso_uno().await;      // estado 0 → espera paso_uno
    let b = paso_dos(a).await;     // estado 1 → espera paso_dos
    a + b                          // estado final
}
```

Por eso las tareas async son **tan baratas**: no hay una pila por tarea, solo **los datos que sobreviven entre `.await`** (el estado de la máquina). Y por eso **no hay "magia"**: es un objeto normal con un `poll`.

## 7.3 Escribir un futuro a mano

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CuentaRegresiva {
    restante: u32,
}

impl Future for CuentaRegresiva {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.restante == 0 {
            Poll::Ready("¡despegue!")
        } else {
            println!("  poll: faltan {}", self.restante);
            self.restante -= 1;
            cx.waker().wake_by_ref();       // "vuelve a llamarme pronto"
            Poll::Pending
        }
    }
}
```

## 7.4 Un runtime mínimo: `block_on` en 20 líneas

Un **runtime** (o *executor*) es, en esencia, **un bucle que llama a `poll`**. Aquí tienes uno funcional **solo con la librería estándar**:

```rust
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

// --- El futuro de la sección anterior ---
struct CuentaRegresiva {
    restante: u32,
}

impl Future for CuentaRegresiva {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.restante == 0 {
            Poll::Ready("¡despegue!")
        } else {
            println!("  poll: faltan {}", self.restante);
            self.restante -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// --- Un "despertador": cuando alguien lo activa, despierta al hilo que espera ---
struct HiloWaker(Thread);

impl Wake for HiloWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

// --- El runtime más pequeño del mundo ---
fn block_on<F: Future>(futuro: F) -> F::Output {
    let mut futuro = std::pin::pin!(futuro);                       // fijar el futuro en la pila
    let waker = Waker::from(Arc::new(HiloWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);

    loop {
        match futuro.as_mut().poll(&mut cx) {
            Poll::Ready(valor) => return valor,                    // terminó
            Poll::Pending => thread::park(),                       // dormir hasta que alguien llame a wake()
        }
    }
}

fn main() {
    let resultado = block_on(async {
        println!("empieza");
        let r = CuentaRegresiva { restante: 3 }.await;
        println!("termina");
        r
    });
    println!("{resultado}");
}
```

Salida **estable**:

```
empieza
  poll: faltan 3
  poll: faltan 2
  poll: faltan 1
termina
¡despegue!
```

**Qué demuestra:**

- Un bloque `async` es **un futuro**: lo único que hizo `block_on` fue llamarlo a `poll` **repetidamente**.
- `CuentaRegresiva` dijo `Pending` tres veces; cada vez **avisó con el `Waker`** (`wake_by_ref`) y el bucle volvió a llamar a `poll`.
- Un futuro real (por ejemplo, esperar un paquete de red) **no** llama a `wake` de inmediato: lo hace **más tarde**, cuando llega el dato. Entre medias, **el hilo queda libre** para hacer poll a *otros* futuros. **Eso es la concurrencia asíncrona.**

## 7.5 Lo que hace un runtime real

Tokio y los demás runtimes son este mismo bucle, **mucho más sofisticado**:

```
        ┌──────────────────────────────────────────────────────────────┐
        │                         RUNTIME                              │
        │                                                              │
        │   ┌────────────┐   poll()   ┌─────────────────────────┐      │
        │   │ Cola de    │──────────▶ │   Tarea (futuro)        │      │
        │   │ tareas     │            └─────────┬───────────────┘      │
        │   │ listas     │       Ready ◀────────┤ Pending              │
        │   └─────▲──────┘     (se descarta)    │ (registra un Waker)  │
        │         │ wake()                      ▼                      │
        │   ┌─────┴───────────────────────────────────────────┐       │
        │   │ Reactor: espera eventos del SO (red, disco),    │       │
        │   │ temporizadores...  y llama a wake() cuando       │       │
        │   │ algo está listo                                  │       │
        │   └──────────────────────────────────────────────────┘       │
        └──────────────────────────────────────────────────────────────┘
```

Componentes:

| Componente | Función |
|---|---|
| **Ejecutor (*executor*)** | Cola de tareas listas; llama a `poll` en un **conjunto de hilos** (*work-stealing* en Tokio multi-hilo) |
| **Reactor** | Se registra en el sistema operativo (epoll, kqueue, IOCP) y **despierta tareas** cuando hay eventos de E/S |
| **Temporizadores** | Despiertan tareas cuando vence un plazo (`sleep`, `timeout`) |
| **Reserva de hilos bloqueantes** | Para `spawn_blocking` y para E/S que el SO no ofrece como asíncrona |

## 7.6 Rust **no trae** runtime

La librería estándar define `Future`, `async`/`await`, `Waker`... pero **no incluye un runtime**. Se elige uno como dependencia:

| Runtime | Comentario |
|---|---|
| **Tokio** | El estándar de facto: más usado, mejor documentado, más bibliotecas compatibles. **El que usaremos** |
| `smol` | Más pequeño y sencillo |
| Otros (embebidos, etc.) | Para casos especiales |

Ventaja: el lenguaje no te impone uno. Inconveniente: las bibliotecas async deben **ser compatibles** con tu runtime (en la práctica, casi todas lo son con Tokio).

## 7.7 Reglas que se derivan de cómo funciona

1. **Cada `.await` es un posible punto de suspensión.** Entre dos `.await`, tu código se ejecuta **sin interrupciones**: si tarda mucho, **bloquea** a las demás tareas del hilo.
2. **Un futuro que nunca llega a un `.await` que ceda** no deja avanzar a nadie.
3. **Cancelar un futuro = dejar de llamarle a `poll`** (descartarlo). Solo puede ocurrir **en un `.await`** (Módulo 10).
4. Lo que vive **a través** de un `.await` forma parte del **estado** del futuro: debe ser `Send` si la tarea se mueve entre hilos, y ocupa memoria.

## 7.8 Resumen del módulo

1. `Future` = *"algo que se puede avanzar con `poll`"* → `Ready` o `Pending`.
2. `async fn` = máquina de estados generada por el compilador.
3. Un **runtime** = ejecutor (bucle de `poll`) + reactor (eventos del SO) + temporizadores.
4. El `Waker` es el mecanismo de **"avísame cuando puedas avanzar"**.
5. Rust no trae runtime: se elige uno (**Tokio**).

---

# Módulo 8: Tokio: tareas

## 8.1 Instalación

```bash
cargo new async_demo
cd async_demo
cargo add tokio --features full
```

o en `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

`full` activa todo (runtime, temporizadores, canales, red, archivos, señales...). En proyectos reales conviene activar **solo** lo necesario (`rt-multi-thread`, `macros`, `time`, `sync`, `net`...), para compilar más rápido.

## 8.2 `#[tokio::main]`

```rust
#[tokio::main]
async fn main() {
    println!("dentro del runtime de Tokio");
}
```

La macro **transforma** `main` en una función normal que construye un runtime y le pasa tu `async fn` con `block_on`. Es equivalente a:

```rust
fn main() {
    let runtime = tokio::runtime::Runtime::new().expect("no se pudo crear el runtime");
    runtime.block_on(async {
        println!("dentro del runtime de Tokio");
    });
}
```

### Tipos de runtime

| Configuración | Significado |
|---|---|
| `#[tokio::main]` | **Multi-hilo** (por defecto): un hilo por núcleo, con *work-stealing* |
| `#[tokio::main(flavor = "current_thread")]` | **Un solo hilo**: todo corre en el hilo actual (más simple, ideal para ejemplos deterministas y para programas pequeños) |
| `#[tokio::main(worker_threads = 4)]` | Multi-hilo con 4 hilos |

```rust
use tokio::runtime::Builder;

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()                       // habilita temporizadores y E/S
        .build()
        .expect("no se pudo crear el runtime");

    runtime.block_on(async {
        println!("runtime personalizado");
    });
}
```

⚠️ **Errores típicos de runtime** (panics):

- *"there is no reactor running, must be called from the context of a Tokio 1.x runtime"* → usaste `tokio::spawn`/`sleep` **fuera** de un runtime.
- *"Cannot start a runtime from within a runtime"* → llamaste `block_on` **dentro** de código `async`.

## 8.3 `tokio::spawn`: crear tareas

Una **tarea** es un futuro que el runtime **ejecuta por su cuenta**, **concurrentemente** con las demás. Es el equivalente async de `thread::spawn`.

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let manejador = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        42                                           // valor de retorno de la tarea
    });

    println!("mientras tanto, main sigue trabajando...");

    let resultado = manejador.await.expect("la tarea no debe hacer panic");
    println!("la tarea devolvió {resultado}");       // 42
}
```

Detalles clave:

- `tokio::spawn` devuelve un **`JoinHandle<T>`**.
- **`handle.await`** devuelve `Result<T, JoinError>`: `Err` si la tarea hizo **panic** o fue **cancelada**.
- La tarea **empieza a correr de inmediato** (no hay que esperar al `.await` del handle).
- **Soltar** el `JoinHandle` **no cancela** la tarea: la **desvincula** (*detach*), sigue corriendo en segundo plano.
- Al terminar `main`, el runtime se cierra y **las tareas pendientes se cancelan**.

## 8.4 Los requisitos de `spawn`: `Send + 'static`

Igual que `thread::spawn`, la tarea puede ejecutarse **en otro hilo** y **vivir más** que quien la creó:

```rust
use std::rc::Rc;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    // ❌ E0373: async block may outlive the current function, but it borrows `v`
    // tokio::spawn(async {
    //     println!("{:?}", v);
    // });

    // ✅ `async move`: la tarea se lleva `v`
    tokio::spawn(async move {
        println!("{:?}", v);
    })
    .await
    .expect("la tarea no debe hacer panic");

    // ❌ error: future cannot be sent between threads safely (Rc no es Send)
    // let r = Rc::new(5);
    // tokio::spawn(async move {
    //     println!("{r}");
    // });
    let _ = Rc::new(5);
}
```

Los mismos conceptos del Módulo 3 (`Send`) y del Nivel 3 (`'static` / propiedad). Para compartir datos entre tareas: **`Arc`**, o mejor, **canales**.

## 8.5 Muchas tareas, muy poco costo

```rust
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let mut tareas = JoinSet::new();

    for i in 0..10_000u32 {
        tareas.spawn(async move {
            sleep(Duration::from_millis(100)).await;
            i
        });
    }

    let mut suma: u64 = 0;
    while let Some(resultado) = tareas.join_next().await {
        suma += resultado.expect("la tarea no debe hacer panic") as u64;
    }
    println!("{suma}");                              // 49995000
}
```

**Diez mil tareas** esperando 100 ms **a la vez** terminan en ~100 ms y casi sin memoria. Con 10 000 hilos del SO, esto sería muy pesado.

## 8.6 `JoinSet`: un grupo de tareas

`JoinSet` **agrupa** tareas y permite:

- `spawn(futuro)`: añadir una tarea.
- `join_next().await` → `Option<Result<T, JoinError>>`: espera **la siguiente que termine** (en **orden de finalización**, no de creación). `None` cuando no quedan.
- `abort_all()`: cancelar todas.
- **Si el `JoinSet` se destruye, todas sus tareas se cancelan automáticamente.** Esto lo hace ideal para **concurrencia estructurada**: *"las tareas hijas no sobreviven a su padre"*.

```rust
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let mut tareas = JoinSet::new();
    tareas.spawn(async { sleep(Duration::from_millis(300)).await; "lenta" });
    tareas.spawn(async { sleep(Duration::from_millis(100)).await; "rápida" });
    tareas.spawn(async { sleep(Duration::from_millis(200)).await; "media" });

    while let Some(resultado) = tareas.join_next().await {
        println!("terminó: {}", resultado.expect("la tarea no debe hacer panic"));
    }
}
```

Salida **estable** (por los tiempos): `terminó: rápida`, `terminó: media`, `terminó: lenta`.

## 8.7 Errores y `panic` dentro de las tareas

El `panic` de una tarea **no tumba** al runtime ni a las demás: se convierte en `Err(JoinError)`:

```rust
#[tokio::main]
async fn main() {
    let h = tokio::spawn(async {
        panic!("¡algo salió mal en la tarea!");
    });

    match h.await {
        Ok(()) => println!("la tarea terminó bien"),
        Err(e) if e.is_panic() => println!("la tarea hizo panic"),
        Err(e) if e.is_cancelled() => println!("la tarea fue cancelada"),
        Err(e) => println!("otro error: {e}"),
    }
}
```

Y si la tarea devuelve un `Result`, tienes **dos niveles**: `handle.await` (¿terminó la tarea?) y el `Result` interno (¿tuvo éxito la operación?):

```rust
#[tokio::main]
async fn main() {
    let h = tokio::spawn(async {
        let n: Result<i32, String> = Err(String::from("fallo de negocio"));
        n
    });

    match h.await {
        Ok(Ok(valor)) => println!("éxito: {valor}"),
        Ok(Err(e)) => println!("la tarea terminó, pero la operación falló: {e}"),
        Err(e) => println!("la tarea no pudo completarse: {e}"),
    }
}
```

## 8.8 Combinar futuros sin crear tareas: `join!` y `try_join!`

```rust
use tokio::time::{sleep, Duration};

async fn paso(nombre: &str, ms: u64, falla: bool) -> Result<String, String> {
    sleep(Duration::from_millis(ms)).await;
    if falla {
        Err(format!("{nombre} falló"))
    } else {
        Ok(format!("{nombre} ok"))
    }
}

#[tokio::main]
async fn main() {
    // join!: espera a TODOS (aunque alguno falle)
    let (a, b) = tokio::join!(paso("A", 100, false), paso("B", 50, true));
    println!("{:?} / {:?}", a, b);       // Ok("A ok") / Err("B falló")

    // try_join!: si UNO devuelve Err, termina de inmediato y CANCELA a los demás
    match tokio::try_join!(paso("C", 300, false), paso("D", 50, true)) {
        Ok((c, d)) => println!("{c}, {d}"),
        Err(e) => println!("falló la combinación: {e}"),         // falló la combinación: D falló
    }
}
```

| | `join!` / `try_join!` | `spawn` |
|---|---|---|
| Corren en... | **La misma tarea** (concurrentes, no paralelas) | **Tareas distintas** (pueden ir en varios hilos) |
| Requieren `'static` / `Send` | ❌ No | ✅ Sí |
| Pueden tomar prestados datos locales | ✅ Sí | ❌ No (usa `move`/`Arc`) |
| Cuándo | Pocas operaciones conocidas, que se esperan juntas | Trabajo independiente, en segundo plano, muchas |

## 8.9 Trabajo bloqueante o pesado: `spawn_blocking`

Si debes ejecutar código **que bloquea** (cálculo pesado, una biblioteca sin `async`), **no lo hagas en una tarea normal**. Envíalo a la reserva de hilos bloqueantes:

```rust
fn calculo_pesado(n: u64) -> u64 {
    let mut suma = 0;
    for i in 0..n {
        suma += i % 7;
    }
    suma
}

#[tokio::main]
async fn main() {
    let h = tokio::task::spawn_blocking(|| calculo_pesado(50_000_000));
    println!("el runtime sigue libre mientras se calcula...");
    let resultado = h.await.expect("el cálculo no debe hacer panic");
    println!("{resultado}");
}
```

⚠️ Una tarea de `spawn_blocking` **no se puede cancelar** una vez que empezó.

## 8.10 Ceder el paso voluntariamente

Si una tarea hace un bucle largo **sin ningún `.await`**, monopoliza el hilo. `tokio::task::yield_now().await` **cede** el turno a las demás:

```rust
async fn bucle_largo() {
    for i in 0..1_000_000u32 {
        // ... trabajo ...
        if i % 10_000 == 0 {
            tokio::task::yield_now().await;      // "dejo pasar a otras tareas"
        }
    }
}
```

## 8.11 Resumen del módulo

1. `#[tokio::main]` crea el runtime y ejecuta tu `main` async.
2. `tokio::spawn(async move { ... })` crea una **tarea**; devuelve `JoinHandle`; exige `Send + 'static`.
3. `handle.await` → `Result<T, JoinError>`; el `panic` de una tarea no afecta a las demás.
4. `JoinSet` agrupa tareas y las cancela al destruirse.
5. `join!`/`try_join!` combinan futuros **sin crear tareas**.
6. Lo bloqueante va a `spawn_blocking`.

---

# Módulo 9: Tokio: canales y sincronización

Tokio ofrece versiones **async** de las herramientas del módulo 4 y 5: **esperan con `.await`** en lugar de **bloquear el hilo**.

## 9.1 `mpsc`: varios productores, un consumidor

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(8);        // capacidad 8 (ACOTADO)

    let productor = tokio::spawn(async move {
        for i in 1..=3 {
            tx.send(format!("mensaje {i}")).await.expect("el receptor sigue vivo");
        }
    });                                                   // `tx` se destruye al terminar la tarea

    while let Some(mensaje) = rx.recv().await {           // None cuando se cierran todos los Sender
        println!("{mensaje}");
    }
    productor.await.expect("el productor no debe hacer panic");
}
```

Salida **estable**: `mensaje 1`, `mensaje 2`, `mensaje 3`.

**Diferencias respecto a `std::sync::mpsc`:**

| | `std::sync::mpsc` | `tokio::sync::mpsc` |
|---|---|---|
| Enviar | `tx.send(v)` (bloquea solo en `sync_channel`) | `tx.send(v).await` (espera sin bloquear si está lleno) |
| Recibir | `rx.recv()` → `Result<T, RecvError>` (bloquea) | `rx.recv().await` → **`Option<T>`** (`None` = cerrado) |
| `Receiver` mutable | No | `let (tx, mut rx)` (`recv(&mut self)`) |
| Acotado | `sync_channel(n)` | **`channel(n)`** (el normal; `n` debe ser > 0) |
| No acotado | `channel()` | `unbounded_channel()` (¡con cuidado!) |

### Varios productores

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<(u32, u32)>(16);

    for id in 0..3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send((id, id * 10)).await.expect("receptor vivo");
        });
    }
    drop(tx);                                             // soltar el transmisor original

    let mut recibidos = Vec::new();
    while let Some(m) = rx.recv().await {
        recibidos.push(m);
    }
    recibidos.sort();
    println!("{:?}", recibidos);                          // [(0, 0), (1, 10), (2, 20)]
}
```

Igual que en el módulo 5: **si no sueltas el `tx` original, `recv` nunca devuelve `None`**.

### `try_send`: no esperar

```rust
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TrySendError;

#[tokio::main]
async fn main() {
    let (tx, _rx) = mpsc::channel::<u32>(2);

    for i in 1..=4 {
        match tx.try_send(i) {
            Ok(()) => println!("encolado {i}"),
            Err(TrySendError::Full(v)) => println!("cola llena: descarto {v}"),
            Err(TrySendError::Closed(v)) => println!("canal cerrado: {v}"),
        }
    }
}
```

Salida **estable**: `encolado 1`, `encolado 2`, `cola llena: descarto 3`, `cola llena: descarto 4`.

## 9.2 `oneshot`: una única respuesta

Un canal de **un solo mensaje**: perfecto para **"pregunta → respuesta"**.

```rust
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<String>();

    tokio::spawn(async move {
        let _ = tx.send(String::from("respuesta calculada"));     // send NO es async; devuelve Result
    });

    match rx.await {                                              // el receptor ES un futuro
        Ok(valor) => println!("{valor}"),
        Err(_) => println!("quien debía responder desapareció"),
    }
}
```

### Patrón *actor*: `mpsc` + `oneshot`

Una tarea **es dueña de un estado**; los demás le **envían comandos**. **Sin locks.**

```rust
use tokio::sync::{mpsc, oneshot};

enum Comando {
    Incrementar,
    Obtener(oneshot::Sender<u64>),          // lleva el canal de respuesta
}

async fn actor(mut rx: mpsc::Receiver<Comando>) {
    let mut valor: u64 = 0;
    while let Some(comando) = rx.recv().await {
        match comando {
            Comando::Incrementar => valor += 1,
            Comando::Obtener(respuesta) => {
                let _ = respuesta.send(valor);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<Comando>(16);
    let tarea_actor = tokio::spawn(actor(rx));

    for _ in 0..3 {
        tx.send(Comando::Incrementar).await.expect("el actor sigue vivo");
    }

    let (tx_resp, rx_resp) = oneshot::channel();
    tx.send(Comando::Obtener(tx_resp)).await.expect("el actor sigue vivo");
    println!("valor = {}", rx_resp.await.expect("el actor respondió"));    // valor = 3

    drop(tx);                                                             // cierra el canal: el actor termina
    tarea_actor.await.expect("el actor no debe hacer panic");
}
```

## 9.3 `broadcast`: un mensaje para **todos**

Cada receptor recibe **su propia copia** (el tipo debe ser `Clone`).

```rust
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<String>(16);
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();

    let t1 = tokio::spawn(async move {
        while let Ok(m) = rx1.recv().await {
            println!("[suscriptor 1] {m}");
        }
    });
    let t2 = tokio::spawn(async move {
        while let Ok(m) = rx2.recv().await {
            println!("[suscriptor 2] {m}");
        }
    });

    tx.send(String::from("hola a todos")).expect("hay suscriptores");
    tx.send(String::from("segundo aviso")).expect("hay suscriptores");
    drop(tx);                                              // cierra: los `recv` devuelven Err(Closed)

    t1.await.expect("no debe hacer panic");
    t2.await.expect("no debe hacer panic");
}
```

Cada suscriptor imprime los dos mensajes (el orden **entre** suscriptores puede variar). Si un receptor va **demasiado lento** y la cola de 16 se desborda, `recv` devuelve `Err(RecvError::Lagged(n))`: **perdió `n` mensajes**.

## 9.4 `watch`: "el último valor"

Un canal que **guarda solo el valor más reciente**; los receptores esperan **cambios**. Ideal para **configuración** y **señales de apagado**.

```rust
use tokio::sync::watch;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel(false);              // false = "seguir trabajando"

    let trabajador = tokio::spawn(async move {
        let mut vueltas = 0;
        loop {
            tokio::select! {
                _ = rx.changed() => {
                    if *rx.borrow() {
                        println!("apagando...");
                        break;
                    }
                }
                _ = sleep(Duration::from_millis(100)) => {
                    vueltas += 1;
                }
            }
        }
        vueltas
    });

    sleep(Duration::from_millis(350)).await;
    tx.send(true).expect("el trabajador sigue vivo");      // señal de apagado

    let vueltas = trabajador.await.expect("no debe hacer panic");
    println!("vueltas completadas: {vueltas}");            // vueltas completadas: 3
}
```

## 9.5 Resumen de canales de Tokio

| Canal | Productores → Consumidores | Cuándo |
|---|---|---|
| `mpsc` | muchos → **uno** | Cola de trabajo, actores |
| `oneshot` | uno → uno (un único mensaje) | Respuestas, "avísame cuando termines" |
| `broadcast` | muchos → **muchos** (todos reciben todo) | Eventos, notificaciones |
| `watch` | uno → muchos (solo el **último** valor) | Configuración, señales de apagado |

## 9.6 `tokio::sync::Mutex` y `RwLock`

Son versiones **async**: `lock().await` **espera sin bloquear el hilo**, y el guardia **puede mantenerse a través de un `.await`**.

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let datos = Arc::new(Mutex::new(Vec::<u32>::new()));
    let mut tareas = JoinSet::new();

    for i in 0..5 {
        let datos = Arc::clone(&datos);
        tareas.spawn(async move {
            let mut guardia = datos.lock().await;          // .await en vez de .expect(...): sin envenenamiento
            guardia.push(i);
        });
    }

    while let Some(r) = tareas.join_next().await {
        r.expect("la tarea no debe hacer panic");
    }

    let mut v = datos.lock().await.clone();
    v.sort();
    println!("{:?}", v);                                   // [0, 1, 2, 3, 4]
}
```

### ¿`std::sync::Mutex` o `tokio::sync::Mutex`?

| Situación | Elige |
|---|---|
| Sección crítica **corta, sin `.await`** dentro | **`std::sync::Mutex`** (más rápido; la recomendación habitual) |
| Necesitas **mantener el lock a través de un `.await`** | **`tokio::sync::Mutex`** |
| Estado complejo con mucha E/S | Mejor un **actor** (canal + una tarea dueña del estado) |

⚠️ El `tokio::sync::Mutex` es **más lento** y **no** evita los deadlocks. Mantener un lock durante un `.await` largo hace que **todas las demás tareas** que lo necesiten se queden esperando.

## 9.7 `Notify`: avisar sin enviar datos

```rust
use std::sync::Arc;
use tokio::sync::Notify;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let aviso = Arc::new(Notify::new());
    let aviso2 = Arc::clone(&aviso);

    let tarea = tokio::spawn(async move {
        aviso2.notified().await;                           // espera la señal
        println!("¡desperté!");
    });

    sleep(Duration::from_millis(100)).await;
    aviso.notify_one();                                    // despierta a una tarea en espera
    tarea.await.expect("la tarea no debe hacer panic");
}
```

## 9.8 Resumen del módulo

1. Los canales de Tokio **esperan con `.await`**; el `mpsc` es **acotado** (`channel(n)`).
2. `recv().await` devuelve **`Option`**: `None` = canal cerrado.
3. **Actor** = una tarea dueña del estado + `mpsc` para comandos + `oneshot` para respuestas.
4. `broadcast` = eventos para todos; `watch` = último valor / señales.
5. `std::sync::Mutex` si la sección es corta y sin `.await`; `tokio::sync::Mutex` si el lock cruza un `.await`.

---

# Módulo 10: Tokio: cancelación

## 10.1 Cancelar en `async` = **dejar de ejecutar** el futuro

Como los futuros son **perezosos** y avanzan **solo cuando se les hace `poll`**, **cancelarlos es trivial**: basta con **no llamarlos más** (es decir, **destruirlos**, hacer `drop`).

> **En Rust async, cancelar = soltar (`drop`) el futuro.** Y la cancelación solo puede ocurrir **en un `.await`** (el único lugar donde el futuro devuelve el control).

Consecuencias:

- Cuando un futuro se cancela, **todas sus variables locales se destruyen** (se ejecutan los `Drop`): **la limpieza es automática** (ownership, Nivel 3).
- **El código posterior al `.await` en que se canceló no se ejecuta nunca.**

## 10.2 Formas de cancelar

| Mecanismo | Qué cancela |
|---|---|
| `select!` | Los futuros que **pierden** la carrera |
| `timeout(...)` | El futuro interior, si vence el plazo |
| `handle.abort()` | Una **tarea** (`spawn`) |
| `JoinSet` destruido / `abort_all()` | Un **grupo de tareas** |
| `try_join!` con un error | Los demás futuros |
| `drop(futuro)` | Cualquier futuro |
| Señal cooperativa (`watch`, `CancellationToken`) | Pide a la tarea que **termine por sí misma** |

## 10.3 `select!`: la primera que termine gana

```rust
use tokio::time::{sleep, Duration};

async fn trabajo_lento() -> &'static str {
    sleep(Duration::from_secs(10)).await;
    "terminé"
}

#[tokio::main]
async fn main() {
    tokio::select! {
        resultado = trabajo_lento() => {
            println!("el trabajo terminó: {resultado}");
        }
        _ = sleep(Duration::from_millis(200)) => {
            println!("se acabó la paciencia: el trabajo se cancela");
        }
    }
    println!("fin");
}
```

Salida **estable** (tras ~200 ms): `se acabó la paciencia: el trabajo se cancela` y luego `fin`. **`trabajo_lento` fue descartado** y no llegó a terminar.

**Sintaxis:** `patrón = futuro => { manejador }`, una rama por cada futuro. Detalles:

- Todas las ramas se esperan **a la vez**; **la primera que se complete** ejecuta su manejador.
- Las demás se **cancelan** (se destruyen).
- `biased;` al inicio hace que se revisen **en orden** (por defecto el orden es aleatorio, para ser justo).
- Puedes usar `if` como precondición de una rama.

### `select!` en un bucle: el patrón del servidor

```rust
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<u32>(8);
    let mut latido = interval(Duration::from_millis(100));

    tokio::spawn(async move {
        for i in 1..=3 {
            tx.send(i).await.expect("receptor vivo");
            tokio::time::sleep(Duration::from_millis(150)).await;
        }
    });

    loop {
        tokio::select! {
            mensaje = rx.recv() => {
                match mensaje {
                    Some(n) => println!("mensaje {n}"),
                    None => {
                        println!("canal cerrado: salgo");
                        break;
                    }
                }
            }
            _ = latido.tick() => {
                println!("  (latido)");
            }
        }
    }
}
```

Mezcla "esperar mensajes" con "hacer algo periódicamente": el patrón típico de un servicio.

## 10.4 Cancelar una tarea con `abort()`

```rust
use tokio::time::{sleep, Duration};

struct Aviso(&'static str);

impl Drop for Aviso {
    fn drop(&mut self) {
        println!("   [drop] liberando {}", self.0);
    }
}

async fn trabajo_largo() {
    let _recurso = Aviso("recursos de trabajo_largo");
    println!("empiezo");
    sleep(Duration::from_secs(10)).await;
    println!("termino");                          // ← NUNCA se imprime si se cancela
}

#[tokio::main]
async fn main() {
    let h = tokio::spawn(trabajo_largo());
    sleep(Duration::from_millis(200)).await;

    h.abort();                                    // pide cancelar la tarea

    match h.await {
        Ok(()) => println!("la tarea terminó"),
        Err(e) if e.is_cancelled() => println!("la tarea fue cancelada"),
        Err(e) => println!("la tarea hizo panic: {e}"),
    }
}
```

Salida **estable**:

```
empiezo
   [drop] liberando recursos de trabajo_largo
la tarea fue cancelada
```

La cancelación **destruyó** el futuro en su `.await`; **la limpieza (`Drop`) se ejecutó**, pero `println!("termino")` **nunca**.

⚠️ **Límites de `abort()`:**

- Solo actúa **en el siguiente `.await`**: una tarea en un bucle sin `.await` no se cancela.
- No cancela `spawn_blocking` una vez iniciado.

## 10.5 Seguridad frente a la cancelación (*cancel safety*)

> Un futuro es **seguro ante cancelación** si, al descartarlo a medias, **no se pierde ni se corrompe nada**.

El peligro: **un `.await` en medio de una operación de varios pasos**.

```rust
use tokio::time::{sleep, Duration};

// ⚠️ NO es seguro ante cancelación:
async fn mover_mal(origen: &mut Vec<u32>, destino: &mut Vec<u32>) {
    while let Some(x) = origen.pop() {                  // 1) SACA el elemento
        sleep(Duration::from_millis(10)).await;         //    ← si se cancela AQUÍ...
        destino.push(x);                                // 2) ...este push nunca ocurre: ¡x se perdió!
    }
}

// ✅ Seguro: no se quita del origen hasta haberlo puesto en el destino
async fn mover_bien(origen: &mut Vec<u32>, destino: &mut Vec<u32>) {
    while let Some(&x) = origen.last() {                // 1) solo MIRA
        sleep(Duration::from_millis(10)).await;
        destino.push(x);                                // 2) lo copia
        origen.pop();                                   // 3) recién ahora lo quita (sin `.await` entre 2 y 3)
    }
}

#[tokio::main]
async fn main() {
    let mut a = vec![1, 2, 3];
    let mut b = Vec::new();
    mover_bien(&mut a, &mut b).await;
    println!("{:?} {:?}", a, b);                        // [] [3, 2, 1]
}
```

**Reglas prácticas:**

1. Entre "tomar" y "dejar" un dato **no pongas `.await`**, o hazlo reversible.
2. Antes de usar un futuro en una rama de `select!` (sobre todo dentro de un bucle), **lee su documentación**: Tokio indica si es *cancel safe*. (`mpsc::Receiver::recv`, `sleep`, `interval.tick` lo son.)
3. Para operaciones críticas, **no canceles**: ejecútalas en una **tarea aparte** y **espera su `JoinHandle`**, o diseña un cierre ordenado.

## 10.6 Cancelación **cooperativa** y cierre ordenado

`abort()` es brusco. Para **avisar** y dejar que la tarea **limpie y salga por sí misma**, usa una señal. La opción más cómoda es `CancellationToken` del crate **`tokio-util`**:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-util = "0.7"
```

```rust
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

async fn trabajador(id: u32, token: CancellationToken) {
    let mut vueltas = 0;
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                println!("trabajador {id}: limpiando y saliendo tras {vueltas} vueltas");
                return;
            }
            _ = sleep(Duration::from_millis(100)) => {
                vueltas += 1;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let mut tareas = JoinSet::new();

    for id in 1..=3 {
        tareas.spawn(trabajador(id, token.clone()));          // todos comparten la misma señal
    }

    sleep(Duration::from_millis(350)).await;
    println!("pidiendo cancelación...");
    token.cancel();                                           // avisa a todos a la vez

    while let Some(r) = tareas.join_next().await {
        r.expect("el trabajador no debe hacer panic");
    }
    println!("todos terminaron");
}
```

Salida: `pidiendo cancelación...`, tres líneas `trabajador N: limpiando y saliendo tras 3 vueltas` (en **cualquier orden**) y `todos terminaron`.

- `token.clone()`: todos los clones **ven la misma señal**.
- `token.child_token()`: un token hijo que **también se cancela** si se cancela el padre (jerarquías).
- La tarea decide **dónde** reaccionar: puede terminar el paso actual y salir ordenadamente.

## 10.7 Apagado ordenado de un servicio (*graceful shutdown*)

Receta de cuatro fases:

```text
1. Llega la señal (Ctrl+C, SIGTERM, comando de administración)     → tokio::signal::ctrl_c()
2. Dejar de aceptar trabajo nuevo                                  → cerrar el listener / el canal
3. Pedir a las tareas que terminen y esperar (con un plazo)        → CancellationToken + timeout
4. Pasado el plazo, forzar                                         → JoinSet::abort_all() / destruir
```

```rust
use tokio::time::{sleep, timeout, Duration};

#[tokio::main]
async fn main() {
    let servicio = tokio::spawn(async {
        loop {
            sleep(Duration::from_millis(100)).await;
        }
    });

    println!("servicio en marcha; pulsa Ctrl+C para detenerlo (o espera 1 s)...");

    tokio::select! {
        _ = tokio::signal::ctrl_c() => println!("llegó Ctrl+C"),
        _ = sleep(Duration::from_secs(1)) => println!("tiempo de demostración agotado"),
    }

    servicio.abort();
    // Esperar un máximo de 2 s a que termine de verdad:
    match timeout(Duration::from_secs(2), servicio).await {
        Ok(_) => println!("servicio detenido"),
        Err(_) => println!("el servicio no respondió a tiempo"),
    }
}
```

## 10.8 Cancelación estructurada con `JoinSet`

Recuerda: **al destruirse un `JoinSet`, cancela todas sus tareas**. Combinado con `select!`, cancelas un **lote entero** descartando un solo futuro:

```rust
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

async fn lote() {
    let mut tareas = JoinSet::new();
    for i in 1..=3 {
        tareas.spawn(async move {
            sleep(Duration::from_secs(10)).await;
            println!("tarea {i} terminó");                    // nunca ocurre
        });
    }
    while let Some(_) = tareas.join_next().await {}
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = lote() => println!("lote completo"),
        _ = sleep(Duration::from_millis(300)) => println!("plazo vencido: se cancela TODO el lote"),
    }
    // Al salir del select!, `lote()` se destruye → su JoinSet se destruye → las 3 tareas se abortan.
}
```

Esta es la base del proyecto integrador asíncrono.

## 10.9 Resumen del módulo

1. **Cancelar = destruir el futuro.** Solo ocurre en un `.await`; los `Drop` se ejecutan.
2. `select!` cancela a los perdedores; `timeout` cancela al interior; `abort()` cancela una tarea; destruir un `JoinSet` cancela a sus tareas.
3. Cuida la **seguridad ante cancelación**: no dejes operaciones a medias entre dos `.await`.
4. Para cierres limpios: **señal cooperativa** (`CancellationToken`, `watch`) + plazo + `abort` como último recurso.

---

# Módulo 11: Timeouts, backpressure y límites de concurrencia

Los tres temas responden a la misma pregunta: **"¿qué pasa cuando las cosas se ponen lentas o hay demasiada carga?"** Un sistema robusto **acota** el tiempo, la cola y la cantidad de trabajo simultáneo.

## 11.1 Timeouts

### Con Tokio: `tokio::time::timeout`

```rust
use tokio::time::{sleep, timeout, Duration};

#[tokio::main]
async fn main() {
    match timeout(Duration::from_millis(100), sleep(Duration::from_secs(1))).await {
        Ok(()) => println!("terminó a tiempo"),
        Err(e) => println!("timeout: {e}"),              // timeout: deadline has elapsed
    }
}
```

- `timeout(duración, futuro).await` devuelve `Result<T, Elapsed>`.
- Si vence el plazo, el futuro interior **se cancela** (se destruye).
- Si la operación **ya devuelve `Result`**, quedan **dos niveles**:

```rust
use tokio::time::{sleep, timeout, Duration};

async fn consultar() -> Result<String, String> {
    sleep(Duration::from_millis(50)).await;
    Ok(String::from("datos"))
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_millis(200), consultar()).await {
        Ok(Ok(datos)) => println!("éxito: {datos}"),
        Ok(Err(e)) => println!("la operación falló: {e}"),
        Err(_) => println!("la operación tardó demasiado"),
    }
}
```

### ⚠️ Un timeout solo puede actuar **en un `.await`**

```rust
// ❌ No se puede interrumpir: nunca cede el control
// timeout(Duration::from_millis(10), async {
//     std::thread::sleep(std::time::Duration::from_secs(5));     // bloquea; el timeout no puede intervenir
// }).await;
```

Un timeout **no mata** código bloqueante; por eso, **nunca bloquees dentro de `async`**.

### Dónde poner timeouts

| Operación | ¿Timeout? |
|---|---|
| Conexiones de red, peticiones HTTP, consultas a base de datos | ✅ **Siempre** (la red falla de formas silenciosas) |
| Esperar una respuesta de otro servicio | ✅ |
| Cola de espera por un recurso (permiso, lock) | ✅ |
| Tareas internas puramente en memoria | Normalmente no |

> **Regla de oro:** *toda operación que dependa de algo externo debe tener un plazo.* Sin plazo, un fallo remoto se convierte en **un servicio colgado para siempre**.

### Timeout total vs. por intento

```rust
use tokio::time::{sleep, timeout, Duration};

async fn paso(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}

#[tokio::main]
async fn main() {
    // Plazo TOTAL para varias etapas:
    let resultado = timeout(Duration::from_millis(250), async {
        paso(100).await;
        paso(100).await;
        paso(100).await;                                    // en total 300 ms > 250 ms
    })
    .await;
    println!("{}", if resultado.is_ok() { "a tiempo" } else { "plazo total vencido" });
}
```

### Reintentos con espera exponencial (*backoff*)

```rust
use tokio::time::{sleep, timeout, Duration};

async fn operacion_inestable(intento: u32) -> Result<String, String> {
    sleep(Duration::from_millis(20)).await;
    if intento < 3 {
        Err(format!("fallo en el intento {intento}"))
    } else {
        Ok(String::from("éxito"))
    }
}

async fn con_reintentos() -> Result<String, String> {
    let mut espera = Duration::from_millis(100);

    for intento in 1..=5 {
        // Cada intento tiene su propio plazo:
        match timeout(Duration::from_millis(500), operacion_inestable(intento)).await {
            Ok(Ok(valor)) => return Ok(valor),
            Ok(Err(e)) => println!("{e}; reintentando en {espera:?}"),
            Err(_) => println!("intento {intento} excedió el plazo; reintentando en {espera:?}"),
        }
        sleep(espera).await;
        espera *= 2;                                       // 100 ms, 200 ms, 400 ms...
    }
    Err(String::from("se agotaron los reintentos"))
}

#[tokio::main]
async fn main() {
    println!("{:?}", con_reintentos().await);
}
```

Salida **estable**:

```
fallo en el intento 1; reintentando en 100ms
fallo en el intento 2; reintentando en 200ms
Ok("éxito")
```

Buenas prácticas de reintentos: **limita** el número, usa **espera creciente** (y, en sistemas reales, algo de **aleatoriedad** — *jitter* — para no sincronizar a todos los clientes) y **reintenta solo errores que lo merezcan** (fallos transitorios, no "contraseña incorrecta").

### Timeouts con hilos (sin async)

Con `std` no puedes "matar" un hilo; lo que sí puedes es **dejar de esperarlo**:

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));            // trabajo lento
        let _ = tx.send("resultado");
    });

    match rx.recv_timeout(Duration::from_millis(300)) {
        Ok(v) => println!("recibido: {v}"),
        Err(mpsc::RecvTimeoutError::Timeout) => println!("no esperé más (el hilo sigue corriendo)"),
        Err(mpsc::RecvTimeoutError::Disconnected) => println!("el hilo terminó sin responder"),
    }
}
```

### Probar timeouts sin esperar: tiempo "pausado"

```toml
[dev-dependencies]
tokio = { version = "1", features = ["full", "test-util"] }
```

```rust
#[cfg(test)]
mod tests {
    use tokio::time::{sleep, timeout, Duration};

    #[tokio::test(start_paused = true)]               // el reloj de Tokio avanza solo cuando todo está inactivo
    async fn el_timeout_expira() {
        let r = timeout(Duration::from_secs(5), sleep(Duration::from_secs(60))).await;
        assert!(r.is_err());                          // ¡el test dura milisegundos, no 5 s!
    }
}
```

## 11.2 Backpressure (contrapresión)

### El problema

Cuando un **productor** es más rápido que un **consumidor**, el trabajo pendiente se **acumula**:

```text
Productor:  ████████████████  (1000 / s)
Cola:       ▁▂▃▄▅▆▇█████████▶ ¡crece sin límite!  → memoria agotada → el servicio muere
Consumidor: ████              (100 / s)
```

**Backpressure** es el mecanismo por el cual **el consumidor "empuja hacia atrás"** y **frena al productor**. La forma más natural en Rust: **canales acotados**.

### Cola **acotada**: `send().await` espera

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<u32>(2);              // capacidad: 2

    let productor = tokio::spawn(async move {
        for i in 1..=6 {
            println!("[prod] enviando {i}");
            tx.send(i).await.expect("receptor vivo");        // ← se SUSPENDE si la cola está llena
            println!("[prod] enviado   {i}");
        }
    });

    let consumidor = tokio::spawn(async move {
        while let Some(i) = rx.recv().await {
            println!("            [cons] procesando {i}");
            sleep(Duration::from_millis(100)).await;         // consumidor lento
        }
    });

    productor.await.expect("el productor no debe hacer panic");
    consumidor.await.expect("el consumidor no debe hacer panic");
}
```

Salida **típica** (el detalle de las primeras líneas puede variar levemente):

```
[prod] enviando 1
[prod] enviado   1
[prod] enviando 2
[prod] enviado   2
[prod] enviando 3
            [cons] procesando 1
[prod] enviado   3
[prod] enviando 4
            [cons] procesando 2       ← cada ~100 ms: el productor avanza al ritmo del consumidor
[prod] enviado   4
[prod] enviando 5
...
```

El productor **no se adelanta más de unos pocos mensajes**: la cola acotada lo frena. **La memoria usada está acotada**, sin importar cuánto dure la sobrecarga.

### Cola **no acotada**: la trampa

```rust
// ⚠️ unbounded_channel: send() NUNCA espera → la cola puede crecer hasta agotar la memoria
// let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
```

> **Regla:** usa canales **acotados por defecto**. Un canal no acotado es una promesa de que "el consumidor siempre podrá con todo", y rara vez se puede cumplir.

### ¿Qué hacer cuando la cola está llena?

| Estrategia | Cómo | Cuándo |
|---|---|---|
| **Esperar** (backpressure puro) | `tx.send(v).await` | El productor puede esperar (procesos internos, lotes) |
| **Rechazar** (*load shedding*) | `tx.try_send(v)` → `Err(Full)` y responder "ocupado" (p. ej. HTTP 503) | Servidores: es mejor rechazar rápido que acumular latencia |
| **Descartar lo viejo** | Canal `broadcast` (los lentos "pierden" mensajes) o buffer circular | Telemetría, cotizaciones: solo importa lo reciente |
| **Reservar antes de producir** | `let permiso = tx.reserve().await?; permiso.send(v);` | Cuando **crear** el mensaje es costoso: reserva el sitio primero |
| **Agrupar** (*batching*) | El consumidor procesa de a lotes | Escrituras a base de datos |

### Backpressure con hilos

`std::sync::mpsc::sync_channel(n)` (Módulo 5) aplica lo mismo, **bloqueando** el hilo productor.

### La contrapresión debe **propagarse**

```text
Cliente ──▶ Servidor ──▶ Cola acotada ──▶ Trabajadores ──▶ Base de datos
   ▲                          │ (llena)
   └── "503 ocupado" ◀────────┘
```

Si **un solo eslabón** es no acotado, el problema reaparece ahí. Hay que **acotar de punta a punta**.

### ⚠️ Cuidado con los ciclos de canales acotados

Si la tarea A envía a B con un canal lleno, y B envía a A con otro canal lleno, **ambas se esperan mutuamente** → **deadlock**. Evita ciclos, o usa `try_send`/`reserve` en uno de los sentidos.

## 11.3 Límites de concurrencia

Sin un tope, un bucle que crea una tarea por elemento puede disparar **miles de operaciones simultáneas**: agotarás **conexiones**, **descriptores de archivo**, **memoria** o harás **caer al servicio remoto**.

### `Semaphore`: contar permisos

Un **semáforo** reparte **N permisos**. Para continuar, hay que **adquirir** uno; cuando se acaban, los demás **esperan**. Al soltar el permiso (al destruirlo), otro puede entrar.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let semaforo = Arc::new(Semaphore::new(3));              // máximo 3 tareas simultáneas
    let activas = Arc::new(AtomicUsize::new(0));
    let maximo = Arc::new(AtomicUsize::new(0));
    let mut tareas = JoinSet::new();

    for id in 1..=8 {
        // Adquirir el permiso ANTES de crear la tarea: así también frenamos el bucle (backpressure)
        let permiso = Arc::clone(&semaforo)
            .acquire_owned()
            .await
            .expect("el semáforo nunca se cierra");

        let activas = Arc::clone(&activas);
        let maximo = Arc::clone(&maximo);

        tareas.spawn(async move {
            let ahora = activas.fetch_add(1, Ordering::SeqCst) + 1;
            maximo.fetch_max(ahora, Ordering::SeqCst);

            println!("tarea {id} empieza  (activas: {ahora})");
            sleep(Duration::from_millis(200)).await;
            println!("tarea {id} termina");

            activas.fetch_sub(1, Ordering::SeqCst);
            drop(permiso);                                  // devuelve el permiso
        });
    }

    while let Some(r) = tareas.join_next().await {
        r.expect("la tarea no debe hacer panic");
    }
    println!("máximo simultáneo observado: {}", maximo.load(Ordering::SeqCst));   // 3
}
```

El orden de las líneas varía, pero **el máximo simultáneo nunca pasa de 3**. Los puntos clave:

- `Semaphore::new(n)`: `n` permisos.
- `acquire_owned()` (sobre un `Arc<Semaphore>`) devuelve un permiso **que puedes mover a la tarea**; se libera al destruirse (`Drop`).
- **Adquirir antes del `spawn`** limita también cuántas tareas existen.
- `try_acquire()` → `Err(NoPermits)` si no hay: útil para **rechazar** en vez de esperar.
- Con un semáforo de **1 permiso** obtienes un **mutex async**.

### *Pool* de trabajadores fijo

Otra forma de limitar: **N trabajadores** que extraen de **una cola acotada**. Combina límite de concurrencia **y** backpressure.

```rust
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<u32>(4);                   // cola acotada
    let rx = Arc::new(Mutex::new(rx));                        // varios trabajadores comparten el receptor

    let mut trabajadores = JoinSet::new();
    for id in 0..3 {
        let rx = Arc::clone(&rx);
        trabajadores.spawn(async move {
            let mut hechos = 0;
            loop {
                // Bloque corto: el guardia se libera antes de procesar
                let trabajo = {
                    let mut guardia = rx.lock().await;
                    guardia.recv().await
                };
                match trabajo {
                    Some(n) => {
                        sleep(Duration::from_millis(50)).await;       // "procesar" n
                        hechos += 1;
                        let _ = n;
                    }
                    None => break,                                    // canal cerrado y vacío
                }
            }
            (id, hechos)
        });
    }

    for n in 1..=12 {
        tx.send(n).await.expect("hay trabajadores vivos");           // se frena si la cola está llena
    }
    drop(tx);                                                         // "no hay más trabajo"

    let mut total = 0;
    while let Some(r) = trabajadores.join_next().await {
        let (_id, hechos) = r.expect("el trabajador no debe hacer panic");
        total += hechos;
    }
    println!("trabajos procesados: {total}");                         // 12
}
```

### Limitar la **tasa** (*rate limiting*)

Un tope de **peticiones por segundo** (no de simultaneidad) se logra con un temporizador:

```rust
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    let mut cada_100ms = interval(Duration::from_millis(100));    // el primer tick es inmediato

    for i in 0..5 {
        cada_100ms.tick().await;                                  // espera al siguiente "turno"
        println!("petición {i}");                                 // ≈ 10 por segundo
    }
}
```

### Esquema de un servidor con tope de conexiones

```text
(esquema, no es un programa completo)

loop {
    let permiso = limite.clone().acquire_owned().await;   // si ya hay 100 conexiones, se ESPERA aquí
    let (socket, _) = listener.accept().await;
    tokio::spawn(async move {
        atender(socket).await;                            // con su propio timeout
        drop(permiso);                                    // al terminar, otra conexión puede entrar
    });
}
```

### ¿Qué límite poner?

| Recurso | Límite típico |
|---|---|
| Conexiones a una base de datos | El tamaño del *pool* (decenas) |
| Peticiones simultáneas a un servicio externo | Lo que ese servicio tolere (consulta su documentación) |
| Archivos abiertos | Bastante menos que el límite del SO (`ulimit -n`) |
| Tareas CPU-pesadas | ≈ número de núcleos |
| Conexiones entrantes de un servidor | Según la memoria y el tiempo medio por conexión |

## 11.4 Los tres conceptos juntos

| Mecanismo | Protege contra | Herramienta |
|---|---|---|
| **Timeout** | Esperas **infinitas** | `timeout`, `recv_timeout` |
| **Backpressure** | **Acumulación** de trabajo pendiente | Canales **acotados**, `try_send`, `reserve` |
| **Límite de concurrencia** | **Demasiado** trabajo simultáneo | `Semaphore`, *pool* de trabajadores |
| **Cancelación** | Trabajo **que ya no interesa** | `select!`, `abort`, `JoinSet`, tokens |
| **Reintentos con backoff** | Fallos **transitorios** | Bucle + `sleep` creciente |

Un servicio robusto **usa los cinco**: *"acota el tiempo, acota la cola, acota la simultaneidad, cancela lo inútil y reintenta con cabeza."*

## 11.5 Errores frecuentes en esta zona

| Error | Consecuencia | Solución |
|---|---|---|
| Sin timeout en llamadas externas | Tareas colgadas para siempre | `timeout(...)` |
| `unbounded_channel` "porque es más fácil" | Memoria que crece sin límite | `channel(n)` + política para cola llena |
| Crear una tarea por elemento sin tope | Agotar conexiones o memoria | `Semaphore` / *pool* fijo |
| Mantener un permiso o lock mientras se espera algo lento | Estrangula a todos los demás | Soltar antes de esperar |
| Reintentar sin límite ni espera | Tormenta de peticiones que agrava la caída | Máximo de intentos + *backoff* |
| `select!` con futuros que no son *cancel safe* | Pérdida de datos | Revisar la documentación; reestructurar |
| Ciclos de canales acotados | Deadlock | Evitar ciclos / `try_send` |

---

# Proyecto integrador: ingesta de eventos de pedidos en tiempo real

El encargo empresarial de este nivel consiste en implementar una canalización concurrente de eventos, primero con threads y después con Tokio, con límites, timeouts y cierre ordenado.

**Proyecto independiente:** [Abrir el enunciado completo](/proyectos/rust-nivel-5-ingesta-eventos/).

**Módulos integrados:** revisa en el enunciado qué requisito empresarial pone en práctica cada módulo del nivel.

---


# Ejercicios con soluciones

> 💡 Pide siempre que **expliquen el porqué** de su elección (hilos vs. async, `Mutex` vs. canal, qué se cancela...).

## Módulo 2: Threads

**2.1** Crea 4 hilos que calculen el cuadrado de `id` (0 a 3) y recoge los resultados **en orden** en un `Vec`.

**2.2** Usando `thread::scope` y `split_at_mut`, duplica la mitad izquierda de un `Vec<i32>` y triplica la derecha, en dos hilos.

<details>
<summary>Ver soluciones</summary>

```rust
use std::thread;

fn main() {
    // 2.1
    let mut manejadores = Vec::new();
    for id in 0..4u32 {
        manejadores.push(thread::spawn(move || id * id));
    }
    let mut resultados = Vec::new();
    for h in manejadores {
        resultados.push(h.join().expect("el hilo no debe hacer panic"));
    }
    println!("{:?}", resultados);                  // [0, 1, 4, 9]

    // 2.2
    let mut datos = vec![1, 2, 3, 4, 5, 6];
    let mitad = datos.len() / 2;
    let (izq, der) = datos.split_at_mut(mitad);
    thread::scope(|s| {
        s.spawn(move || {
            for x in izq.iter_mut() {
                *x *= 2;
            }
        });
        s.spawn(move || {
            for x in der.iter_mut() {
                *x *= 3;
            }
        });
    });
    println!("{:?}", datos);                       // [2, 4, 6, 12, 15, 18]
}
```
</details>

## Módulo 3: `Send` y `Sync`

**3.1** Sin compilar, ¿cuáles de estas líneas dan error? Explica por qué.

```rust
fn exige_send<T: Send>() {}
fn exige_sync<T: Sync>() {}

exige_send::<Vec<String>>();           // a
exige_send::<Rc<String>>();            // b
exige_sync::<Cell<i32>>();             // c
exige_send::<Arc<Mutex<Vec<i32>>>>();  // d
exige_sync::<Mutex<i32>>();            // e
```

<details>
<summary>Ver solución</summary>

- **a** ✅ `Vec<String>` es `Send`.
- **b** ❌ `Rc` no es `Send` (contador no atómico).
- **c** ❌ `Cell` no es `Sync` (mutabilidad interior sin sincronización).
- **d** ✅ `Arc<Mutex<_>>` es `Send` si el contenido lo es.
- **e** ✅ `Mutex<i32>` es `Sync`: sincroniza el acceso.
</details>

## Módulo 4: `Arc`, `Mutex`, `RwLock`

**4.1** Cuatro hilos suman cada uno los números 1..=100 **a un total compartido**. Usa `Arc<Mutex<u64>>`. Resultado esperado: `20200`.

**4.2** ¿Por qué esta solución es peor? (¿qué lock se toma y cuándo?)

```rust
// por cada número:   *total.lock().unwrap() += n;
```

<details>
<summary>Ver soluciones</summary>

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let total = Arc::new(Mutex::new(0u64));
    let mut manejadores = Vec::new();

    for _ in 0..4 {
        let total = Arc::clone(&total);
        manejadores.push(thread::spawn(move || {
            // Mejor: acumular localmente y tomar el lock UNA vez
            let mut parcial = 0u64;
            for n in 1..=100u64 {
                parcial += n;
            }
            *total.lock().expect("mutex envenenado") += parcial;
        }));
    }

    for h in manejadores {
        h.join().expect("el hilo no debe hacer panic");
    }
    println!("{}", *total.lock().expect("mutex envenenado"));        // 20200
}
```

**4.2** Tomar el lock **en cada número** genera **mucha contención** (400 adquisiciones del lock en vez de 4) sin ninguna necesidad. La regla: **trabaja con datos locales y toma el lock solo para publicar el resultado.**
</details>

## Módulo 5: Canales

**5.1** Crea un productor que envíe los números 1..=5 y un consumidor que los sume. Debe terminar solo.

**5.2** ¿Por qué este programa no termina nunca?

```rust
let (tx, rx) = mpsc::channel();
for i in 0..3 {
    let tx = tx.clone();
    thread::spawn(move || tx.send(i).unwrap());
}
for v in rx { println!("{v}"); }
```

<details>
<summary>Ver soluciones</summary>

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let productor = thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).expect("receptor vivo");
        }
    });                                       // `tx` se destruye al terminar el hilo

    let mut suma = 0;
    for v in rx {
        suma += v;
    }
    productor.join().expect("el productor no debe hacer panic");
    println!("{suma}");                       // 15
}
```

**5.2** Falta `drop(tx)`: el `tx` **original** sigue vivo en `main`, así que el canal **nunca se cierra** y `for v in rx` espera para siempre. Se arregla con `drop(tx);` antes del `for`.
</details>

## Módulo 6 y 7: `async` y futuros

**6.1** ¿Qué imprime este programa y cuánto tarda aproximadamente? ¿Y si se cambia por `join!`?

```rust
#[tokio::main]
async fn main() {
    let a = espera("A", 200).await;
    let b = espera("B", 300).await;
    println!("{a} {b}");
}
// async fn espera(n: &str, ms: u64) -> String { sleep(ms).await; format!("{n} ok") }
```

**7.1** Explica con tus palabras: ¿por qué una `async fn` que se llama pero **no** se `.await`-ea no hace nada?

<details>
<summary>Ver soluciones</summary>

**6.1** Imprime `A ok B ok` y tarda **~500 ms** (secuencial: 200 + 300). Con `let (a, b) = tokio::join!(espera("A", 200), espera("B", 300));` tarda **~300 ms** (el máximo).

**7.1** Una `async fn` devuelve un **futuro**, que es un objeto inerte: describe el trabajo, pero **nada lo ejecuta** hasta que alguien llama a `poll` (lo que hace `.await`, `spawn` o el runtime). Crearlo no ejecuta una sola línea de su cuerpo.
</details>

## Módulo 8: Tareas en Tokio

**8.1** Lanza 5 tareas con `JoinSet` que devuelvan su id tras esperar `(5 - id) * 50` ms. Imprime los ids **en orden de finalización**.

**8.2** ¿Qué ocurre si una tarea hace panic? ¿Y si no haces `await` a su `JoinHandle`?

<details>
<summary>Ver soluciones</summary>

```rust
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let mut tareas = JoinSet::new();
    for id in 0..5u64 {
        tareas.spawn(async move {
            sleep(Duration::from_millis((5 - id) * 50)).await;
            id
        });
    }
    while let Some(r) = tareas.join_next().await {
        print!("{} ", r.expect("la tarea no debe hacer panic"));
    }
    println!();                                           // 4 3 2 1 0
}
```

**8.2** El panic **solo mata a esa tarea**; el runtime y las demás siguen. Si **no** haces `await` al `JoinHandle`, el panic **se pierde en silencio** (solo verás el mensaje en stderr): por eso conviene recoger los `JoinHandle` o usar un `JoinSet` y revisar los `Result`.
</details>

## Módulo 9: Canales y sincronización de Tokio

**9.1** Implementa un **actor** de "libro de visitas": comandos `Firmar(String)` y `Listar(oneshot::Sender<Vec<String>>)`.

<details>
<summary>Ver solución</summary>

```rust
use tokio::sync::{mpsc, oneshot};

enum Comando {
    Firmar(String),
    Listar(oneshot::Sender<Vec<String>>),
}

async fn libro(mut rx: mpsc::Receiver<Comando>) {
    let mut firmas: Vec<String> = Vec::new();
    while let Some(c) = rx.recv().await {
        match c {
            Comando::Firmar(nombre) => firmas.push(nombre),
            Comando::Listar(resp) => {
                let _ = resp.send(firmas.clone());
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(8);
    let actor = tokio::spawn(libro(rx));

    tx.send(Comando::Firmar(String::from("Ana"))).await.expect("actor vivo");
    tx.send(Comando::Firmar(String::from("Luis"))).await.expect("actor vivo");

    let (tx_r, rx_r) = oneshot::channel();
    tx.send(Comando::Listar(tx_r)).await.expect("actor vivo");
    println!("{:?}", rx_r.await.expect("el actor respondió"));      // ["Ana", "Luis"]

    drop(tx);
    actor.await.expect("el actor no debe hacer panic");
}
```
</details>

## Módulo 10: Cancelación

**10.1** Escribe un `select!` que ejecute `tarea_larga()` pero la cancele a los 300 ms e imprima qué ocurrió.

**10.2** ¿Qué problema de *cancel safety* tiene esta función y cómo se arregla?

```rust
async fn procesar(cola: &mut Vec<u32>, hechos: &mut Vec<u32>) {
    while let Some(x) = cola.pop() {
        trabajo_async(x).await;
        hechos.push(x);
    }
}
```

<details>
<summary>Ver soluciones</summary>

```rust
use tokio::time::{sleep, Duration};

async fn tarea_larga() -> &'static str {
    sleep(Duration::from_secs(5)).await;
    "listo"
}

#[tokio::main]
async fn main() {
    tokio::select! {
        r = tarea_larga() => println!("terminó: {r}"),
        _ = sleep(Duration::from_millis(300)) => println!("cancelada a los 300 ms"),
    }
}
```

**10.2** Si se cancela durante `trabajo_async(x).await`, `x` **ya se sacó** de `cola` (`pop`) pero **no llegó** a `hechos`: **se pierde**. Arreglo: **mirar primero, sacar después**:

```rust
async fn procesar(cola: &mut Vec<u32>, hechos: &mut Vec<u32>) {
    while let Some(&x) = cola.last() {
        trabajo_async(x).await;        // si se cancela aquí, `x` sigue en `cola`
        hechos.push(x);
        cola.pop();                    // sin .await entre estas dos líneas
    }
}
```
</details>

## Módulo 11: Timeouts, backpressure y límites

**11.1** Escribe `async fn consultar(id: u32) -> Result<String, String>` que tarde `id * 100` ms, y úsala con un timeout de 250 ms para los ids 1, 2 y 3. Imprime qué ocurre con cada uno.

**11.2** Procesa 10 tareas de 100 ms con **un máximo de 2 simultáneas** usando `Semaphore`. ¿Cuánto debe tardar aproximadamente?

**11.3** Con `mpsc::channel(1)` y `try_send`, envía 3 mensajes sin receptor activo. ¿Cuántos se encolan y cuántos se rechazan?

<details>
<summary>Ver soluciones</summary>

```rust
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::sync::mpsc::error::TrySendError;
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout, Duration};

async fn consultar(id: u32) -> Result<String, String> {
    sleep(Duration::from_millis(id as u64 * 100)).await;
    Ok(format!("respuesta {id}"))
}

#[tokio::main]
async fn main() {
    // 11.1
    for id in 1..=3 {
        match timeout(Duration::from_millis(250), consultar(id)).await {
            Ok(Ok(r)) => println!("id {id}: {r}"),
            Ok(Err(e)) => println!("id {id}: error {e}"),
            Err(_) => println!("id {id}: timeout"),
        }
    }
    // id 1: respuesta 1 / id 2: respuesta 2 / id 3: timeout (300 ms > 250 ms)

    // 11.2
    let inicio = std::time::Instant::now();
    let semaforo = Arc::new(Semaphore::new(2));
    let mut tareas = JoinSet::new();
    for _ in 0..10 {
        let permiso = Arc::clone(&semaforo).acquire_owned().await.expect("semáforo abierto");
        tareas.spawn(async move {
            sleep(Duration::from_millis(100)).await;
            drop(permiso);
        });
    }
    while let Some(r) = tareas.join_next().await {
        r.expect("la tarea no debe hacer panic");
    }
    println!("~{} ms", inicio.elapsed().as_millis());        // ~500 ms (10 tareas / 2 a la vez × 100 ms)

    // 11.3
    let (tx, _rx) = mpsc::channel::<u32>(1);
    for i in 1..=3 {
        match tx.try_send(i) {
            Ok(()) => println!("encolado {i}"),              // solo el 1
            Err(TrySendError::Full(v)) => println!("rechazado {v}"),      // 2 y 3
            Err(TrySendError::Closed(v)) => println!("cerrado {v}"),
        }
    }
}
```
</details>

---

# Hoja de referencia rápida

```rust
// ---------- CLOSURES ----------
|a, b| a + b        || println!("x")        move || { ... }      // `move`: se lleva lo capturado
// Fn (lee) · FnMut (modifica) · FnOnce (consume, una vez)

// ---------- HILOS ----------
let h = thread::spawn(move || { ...; valor });        // F: FnOnce + Send + 'static
let v = h.join().expect("...");                        // Err si el hilo hizo panic
thread::scope(|s| { s.spawn(|| ...); });               // hilos que PRESTAN datos locales
thread::sleep(Duration::from_millis(50));
data.split_at_mut(n)    data.chunks(n)                 // repartir sin locks

// ---------- SEND / SYNC ----------
// Send = se puede MOVER a otro hilo · Sync = se puede COMPARTIR (&T) entre hilos
// ✗ Rc · ✗ Cell/RefCell (Sync) · ✗ punteros crudos · ✗ MutexGuard (Send)

// ---------- COMPARTIR ----------
Arc::new(x)    Arc::clone(&a)    Arc::strong_count(&a)         // propiedad compartida (solo lectura)
Arc<Mutex<T>>:    let mut g = m.lock().expect("..."); *g += 1;  // el guardia libera al salir de ámbito
Arc<RwLock<T>>:   m.read().expect("...")   m.write().expect("...")
AtomicUsize::new(0)  .fetch_add(1, Ordering::SeqCst)  .load(Ordering::SeqCst)
// Deadlock: locks en orden fijo, secciones cortas, nada lento con el lock tomado

// ---------- CANALES (std) ----------
let (tx, rx) = mpsc::channel();         let (tx, rx) = mpsc::sync_channel(n);   // acotado
tx.send(v)   rx.recv()   rx.try_recv()   rx.recv_timeout(d)   for v in rx { }
let tx2 = tx.clone();   drop(tx);       // el canal se cierra al destruirse TODOS los Sender

// ---------- ASYNC ----------
async fn f() -> T { ... }               let fut = f();   let v = fut.await;     // perezoso
tokio::join!(a(), b())                  tokio::try_join!(a(), b())              // concurrentes en la misma tarea
async move { ... }                       // bloque async que se lleva lo capturado

// ---------- TOKIO ----------
#[tokio::main]   #[tokio::main(flavor = "current_thread")]
let h = tokio::spawn(async move { ... });     h.await   h.abort()   // Send + 'static
let mut set = JoinSet::new();  set.spawn(...);  while let Some(r) = set.join_next().await { }
tokio::task::spawn_blocking(|| trabajo_pesado())
sleep(d).await   timeout(d, fut).await   interval(d).tick().await

// ---------- CANALES / SYNC (tokio) ----------
mpsc::channel(n)  tx.send(v).await  rx.recv().await -> Option    tx.try_send(v)    // acotado
oneshot::channel()     broadcast::channel(n)     watch::channel(v)     Notify
tokio::sync::Mutex (lock().await)     Semaphore::new(n)  .acquire_owned().await

// ---------- CANCELACIÓN ----------
tokio::select! { r = fut_a => {...}  _ = fut_b => {...} }     // la 1ª gana; las otras se CANCELAN
CancellationToken (tokio-util): token.clone()  token.cancel()  token.cancelled().await
// Cancelar = destruir el futuro (solo ocurre en un .await). Cuidado con operaciones a medias.
```

### ¿Hilos, async o canales?

```
¿Trabajo de CPU pesado?                         → hilos (thread::scope / spawn / rayon) o spawn_blocking
¿Mucha espera de red/disco con muchas tareas?   → async + Tokio
¿Varios hilos modifican un mismo dato?          → Arc<Mutex<T>>  (o un actor con canal)
¿Hay un flujo productor → consumidor?           → canal ACOTADO
¿Quiero limitar la simultaneidad?               → Semaphore / pool de trabajadores
¿Dependo de algo externo?                       → timeout SIEMPRE
```

---

# Errores frecuentes

| Código / mensaje | Causa típica | Solución |
|---|---|---|
| `E0373` closure/async block may outlive the current function | El hilo/tarea toma prestada una variable local | `move` / `async move` / `thread::scope` |
| `E0382` value moved into closure, in previous iteration of loop | Mover un `Mutex`, `String`... al closure dentro de un bucle | `Arc::clone` / `.clone()` por iteración |
| `E0277` `Rc<T>` cannot be sent between threads safely | `Rc` en un hilo/tarea | `Arc` |
| `E0277` `Cell<T>` cannot be shared between threads safely | `Cell`/`RefCell` en estado compartido | `Mutex`/`RwLock`/atómicos |
| error: future cannot be sent between threads safely | Algo `!Send` (un `MutexGuard`, un `Rc`) vive a través de un `.await` | Acortar su vida antes del `.await`, o usar `tokio::sync::Mutex` |
| `E0596` cannot borrow data in an `Arc` as mutable | Intentar mutar el contenido de un `Arc` | `Arc<Mutex<T>>` |
| `E0499` cannot borrow as mutable more than once (hilos con alcance) | Dos hilos con `&mut` al mismo dato | `split_at_mut` / dar un dato exclusivo a cada hilo |
| `E0728` `await` is only allowed inside `async` functions and blocks | `.await` en una función normal | Hacerla `async` |
| `E0752` `main` function is not allowed to be `async` | `async fn main` sin runtime | `#[tokio::main]` |
| warning: unused implementer of `Future` that must be used | Olvidaste `.await` | `.await` o `spawn` |
| panic: there is no reactor running | `spawn`/`sleep` de Tokio fuera de un runtime | Ejecutar dentro de `#[tokio::main]` / `block_on` |
| panic: Cannot start a runtime from within a runtime | `block_on` dentro de código async | Usar `.await` |
| panic: `PoisonError` al hacer `lock().unwrap()` | Otro hilo hizo panic con el lock tomado | Tratar el envenenamiento (`into_inner`) o corregir el panic original |
| El programa se queda colgado (hilos) | Deadlock, o falta `drop(tx)` | Orden fijo de locks; soltar el `Sender` original |
| El programa se queda colgado (async) | `recv()` esperando un canal sin cerrar; ciclo de canales acotados; bloqueo con `std` dentro de `async` | `drop(tx)`; evitar ciclos; usar equivalentes async |
| Una tarea parece "congelada" o todo va lento | Bloqueo (`std::thread::sleep`, cálculo pesado) en un hilo del runtime | `.await` / `spawn_blocking` |
| Memoria que crece sin parar | Cola `unbounded` o tareas sin tope | Canal acotado + `Semaphore` |

---

# Plan de clases sugerido

Propuesta de 7 sesiones de ~90 minutos:

| Sesión | Contenido | Actividad práctica |
|---|---|---|
| **1** | Módulo 0 (closures) + Módulo 1 (mapa mental) + Módulo 2 (hilos) | Ejercicios 2.1 y 2.2; provocar `E0373` y la trampa del `move` con `Copy` |
| **2** | Módulo 3 (`Send`/`Sync`) + Módulo 4 (`Arc`, `Mutex`, `RwLock`) | Ejercicios 3.1, 4.1, 4.2; envenenar un mutex a propósito |
| **3** | Módulo 5 (canales) | Ejercicios 5.1, 5.2; el *pool* de trabajadores; **Proyecto A** |
| **4** | Módulo 6 (`async`/`.await`) + Módulo 7 (futuros) | Ejercicios 6.1, 7.1; ejecutar el `block_on` casero y modificarlo |
| **5** | Módulo 8 (tareas en Tokio) + Módulo 9 (canales y sincronización) | Ejercicios 8.1, 8.2, 9.1; el actor con `oneshot` |
| **6** | Módulo 10 (cancelación) | Ejercicios 10.1, 10.2; `CancellationToken`; cierre ordenado |
| **7** | Módulo 11 (timeouts, backpressure, límites) + **Proyecto B** | Ejercicios 11.1–11.3; los experimentos del proyecto |

### Recomendaciones de enseñanza

1. **Haz visible el tiempo.** Con `sleep` y `Instant`, que **vean** la diferencia entre secuencial y concurrente. Un cronómetro vale más que mil explicaciones.
2. **Ejecuta cada ejemplo varias veces.** Que vean cómo cambia el orden de las líneas, y por qué **hay que ordenar** antes de comparar resultados.
3. **Provoca los errores del compilador** (`Rc` en un hilo, un `MutexGuard` a través de un `.await`, olvidar `move`) y léanlos en voz alta: aquí el compilador es un **excelente profesor** de concurrencia.
4. **Analogías que funcionan:**
   - *Mutex* = el único baño de la casa con llave.
   - *Canal* = una ventanilla/pasaplatos entre cocina y sala.
   - *Hilo* = un cocinero más; *tarea async* = una receta que un cocinero atiende cuando toca.
   - *Backpressure* = el camarero que deja de tomar pedidos cuando la cocina está saturada.
   - *Semáforo* = N mesas disponibles en el restaurante.
5. **Dibuja líneas de tiempo** en la pizarra: quién espera a quién, y dónde está el `.await`.
6. **Insiste en las "reglas de higiene":**
   - *Nunca bloquees dentro de `async`.*
   - *Toda operación externa lleva timeout.*
   - *Canales acotados por defecto.*
   - *Lock corto, y nunca durante algo lento.*
   - *Cuando sea posible, comunica en vez de compartir.*
7. **Ideas centrales para repetir:**
   - *Si compila, no hay carreras de datos; deadlocks y lógica siguen siendo cosa tuya.*
   - *`.await` no es paralelismo: la concurrencia viene de `join!` y `spawn`.*
   - *Cancelar un futuro es destruirlo; ocurre solo en los `.await`.*
   - *Un sistema robusto acota el tiempo, la cola y la simultaneidad.*

### Qué sigue

Con este nivel, el siguiente paso natural es el **ecosistema práctico**: **redes y servicios** (HTTP con `axum`/`reqwest`, TCP con `tokio::net`), **serialización** (`serde`), **trazas y métricas** (`tracing`), **iteradores y closures a fondo**, **paralelismo de datos** con `rayon` y **trait objects** (`dyn Trait`), que permiten colecciones de tipos distintos y se usan muchísimo en servicios reales.

---

*Fin del Nivel 5. En Rust, la concurrencia no es un campo minado: es un terreno con vallas. 🦀*
