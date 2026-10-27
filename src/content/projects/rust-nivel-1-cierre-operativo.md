---
title: "Proyecto Nivel 1: Cierre operativo de una cafetería"
description: "Construye un reporte de cierre diario de ventas para una cafetería, usando los fundamentos del lenguaje Rust."
language: rust
level: 1
levelLabel: "Nivel 01"
category: "Proyecto empresarial"
duration: "6–8 horas"
order: 1
slug: rust-nivel-1-cierre-operativo
courseSlug: rust-nivel-1
company: "Café Central (empresa simulada)"
role: "Desarrollador/a junior del equipo de operaciones"
moduleName: "Módulos 1–8: variables, tipos, operadores, condicionales, bucles, funciones, scope, tuplas y arrays"
---

# Misión del proyecto

## Contexto de empresa

Café Central opera varias cafeterías y cada encargado debe entregar al final del turno un resumen confiable de ventas. Hoy el equipo copia cifras a mano en una hoja de cálculo y comete errores al sumar unidades, calcular importes y clasificar el resultado del día.

Tu líder técnico te asignó el primer prototipo de una herramienta de consola en Rust. Debe procesar un conjunto pequeño de líneas de venta de ejemplo y generar un reporte legible. En este nivel no necesitas base de datos, archivos ni una interfaz gráfica: el objetivo es construir una primera versión correcta y fácil de revisar.

## Objetivo de negocio

Entregar un cierre diario reproducible que permita al encargado ver cuánto vendió la cafetería, qué productos aportaron más ingresos y si se cumplieron los objetivos operativos.

## Alcance funcional

1. Definir constantes para la meta diaria y los límites usados en el reporte.
2. Representar las ventas con un array de tuplas de tamaño fijo. Cada tupla debe contener el nombre del producto, su precio unitario en pesos colombianos y las unidades vendidas.
3. Calcular el ingreso por línea, el ingreso total, las unidades vendidas, el promedio por línea y la línea con mayor ingreso.
4. Clasificar el cierre con `if` y `match`, por ejemplo: `Sin ventas`, `Por debajo de la meta` o `Meta cumplida`.
5. Imprimir un reporte numerado que muestre datos, importes y una conclusión del día.
6. Separar los cálculos en funciones pequeñas con nombres claros y parámetros explícitos.
7. Tratar el caso de una venta sin unidades o un día sin ventas sin generar resultados engañosos.

## Datos de prueba iniciales

Usa datos simulados coherentes con una cafetería colombiana. Puedes iniciar con una lista fija como la siguiente y ampliarla hasta tener entre seis y diez líneas:

| Producto | Precio unitario (COP) | Unidades vendidas |
|---|---:|---:|
| Café americano | 4.500 | 18 |
| Capuchino | 6.500 | 12 |
| Chocolate | 5.000 | 8 |
| Croissant | 4.000 | 10 |
| Sándwich | 11.000 | 7 |
| Agua | 3.000 | 9 |

Los valores son datos de entrenamiento, no precios oficiales. Mantén los importes en pesos enteros para el reporte monetario y usa `f64` cuando necesites mostrar promedios o porcentajes. Explica esa elección en el código.

## Reglas de negocio

- Una línea con cero unidades genera un total de cero y no debe inflar los conteos.
- El total de una línea es precio unitario multiplicado por unidades.
- El total del día es la suma de todas las líneas.
- Si no hay registros, el promedio debe presentarse como no disponible o como cero según una decisión documentada; no se debe dividir por cero.
- El resultado final debe ser determinista: con los mismos datos de entrada, el programa genera el mismo reporte.
- Usa nombres en `snake_case`; reserva `MAYUSCULAS_SNAKE_CASE` para las constantes.

## Relación con los módulos del Nivel 1

