# 🦀 Curso de Rust — Nivel 4: Manejo de errores y código seguro

> **Objetivo:** que tus estudiantes escriban programas que **no se caigan por sorpresa**: que cada cosa que pueda fallar o faltar esté **visible en el tipo**, que sepan **elegir** entre devolver un error o hacer `panic!`, y que puedan diseñar **sus propios errores** de forma clara.
>
> **Requisitos:** Niveles 1 a 3 (en particular `match`, enums, structs, traits básicos, ownership y referencias).
>
> **Idea central del nivel:**
> *En Rust, el fallo y la ausencia **no son sorpresas invisibles**: son **valores** que viajan en el tipo de retorno, y el compilador te obliga a decidir qué hacer con ellos.*

---

## Índice

1. [Filosofía: los errores como valores](#módulo-1-filosofía-los-errores-como-valores)
2. [`Option<T>`: un valor puede existir o no](#módulo-2-optiont-un-valor-puede-existir-o-no)
3. [`Result<T, E>`: una operación puede fallar](#módulo-3-resultt-e-una-operación-puede-fallar)
4. [Pattern matching, `if let`, `let else` y `while let`](#módulo-4-pattern-matching-if-let-let-else-y-while-let)
5. [El operador `?`](#módulo-5-el-operador-)
6. [`panic!` y cuándo evitarlo](#módulo-6-panic-y-cuándo-evitarlo)
7. [`unwrap()` y `expect()` con criterio](#módulo-7-unwrap-y-expect-con-criterio)
8. [Propagación de errores y errores personalizados](#módulo-8-propagación-de-errores-y-errores-personalizados)
9. [Código seguro: diseñar para que el error sea explícito o imposible](#módulo-9-código-seguro-diseñar-para-que-el-error-sea-explícito-o-imposible)
10. [Proyecto integrador: procesador de ventas](#proyecto-integrador-procesador-de-ventas)
11. [Ejercicios con soluciones](#ejercicios-con-soluciones)
12. [Hoja de referencia rápida](#hoja-de-referencia-rápida)
13. [Errores frecuentes del compilador](#errores-frecuentes-del-compilador)
14. [Plan de clases sugerido](#plan-de-clases-sugerido)

**Leyenda:**

- ✅ = se puede / es correcto
- ❌ = NO se puede / no compila
- ⚠️ = compila, pero cuidado (puede fallar al ejecutar o es mala práctica)

> **Nota sobre el orden:** el temario menciona `Some/None/Ok/Err` como un punto aparte; aquí van **dentro** de los módulos de `Option` y `Result`. Además, *pattern matching* va **antes** del operador `?`, porque es la herramienta básica para "abrir" estos valores.
>
> **Nota sobre el alcance:** en este nivel **no usamos closures** (funciones anónimas `|x| ...`), que se ven más adelante. Por eso no aparecen métodos como `map`, `and_then` o `unwrap_or_else`; los mencionamos solo como "lo que viene".

---

# Módulo 1: Filosofía: los errores como valores

## 1.1 ¿Cómo fallan los programas en otros lenguajes?

| Enfoque | Dónde se usa | Problema |
|---|---|---|
| **Valor `null`** | Java, C#, JavaScript, Python (`None`), C | Cualquier variable puede ser `null` y explotar al usarla (*el "error del billón de dólares"*) |
| **Excepciones** | Java, Python, JavaScript, C# | **No aparecen en la firma** de la función: no sabes qué puede fallar sin leer el código |
| **Códigos de retorno** | C (`-1`, `errno`) | Fáciles de **ignorar**; el éxito y el error se mezclan en el mismo valor |
| **(valor, error)** | Go | Se puede **ignorar** el error sin que el compilador proteste |

## 1.2 La respuesta de Rust

Rust **no tiene `null`** ni **excepciones**. En su lugar:

| Situación | Tipo | Ejemplo |
|---|---|---|
| Un valor **puede no existir** | `Option<T>` | Buscar un elemento, primer carácter de un texto vacío |
| Una operación **puede fallar** (y quieres saber por qué) | `Result<T, E>` | Convertir texto a número, abrir un archivo |
| Algo **no debería pasar jamás** (es un bug) | `panic!` | Violación de una regla interna del programa |

Los dos primeros son **recuperables** (el programa puede continuar); el tercero es **irrecuperable** (el programa se detiene).

## 1.3 El compilador te obliga a decidir

```rust
fn main() {
    // ❌ E0308: mismatched types — parse() NO devuelve un i32, devuelve un Result
    // let n: i32 = "42".parse::<i32>();

    // ✅ Tienes que "abrir" el Result: decidir qué hacer si falla
    let n: i32 = "42".parse::<i32>().unwrap_or(0);
    println!("{n}");
}
```

> **Frase para clase:** *"En Rust, la firma de la función dice la verdad: si puede fallar, el tipo lo dice, y no puedes usar el valor hasta haber manejado esa posibilidad."*

## 1.4 Comparación

```text
Python:   def parsear(t): return int(t)        → puede lanzar ValueError (no se ve en la firma)
Rust:     fn parsear(t: &str) -> Result<i32, ParseIntError>   → el fallo está en el tipo
```

---

# Módulo 2: `Option<T>`: un valor puede existir o no

## 2.1 Definición

`Option` es un **enum** de la librería estándar (ya disponible sin importar nada):

```rust
enum Option<T> {
    Some(T),     // hay un valor de tipo T
    None,        // no hay valor
}
```

- `Some(valor)` → **existe** un valor.
- `None` → **no existe**.
- `T` es un **genérico** (Nivel 2): `Option<i32>`, `Option<String>`, `Option<&str>`...

```rust
fn main() {
    let a: Option<i32> = Some(5);
    let b: Option<i32> = None;
    let c = Some("hola");              // Option<&str> inferido
    let d: Option<String> = None;      // con None hay que indicar el tipo

    // let e = None;                   // ❌ E0282: type annotations needed (¿Option de qué?)

    println!("{:?} {:?} {:?} {:?}", a, b, c, d);   // Some(5) None Some("hola") None
}
```

## 2.2 Crear funciones que devuelven `Option`

```rust
fn dividir(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None                 // no hay resultado válido
    } else {
        Some(a / b)
    }
}

fn buscar_usuario(id: u32) -> Option<&'static str> {
    match id {
        1 => Some("Ana"),
        2 => Some("Luis"),
        _ => None,
    }
}

fn main() {
    println!("{:?}", dividir(10, 2));        // Some(5)
    println!("{:?}", dividir(10, 0));        // None
    println!("{:?}", buscar_usuario(2));     // Some("Luis")
    println!("{:?}", buscar_usuario(99));    // None
}
```

**Cuándo usar `Option`:** cuando la ausencia es **normal y esperable**, y **no necesitas explicar por qué** falta (buscar en una lista, campo opcional, primer elemento de algo que puede estar vacío).

## 2.3 Funciones de la librería estándar que devuelven `Option`

Ya las usaste en niveles anteriores. Ahora sabemos qué significan:

```rust
use std::collections::HashMap;

fn main() {
    let v = vec![10, 20, 30];
    println!("{:?}", v.get(1));                  // Some(20)
    println!("{:?}", v.get(9));                  // None
    println!("{:?}", v.first());                 // Some(10)
    println!("{:?}", v.last());                  // Some(30)
    println!("{:?}", v.iter().max());            // Some(30)

    let mut pila = vec![1, 2];
    println!("{:?}", pila.pop());                // Some(2)

    println!("{:?}", "hola".find('l'));          // Some(2)   posición del carácter
    println!("{:?}", "hola".find('z'));          // None
    println!("{:?}", "clave=valor".split_once('='));   // Some(("clave", "valor"))
    println!("{:?}", "sin igual".split_once('='));     // None
    println!("{:?}", "prefijo_dato".strip_prefix("prefijo_"));   // Some("dato")

    println!("{:?}", "hola".chars().next());     // Some('h')
    println!("{:?}", "".chars().next());         // None
    println!("{:?}", '7'.to_digit(10));          // Some(7)
    println!("{:?}", 'x'.to_digit(10));          // None

    println!("{:?}", 10_i32.checked_div(0));     // None  (división segura)
    println!("{:?}", 250_u8.checked_add(10));    // None  (se desbordaría)

    let mut mapa = HashMap::new();
    mapa.insert("a", 1);
    println!("{:?}", mapa.get("a"));             // Some(1)
    println!("{:?}", mapa.get("b"));             // None
}
```

## 2.4 "Abrir" un `Option`: obtener el valor de adentro

No puedes usar un `Option<i32>` como si fuera un `i32`. Debes decidir qué hacer en **ambos** casos. Herramientas:

### a) `match` (exhaustivo)

```rust
fn main() {
    let numero: Option<i32> = Some(7);

    match numero {
        Some(n) => println!("El número es {n}"),
        None => println!("No hay número"),
    }

    // Como expresión:
    let texto = match numero {
        Some(n) if n > 5 => "grande",
        Some(_) => "pequeño",
        None => "ninguno",
    };
    println!("{texto}");
}
```

### b) `if let`: cuando solo te interesa el caso `Some`

```rust
fn main() {
    let numero: Option<i32> = Some(7);

    if let Some(n) = numero {
        println!("El número es {n}");
    }

    if let Some(n) = numero {
        println!("{n}");
    } else {
        println!("No hay número");
    }
}
```

### c) `let else`: "o me salgo" (ideal al inicio de funciones)

```rust
fn describir_primero(v: &[i32]) -> String {
    let Some(primero) = v.first() else {
        return String::from("lista vacía");       // el bloque else DEBE salir (return, panic, continue...)
    };
    format!("el primero es {primero}")
}

fn main() {
    println!("{}", describir_primero(&[5, 6]));    // el primero es 5
    println!("{}", describir_primero(&[]));        // lista vacía
}
```

### d) Valores por defecto: `unwrap_or` y `unwrap_or_default`

```rust
fn main() {
    let a: Option<i32> = Some(3);
    let b: Option<i32> = None;

    println!("{}", a.unwrap_or(0));          // 3
    println!("{}", b.unwrap_or(0));          // 0
    println!("{}", b.unwrap_or(-1));         // -1

    let nombre: Option<String> = None;
    println!("[{}]", nombre.unwrap_or_default());    // []  (String vacío: el valor por defecto de String)
    let n: Option<i32> = None;
    println!("{}", n.unwrap_or_default());           // 0   (valor por defecto de i32)
}
```

### e) Consultas: `is_some()` e `is_none()`

```rust
fn main() {
    let a = Some(5);
    println!("{} {}", a.is_some(), a.is_none());     // true false
}
```

### f) `unwrap()` y `expect()`: extraer o hacer `panic` (Módulo 7)

```rust
fn main() {
    let a = Some(5);
    println!("{}", a.unwrap());          // 5
    // let b: Option<i32> = None;
    // b.unwrap();                      // ⚠️ panic: called `Option::unwrap()` on a `None` value
}
```

### g) Combinar dos `Option`: `or`

```rust
fn main() {
    let a: Option<i32> = None;
    let b: Option<i32> = Some(9);
    println!("{:?}", a.or(b));           // Some(9)   (si a tiene valor, devuelve a; si no, b)
    println!("{:?}", Some(1).or(b));     // Some(1)
}
```

## 2.5 Lo que NO se puede hacer con un `Option`

```rust
fn main() {
    let a: Option<i32> = Some(5);

    // let s = a + 1;                 // ❌ E0369: cannot add `{integer}` to `Option<i32>`
    // if a == 5 { }                  // ❌ E0308: mismatched types (Option<i32> vs entero)
    // println!("{}", a);             // ❌ E0277: `Option<i32>` no implementa Display (usa {:?})
    // let l = Some("hola").len();    // ❌ E0599: no existe `len` en Option (está en el &str de adentro)
    // let x: i32 = a;                // ❌ E0308

    println!("{}", a == Some(5));     // ✅ Option se compara con Option
    println!("{:?}", a);              // ✅ Debug sí funciona
}
```

> **Ventaja:** el compilador te impide "olvidar" que el valor podía faltar. En otros lenguajes, esa omisión es un error en ejecución.

## 2.6 `Option` y propiedad (ownership)

Al "abrir" un `Option<String>` con `match` o `if let`, el `String` **se mueve** fuera:

```rust
fn main() {
    let apodo: Option<String> = Some(String::from("Ani"));

    // ❌ E0382: el String se movió dentro de `a`; apodo quedó parcialmente movido
    // if let Some(a) = apodo { println!("{a}"); }
    // println!("{:?}", apodo);

    // ✅ Opción 1: abrir una REFERENCIA al Option
    if let Some(a) = &apodo {
        println!("{a}");                       // a es &String
    }

    // ✅ Opción 2: as_ref() convierte &Option<T> en Option<&T>
    if let Some(a) = apodo.as_ref() {
        println!("{a}");
    }

    println!("{:?}", apodo);                   // ✅ apodo sigue siendo nuestro
}
```

### De `Option<&T>` a `Option<T>`: `copied()` y `cloned()`

```rust
fn main() {
    let v = vec![1, 2, 3];
    let referencia: Option<&i32> = v.first();       // Some(&1)
    let valor: Option<i32> = v.first().copied();    // Some(1)   (para tipos Copy)

    let nombres = vec![String::from("Ana")];
    let propio: Option<String> = nombres.first().cloned();   // Some("Ana") (con clone)

    println!("{:?} {:?} {:?}", referencia, valor, propio);
}
```

### Modificar el contenido: `&mut`

```rust
fn main() {
    let mut contador: Option<u32> = Some(1);

    if let Some(c) = &mut contador {
        *c += 1;
    }
    println!("{:?}", contador);                     // Some(2)
}
```

### `take()`: sacar el valor y dejar `None`

```rust
fn main() {
    let mut turno: Option<String> = Some(String::from("Ana"));

    let anterior = turno.take();                    // se lleva el valor y deja None
    println!("{:?} {:?}", anterior, turno);         // Some("Ana") None
}
```

## 2.7 `Option` en structs: campos opcionales

```rust
#[derive(Debug)]
struct Perfil {
    nombre: String,
    apodo: Option<String>,       // puede no tener apodo
    edad: Option<u8>,            // puede no declarar su edad
}

impl Perfil {
    fn nombre_para_mostrar(&self) -> &str {
        match &self.apodo {
            Some(a) => a.as_str(),
            None => self.nombre.as_str(),
        }
    }
}

fn main() {
    let p1 = Perfil { nombre: String::from("Ana"), apodo: Some(String::from("Ani")), edad: None };
    let p2 = Perfil { nombre: String::from("Luis"), apodo: None, edad: Some(30) };

    println!("{} / {}", p1.nombre_para_mostrar(), p2.nombre_para_mostrar());   // Ani / Luis
    println!("{:?}", p2.edad);
}
```

## 2.8 `Option` vs. valores "centinela"

Otros lenguajes usan valores especiales para "no hay dato": `-1`, `0`, `""`, `"N/A"`. **Es una trampa**, porque nada te obliga a revisarlos:

```rust
// ❌ Diseño frágil: ¿-1 significa "no encontrado" o es un índice válido en otro contexto?
fn buscar_malo(v: &[i32], objetivo: i32) -> i32 {
    for (i, x) in v.iter().enumerate() {
        if *x == objetivo {
            return i as i32;
        }
    }
    -1
}

// ✅ El tipo dice la verdad
fn buscar(v: &[i32], objetivo: i32) -> Option<usize> {
    for (i, x) in v.iter().enumerate() {
        if *x == objetivo {
            return Some(i);
        }
    }
    None
}

fn main() {
    let datos = [4, 8, 15];
    let idx = buscar_malo(&datos, 99);
    println!("{}", datos[(idx + 1) as usize]);      // ⚠️ se usó -1 "sin revisar" y salió un dato falso (4)

    match buscar(&datos, 99) {                      // ✅ aquí es imposible ignorar el None
        Some(i) => println!("en {i}"),
        None => println!("no está"),
    }
}
```

## 2.9 Aritmética segura con `Option`

Los métodos `checked_*` devuelven `None` si la operación se desbordaría o es inválida:

```rust
fn main() {
    let a: u32 = 5;
    let b: u32 = 10;

    println!("{:?}", a.checked_sub(b));        // None  (5 - 10 en u32 sería negativo)
    println!("{:?}", b.checked_sub(a));        // Some(5)
    println!("{:?}", a.checked_add(b));        // Some(15)
    println!("{:?}", u32::MAX.checked_add(1)); // None
    println!("{:?}", a.checked_mul(b));        // Some(50)
    println!("{:?}", 7_i32.checked_div(0));    // None
}
```

## 2.10 Resumen del módulo

| Quiero... | Uso |
|---|---|
| Representar "puede no haber valor" | `Option<T>` |
| Manejar ambos casos | `match` |
| Solo hacer algo si hay valor | `if let Some(x) = ...` |
| Salir de la función si no hay valor | `let Some(x) = ... else { return ... };` |
| Un valor de reserva | `unwrap_or(valor)` / `unwrap_or_default()` |
| Mirar sin mover | `&opt` o `opt.as_ref()` |
| Pasar de `Option<&T>` a `Option<T>` | `.copied()` / `.cloned()` |
| Sacar el valor dejando `None` | `.take()` |

---

# Módulo 3: `Result<T, E>`: una operación puede fallar

## 3.1 Definición

`Result` es otro enum de la librería estándar, con **dos** tipos genéricos:

```rust
enum Result<T, E> {
    Ok(T),       // éxito, con el valor de tipo T
    Err(E),      // fallo, con el error de tipo E
}
```

- `Ok(valor)` → la operación **salió bien**.
- `Err(error)` → **falló**, y el error explica **por qué**.

**`Option` vs `Result`:**

| | `Option<T>` | `Result<T, E>` |
|---|---|---|
| Pregunta que responde | ¿Hay valor? | ¿Salió bien? Si no, ¿por qué? |
| Variantes | `Some(T)` / `None` | `Ok(T)` / `Err(E)` |
| El "no" lleva información | ❌ | ✅ (`E`) |
| Ejemplo | Buscar en una lista | Convertir texto a número |

## 3.2 Crear funciones que devuelven `Result`

```rust
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("no se puede dividir entre cero"))
    } else {
        Ok(a / b)
    }
}

fn validar_edad(edad: i32) -> Result<u8, String> {
    if edad < 0 {
        Err(format!("la edad no puede ser negativa ({edad})"))
    } else if edad > 120 {
        Err(format!("la edad {edad} es demasiado alta"))
    } else {
        Ok(edad as u8)
    }
}

fn main() {
    println!("{:?}", dividir(10.0, 4.0));     // Ok(2.5)
    println!("{:?}", dividir(1.0, 0.0));      // Err("no se puede dividir entre cero")
    println!("{:?}", validar_edad(30));       // Ok(30)
    println!("{:?}", validar_edad(-5));       // Err("la edad no puede ser negativa (-5)")
}
```

Regla de oro: el **último valor** de la función debe estar envuelto: `Ok(...)` si salió bien, `Err(...)` si no. Olvidar el `Ok(...)` es el error más común:

```rust
// ❌ E0308: expected `Result<i32, String>`, found `i32`
// fn doble(n: i32) -> Result<i32, String> {
//     n * 2
// }

fn doble(n: i32) -> Result<i32, String> {
    Ok(n * 2)                                 // ✅
}

fn main() {
    println!("{:?}", doble(4));
}
```

## 3.3 Errores de la librería estándar

Muchas funciones que ya usaste devuelven `Result`:

```rust
fn main() {
    println!("{:?}", "42".parse::<i32>());        // Ok(42)
    println!("{:?}", "abc".parse::<i32>());       // Err(ParseIntError { kind: InvalidDigit })
    println!("{:?}", "3.14".parse::<f64>());      // Ok(3.14)
    println!("{:?}", "x".parse::<f64>());         // Err(ParseFloatError { kind: Invalid })
    println!("{:?}", "true".parse::<bool>());     // Ok(true)
    println!("{:?}", "a".parse::<char>());        // Ok('a')
    println!("{:?}", u8::try_from(300_i32));      // Err(TryFromIntError(()))
}
```

| Operación | Tipo de error |
|---|---|
| `"..".parse::<i32>()` (y otros enteros) | `std::num::ParseIntError` |
| `"..".parse::<f64>()` | `std::num::ParseFloatError` |
| `"..".parse::<bool>()` | `std::str::ParseBoolError` |
| `u8::try_from(...)` | `std::num::TryFromIntError` |
| Leer/escribir archivos, entrada estándar | `std::io::Error` |

## 3.4 "Abrir" un `Result`

### a) `match`

```rust
fn main() {
    let entrada = "42";

    match entrada.parse::<i32>() {
        Ok(n) => println!("Número válido: {n}"),
        Err(e) => println!("Error al convertir '{entrada}': {e}"),
    }
    // Para "abc" imprimiría: Error al convertir 'abc': invalid digit found in string
}
```

Los errores implementan `Display`, así que `{e}` muestra un mensaje legible; con `{e:?}` ves la versión técnica (`Debug`).

### b) `if let`

```rust
fn main() {
    if let Ok(n) = "42".parse::<i32>() {
        println!("Es {n}");
    }

    if let Err(e) = "abc".parse::<i32>() {
        println!("Falló: {e}");
    }
}
```

### c) Valores por defecto

```rust
fn main() {
    let a = "12x".parse::<i32>().unwrap_or(0);          // 0
    let b = "12x".parse::<i32>().unwrap_or(-1);         // -1
    let c = "12x".parse::<i32>().unwrap_or_default();   // 0
    println!("{a} {b} {c}");
}
```

⚠️ Usar un valor por defecto **descarta el error en silencio**. Está bien cuando de verdad da lo mismo (un valor opcional de configuración); es peligroso cuando el error importa (dinero, datos de usuario).

### d) Consultas y conversión a `Option`

```rust
fn main() {
    let r: Result<i32, _> = "12".parse::<i32>();
    println!("{} {}", r.is_ok(), r.is_err());          // true false

    let valor: Option<i32> = "12".parse::<i32>().ok();    // Ok(12) → Some(12)
    let falla: Option<i32> = "zz".parse::<i32>().ok();    // Err(_) → None  (se pierde el motivo)
    let motivo = "zz".parse::<i32>().err();               // Some(ParseIntError)
    println!("{:?} {:?} {}", valor, falla, motivo.is_some());
}
```

## 3.5 Distinguir **qué** error ocurrió

El error es un valor: puedes inspeccionarlo con `match`.

```rust
use std::num::IntErrorKind;

fn explicar(texto: &str) {
    match texto.parse::<u8>() {
        Ok(n) => println!("'{texto}' → {n}"),
        Err(e) => match e.kind() {
            IntErrorKind::Empty => println!("'{texto}': está vacío"),
            IntErrorKind::InvalidDigit => println!("'{texto}': tiene caracteres no numéricos"),
            IntErrorKind::PosOverflow => println!("'{texto}': es demasiado grande para u8"),
            _ => println!("'{texto}': otro problema ({e})"),
        },
    }
}

fn main() {
    explicar("42");
    explicar("");
    explicar("4x");
    explicar("300");
    explicar("-1");
}
```

Salida:

```
'42' → 42
'': está vacío
'4x': tiene caracteres no numéricos
'300': es demasiado grande para u8
'-1': tiene caracteres no numéricos
```

(El `_` final es obligatorio: `IntErrorKind` puede ganar variantes en el futuro.)

⚠️ `parse` **no** ignora espacios: `" 42".parse::<i32>()` falla. Usa `.trim()` antes.

## 3.6 Errores de entrada/salida (`std::io::Error`)

Leer un archivo puede fallar por muchas razones (no existe, sin permisos...). El tipo de error permite distinguirlas:

```rust
use std::fs;
use std::io::ErrorKind;

fn main() {
    match fs::read_to_string("no_existe.txt") {
        Ok(contenido) => println!("{} bytes leídos", contenido.len()),
        Err(e) if e.kind() == ErrorKind::NotFound => println!("el archivo no existe"),
        Err(e) => println!("otro error de E/S: {e}"),
    }
}
```

Leer una línea del teclado también devuelve `Result`:

```rust
use std::io;

fn main() {
    println!("Escribe tu edad:");
    let mut entrada = String::new();

    match io::stdin().read_line(&mut entrada) {
        Ok(_) => match entrada.trim().parse::<u8>() {
            Ok(edad) => println!("Tienes {edad} años"),
            Err(_) => println!("Eso no es una edad válida"),
        },
        Err(e) => println!("No pude leer la entrada: {e}"),
    }
}
```

## 3.7 `Result` no se puede ignorar sin que Rust avise

`Result` está marcado como `#[must_use]`: si lo descartas, el compilador emite una **advertencia**.

```rust
fn main() {
    // "abc".parse::<i32>();            // ⚠️ warning: unused `Result` that must be used
    let _ = "abc".parse::<i32>();       // ✅ ignorar A PROPÓSITO (se nota en el código)
}
```

## 3.8 `Result<(), E>`: acciones que no devuelven nada útil

```rust
fn guardar_nombre(nombre: &str) -> Result<(), String> {
    if nombre.is_empty() {
        return Err(String::from("el nombre no puede estar vacío"));
    }
    println!("Guardado: {nombre}");
    Ok(())                              // éxito sin valor: `()`
}

fn main() {
    match guardar_nombre("Ana") {
        Ok(()) => println!("todo bien"),
        Err(e) => println!("error: {e}"),
    }
    if let Err(e) = guardar_nombre("") {
        println!("error: {e}");
    }
}
```

## 3.9 Convertir entre `Option` y `Result`

```rust
fn buscar(id: u32) -> Option<&'static str> {
    match id {
        1 => Some("Ana"),
        _ => None,
    }
}

fn main() {
    // Option → Result: ok_or(error)   (le das el error para el caso None)
    let r1: Result<&str, String> = buscar(1).ok_or(String::from("usuario no encontrado"));
    let r2: Result<&str, String> = buscar(9).ok_or(String::from("usuario no encontrado"));
    println!("{:?} {:?}", r1, r2);       // Ok("Ana") Err("usuario no encontrado")

    // Result → Option: ok()   (se descarta el motivo del error)
    let o: Option<i32> = "5".parse::<i32>().ok();
    println!("{:?}", o);                 // Some(5)
}
```

## 3.10 Recolectar resultados: estricto vs. tolerante

Con varios elementos, hay dos filosofías:

```rust
// ESTRICTO: se detiene en el primer error
fn parsear_todos_estricto(textos: &[&str]) -> Result<Vec<i32>, String> {
    let mut numeros = Vec::new();
    for t in textos {
        match t.trim().parse::<i32>() {
            Ok(n) => numeros.push(n),
            Err(_) => return Err(format!("'{t}' no es un número")),
        }
    }
    Ok(numeros)
}

// TOLERANTE: recoge lo bueno y lo malo por separado
fn parsear_todos_tolerante(textos: &[&str]) -> (Vec<i32>, Vec<String>) {
    let mut buenos = Vec::new();
    let mut malos = Vec::new();
    for t in textos {
        match t.trim().parse::<i32>() {
            Ok(n) => buenos.push(n),
            Err(_) => malos.push(format!("'{t}' no es un número")),
        }
    }
    (buenos, malos)
}

fn main() {
    let datos = ["1", "2", "x", "4"];
    println!("{:?}", parsear_todos_estricto(&datos));      // Err("'x' no es un número")
    println!("{:?}", parsear_todos_tolerante(&datos));     // ([1, 2, 4], ["'x' no es un número"])
}
```

> **Cuándo cada uno:** estricto para datos que deben ser **todos** correctos (una transacción); tolerante para importaciones o lecturas donde conviene **reportar** los problemas y seguir.

## 3.11 Lo que NO se puede hacer con un `Result`

```rust
fn main() {
    let r: Result<i32, String> = Ok(5);

    // let s = r + 1;                    // ❌ E0369: no se suma un Result
    // let x: i32 = r;                   // ❌ E0308: mismatched types
    // println!("{}", r);                // ❌ E0277: Result no implementa Display
    // let l = "5".parse::<i32>().abs(); // ❌ E0599: `abs` no existe en Result

    println!("{:?}", r);                 // ✅ con {:?}
}
```

## 3.12 Resumen del módulo

| Quiero... | Uso |
|---|---|
| Función que puede fallar con motivo | `-> Result<T, E>` |
| Devolver éxito / fallo | `Ok(valor)` / `Err(error)` |
| Manejar ambos casos | `match` |
| Solo reaccionar al éxito o solo al fallo | `if let Ok(x) = ...` / `if let Err(e) = ...` |
| Valor de reserva (descartando el error) | `unwrap_or(v)` / `unwrap_or_default()` |
| `Option` → `Result` | `.ok_or(error)` |
| `Result` → `Option` | `.ok()` |

---

# Módulo 4: Pattern matching, `if let`, `let else` y `while let`

El **pattern matching** (coincidencia de patrones) es la herramienta con la que "desarmas" valores como `Option`, `Result`, enums, structs, tuplas y slices. Ya usaste `match`; aquí ves todas sus formas.

## 4.1 Refutable vs. irrefutable

- Un patrón **irrefutable** **siempre** coincide (`let (a, b) = (1, 2);`).
- Un patrón **refutable** **puede no** coincidir (`Some(x)`, `Ok(n)`, `3`).

| Dónde se usa | ¿Qué patrones admite? |
|---|---|
| `let x = ...;`, parámetros de función, `for` | **Solo irrefutables** |
| `match`, `if let`, `while let`, `let else` | Refutables (el compilador exige cubrir lo que falte) |

```rust
fn main() {
    let (a, b) = (1, 2);                  // ✅ irrefutable: siempre coincide
    println!("{a} {b}");

    let opcion = Some(5);
    // let Some(x) = opcion;              // ❌ E0005: refutable pattern in local binding
    //                                    //   (¿y si fuera None?)
    let Some(x) = opcion else { return; };    // ✅ let else: define qué pasa si NO coincide
    println!("{x}");
}
```

## 4.2 Catálogo de patrones

```rust
fn main() {
    let n = 7;

    // 1) Literales, múltiples valores y rangos
    match n {
        0 => println!("cero"),
        1 | 2 | 3 => println!("pequeño"),
        4..=9 => println!("un dígito"),
        _ => println!("grande"),
    }

    // 2) Guardas (if dentro de la rama)
    match n {
        x if x % 2 == 0 => println!("{x} es par"),
        x => println!("{x} es impar"),
    }

    // 3) Capturar con @ (comprobar un rango Y guardar el valor)
    match n {
        edad @ 0..=17 => println!("menor: {edad}"),
        edad @ 18..=64 => println!("adulto: {edad}"),
        edad => println!("mayor: {edad}"),
    }

    // 4) Tuplas
    let par = (3, -3);
    match par {
        (0, 0) => println!("origen"),
        (x, 0) | (0, x) => println!("sobre un eje: {x}"),
        (x, y) if x == -y => println!("opuestos"),
        _ => println!("otro"),
    }
}
```

## 4.3 Desestructurar structs y enums

```rust
#[derive(Debug)]
struct Punto {
    x: i32,
    y: i32,
}

enum Figura {
    Circulo { centro: Punto, radio: u32 },
    Rectangulo(Punto, Punto),
    Vacia,
}

fn describir(f: &Figura) -> String {
    match f {
        Figura::Circulo { centro: Punto { x: 0, y: 0 }, radio } => {
            format!("círculo en el origen, radio {radio}")
        }
        Figura::Circulo { centro, radio } => {
            format!("círculo en ({}, {}), radio {radio}", centro.x, centro.y)
        }
        Figura::Rectangulo(a, b) => format!("rectángulo de {:?} a {:?}", a, b),
        Figura::Vacia => String::from("figura vacía"),
    }
}

fn main() {
    let c = Figura::Circulo { centro: Punto { x: 0, y: 0 }, radio: 5 };
    let r = Figura::Rectangulo(Punto { x: 1, y: 1 }, Punto { x: 4, y: 3 });
    println!("{}", describir(&c));
    println!("{}", describir(&r));
    println!("{}", describir(&Figura::Vacia));

    // Desestructurar un struct en un let (irrefutable):
    let Punto { x, y } = Punto { x: 7, y: 9 };
    println!("{x} {y}");
}
```

## 4.4 Patrones **anidados**: `Option` y `Result` combinados

```rust
fn clasificar(r: Result<Option<i32>, String>) -> String {
    match r {
        Ok(Some(n)) if n < 0 => format!("negativo: {n}"),
        Ok(Some(n)) => format!("valor: {n}"),
        Ok(None) => String::from("sin valor"),
        Err(e) => format!("error: {e}"),
    }
}

fn main() {
    println!("{}", clasificar(Ok(Some(-3))));         // negativo: -3
    println!("{}", clasificar(Ok(Some(8))));          // valor: 8
    println!("{}", clasificar(Ok(None)));             // sin valor
    println!("{}", clasificar(Err(String::from("x")))); // error: x
}
```

Combinando dos `Option` a la vez:

```rust
fn sumar_opcionales(a: Option<i32>, b: Option<i32>) -> i32 {
    match (a, b) {
        (Some(x), Some(y)) => x + y,
        (Some(x), None) | (None, Some(x)) => x,
        (None, None) => 0,
    }
}

fn main() {
    println!("{}", sumar_opcionales(Some(2), Some(3)));   // 5
    println!("{}", sumar_opcionales(None, Some(7)));      // 7
    println!("{}", sumar_opcionales(None, None));         // 0
}
```

## 4.5 Patrones sobre referencias y `&`

```rust
fn main() {
    let v = vec![5, 6, 7];

    // v.first() es Option<&i32>. Con `&x` en el patrón obtienes el i32 directamente:
    if let Some(&primero) = v.first() {
        println!("{}", primero + 1);                  // 6
    }

    // Al hacer match sobre una referencia, los bindings pasan a ser referencias:
    let opcion: Option<String> = Some(String::from("hola"));
    match &opcion {
        Some(texto) => println!("{texto} ({} letras)", texto.len()),   // texto: &String
        None => println!("nada"),
    }
    println!("{:?}", opcion);                         // ✅ no se movió nada
}
```

## 4.6 Patrones sobre slices (secuencias)

```rust
fn describir(v: &[i32]) -> String {
    match v {
        [] => String::from("vacía"),
        [x] => format!("un solo elemento: {x}"),
        [primero, .., ultimo] => format!("de {primero} a {ultimo}"),
    }
}

fn suma(v: &[i32]) -> i32 {
    match v {
        [] => 0,
        [cabeza, resto @ ..] => cabeza + suma(resto),    // `resto` es el slice con lo que queda
    }
}

fn main() {
    println!("{}", describir(&[]));                // vacía
    println!("{}", describir(&[7]));               // un solo elemento: 7
    println!("{}", describir(&[1, 2, 3, 4]));      // de 1 a 4
    println!("{}", suma(&[1, 2, 3, 4]));           // 10
}
```

Es una forma **segura** de acceder a los extremos de una lista: sin índices, sin `panic`.

## 4.7 `if let` y `else if let`

```rust
fn main() {
    let a: Option<i32> = None;
    let b: Result<i32, String> = Ok(3);

    if let Some(n) = a {
        println!("a = {n}");
    } else if let Ok(m) = b {
        println!("b = {m}");
    } else {
        println!("nada de nada");
    }
}
```

### `if let` vs `match`

| Situación | Usa |
|---|---|
| Me importa **un solo** caso y el resto se ignora | `if let` |
| Quiero manejar **todos** los casos explícitamente | `match` (el compilador verifica que no falte ninguno) |
| Salir temprano si algo no está | `let else` |

⚠️ `if let` **no verifica exhaustividad**: si luego agregas una variante nueva a un enum, un `if let` seguirá compilando sin avisarte, mientras que un `match` te marcaría el error. Para enums propios que van a crecer, prefiere `match`.

## 4.8 `let else`: el "guard clause" de Rust

Ideal para **validar al inicio** de una función y mantener el camino feliz sin anidar:

```rust
fn parsear_par(texto: &str) -> Option<(i32, i32)> {
    let Some((izq, der)) = texto.split_once(',') else {
        return None;
    };
    let Ok(a) = izq.trim().parse::<i32>() else {
        return None;
    };
    let Ok(b) = der.trim().parse::<i32>() else {
        return None;
    };
    Some((a, b))
}

fn main() {
    println!("{:?}", parsear_par("3, 4"));       // Some((3, 4))
    println!("{:?}", parsear_par("3-4"));        // None
    println!("{:?}", parsear_par("3, x"));       // None
}
```

Reglas del `let else`: el bloque `else` **debe divergir** (terminar con `return`, `continue`, `break` o `panic!`); las variables del patrón **quedan disponibles** después.

## 4.9 `while let`: repetir mientras haya valor

```rust
fn main() {
    let mut pila = vec![1, 2, 3];

    while let Some(tope) = pila.pop() {      // pop() devuelve None cuando se vacía
        println!("sacando {tope}");
    }
    println!("pila vacía: {}", pila.is_empty());
}
```

Salida: `sacando 3`, `sacando 2`, `sacando 1`, `pila vacía: true`.

## 4.10 `matches!`: "¿coincide con este patrón?" → `bool`

```rust
fn main() {
    let r: Result<i32, String> = Err(String::from("x"));

    println!("{}", matches!(r, Err(_)));                  // true
    println!("{}", matches!(Some(5), Some(n) if n > 3));  // true (admite guardas)
    println!("{}", matches!('k', 'a'..='z'));             // true
}
```

## 4.11 Exhaustividad: el compilador es tu red de seguridad

```rust
// ❌ E0004: non-exhaustive patterns: `None` not covered
// fn f(o: Option<i32>) -> i32 {
//     match o {
//         Some(n) => n,
//     }
// }
```

> Es **imposible olvidar el caso `None`** o el caso `Err`. Esa es la garantía más importante de este nivel.

---

# Módulo 5: El operador `?`

## 5.1 El problema: demasiados `match` anidados

```rust
use std::num::ParseIntError;

fn sumar_textos(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = match a.trim().parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let y = match b.trim().parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    Ok(x + y)
}
```

Funciona, pero es **repetitivo**. El operador `?` hace exactamente lo mismo en **un carácter**.

## 5.2 `?` con `Result`

```rust
use std::num::ParseIntError;

fn sumar_textos(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.trim().parse::<i32>()?;
    let y = b.trim().parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    println!("{:?}", sumar_textos("2", "3"));        // Ok(5)
    println!("{:?}", sumar_textos("2", "tres"));     // Err(ParseIntError { kind: InvalidDigit })
}
```

**Qué hace `expresión?`:**

- Si es `Ok(valor)` → **extrae** `valor` y la ejecución sigue.
- Si es `Err(e)` → **sale de la función inmediatamente** devolviendo `Err(e)` (convirtiéndolo con `From` si hace falta, ver 5.6).

Equivale a:

```rust
let x = match a.trim().parse::<i32>() {
    Ok(valor) => valor,
    Err(e) => return Err(From::from(e)),
};
```

## 5.3 Encadenar con `?`

```rust
use std::num::ParseIntError;

fn promedio_de_tres(a: &str, b: &str, c: &str) -> Result<f64, ParseIntError> {
    let x: i32 = a.trim().parse()?;
    let y: i32 = b.trim().parse()?;
    let z: i32 = c.trim().parse()?;
    Ok((x + y + z) as f64 / 3.0)
}

fn main() {
    println!("{:?}", promedio_de_tres("3", "6", "9"));       // Ok(6.0)
    println!("{:?}", promedio_de_tres("3", "?", "9"));       // Err(...) en la segunda
}
```

Si **cualquier** paso falla, la función termina ahí y entrega ese error a quien la llamó: eso es **propagar** el error.

## 5.4 `?` con `Option`

En una función que devuelve `Option`, `?` sale con `None` si encuentra `None`:

```rust
fn inicial_y_final(texto: &str) -> Option<(char, char)> {
    let inicial = texto.chars().next()?;      // si el texto está vacío → return None
    let final_ = texto.chars().last()?;
    Some((inicial, final_))
}

fn primera_palabra_mayuscula(texto: &str) -> Option<String> {
    let palabra = texto.split_whitespace().next()?;
    Some(palabra.to_uppercase())
}

fn main() {
    println!("{:?}", inicial_y_final("rust"));            // Some(('r', 't'))
    println!("{:?}", inicial_y_final(""));                // None
    println!("{:?}", primera_palabra_mayuscula("hola mundo"));   // Some("HOLA")
    println!("{:?}", primera_palabra_mayuscula("   "));          // None
}
```

## 5.5 Dónde SÍ y dónde NO se puede usar `?`

```rust
// ❌ E0277: the `?` operator can only be used in a function that returns `Result` or `Option`
// fn main() {
//     let n: i32 = "5".parse()?;
// }

// ✅ main puede devolver Result:
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let n: i32 = "42".parse()?;
    println!("{n}");

    let m: i32 = "x".parse()?;      // falla: main devuelve el Err
    println!("{m}");                // (no se llega aquí)

    Ok(())
}
```

Cuando `main` devuelve `Err(e)`, el programa imprime `Error: ` seguido del `Debug` del error y termina con **código de salida 1**. Para `main`, el tipo de error debe implementar `Debug`.

| Contexto | ¿`?` permitido? |
|---|---|
| Función que devuelve `Result<_, E>` | ✅ con expresiones `Result` (y `Option` convertidos con `ok_or`) |
| Función que devuelve `Option<_>` | ✅ con expresiones `Option` |
| `main() -> Result<(), E>` | ✅ |
| Función que devuelve `()`, `i32`, `String`... | ❌ |

## 5.6 Mezclar `Option` y `Result`

`?` **no** mezcla los dos mundos automáticamente: hay que convertir.

```rust
// ❌ E0277: `?` aplicado a un Option dentro de una función que devuelve Result
// fn primera_letra(texto: &str) -> Result<char, String> {
//     let c = texto.chars().next()?;
//     Ok(c)
// }

// ✅ Option → Result con ok_or
fn primera_letra(texto: &str) -> Result<char, String> {
    let c = texto.chars().next().ok_or(String::from("el texto está vacío"))?;
    Ok(c)
}

// ✅ Result → Option con ok()
fn doble(texto: &str) -> Option<i32> {
    let n = texto.trim().parse::<i32>().ok()?;
    Some(n * 2)
}

fn main() {
    println!("{:?}", primera_letra("hola"));    // Ok('h')
    println!("{:?}", primera_letra(""));        // Err("el texto está vacío")
    println!("{:?}", doble("21"));              // Some(42)
    println!("{:?}", doble("x"));               // None
}
```

## 5.7 `?` y la conversión de errores (`From`)

Cuando el error que produce la expresión **no es del mismo tipo** que el error que devuelve tu función, `?` intenta convertirlo con el trait `From`. Si no existe esa conversión, no compila:

```rust
// ❌ E0277: `?` couldn't convert the error to `String`
//    (no existe From<ParseIntError> para String)
// fn leer(texto: &str) -> Result<i32, String> {
//     let n: i32 = texto.parse()?;
//     Ok(n)
// }
```

Soluciones: manejar con `match` (Módulo 3), o definir **tu propio tipo de error** con `From` (Módulo 8), que es la forma recomendada.

## 5.8 Errores frecuentes con `?`

| Error | Causa | Solución |
|---|---|---|
| E0277 "`?` can only be used in a function that returns `Result` or `Option`" | Usar `?` en `main()` (sin tipo de retorno) o en una función que devuelve otro tipo | Cambiar el retorno a `Result`, o usar `match` |
| E0277 "`?` couldn't convert the error to `X`" | No existe `From<ErrorOrigen> for X` | Implementar `From`, o cambiar el tipo de error |
| E0277 al usar `?` sobre `Option` en función que devuelve `Result` (o al revés) | Mezcla de tipos | `.ok_or(...)` / `.ok()` |
| E0308 "expected `Result`, found ..." | Olvidar `Ok(...)` al final | Envolver el valor final en `Ok(...)` |

## 5.9 Resumen del módulo

1. `?` = "si va bien, sigue; si va mal, **sal de aquí y devuelve el error**".
2. Funciona con `Result` (en funciones que devuelven `Result`) y con `Option` (en funciones que devuelven `Option`).
3. Convierte el error con `From` si hace falta.
4. Es la forma estándar de **propagar** errores hacia quien sabe qué hacer con ellos.

---

# Módulo 6: `panic!` y cuándo evitarlo

## 6.1 Qué es un `panic`

Un **panic** es un error **irrecuperable**: el programa **se detiene** (el hilo actual termina), imprime un mensaje y sale con **código 101**.

```rust
fn main() {
    println!("antes");
    panic!("algo salió muy mal: código {}", 42);
    // println!("después");        // ⚠️ nunca se ejecuta (el compilador avisa: unreachable)
}
```

Salida (por stderr):

```
antes
thread 'main' panicked at src/main.rs:3:5:
algo salió muy mal: código 42
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- El mensaje indica **archivo, línea y columna**.
- `RUST_BACKTRACE=1 cargo run` muestra la **pila de llamadas** que llevó al fallo.

### Los valores se liberan al "desenrollar"

Cuando hay un panic, Rust **desenrolla** la pila y ejecuta los `drop` de las variables vivas (lo que vimos en el Nivel 3):

```rust
struct Aviso(String);

impl Drop for Aviso {
    fn drop(&mut self) {
        println!("   [drop] liberando {}", self.0);
    }
}

fn main() {
    let _a = Aviso(String::from("A"));
    panic!("boom");
}
// Salida: el mensaje de panic y luego "[drop] liberando A"
```

## 6.2 ¿Qué provoca un panic?

| Causa | Ejemplo | Mensaje típico |
|---|---|---|
| `panic!` explícito | `panic!("mensaje")` | tu mensaje |
| `unwrap()` sobre `None` | `None::<i32>.unwrap()` | called `Option::unwrap()` on a `None` value |
| `unwrap()` sobre `Err` | `"x".parse::<i32>().unwrap()` | called `Result::unwrap()` on an `Err` value: ... |
| `expect("msg")` fallido | `None::<i32>.expect("falta")` | falta |
| Índice fuera de rango | `v[10]` | index out of bounds: the len is 3 but the index is 10 |
| División entera entre cero | `a / 0` | attempt to divide by zero |
| Desbordamiento aritmético (modo debug) | `255_u8 + 1` | attempt to add with overflow |
| Slice fuera de rango o en medio de una letra | `&s[0..1]` en `"ñ"` | byte index 1 is not a char boundary |
| `assert!` / `assert_eq!` falso | `assert!(x > 0)` | assertion failed: x > 0 |
| `unreachable!()` | rama "imposible" | internal error: entered unreachable code |
| `todo!()` / `unimplemented!()` | código pendiente | not yet implemented |

## 6.3 Macros relacionadas

```rust
fn main() {
    let x = 5;

    // assert!: verifica una condición; si es falsa, panic (útil para invariantes)
    assert!(x > 0);
    assert!(x > 0, "x debe ser positivo, llegó {x}");

    // assert_eq! / assert_ne!: comparan y muestran ambos valores si fallan
    assert_eq!(x, 5);
    assert_ne!(x, 6);

    // debug_assert!: igual que assert!, pero SOLO en modo debug (no en release)
    debug_assert!(x < 1000);
}

// todo!() y unimplemented!(): marcan código pendiente y compilan
fn funcion_pendiente() -> i32 {
    todo!("falta implementar el cálculo")
}

// unreachable!(): "este camino no puede ocurrir" (si ocurre, hay un bug)
fn nombre_de_dia(n: u8) -> &'static str {
    match n {
        1 => "lunes",
        2 => "martes",
        3 => "miércoles",
        _ => unreachable!("n ya fue validado antes: {n}"),
    }
}
```

## 6.4 La pregunta clave: ¿`Result` o `panic!`?

> **Si el fallo puede ocurrir en un programa correcto → devuelve `Result` u `Option`.**
> **Si el fallo solo puede ocurrir por un bug del programador → `panic!` es aceptable.**

| Situación | ¿Puede pasar en un programa correcto? | Qué hacer |
|---|---|---|
| El usuario escribe "abc" donde se espera un número | ✅ Sí | `Result` |
| El archivo no existe | ✅ Sí | `Result` |
| La red se cae, el servidor no responde | ✅ Sí | `Result` |
| Buscar una clave que quizá no esté en el mapa | ✅ Sí | `Option` |
| Dividir entre un valor que viene de fuera | ✅ Sí | `Option`/`Result` (o validar antes) |
| Una función interna recibe un valor que **ya validaste** y que viola su contrato | ❌ No (es un bug) | `panic!` / `assert!` aceptable |
| Un `match` con una rama que **por construcción** es imposible | ❌ No | `unreachable!` |
| Prototipo rápido, ejemplo, test | (da igual) | `unwrap` / `expect` / `panic!` |

### ✅ Cuándo `panic!` es razonable

1. **Bugs**: se violó una condición interna que "nunca debería pasar". Continuar con datos corruptos sería peor.
2. **Tests**: un `assert!` que falla *es* la forma de reportar un fallo.
3. **Prototipos y ejemplos**: se simplifica el código; luego se reemplaza.
4. **Estado imposible de recuperar** (el programa no puede funcionar sin ese recurso y no hay alternativa razonable).

### ❌ Cuándo evitarlo

1. **Cualquier dato que venga de fuera** del programa: teclado, archivos, red, variables de entorno, argumentos.
2. **Código de biblioteca** (el que otros usan): quien llama debe decidir cómo reaccionar, no tú.
3. **Operaciones que fallan "normalmente"** (parseos, búsquedas, validaciones).
4. **Programas de larga ejecución** (servidores): un panic por una entrada mala **tumba el servicio**.

## 6.5 Panics ocultos y su alternativa segura

Muchos panics vienen de operaciones que parecen inocentes:

| Operación que puede hacer panic | Alternativa segura |
|---|---|
| `v[i]` | `v.get(i)` → `Option` |
| `&v[a..b]` | `v.get(a..b)` → `Option` |
| `&texto[0..n]` | `texto.get(0..n)` → `Option` |
| `a / b` (enteros) | `a.checked_div(b)` → `Option` |
| `a + b`, `a - b`, `a * b` (desbordamiento) | `a.checked_add(b)` / `checked_sub` / `checked_mul` → `Option`; o `saturating_*` / `wrapping_*` |
| `x.parse::<T>().unwrap()` | `match` / `?` / `unwrap_or` |
| `opt.unwrap()` | `match` / `if let` / `let else` / `?` |
| `mapa["clave"]` | `mapa.get("clave")` → `Option` |
| `v.remove(i)` (índice inexistente) | Comprobar `i < v.len()` antes, o usar `get` |
| `numero as u8` (pierde datos, **no** hace panic) | `u8::try_from(numero)` → `Result` |

```rust
fn main() {
    let v = vec![10, 20, 30];
    let i = 7;

    // ⚠️ let x = v[i];                       // panic
    match v.get(i) {
        Some(x) => println!("{x}"),
        None => println!("posición {i} inexistente"),
    }

    let a: u32 = 5;
    let b: u32 = 10;
    // ⚠️ let r = a - b;                      // panic en debug (overflow)
    println!("{:?}", a.checked_sub(b));       // None
    println!("{}", a.saturating_sub(b));      // 0   (se queda en el mínimo)
    println!("{}", a.wrapping_sub(b));        // 4294967291 (da la vuelta)
}
```

## 6.6 Documentar los panics

Si una función **puede** hacer panic por contrato, dilo en su documentación con la sección `# Panics`:

```rust
/// Devuelve el elemento del medio.
///
/// # Panics
/// Hace panic si `datos` está vacío.
fn medio(datos: &[i32]) -> i32 {
    assert!(!datos.is_empty(), "medio() exige una lista no vacía");
    datos[datos.len() / 2]
}

// Versión sin panic:
fn medio_seguro(datos: &[i32]) -> Option<i32> {
    datos.get(datos.len() / 2).copied()
}

fn main() {
    println!("{}", medio(&[1, 2, 3]));
    println!("{:?}", medio_seguro(&[]));      // None
}
```

## 6.7 Probar que algo hace panic

```rust
fn dividir(a: i32, b: i32) -> i32 {
    a / b                                  // ⚠️ hace panic si b == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn dividir_entre_cero_hace_panic() {
        dividir(10, 0);
    }
}
```

## 6.8 Otras opciones de configuración (solo referencia)

- Por defecto, un panic **desenrolla** la pila (limpia los valores).
- En `Cargo.toml` puedes pedir que **aborte** directamente (binario más pequeño, sin limpieza):

```toml
[profile.release]
panic = "abort"
```

## 6.9 Resumen del módulo

1. `panic!` = fallo **irrecuperable**: el programa se detiene.
2. Reserva el `panic` para **bugs** (violaciones de invariantes), tests y prototipos.
3. Para todo lo que pueda pasar con entradas reales: **`Result` / `Option`**.
4. Aprende los **panics ocultos** (`v[i]`, `/`, `unwrap`, desbordamientos) y sus alternativas seguras.

---

# Módulo 7: `unwrap()` y `expect()` con criterio

## 7.1 Qué hacen

Ambos **extraen** el valor de un `Option` o `Result` y, si no hay valor (o es `Err`), hacen **`panic!`**.

```rust
fn main() {
    let a: Option<i32> = Some(5);
    let b: Result<i32, String> = Ok(7);

    println!("{}", a.unwrap());                    // 5
    println!("{}", b.unwrap());                    // 7
    println!("{}", a.expect("a debería tener valor"));   // 5
}
```

| Método | Si hay valor | Si falla |
|---|---|---|
| `unwrap()` | devuelve el valor | panic con mensaje **genérico** |
| `expect("msg")` | devuelve el valor | panic con **tu mensaje** |

```rust
fn main() {
    // let n: Option<i32> = None;
    // n.unwrap();                // panic: called `Option::unwrap()` on a `None` value
    // n.expect("falta el valor de n");     // panic: falta el valor de n

    // let r: Result<i32, _> = "x".parse::<i32>();
    // r.unwrap();                // panic: called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
    // r.expect("no era número"); // panic: no era número: ParseIntError { kind: InvalidDigit }
}
```

> **`expect` es casi siempre mejor que `unwrap`**: el mensaje deja pistas de **qué suposición falló**.

## 7.2 ¿Cuándo es aceptable usarlos?

### ✅ 1. En tests, ejemplos y prototipos

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn parsea_bien() {
        let n: i32 = "42".parse().unwrap();
        assert_eq!(n, 42);
    }
}
```

### ✅ 2. Cuando el fallo es **imposible por construcción** (con `expect` que explique por qué)

```rust
fn main() {
    // Literal escrito por nosotros: es un número válido SIEMPRE
    let iva: f64 = "0.19".parse().expect("IVA es un literal numérico válido");

    // 5 siempre cabe en u32
    let n = u32::try_from(5_usize).expect("5 cabe en u32");

    println!("{iva} {n}");
}
```

### ✅ 3. Cuando una condición garantiza el valor **y** no hay mejor forma

Y aun así, normalmente hay una forma mejor (ver 7.4).

## 7.3 ¿Cuándo NO usarlos?

### ❌ Con datos externos (siempre pueden fallar)

```rust
use std::fs;
use std::io;

fn main() {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();         // ❌ ¿y si falla la lectura?
    let n: i32 = entrada.trim().parse().unwrap();         // ❌ el usuario puede escribir "hola"
    let datos = fs::read_to_string("datos.txt").unwrap(); // ❌ el archivo puede no existir
    println!("{n} {datos}");
}
```

### ❌ En código de biblioteca, en servidores, o donde "se cae" no es opción

## 7.4 Las alternativas, en orden de preferencia

| Alternativa | Cuándo |
|---|---|
| **`?`** | Quien me llamó sabe qué hacer con el error (lo más común) |
| **`match` / `if let`** | Quiero reaccionar aquí mismo de distintas maneras |
| **`let else`** | Si falta, me salgo de la función |
| **`unwrap_or(valor)` / `unwrap_or_default()`** | Hay un valor de reserva razonable |
| **`expect("por qué no puede fallar")`** | Es un invariante que se puede demostrar |
| **`unwrap()`** | Solo en tests y prototipos |

## 7.5 Refactorizar: de `unwrap` a código robusto

```rust
use std::num::ParseIntError;

// ❌ Versión frágil: un texto malo tumba todo el programa
fn suma_fragil(a: &str, b: &str) -> i32 {
    a.parse::<i32>().unwrap() + b.parse::<i32>().unwrap()
}

// ✅ Versión robusta: el error viaja hacia quien llama
fn suma_robusta(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.trim().parse::<i32>()?;
    let y = b.trim().parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    println!("{}", suma_fragil("2", "3"));                 // 5
    // println!("{}", suma_fragil("2", "x"));              // ⚠️ panic

    match suma_robusta("2", "x") {
        Ok(n) => println!("{n}"),
        Err(e) => println!("entrada inválida: {e}"),
    }
}
```

## 7.6 Antipatrones

```rust
fn main() {
    let v = vec![1, 2, 3];

    // ❌ Comprobar y luego unwrap (hace el trabajo dos veces y es frágil si cambia el código)
    if v.first().is_some() {
        println!("{}", v.first().unwrap());
    }
    // ✅ if let
    if let Some(primero) = v.first() {
        println!("{primero}");
    }

    // ❌ expect con mensaje inútil
    // let n: i32 = "x".parse().expect("");
    // ❌ expect que describe el síntoma, no la suposición
    // let n: i32 = "x".parse().expect("falló");
    // ✅ expect que explica la SUPOSICIÓN
    let n: i32 = "5".parse().expect("la constante '5' siempre es un entero válido");
    println!("{n}");
}
```

Estilo de los mensajes de `expect`: describe **por qué crees que no puede fallar**, no solo que falló.

## 7.7 Una nota sobre `unwrap_or(...)` y valores costosos

`unwrap_or(valor)` **evalúa `valor` siempre**, aunque no haga falta. Si calcular ese valor por defecto es costoso, existe `unwrap_or_else`, que usa una función y se aprende junto con los *closures* (siguiente nivel de la ruta).

## 7.8 Herramienta: Clippy puede vigilar

```rust
// Al inicio de main.rs, para recibir advertencias de `cargo clippy`:
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
```

Útil en proyectos donde quieres controlar que no queden `unwrap` olvidados.

## 7.9 Diagrama de decisión

```
Tengo un Option/Result y necesito el valor
          │
          ▼
¿Estoy en un test, ejemplo o prototipo?
   ├─ Sí → unwrap() / expect()
   └─ No
       ▼
¿El dato viene de fuera (usuario, archivo, red...)?
   ├─ Sí → NO unwrap. Usa  ?  /  match  /  if let  /  let else
   └─ No
       ▼
¿Puedo DEMOSTRAR que nunca falla?
   ├─ Sí → expect("por qué no puede fallar")
   └─ No → trátalo como "puede fallar": ?  /  match  /  unwrap_or
```

---

# Módulo 8: Propagación de errores y errores personalizados

## 8.1 Del `String` al error propio: una escalera

| Nivel | Tipo de error | Ejemplo | Comentario |
|---|---|---|---|
| 1 | `String` | `Result<u8, String>` | Fácil, pero sin estructura: el que llama solo puede mostrar el texto |
| 2 | **`enum` propio** | `Result<u8, ErrorEdad>` | Cada tipo de fallo es una variante: se puede hacer `match` |
| 3 | enum + **`Display`** | `println!("{e}")` | Mensajes legibles |
| 4 | enum + **`Error`** | `impl std::error::Error` | Integración con el ecosistema |
| 5 | enum + **`From`** | `?` convierte solo | Propagación cómoda |

## 8.2 Propagar: decidir **dónde** manejar el error

> **Principio:** los niveles bajos **propagan** (`?`); el nivel alto (quien tiene contexto) **decide** qué hacer: mostrar un mensaje, reintentar, usar un valor por defecto, terminar.

```rust
use std::num::ParseIntError;

// Nivel bajo: solo parsea. No decide nada: propaga.
fn leer_numero(texto: &str) -> Result<i32, ParseIntError> {
    texto.trim().parse::<i32>()
}

// Nivel medio: usa leer_numero y propaga.
fn calcular_doble(texto: &str) -> Result<i32, ParseIntError> {
    let n = leer_numero(texto)?;
    Ok(n * 2)
}

// Nivel alto: aquí sí se decide qué hacer.
fn main() {
    for entrada in ["21", "abc"] {
        match calcular_doble(entrada) {
            Ok(r) => println!("El doble de '{entrada}' es {r}"),
            Err(e) => println!("No pude calcular con '{entrada}': {e}"),
        }
    }
}
```

## 8.3 Crear un error propio: el `enum`

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum ErrorConfig {
    Vacio,
    PuertoInvalido(ParseIntError),     // guarda el error original
    PuertoReservado(u16),              // guarda el valor problemático
}
```

Cada **variante** es un modo distinto de fallar, con **los datos** que ayudan a entender el problema.

## 8.4 Implementar `Display`: el mensaje para humanos

```rust
use std::fmt;

impl fmt::Display for ErrorConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorConfig::Vacio => write!(f, "el puerto no puede estar vacío"),
            ErrorConfig::PuertoInvalido(e) => write!(f, "puerto inválido: {e}"),
            ErrorConfig::PuertoReservado(p) => write!(f, "el puerto {p} está reservado (< 1024)"),
        }
    }
}
```

Con esto, `println!("{error}")` imprime un mensaje claro. (`Debug`, con `#[derive(Debug)]`, sigue sirviendo para depurar.)

## 8.5 Implementar `Error`: el "sello" de error

```rust
impl std::error::Error for ErrorConfig {}
```

Una línea vacía basta: exige que el tipo ya tenga `Debug` y `Display`. Convierte tu tipo en un **error "oficial"** que otras herramientas del ecosistema reconocen.

## 8.6 Implementar `From`: para que `?` convierta solo

```rust
impl From<ParseIntError> for ErrorConfig {
    fn from(e: ParseIntError) -> Self {
        ErrorConfig::PuertoInvalido(e)
    }
}
```

Ahora, en una función que devuelve `Result<_, ErrorConfig>`, un `?` sobre un `Result<_, ParseIntError>` se **convierte automáticamente** en `ErrorConfig::PuertoInvalido(...)`.

## 8.7 Todo junto

```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ErrorConfig {
    Vacio,
    PuertoInvalido(ParseIntError),
    PuertoReservado(u16),
}

impl fmt::Display for ErrorConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorConfig::Vacio => write!(f, "el puerto no puede estar vacío"),
            ErrorConfig::PuertoInvalido(e) => write!(f, "puerto inválido: {e}"),
            ErrorConfig::PuertoReservado(p) => write!(f, "el puerto {p} está reservado (< 1024)"),
        }
    }
}

impl std::error::Error for ErrorConfig {}

impl From<ParseIntError> for ErrorConfig {
    fn from(e: ParseIntError) -> Self {
        ErrorConfig::PuertoInvalido(e)
    }
}

fn leer_puerto(texto: &str) -> Result<u16, ErrorConfig> {
    let texto = texto.trim();
    if texto.is_empty() {
        return Err(ErrorConfig::Vacio);
    }
    let puerto: u16 = texto.parse()?;                 // ParseIntError → ErrorConfig (con From)
    if puerto < 1024 {
        return Err(ErrorConfig::PuertoReservado(puerto));
    }
    Ok(puerto)
}

fn main() {
    for entrada in ["8080", "", "abc", "80", "70000"] {
        match leer_puerto(entrada) {
            Ok(p) => println!("'{entrada}' → puerto {p}"),
            Err(e) => println!("'{entrada}' → error: {e}"),
        }
    }
}
```

Salida:

```
'8080' → puerto 8080
'' → error: el puerto no puede estar vacío
'abc' → error: puerto inválido: invalid digit found in string
'80' → error: el puerto 80 está reservado (< 1024)
'70000' → error: puerto inválido: number too large to fit in target type
```

### Reaccionar distinto según el tipo de error

Esta es la gran ventaja del `enum` sobre un `String`:

```rust
fn main() {
    match leer_puerto("80") {
        Ok(p) => println!("usando {p}"),
        Err(ErrorConfig::Vacio) => println!("usando el puerto por defecto 8080"),
        Err(ErrorConfig::PuertoReservado(p)) => println!("el {p} requiere permisos; probando 8080"),
        Err(e) => println!("configuración inválida: {e}"),
    }
}
```

## 8.8 Conversión **sin** `From`: `map_err` con una variante

Si no quieres (o no puedes) implementar `From`, `map_err` convierte el error **a mano**. Una variante con datos funciona como función:

```rust
fn leer_puerto_v2(texto: &str) -> Result<u16, ErrorConfig> {
    let puerto = texto.trim().parse::<u16>().map_err(ErrorConfig::PuertoInvalido)?;
    Ok(puerto)
}
```

Es útil cuando **el mismo tipo de error** (por ejemplo `ParseIntError`) puede significar **cosas distintas** según el campo que se leía. Con `From` solo puede haber **una** conversión por tipo de origen.

## 8.9 Un error que reúne varias fuentes

```rust
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum ErrorDatos {
    Io(io::Error),
    Parseo(ParseIntError),
}

impl fmt::Display for ErrorDatos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorDatos::Io(e) => write!(f, "error de lectura: {e}"),
            ErrorDatos::Parseo(e) => write!(f, "dato inválido: {e}"),
        }
    }
}

impl std::error::Error for ErrorDatos {}

impl From<io::Error> for ErrorDatos {
    fn from(e: io::Error) -> Self {
        ErrorDatos::Io(e)
    }
}

impl From<ParseIntError> for ErrorDatos {
    fn from(e: ParseIntError) -> Self {
        ErrorDatos::Parseo(e)
    }
}

/// Suma los números de un archivo (uno por línea).
fn sumar_archivo(ruta: &str) -> Result<i32, ErrorDatos> {
    let contenido = fs::read_to_string(ruta)?;          // io::Error → ErrorDatos::Io
    let mut total = 0;
    for linea in contenido.lines() {
        if linea.trim().is_empty() {
            continue;
        }
        let n: i32 = linea.trim().parse()?;             // ParseIntError → ErrorDatos::Parseo
        total += n;
    }
    Ok(total)
}

fn main() {
    match sumar_archivo("no_existe.txt") {
        Ok(t) => println!("Total: {t}"),
        Err(e) => println!("{e}"),
        // En Linux/macOS: error de lectura: No such file or directory (os error 2)
        // (el texto exacto depende del sistema operativo)
    }
}
```

Dos `?` con **dos tipos de error distintos** conviven gracias a las dos implementaciones de `From`.

## 8.10 Alias de tipo para simplificar

```rust
type Resultado<T> = std::result::Result<T, ErrorConfig>;

fn leer_puerto_corto(texto: &str) -> Resultado<u16> {      // en vez de Result<u16, ErrorConfig>
    leer_puerto(texto)
}
```

Muy habitual en bibliotecas: un `Result` propio que ya incluye el tipo de error.

## 8.11 Un error con **contexto**: structs

A veces el error necesita contar **dónde** ocurrió. Un struct puede envolver a otro error:

```rust
use std::fmt;

#[derive(Debug)]
struct ErrorEnLinea {
    linea: usize,
    causa: ErrorConfig,
}

impl fmt::Display for ErrorEnLinea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "línea {}: {}", self.linea, self.causa)
    }
}

impl std::error::Error for ErrorEnLinea {}

fn validar_puertos(lineas: &[&str]) -> Result<Vec<u16>, ErrorEnLinea> {
    let mut puertos = Vec::new();
    for (i, texto) in lineas.iter().enumerate() {
        match leer_puerto(texto) {
            Ok(p) => puertos.push(p),
            Err(causa) => return Err(ErrorEnLinea { linea: i + 1, causa }),
        }
    }
    Ok(puertos)
}
```

(Reutiliza `ErrorConfig` y `leer_puerto` de 8.7.) Un error como `línea 3: el puerto 80 está reservado (< 1024)` es mucho más útil que uno sin contexto.

## 8.12 `main` que devuelve un error propio

```rust
fn main() -> Result<(), ErrorConfig> {
    let p = leer_puerto("8080")?;
    println!("Puerto: {p}");

    let q = leer_puerto("80")?;      // falla → main termina con código 1
    println!("Puerto: {q}");         // no se llega aquí
    Ok(())
}
// Salida de error: Error: PuertoReservado(80)      (usa Debug)
```

⚠️ Con este patrón, el mensaje final usa **`Debug`**, no `Display`. Si quieres un mensaje amigable al usuario, captura el error y muéstralo tú:

```rust
fn main() {
    if let Err(e) = ejecutar() {
        eprintln!("Error: {e}");              // eprintln! escribe en stderr
        std::process::exit(1);                // código de salida distinto de 0
    }
}

fn ejecutar() -> Result<(), ErrorConfig> {
    let p = leer_puerto("80")?;
    println!("Puerto: {p}");
    Ok(())
}
```

Este patrón (`main` pequeño + función `ejecutar` que devuelve `Result`) es **muy común** en programas reales.

## 8.13 Probar los errores

Los errores son valores: se pueden **comprobar en tests**. (Para poder comparar, deriva `PartialEq`; `ParseIntError` ya lo soporta.)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rechaza_vacio() {
        assert!(matches!(leer_puerto("  "), Err(ErrorConfig::Vacio)));
    }

    #[test]
    fn rechaza_reservado() {
        assert!(matches!(leer_puerto("80"), Err(ErrorConfig::PuertoReservado(80))));
    }

    #[test]
    fn acepta_valido() {
        assert_eq!(leer_puerto("8080").unwrap(), 8080);     // unwrap en tests: aceptable
    }
}
```

## 8.14 Guía de estilo para errores

| Recomendación | Por qué |
|---|---|
| Un **enum** con una variante por tipo de fallo | El que llama puede hacer `match` |
| Datos útiles dentro de las variantes (valor recibido, línea, campo) | Mensajes que ayudan a corregir |
| Mensajes en **minúscula**, sin punto final, que digan **qué** pasó | Se pueden encadenar: `error al cargar: puerto inválido: ...` |
| **No** exponer detalles internos innecesarios | Seguridad y claridad |
| `Display` para humanos, `Debug` para programadores | Cada uno con su propósito |
| Implementar `Error` + `From` para los errores que vas a propagar | `?` fluye sin esfuerzo |
| **Evitar `String` como tipo de error en bibliotecas** | No permite distinguir casos |
| Propagar con `?` y **manejar arriba** | Quien tiene el contexto decide |

## 8.15 Nota: lo que viene después

Existe una forma de devolver "cualquier error" en una sola firma (`Box<dyn Error>`), muy usada en programas pequeños; y hay bibliotecas del ecosistema (`thiserror`, `anyhow`) que automatizan gran parte de lo de este módulo. Ambos pertenecen a niveles posteriores: primero conviene dominar los errores propios como en este módulo, porque son la base de todo lo demás.

## 8.16 Resumen del módulo

1. **Propaga** con `?` desde los niveles bajos; **decide** en el nivel alto.
2. Un **enum** de errores permite `match` por tipo de fallo.
3. `Display` (mensaje) + `Error` (sello) + `From` (conversión para `?`) forman el trío estándar.
4. `map_err(Variante)` convierte a mano cuando `From` no basta.
5. Un **struct** puede añadir contexto (línea, archivo, campo).
6. `main` pequeño + `ejecutar() -> Result` es el patrón habitual.

---

# Módulo 9: Código seguro: diseñar para que el error sea explícito o imposible

"Código seguro" no es solo manejar errores: es **diseñar** para que la mayoría de los errores **no puedan ocurrir** o **no puedan ignorarse**. Rust te da las herramientas; este módulo las reúne.

## 9.1 Rust es seguro por defecto

Gracias a ownership, borrowing y lifetimes (Nivel 3), el código "normal" de Rust **no tiene**: referencias nulas, punteros colgantes, doble liberación ni *data races*. Existe una palabra clave `unsafe` para casos muy especiales (interactuar con C, optimizaciones extremas); **no se usa en este curso** y la mayoría de los programas nunca la necesitan. En este nivel, el trabajo de seguridad lo hacen **los tipos**.

## 9.2 Principio 1: validar en el borde, después confiar en los tipos

*"Parsea, no valides"*: convierte los datos crudos en un **tipo que garantiza** su validez **una sola vez**, en la entrada del sistema. A partir de ahí, el resto del código no necesita re-validar.

```rust
mod dominio {
    use std::fmt;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Edad(u8);                       // el campo es PRIVADO: solo se crea con `new`

    #[derive(Debug, PartialEq)]
    pub enum ErrorEdad {
        Negativa(i32),
        Excesiva(i32),
    }

    impl fmt::Display for ErrorEdad {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorEdad::Negativa(n) => write!(f, "la edad no puede ser negativa ({n})"),
                ErrorEdad::Excesiva(n) => write!(f, "la edad {n} supera el máximo de 120"),
            }
        }
    }

    impl Edad {
        pub fn new(valor: i32) -> Result<Edad, ErrorEdad> {
            if valor < 0 {
                Err(ErrorEdad::Negativa(valor))
            } else if valor > 120 {
                Err(ErrorEdad::Excesiva(valor))
            } else {
                Ok(Edad(valor as u8))
            }
        }

        pub fn valor(&self) -> u8 {
            self.0
        }
    }
}

use dominio::Edad;

// Esta función RECIBE una Edad: no necesita comprobar rango. Es imposible pasarle una edad inválida.
fn es_mayor_de_edad(edad: Edad) -> bool {
    edad.valor() >= 18
}

fn main() {
    match Edad::new(25) {
        Ok(e) => println!("{}", es_mayor_de_edad(e)),          // true
        Err(e) => println!("{e}"),
    }

    match Edad::new(500) {
        Ok(e) => println!("{}", es_mayor_de_edad(e)),
        Err(e) => println!("{e}"),                              // la edad 500 supera el máximo de 120
    }

    println!("{:?}", Edad::new(-3));                            // Err(Negativa(-3))

    // let trampa = Edad(200);       // ❌ E0603: el constructor es privado; no se puede crear una Edad desde fuera
}
```

> **Idea para clase:** el constructor `new` es la **única puerta** de entrada. Después de ella, **todo `Edad` del programa es válido**. El compilador hace cumplir esa regla por ti.

## 9.3 Principio 2: hacer imposibles los estados inválidos

Si tu modelo permite combinaciones que no tienen sentido, tarde o temprano ocurrirán.

```rust
// ❌ Diseño frágil: ¿qué significa conectado = true con direccion = None?
struct ConexionFragil {
    conectado: bool,
    direccion: Option<String>,
}

// ✅ Cada estado lleva exactamente los datos que le corresponden
enum Conexion {
    Desconectada,
    Conectada { direccion: String },
}

fn describir(c: &Conexion) -> String {
    match c {
        Conexion::Desconectada => String::from("sin conexión"),
        Conexion::Conectada { direccion } => format!("conectado a {direccion}"),
    }
}

fn main() {
    let c = Conexion::Conectada { direccion: String::from("10.0.0.1") };
    println!("{}", describir(&c));
    println!("{}", describir(&Conexion::Desconectada));
}
```

## 9.4 Principio 3: no usar tipos que "mienten"

| Tipo que miente | Alternativa honesta |
|---|---|
| `i32` con `-1` = "no hay" | `Option<usize>` / `Option<i32>` |
| `String` vacío = "no hay" | `Option<String>` |
| `bool` + campo opcional | `enum` con datos por variante |
| `u32` para algo que nunca debe ser 0 | Tipo propio con constructor validado (`Edad`, `Porcentaje`) |
| `String` para un correo, una ruta, un estado | Tipo propio o `enum` |

## 9.5 Principio 4: aritmética explícita

Elige **conscientemente** qué ocurre en los extremos:

| Quiero... | Método |
|---|---|
| Enterarme si se desborda | `checked_add`, `checked_sub`, `checked_mul`, `checked_div` → `Option` |
| Quedarme en el límite | `saturating_add`, `saturating_sub` |
| Dar la vuelta a propósito (hashes, contadores cíclicos) | `wrapping_add`, `wrapping_mul` |
| Conversión que puede perder datos | `u8::try_from(x)` → `Result` (en lugar de `as`) |

```rust
fn main() {
    let saldo: u32 = 100;
    let retiro: u32 = 250;

    match saldo.checked_sub(retiro) {
        Some(nuevo) => println!("nuevo saldo: {nuevo}"),
        None => println!("fondos insuficientes"),
    }

    let grande: i64 = 5_000_000_000;
    println!("{}", grande as i32);                      // ⚠️ as: 705032704 (pierde datos EN SILENCIO)
    println!("{:?}", i32::try_from(grande));            // ✅ Err(TryFromIntError(()))
}
```

## 9.6 Principio 5: acceso seguro a colecciones

```rust
use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3];
    let mapa: HashMap<&str, i32> = HashMap::from([("a", 1)]);

    // ⚠️ v[10]  mapa["z"]  &v[2..9]   → panic
    // ✅
    println!("{:?}", v.get(10));          // None
    println!("{:?}", mapa.get("z"));      // None
    println!("{:?}", v.get(2..9));        // None
    println!("{:?}", v.first());          // Some(1)
}
```

## 9.7 Principio 6: que el error no se pueda ignorar

- `Result` es `#[must_use]`: descartarlo da advertencia.
- Ignorarlo a propósito se escribe `let _ = ...;` (queda visible en la revisión de código).
- Puedes marcar **tus propias** funciones o tipos:

```rust
#[must_use = "el resultado de la validación debe revisarse"]
fn validar(texto: &str) -> bool {
    !texto.trim().is_empty()
}

fn main() {
    // validar("hola");              // ⚠️ warning: unused return value that must be used
    if validar("hola") {
        println!("válido");
    }
}
```

## 9.8 Principio 7: tratar el error en el nivel adecuado

```text
Entrada (teclado, archivo, red)   ←  valida y convierte a tipos propios (Principio 1)
        │
Lógica del programa                ←  confía en los tipos; propaga con ?
        │
Nivel superior (main)              ←  decide: mostrar, reintentar, terminar con código ≠ 0
```

## 9.9 Principio 8: probar los caminos de error

Un programa seguro tiene tests **para lo que sale mal**, no solo para lo que sale bien:

```rust
fn parsear_edad(texto: &str) -> Result<u8, String> {
    match texto.trim().parse::<u8>() {
        Ok(edad) if edad <= 120 => Ok(edad),
        Ok(edad) => Err(format!("edad fuera de rango: {edad}")),
        Err(_) => Err(format!("'{texto}' no es un número válido")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caso_feliz() {
        assert_eq!(parsear_edad("30"), Ok(30));
    }

    #[test]
    fn texto_invalido() {
        assert!(parsear_edad("abc").is_err());
    }

    #[test]
    fn fuera_de_rango() {
        assert_eq!(parsear_edad("200"), Err(String::from("edad fuera de rango: 200")));
    }

    #[test]
    fn vacio() {
        assert!(parsear_edad("").is_err());
    }
}
```

## 9.10 Lista de verificación de código seguro

- [ ] ¿Cada función que **puede fallar** lo dice en su tipo (`Option` / `Result`)?
- [ ] ¿Los datos externos se **validan una vez**, en el borde, y se convierten en tipos propios?
- [ ] ¿Hay `unwrap()` / `expect()` fuera de tests? ¿Cada uno está **justificado**?
- [ ] ¿Hay `v[i]`, `a / b`, `a - b`, `mapa["x"]` sobre datos externos? → alternativas seguras.
- [ ] ¿Los errores tienen **tipos propios** con mensajes claros?
- [ ] ¿`main` es pequeño y el manejo final de errores está **en un solo lugar**?
- [ ] ¿Hay tests de los **caminos de error**?

---

# Proyecto integrador: procesador de ventas

Este programa une **todo el Nivel 4**: `Option`, `Result`, `match`, `if let`, `let else`, `?`, errores personalizados (`enum` + `Display` + `Error` + `From`), un error con contexto (`struct`), `expect` con una invariante justificada y modos **estricto** y **tolerante**.

Crea un proyecto con `cargo new ventas` y pega esto en `src/main.rs`:

```rust
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

// ---------- Modelo ----------

#[derive(Debug)]
struct Venta {
    producto: String,
    cantidad: u32,
    precio: f64,
}

impl Venta {
    fn total(&self) -> f64 {
        self.cantidad as f64 * self.precio
    }
}

// ---------- Errores ----------

/// Qué puede salir mal al leer UNA línea.
#[derive(Debug)]
enum ErrorLinea {
    CamposIncorrectos(usize),
    ProductoVacio,
    CantidadInvalida(ParseIntError),
    PrecioInvalido(ParseFloatError),
    PrecioNegativo(f64),
}

impl fmt::Display for ErrorLinea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorLinea::CamposIncorrectos(n) => write!(f, "se esperaban 3 campos y llegaron {n}"),
            ErrorLinea::ProductoVacio => write!(f, "el nombre del producto está vacío"),
            ErrorLinea::CantidadInvalida(e) => write!(f, "cantidad inválida: {e}"),
            ErrorLinea::PrecioInvalido(e) => write!(f, "precio inválido: {e}"),
            ErrorLinea::PrecioNegativo(p) => write!(f, "el precio no puede ser negativo ({p})"),
        }
    }
}

impl Error for ErrorLinea {}

impl From<ParseIntError> for ErrorLinea {
    fn from(e: ParseIntError) -> Self {
        ErrorLinea::CantidadInvalida(e)          // el único entero que parseamos es la cantidad
    }
}

impl From<ParseFloatError> for ErrorLinea {
    fn from(e: ParseFloatError) -> Self {
        ErrorLinea::PrecioInvalido(e)
    }
}

/// Un error de línea + el número de línea (contexto).
#[derive(Debug)]
struct ErrorProceso {
    linea: usize,
    causa: ErrorLinea,
}

impl fmt::Display for ErrorProceso {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "línea {}: {}", self.linea, self.causa)
    }
}

impl Error for ErrorProceso {}

// ---------- Lógica ----------

fn parsear_linea(linea: &str) -> Result<Venta, ErrorLinea> {
    let campos: Vec<&str> = linea.split(',').collect();
    if campos.len() != 3 {
        return Err(ErrorLinea::CamposIncorrectos(campos.len()));
    }

    let producto = campos[0].trim();
    if producto.is_empty() {
        return Err(ErrorLinea::ProductoVacio);
    }

    let cantidad: u32 = campos[1].trim().parse()?;     // ParseIntError → ErrorLinea
    let precio: f64 = campos[2].trim().parse()?;       // ParseFloatError → ErrorLinea

    if precio < 0.0 {
        return Err(ErrorLinea::PrecioNegativo(precio));
    }

    Ok(Venta { producto: producto.to_string(), cantidad, precio })
}

/// ESTRICTO: se detiene en el primer error.
fn procesar_estricto(texto: &str) -> Result<Vec<Venta>, ErrorProceso> {
    let mut ventas = Vec::new();
    for (i, linea) in texto.lines().enumerate() {
        if linea.trim().is_empty() {
            continue;
        }
        match parsear_linea(linea) {
            Ok(v) => ventas.push(v),
            Err(causa) => return Err(ErrorProceso { linea: i + 1, causa }),
        }
    }
    Ok(ventas)
}

/// TOLERANTE: separa lo bueno de lo malo.
fn procesar_tolerante(texto: &str) -> (Vec<Venta>, Vec<ErrorProceso>) {
    let mut ventas = Vec::new();
    let mut errores = Vec::new();
    for (i, linea) in texto.lines().enumerate() {
        if linea.trim().is_empty() {
            continue;
        }
        match parsear_linea(linea) {
            Ok(v) => ventas.push(v),
            Err(causa) => errores.push(ErrorProceso { linea: i + 1, causa }),
        }
    }
    (ventas, errores)
}

/// Option: puede que no haya ventas.
fn mayor_venta(ventas: &[Venta]) -> Option<&Venta> {
    let mut mayor: Option<&Venta> = None;
    for v in ventas {
        let reemplaza = match mayor {
            Some(actual) => v.total() > actual.total(),
            None => true,
        };
        if reemplaza {
            mayor = Some(v);
        }
    }
    mayor
}

fn totales_por_producto(ventas: &[Venta]) -> HashMap<&str, f64> {
    let mut totales = HashMap::new();
    for v in ventas {
        *totales.entry(v.producto.as_str()).or_insert(0.0) += v.total();
    }
    totales
}

/// `?` propaga el ErrorProceso de procesar_estricto.
fn total_unidades(texto: &str) -> Result<u32, ErrorProceso> {
    let ventas = procesar_estricto(texto)?;
    let mut total = 0;
    for v in &ventas {
        total += v.cantidad;
    }
    Ok(total)
}

// ---------- Datos de prueba ----------

const DATOS: &str = "\
Café, 3, 4500
Pan, 10, 1200.50
Leche, dos, 3000
Té, 2
, 1, 500
Azúcar, 4, -100

Café, 2, 4500";

fn main() {
    // 1) Modo estricto: se detiene en el primer error
    match procesar_estricto(DATOS) {
        Ok(ventas) => println!("Estricto: {} ventas", ventas.len()),
        Err(e) => println!("Estricto falló -> {e}"),
    }

    // 2) Modo tolerante: reporta todos los problemas y sigue
    let (ventas, errores) = procesar_tolerante(DATOS);
    println!("\nVentas válidas: {}", ventas.len());
    println!("Errores: {}", errores.len());
    for e in &errores {
        println!("  - {e}");
    }

    // 3) Option: la mayor venta (puede no existir)
    if let Some(v) = mayor_venta(&ventas) {
        println!("\nMayor venta: {} por ${:.0}", v.producto, v.total());
    }

    // 4) Option en un HashMap: buscar totales
    let totales = totales_por_producto(&ventas);
    for nombre in ["Café", "Pan", "Té"] {
        match totales.get(nombre) {
            Some(total) => println!("{nombre}: ${total:.0}"),
            None => println!("{nombre}: sin ventas"),
        }
    }

    // 5) let else: salir si no hay ventas
    let Some(primera) = ventas.first() else {
        println!("no hay ventas que reportar");
        return;
    };
    println!("Primera venta: {}", primera.producto);

    // 6) expect con una invariante justificada
    let iva: f64 = "0.19".parse().expect("IVA es un literal numérico válido");
    let mut subtotal = 0.0;
    for v in &ventas {
        subtotal += v.total();
    }
    println!("\nSubtotal: ${subtotal:.2} | con IVA: ${:.2}", subtotal * (1.0 + iva));

    // 7) Propagación con ?
    match total_unidades("Café, 2, 100\nPan, 3, 50") {
        Ok(n) => println!("Unidades: {n}"),
        Err(e) => println!("Error: {e}"),
    }
    match total_unidades("Café, 2, 100\nPan, x, 50") {
        Ok(n) => println!("Unidades: {n}"),
        Err(e) => println!("Error: {e}"),
    }
}
```

**Salida esperada** (con `cargo run`):

```
Estricto falló -> línea 3: cantidad inválida: invalid digit found in string

Ventas válidas: 3
Errores: 4
  - línea 3: cantidad inválida: invalid digit found in string
  - línea 4: se esperaban 3 campos y llegaron 2
  - línea 5: el nombre del producto está vacío
  - línea 6: el precio no puede ser negativo (-100)

Mayor venta: Café por $13500
Café: $22500
Pan: $12005
Té: sin ventas
Primera venta: Café

Subtotal: $34505.00 | con IVA: $41060.95
Unidades: 5
Error: línea 2: cantidad inválida: invalid digit found in string
```

**Qué concepto aparece y dónde:**

| Concepto | Dónde |
|---|---|
| `Result` y funciones que fallan | `parsear_linea`, `procesar_*`, `total_unidades` |
| `Option` | `mayor_venta`, `totales.get(..)`, `ventas.first()` |
| `?` + `From` | `parse()?` en `parsear_linea` convierte a `ErrorLinea` |
| `?` entre funciones propias | `procesar_estricto(texto)?` en `total_unidades` |
| Error propio: enum + `Display` + `Error` + `From` | `ErrorLinea` |
| Error con contexto (struct) | `ErrorProceso { linea, causa }` |
| Estricto vs. tolerante | `procesar_estricto` / `procesar_tolerante` |
| `match` con guardas y patrones | `mayor_venta`, todos los `match` de `main` |
| `if let` | `if let Some(v) = mayor_venta(..)` |
| `let else` | `let Some(primera) = ventas.first() else { ... }` |
| `expect` con justificación | el IVA literal |
| Sin `unwrap` en datos externos | Todo el análisis de `DATOS` |

**Preguntas para discutir en clase:**

1. ¿Qué cambiaría si `parsear_linea` devolviera `Result<Venta, String>`? ¿Qué se perdería?
2. ¿Por qué `From<ParseIntError>` produce `CantidadInvalida` y no un error genérico? ¿Qué problema habría si hubiera **dos** campos enteros?
3. ¿Dónde conviene usar el modo estricto y dónde el tolerante?
4. ¿Qué ocurre si `DATOS` queda vacío? ¿Cuál línea de `main` lo maneja?
5. **Reto:** agrega la variante `ErrorLinea::CantidadCero` (cantidad igual a 0) y comprueba que el compilador te señala todos los `match` que debes actualizar.
6. **Reto 2:** haz que `main` use el patrón `ejecutar() -> Result<(), ErrorProceso>` con `eprintln!` y `std::process::exit(1)`.

---

# Ejercicios con soluciones

> 💡 Pide a los estudiantes que **expliquen por qué** eligieron `Option`, `Result` o `panic!` en cada caso, no solo que escriban el código.

## Módulo 2: `Option`

**2.1** Escribe:
- `fn dividir(a: i32, b: i32) -> Option<i32>`
- `fn inicial_mayuscula(s: &str) -> Option<char>`
- `fn indice_de(lista: &[i32], objetivo: i32) -> Option<usize>`

<details>
<summary>Ver solución</summary>

```rust
fn dividir(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn inicial_mayuscula(s: &str) -> Option<char> {
    match s.chars().next() {
        Some(c) => Some(c.to_ascii_uppercase()),
        None => None,
    }
}

fn indice_de(lista: &[i32], objetivo: i32) -> Option<usize> {
    for (i, x) in lista.iter().enumerate() {
        if *x == objetivo {
            return Some(i);
        }
    }
    None
}

fn main() {
    println!("{:?}", dividir(10, 2));              // Some(5)
    println!("{:?}", dividir(1, 0));               // None
    println!("{:?}", inicial_mayuscula("rust"));   // Some('R')
    println!("{:?}", inicial_mayuscula(""));       // None
    println!("{:?}", indice_de(&[4, 5, 6], 6));    // Some(2)
    println!("{:?}", indice_de(&[4, 5, 6], 9));    // None
}
```

(El `match ... Some(c) => Some(...), None => None` es un patrón tan común que existe un método para abreviarlo, `map`, que se aprende con closures.)
</details>

## Módulo 3: `Result`

**3.1** Escribe `fn parsear_edad(texto: &str) -> Result<u8, String>`: debe aceptar solo números entre 0 y 120 (ignorando espacios) y devolver mensajes de error distintos para "no es número" y "fuera de rango".

<details>
<summary>Ver solución</summary>

```rust
fn parsear_edad(texto: &str) -> Result<u8, String> {
    match texto.trim().parse::<u8>() {
        Ok(edad) if edad <= 120 => Ok(edad),
        Ok(edad) => Err(format!("edad fuera de rango: {edad}")),
        Err(_) => Err(format!("'{texto}' no es un número válido")),
    }
}

fn main() {
    println!("{:?}", parsear_edad(" 30 "));    // Ok(30)
    println!("{:?}", parsear_edad("200"));     // Err("edad fuera de rango: 200")
    println!("{:?}", parsear_edad("abc"));     // Err("'abc' no es un número válido")
    println!("{:?}", parsear_edad("-5"));      // Err("'-5' no es un número válido")
}
```
</details>

## Módulo 4: Pattern matching

**4.1** Escribe `fn describir(n: Option<i32>) -> &'static str` que devuelva `"nada"`, `"negativo"`, `"cero"`, `"un dígito"` (1 a 9) o `"grande"`.

**4.2** Escribe `fn resumen(v: &[i32]) -> String` usando patrones de slice: vacío, un elemento, o "de X a Y".

**4.3** Con `while let`, imprime los elementos de un `Vec` de atrás hacia adelante vaciándolo.

**4.4** Escribe `fn primera_palabra(texto: &str) -> String` usando `let else` (devuelve `"(vacío)"` si no hay palabras).

<details>
<summary>Ver soluciones</summary>

```rust
fn describir(n: Option<i32>) -> &'static str {
    match n {
        None => "nada",
        Some(x) if x < 0 => "negativo",
        Some(0) => "cero",
        Some(1..=9) => "un dígito",
        Some(_) => "grande",
    }
}

fn resumen(v: &[i32]) -> String {
    match v {
        [] => String::from("vacía"),
        [x] => format!("solo {x}"),
        [primero, .., ultimo] => format!("de {primero} a {ultimo}"),
    }
}

fn primera_palabra(texto: &str) -> String {
    let Some(palabra) = texto.split_whitespace().next() else {
        return String::from("(vacío)");
    };
    palabra.to_string()
}

fn main() {
    println!("{}", describir(None));          // nada
    println!("{}", describir(Some(-4)));      // negativo
    println!("{}", describir(Some(0)));       // cero
    println!("{}", describir(Some(7)));       // un dígito
    println!("{}", describir(Some(42)));      // grande

    println!("{}", resumen(&[]));             // vacía
    println!("{}", resumen(&[9]));            // solo 9
    println!("{}", resumen(&[1, 2, 3]));      // de 1 a 3

    let mut v = vec![1, 2, 3];
    while let Some(x) = v.pop() {
        println!("{x}");                      // 3, 2, 1
    }

    println!("{}", primera_palabra("hola mundo"));   // hola
    println!("{}", primera_palabra("   "));          // (vacío)
}
```
</details>

## Módulo 5: Operador `?`

**5.1** Escribe `fn suma_de_dos(a: &str, b: &str) -> Result<i32, ParseIntError>` usando `?`.

**5.2** Escribe `fn primera_y_ultima(s: &str) -> Option<(char, char)>` usando `?` sobre `Option`.

**5.3** ¿Por qué no compila y cómo se arregla?

```rust
fn primer_numero(texto: &str) -> Result<i32, String> {
    let palabra = texto.split_whitespace().next()?;
    let n = palabra.parse::<i32>()?;
    Ok(n)
}
```

<details>
<summary>Ver soluciones</summary>

```rust
use std::num::ParseIntError;

fn suma_de_dos(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.trim().parse::<i32>()?;
    let y = b.trim().parse::<i32>()?;
    Ok(x + y)
}

fn primera_y_ultima(s: &str) -> Option<(char, char)> {
    let primera = s.chars().next()?;
    let ultima = s.chars().last()?;
    Some((primera, ultima))
}

fn main() {
    println!("{:?}", suma_de_dos("2", "3"));      // Ok(5)
    println!("{:?}", suma_de_dos("2", "x"));      // Err(...)
    println!("{:?}", primera_y_ultima("rust"));   // Some(('r', 't'))
    println!("{:?}", primera_y_ultima(""));       // None
}
```

**5.3** Tiene **dos** problemas:

1. `texto.split_whitespace().next()?` es un `Option` dentro de una función que devuelve `Result` (E0277): hay que convertirlo con `ok_or`.
2. `palabra.parse::<i32>()?` produce un `ParseIntError`, pero la función devuelve `Result<_, String>` y no existe `From<ParseIntError> for String` (E0277).

Solución sin closures: manejar el error con `match` (o, mejor, crear un error propio como en el Módulo 8):

```rust
fn primer_numero(texto: &str) -> Result<i32, String> {
    let palabra = texto
        .split_whitespace()
        .next()
        .ok_or(String::from("el texto está vacío"))?;

    match palabra.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(format!("'{palabra}' no es un número")),
    }
}

fn main() {
    println!("{:?}", primer_numero("42 y más"));     // Ok(42)
    println!("{:?}", primer_numero("hola"));         // Err("'hola' no es un número")
    println!("{:?}", primer_numero(""));             // Err("el texto está vacío")
}
```
</details>

## Módulo 6: `panic!`

**6.1** Reescribe estas funciones para que **no** puedan hacer panic:

```rust
fn tercero(v: &[i32]) -> i32 {
    v[2]
}

fn dividir(a: i32, b: i32) -> i32 {
    a / b
}

fn prefijo(s: &str) -> &str {
    &s[..3]
}
```

**6.2** Clasifica cada caso: ¿`Result`/`Option` o `panic!`?

a) El usuario escribe su edad.
b) Una función interna recibe un índice que la propia función anterior ya garantizó que es válido.
c) Leer un archivo de configuración.
d) Un `match` sobre un enum con 3 variantes, donde una rama "imposible" nunca debería ejecutarse.
e) Buscar un cliente por su id.

<details>
<summary>Ver soluciones</summary>

**6.1**

```rust
fn tercero(v: &[i32]) -> Option<i32> {
    v.get(2).copied()
}

fn dividir(a: i32, b: i32) -> Option<i32> {
    a.checked_div(b)          // también cubre i32::MIN / -1, que desbordaría
}

fn prefijo(s: &str) -> Option<&str> {
    s.get(..3)                // None si hay menos de 3 bytes o corta una letra a la mitad
}

fn main() {
    println!("{:?}", tercero(&[1, 2]));       // None
    println!("{:?}", dividir(7, 0));          // None
    println!("{:?}", prefijo("ab"));          // None (es demasiado corto)
    println!("{:?}", prefijo("ññ"));          // None (el byte 3 corta la segunda ñ a la mitad)
    println!("{:?}", prefijo("hola"));        // Some("hol")
}
```

**6.2**

| Caso | Respuesta | Motivo |
|---|---|---|
| a | `Result` | El usuario puede escribir cualquier cosa |
| b | `panic!`/`assert!` (o simplemente confiar en el tipo) | Si falla, es un bug interno |
| c | `Result` | El archivo puede no existir o estar mal |
| d | `unreachable!` | Solo ocurriría por un bug |
| e | `Option` | Que no exista es normal |
</details>

## Módulo 7: `unwrap` / `expect`

**7.1** Para cada uso, indica si es aceptable. Si no, escribe la alternativa.

```rust
// a)
let puerto: u16 = "8080".parse().unwrap();
// b)
let edad: u8 = entrada_del_usuario.parse().unwrap();
// c)
if !lista.is_empty() { println!("{}", lista.first().unwrap()); }
// d)
let texto = std::fs::read_to_string(ruta).unwrap();
// e) dentro de un #[test]
assert_eq!(sumar("2", "3").unwrap(), 5);
```

<details>
<summary>Ver solución</summary>

| Caso | ¿Aceptable? | Alternativa |
|---|---|---|
| a | Aceptable **con `expect`** (el literal es válido por construcción) | `.expect("8080 es un puerto válido")` |
| b | ❌ Dato externo | `match` / `?` / `unwrap_or` |
| c | ❌ Hace el trabajo dos veces | `if let Some(p) = lista.first() { println!("{p}"); }` |
| d | ❌ El archivo puede no existir | Propagar con `?` o `match` sobre el `Result` |
| e | ✅ En tests es normal | — |
</details>

## Módulo 8: Errores personalizados

**8.1** Implementa `registrar(nombre: &str, edad: &str) -> Result<Usuario, ErrorRegistro>` con estos errores: nombre vacío; edad que no es número (guarda el `ParseIntError`); edad mayor que 120 (guarda el valor). Incluye `Display`, `Error` y `From`.

<details>
<summary>Ver solución</summary>

```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct Usuario {
    nombre: String,
    edad: u8,
}

#[derive(Debug)]
enum ErrorRegistro {
    NombreVacio,
    EdadInvalida(ParseIntError),
    EdadFueraDeRango(u8),
}

impl fmt::Display for ErrorRegistro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorRegistro::NombreVacio => write!(f, "el nombre no puede estar vacío"),
            ErrorRegistro::EdadInvalida(e) => write!(f, "la edad no es un número válido: {e}"),
            ErrorRegistro::EdadFueraDeRango(n) => write!(f, "la edad {n} está fuera del rango 0-120"),
        }
    }
}

impl std::error::Error for ErrorRegistro {}

impl From<ParseIntError> for ErrorRegistro {
    fn from(e: ParseIntError) -> Self {
        ErrorRegistro::EdadInvalida(e)
    }
}

fn registrar(nombre: &str, edad: &str) -> Result<Usuario, ErrorRegistro> {
    let nombre = nombre.trim();
    if nombre.is_empty() {
        return Err(ErrorRegistro::NombreVacio);
    }
    let edad: u8 = edad.trim().parse()?;
    if edad > 120 {
        return Err(ErrorRegistro::EdadFueraDeRango(edad));
    }
    Ok(Usuario { nombre: nombre.to_string(), edad })
}

fn main() {
    let casos = [("Ana", "25"), ("", "20"), ("Luis", "abc"), ("Eva", "200")];
    for (nombre, edad) in casos {
        match registrar(nombre, edad) {
            Ok(u) => println!("registrado: {} ({} años)", u.nombre, u.edad),
            Err(e) => println!("error: {e}"),
        }
    }
}
```

Salida:

```
registrado: Ana (25 años)
error: el nombre no puede estar vacío
error: la edad no es un número válido: invalid digit found in string
error: la edad 200 está fuera del rango 0-120
```
</details>

## Módulo 9: Código seguro

**9.1** Crea el tipo `Porcentaje` (0 a 100) que **no se pueda construir** con un valor inválido, con constructor `Porcentaje::new(u8) -> Option<Porcentaje>` y un método `aplicar(&self, monto: f64) -> f64`.

<details>
<summary>Ver solución</summary>

```rust
mod dominio {
    #[derive(Debug, Clone, Copy)]
    pub struct Porcentaje(u8);               // campo privado

    impl Porcentaje {
        pub fn new(valor: u8) -> Option<Porcentaje> {
            if valor <= 100 {
                Some(Porcentaje(valor))
            } else {
                None
            }
        }

        pub fn aplicar(&self, monto: f64) -> f64 {
            monto * self.0 as f64 / 100.0
        }
    }
}

use dominio::Porcentaje;

fn main() {
    match Porcentaje::new(15) {
        Some(p) => println!("{}", p.aplicar(200.0)),     // 30
        None => println!("porcentaje inválido"),
    }
    println!("{:?}", Porcentaje::new(150));              // None
    // let malo = Porcentaje(150);                       // ❌ el campo es privado
}
```
</details>

---

# Hoja de referencia rápida

```rust
// ---------- OPTION ----------
let a: Option<i32> = Some(5);   let b: Option<i32> = None;
match a { Some(n) => ..., None => ... }
if let Some(n) = a { ... }
let Some(n) = a else { return ...; };           // let else
a.unwrap_or(0)   a.unwrap_or_default()   a.is_some()   a.is_none()
a.ok_or(error)                                  // Option → Result
opt.as_ref()   opt.take()   v.first().copied()  v.first().cloned()

// ---------- RESULT ----------
fn f() -> Result<T, E> { ... Ok(valor) ... Err(error) }
match r { Ok(v) => ..., Err(e) => ... }
if let Ok(v) = r { ... }     if let Err(e) = r { ... }
r.unwrap_or(0)   r.unwrap_or_default()   r.is_ok()   r.is_err()
r.ok()                                          // Result → Option
r.map_err(MiError::Variante)                    // convertir el error con una variante

// ---------- OPERADOR ? ----------
let n = texto.parse::<i32>()?;                  // Ok → valor; Err → return Err (con From)
let c = texto.chars().next()?;                  // en fn que devuelve Option
let c = opt.ok_or(error)?;                      // Option → Result
fn main() -> Result<(), MiError> { ... Ok(()) }

// ---------- PANIC ----------
panic!("msg {}", x)    assert!(cond, "msg")    assert_eq!(a, b)    debug_assert!(c)
unreachable!()    todo!()    unimplemented!()
// Evitar: v[i]  a / b  a - b (sin checked)  mapa["k"]  s[a..b]  .unwrap() con datos externos
// Alternativas: .get(i)  checked_div / checked_sub  .get("k")  s.get(a..b)

// ---------- UNWRAP / EXPECT ----------
x.unwrap()                                      // solo tests y prototipos
x.expect("por qué NO puede fallar")             // invariantes demostrables

// ---------- ERROR PERSONALIZADO ----------
#[derive(Debug)]
enum MiError { A, B(ParseIntError), C { campo: String } }
impl fmt::Display for MiError { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { match self { ... write!(f, "...") } } }
impl std::error::Error for MiError {}
impl From<ParseIntError> for MiError { fn from(e: ParseIntError) -> Self { MiError::B(e) } }
type Resultado<T> = std::result::Result<T, MiError>;

// ---------- PATRONES ----------
x @ 1..=9    Some(1 | 2)    Some(n) if n > 5    (a, b)    Punto { x, y }
[primero, .., ultimo]    [cabeza, resto @ ..]    Some(&x)
while let Some(x) = pila.pop() { ... }
matches!(valor, Patron)
```

### ¿Option, Result o panic?

```
¿Puede faltar el valor de forma normal, sin que haga falta explicar por qué?  → Option<T>
¿Puede fallar y quien llama necesita saber por qué?                           → Result<T, E>
¿Solo ocurre si hay un bug en MI programa?                                    → panic! / assert!
```

---

# Errores frecuentes del compilador

| Código | Mensaje (resumido) | Causa típica | Solución |
|---|---|---|---|
| `E0308` | mismatched types | Usar `Option<T>`/`Result` donde se espera `T`; olvidar `Ok(...)` en el valor final; comparar `Option<i32>` con un entero | Abrir el valor (`match`, `?`...); envolver con `Ok(...)`; comparar con `Some(5)` |
| `E0277` | `?` can only be used in a function that returns `Result` or `Option` | `?` en `main()` sin retorno `Result`, o en una función que devuelve otro tipo | Cambiar el tipo de retorno o usar `match` |
| `E0277` | `?` couldn't convert the error to `X` | No existe `From<ErrorOrigen> for X` | Implementar `From`, o `map_err` |
| `E0277` | `Option<T>`/`Result` doesn't implement `Display` | Usar `{}` con un `Option`/`Result` | Usar `{:?}` o abrir el valor |
| `E0369` | cannot add `{integer}` to `Option<i32>` | Operar con un `Option` como si fuera número | Abrir el valor antes |
| `E0599` | no method named `x` found for enum `Option` | Llamar un método del tipo interior sin abrirlo | Abrir el valor, o `as_ref`/`match` |
| `E0004` | non-exhaustive patterns | `match` sin cubrir `None`, `Err` u otra variante | Agregar la rama o `_` |
| `E0005` | refutable pattern in local binding | `let Some(x) = opt;` sin `else` | `let else`, `if let` o `match` |
| `E0282` | type annotations needed | `None` o `Vec::new()` sin tipo inferible | Anotar `Option<i32>` |
| `E0382` | borrow of (partially) moved value | `if let Some(s) = opt_string` movió el `String` | `&opt` o `opt.as_ref()` |
| (warning) | unused `Result` that must be used | Se descartó un `Result` | Manejarlo, o `let _ = ...;` a propósito |
| (panic) | called `Option::unwrap()` on a `None` value | `unwrap()` sobre `None` | `match`/`if let`/`?` |
| (panic) | called `Result::unwrap()` on an `Err` value | `unwrap()` sobre `Err` | `match`/`?` |
| (panic) | index out of bounds / divide by zero / overflow | `v[i]`, `a / 0`, desbordamiento | `.get()`, `checked_*` |

💡 **Herramientas:** `cargo check` para errores rápidos, `rustc --explain E0277` para la explicación detallada de un código, `cargo clippy` para sugerencias (incluye avisos sobre `unwrap`).

---

# Plan de clases sugerido

Propuesta de 6 sesiones de ~90 minutos:

| Sesión | Contenido | Actividad práctica |
|---|---|---|
| **1** | Módulo 1 (filosofía) + Módulo 2 (`Option`) | Ejercicio 2.1; "cazar" los `Option` ocultos en funciones de la librería estándar |
| **2** | Módulo 3 (`Result`) | Ejercicio 3.1; clasificar mensajes de error de `parse` con `IntErrorKind` |
| **3** | Módulo 4 (pattern matching, `if let`, `let else`, `while let`) | Ejercicios 4.1–4.4 |
| **4** | Módulo 5 (`?`) + Módulo 6 (`panic!`) | Ejercicios 5.1–5.3 y 6.1–6.2; provocar panics a propósito y leer el mensaje |
| **5** | Módulo 7 (`unwrap`/`expect`) + Módulo 8 (errores personalizados) | Ejercicios 7.1 y 8.1; refactorizar un programa lleno de `unwrap` |
| **6** | Módulo 9 (código seguro) + Proyecto integrador | Ejercicio 9.1; ampliar el procesador de ventas con retos del proyecto |

### Recomendaciones de enseñanza

1. **Empieza por la intuición**: "¿qué pasa si el usuario escribe 'abc' donde pides un número?". Que **vean** el programa caerse con `unwrap` antes de enseñar la alternativa.
2. **La pregunta maestra de todo el nivel:** *"¿Esto puede pasar en un programa **correcto**?"* Si sí → `Result`/`Option`. Si no → `panic`.
3. **Prohíbe `unwrap` en tareas con datos externos.** Exige `expect` con una razón cuando sea inevitable.
4. **Que lean los errores del compilador en voz alta.** En este nivel los mensajes (`E0277`, `E0308`, `E0004`) son especialmente claros y casi siempre indican la solución.
5. **Practica el "diseño con tipos"**: pídeles que reemplacen `-1`, `""` o `bool + campo opcional` por `Option` y `enum`.
6. **Fomenta tests de caminos tristes**: que escriban al menos un test por cada variante de error.
7. **Ideas centrales para repetir:**
   - *El tipo de retorno dice la verdad: si puede fallar o faltar, está en la firma.*
   - *El compilador no te deja olvidar el caso malo.*
   - *Los niveles bajos propagan con `?`; el nivel alto decide qué hacer.*
   - *`panic!` es para bugs, no para entradas malas.*

### Qué sigue

Con el manejo de errores dominado, el siguiente paso natural es trabajar con **closures e iteradores** (que simplifican mucho el código con `Option` y `Result`: `map`, `and_then`, `collect`, `unwrap_or_else`...), y luego con **errores dinámicos** (`Box<dyn Error>`) y el ecosistema (`thiserror`, `anyhow`).

---

*Fin del Nivel 4. Un programa robusto no es el que nunca falla: es el que sabe exactamente cómo falla. 🦀*