| Módulo | Cómo se aplica en el proyecto |
|---|---|
| 1. Variables | `let`, `mut`, `const`, anotaciones y shadowing justificado cuando conviertas datos. |
| 2. Tipos de datos | `u32` o `u64` para importes/unidades, `&str` para productos, `bool` para condiciones y `f64` para promedios. |
| 3. Operadores y conversiones | Multiplicar, sumar, comparar y convertir conteos a `f64` solo para divisiones. |
| 4. Condicionales | Clasificar el resultado diario con `if` y `match` exhaustivo. |
| 5. Bucles | Recorrer las líneas y acumular ingresos, unidades y máximos. |
| 6. Funciones | Encapsular cálculo por línea, suma, promedio y clasificación. |
| 7. Scope | Mantener contadores locales y devolver resultados desde bloques o funciones. |
| 8. Tuplas y arrays | Guardar filas homogéneas en un array de tuplas y devolver varios indicadores en una tupla. |

## Entregables

- Proyecto Cargo llamado `cierre_cafeteria`.
- Código organizado en `src/main.rs`; en este nivel no se exige dividirlo en módulos.
- Reporte de consola de una jornada de ejemplo.
- Tabla breve en el README con los datos de entrada, el resultado esperado y decisiones de diseño.
- Pruebas manuales documentadas para jornada normal, meta cumplida, cero unidades y lista vacía.

## Criterios de aceptación

- [ ] `cargo run` compila y ejecuta sin errores.
- [ ] El ingreso total coincide con la suma de los importes de todas las líneas.
- [ ] Las unidades vendidas y la línea con mayor ingreso son correctas.
- [ ] La clasificación cambia cuando los datos cruzan el límite de la meta.
- [ ] El programa no divide entre cero cuando no hay ventas.
- [ ] Los cálculos están en funciones y no todo queda dentro de `main`.
- [ ] El código usa arrays y tuplas, y no introduce todavía `Vec`, `struct` ni una base de datos.
- [ ] El reporte se entiende sin leer el código fuente.

## Plan de trabajo sugerido

1. Modela los datos y define las constantes.
2. Escribe las funciones de cálculo y prueba cada una con valores simples.
3. Implementa el recorrido y el acumulado.
4. Añade la clasificación del día y el formato del reporte.
5. Comprueba los casos límite y documenta las pruebas.

## Preguntas de revisión técnica

1. ¿Qué información debe ser una constante y cuál cambia durante el cálculo?
2. ¿Por qué es importante convertir a `f64` antes de calcular un promedio?
3. ¿Qué parte del diseño cambiaría si mañana la empresa necesitara añadir productos dinámicamente?
4. ¿Qué prueba detectaría un error por una condición de frontera en la meta diaria?

## Retos opcionales

- Genera un reporte por turno usando una función que reciba el array como argumento.
- Añade una alerta cuando una línea tenga unidades cero.
- Crea una función que devuelva mínimo y máximo usando una tupla.

## Fuera de alcance en este nivel

No implementes todavía persistencia, lectura de CSV, concurrencia, structs, colecciones dinámicas ni una API. Esos requisitos pertenecen a niveles posteriores; aquí importa demostrar los fundamentos con un programa pequeño y sólido.


---

# Solución de referencia completa

Crea la carpeta cierre_cafeteria con esta estructura:

~~~text
cierre_cafeteria/
├── Cargo.toml
└── src/
    └── main.rs
~~~

## Cargo.toml

~~~toml
[package]
name = "cierre_cafeteria"
version = "0.1.0"
edition = "2021"

[dependencies]
~~~

## src/main.rs

~~~rust
const META_DIARIA_COP: u64 = 300_000;
const CANTIDAD_VENTAS: usize = 6;

type Venta = (&'static str, u64, u32);

fn ingreso_linea(venta: &Venta) -> u64 {
    venta.1 * u64::from(venta.2)
}

fn calcular_resumen(ventas: &[Venta; CANTIDAD_VENTAS]) -> (u64, u32, usize) {
    let mut ingreso_total = 0_u64;
    let mut unidades_total = 0_u32;
    let mut indice_mayor_ingreso = 0_usize;
    let mut mayor_ingreso = 0_u64;

    for (indice, venta) in ventas.iter().enumerate() {
        let ingreso = ingreso_linea(venta);
        ingreso_total += ingreso;
        unidades_total += venta.2;

        if ingreso > mayor_ingreso {
            mayor_ingreso = ingreso;
            indice_mayor_ingreso = indice;
        }
    }

    (ingreso_total, unidades_total, indice_mayor_ingreso)
}

fn clasificar_cierre(ingreso_total: u64) -> &'static str {
    if ingreso_total == 0 {
        "Sin ventas"
    } else if ingreso_total >= META_DIARIA_COP {
        match ingreso_total >= META_DIARIA_COP * 2 {
            true => "Meta superada ampliamente",
            false => "Meta cumplida",
        }
    } else {
        "Por debajo de la meta"
    }
}

fn main() {
    let ventas: [Venta; CANTIDAD_VENTAS] = [
        ("Cafe americano", 4_500, 18),
        ("Capuchino", 6_500, 12),
        ("Chocolate", 5_000, 8),
        ("Croissant", 4_000, 10),
        ("Sandwich", 11_000, 7),
        ("Agua", 3_000, 9),
    ];

    let (ingreso_total, unidades_total, indice_mayor) = calcular_resumen(&ventas);

    println!("================================================");
    println!("            CAFE CENTRAL - CIERRE");
    println!("================================================");
    println!("{:<20} {:>8} {:>8} {:>12}", "Producto", "Precio", "Unid.", "Ingreso");
    println!("------------------------------------------------");

    for venta in &ventas {
        println!(
            "{:<20} {:>8} {:>8} {:>12}",
            venta.0,
            venta.1,
            venta.2,
            ingreso_linea(venta)
        );
    }

    let promedio_por_linea = if CANTIDAD_VENTAS > 0 {
        ingreso_total as f64 / CANTIDAD_VENTAS as f64
    } else {
        0.0
    };

    println!("------------------------------------------------");
    println!("Ingreso total:       {} COP", ingreso_total);
    println!("Unidades vendidas:   {}", unidades_total);
    println!("Promedio por linea:  {:.2} COP", promedio_por_linea);
    println!(
        "Mayor ingreso:       {} ({} COP)",
        ventas[indice_mayor].0,
        ingreso_linea(&ventas[indice_mayor])
    );
    println!("Meta diaria:         {} COP", META_DIARIA_COP);
    println!("Resultado:           {}", clasificar_cierre(ingreso_total));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcula_ingreso_por_linea() {
        let venta: Venta = ("Cafe", 4_500, 3);
        assert_eq!(ingreso_linea(&venta), 13_500);
    }

    #[test]
    fn clasifica_la_meta() {
        assert_eq!(clasificar_cierre(0), "Sin ventas");
        assert_eq!(clasificar_cierre(250_000), "Por debajo de la meta");
        assert_eq!(clasificar_cierre(300_000), "Meta cumplida");
        assert_eq!(clasificar_cierre(600_000), "Meta superada ampliamente");
    }
}
~~~

## Ejecutarlo

~~~bash
cargo run
cargo test
~~~

Con los datos incluidos, el ingreso total es 343.000 COP, se venden 64 unidades y el café americano genera el mayor ingreso individual: 81.000 COP. Son datos sintéticos de práctica.


---

# Solución de referencia — código completo

Esta implementación sirve como solución de referencia. Crea una carpeta llamada cierre_cafeteria, ejecuta cargo new cierre_cafeteria y reemplaza src/main.rs por este código. No necesita dependencias externas.

```rust
const META_DIARIA_COP: u64 = 330_000;

type Venta = (&'static str, u64, u32);

const VENTAS: [Venta; 6] = [
    ("Café americano", 4_500, 18),
    ("Capuchino", 6_500, 12),
    ("Chocolate", 5_000, 8),
    ("Croissant", 4_000, 10),
    ("Sándwich", 11_000, 7),
    ("Agua", 3_000, 9),
];

fn ingreso_de_linea(precio_cop: u64, unidades: u32) -> u64 {
    precio_cop * u64::from(unidades)
}

fn ingreso_total(ventas: &[Venta]) -> u64 {
    let mut total = 0_u64;
    for &(_, precio, unidades) in ventas {
        total += ingreso_de_linea(precio, unidades);
    }
    total
}

fn unidades_totales(ventas: &[Venta]) -> u32 {
    let mut total = 0_u32;
    for &(_, _, unidades) in ventas {
        total += unidades;
    }
    total
}

// La tupla devuelta indica (promedio, hay_datos).
fn promedio_por_linea(ventas: &[Venta]) -> (f64, bool) {
    if ventas.is_empty() {
        return (0.0, false);
    }
    (ingreso_total(ventas) as f64 / ventas.len() as f64, true)
}

fn linea_de_mayor_ingreso(ventas: &[Venta]) -> (&'static str, u64, bool) {
    if ventas.is_empty() {
        return ("Sin datos", 0, false);
    }

    let (mut nombre_mejor, precio_inicial, unidades_iniciales) = ventas[0];
    let mut mayor = ingreso_de_linea(precio_inicial, unidades_iniciales);

    for &(nombre, precio, unidades) in &ventas[1..] {
        let ingreso = ingreso_de_linea(precio, unidades);
        if ingreso > mayor {
            mayor = ingreso;
            nombre_mejor = nombre;
        }
    }
    (nombre_mejor, mayor, true)
}

fn clasificar_cierre(total_cop: u64) -> &'static str {
    if total_cop >= META_DIARIA_COP {
        "Meta cumplida"
    } else if total_cop == 0 {
        "Sin ventas"
    } else {
        "Por debajo de la meta"
    }
}

fn imprimir_reporte(ventas: &[Venta]) {
    println!("================================================");
    println!("            CIERRE DIARIO - CAFÉ CENTRAL");
    println!("================================================");

    for (indice, &(producto, precio, unidades)) in ventas.iter().enumerate() {
        let ingreso = ingreso_de_linea(precio, unidades);
        println!(
            "{:>2}. {:<20} {:>3} uds. x ${:>7} = ${:>9}",
            indice + 1, producto, unidades, precio, ingreso
        );
    }

    let total = ingreso_total(ventas);
    let unidades = unidades_totales(ventas);
    let (promedio, hay_promedio) = promedio_por_linea(ventas);
    let (producto_mayor, ingreso_mayor, hay_mayor) = linea_de_mayor_ingreso(ventas);

    println!("------------------------------------------------");
    println!("Unidades vendidas: {unidades}");
    println!("Ingreso total:     ${total}");
    println!("Meta diaria:       ${META_DIARIA_COP}");

    if hay_promedio {
        println!("Promedio por línea: ${promedio:.2}");
    } else {
        println!("Promedio por línea: no disponible (sin ventas)");
    }

    if hay_mayor {
        println!("Mayor ingreso: {producto_mayor} (${ingreso_mayor})");
    } else {
        println!("Mayor ingreso: no disponible (sin ventas)");
    }

    let estado = match clasificar_cierre(total) {
        "Meta cumplida" => "Objetivo diario alcanzado",
        "Sin ventas" => "No se registraron ventas",
        _ => "Revisar el objetivo diario",
    };
    println!("Resultado: {estado}");
    println!("================================================");
}

fn main() {
    imprimir_reporte(&VENTAS);

    // Caso vacío: no divide por cero ni accede a un elemento inexistente.
    let ventas_vacias: [Venta; 0] = [];
    let (promedio, hay_datos) = promedio_por_linea(&ventas_vacias);
    println!(
        "Prueba sin ventas: promedio={promedio}, hay_datos={hay_datos}, estado={}",
        clasificar_cierre(ingreso_total(&ventas_vacias))
    );
}
```

## Ejecutar y comprobar

```bash
cargo run
cargo check
```

Con los datos iniciales, el ingreso total debe ser **343.000 COP**, las unidades vendidas **64**, y el producto con mayor ingreso debe ser **Café americano**, con **81.000 COP**. El promedio por línea es aproximadamente **57.166,67 COP**.

## Qué demuestra la solución

- Mantiene las ventas en un array fijo de tuplas.
- Separa los cálculos en funciones.
- Maneja una lista vacía al calcular el promedio y el máximo.
- Guarda importes como enteros en pesos y convierte a f64 solamente para el promedio.
- Usa constantes, variables, bucles, operadores, condicionales, funciones, tuplas y arrays.

