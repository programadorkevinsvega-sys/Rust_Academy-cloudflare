---
title: "Proyecto Nivel 3: Analizador de eventos de auditoría sin copias innecesarias"
description: "Construye una herramienta para inspeccionar logs de acceso, detectar patrones sospechosos y aprovechar ownership, borrowing, slices y lifetimes."
language: rust
level: 3
levelLabel: "Nivel 03"
category: "Proyecto empresarial"
duration: "10–14 horas"
order: 3
slug: rust-nivel-3-auditoria-eventos
courseSlug: rust-nivel-3
company: "AndesPay (empresa fintech simulada)"
role: "Desarrollador/a del equipo de seguridad y observabilidad"
moduleName: "Módulos 1–8: stack/heap, ownership, move/Copy/Clone, borrowing, aliasing, slices, lifetimes y borrow checker"
---

# Misión del proyecto

## Código fuente ejecutable

El código de referencia explicado en este documento también está disponible como una aplicación Rust independiente, separada en las carpetas `modelo/`, `service/`, `controlador/` y `vista/`.

- [Abrir el proyecto y sus archivos fuente en GitHub](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/tree/main/proyectos/rust-nivel-3-auditoria-eventos)
- Ejecutar desde la raíz del repositorio: `cargo run --manifest-path proyectos/rust-nivel-3-auditoria-eventos/Cargo.toml`
- Ejecutar sus pruebas: `cargo test --manifest-path proyectos/rust-nivel-3-auditoria-eventos/Cargo.toml`

Esta versión utiliza datos simulados; no conecta con sistemas empresariales reales.

## Contexto de empresa

AndesPay debe revisar registros de acceso después de un incidente de seguridad. El sistema existente genera archivos de texto grandes, y el primer prototipo del equipo de soporte copia cada campo a nuevos `String`, aumentando el consumo de memoria y haciendo más compleja la administración de los datos.

Tu tarea es crear un analizador que recorra registros, identifique eventos relevantes y produzca un resumen para el equipo de seguridad. El requisito técnico central es procesar el texto con préstamos y slices siempre que sea posible, evitando clones que no aporten valor.

## Objetivo de negocio

Encontrar rápidamente accesos fallidos, cambios administrativos y patrones de actividad que merezcan revisión, conservando trazabilidad del registro original y usando memoria de forma consciente.

## Formato de entrada

Cada línea contiene cinco campos separados por `|`:

```text
2026-10-27T08:10:02Z|usuario_17|LOGIN|portal|OK
2026-10-27T08:11:14Z|usuario_17|LOGIN|portal|DENIED
2026-10-27T08:11:42Z|usuario_17|RESET_PASSWORD|portal|DENIED
2026-10-27T08:14:03Z|admin_2|CHANGE_ROLE|usuario_17|OK
```

Los registros son sintéticos y no contienen información personal real. Un archivo puede tener líneas malformadas, campos vacíos o eventos que no le interesan al informe.

## Alcance funcional

1. Leer o recibir un `String` con un conjunto de registros de prueba y mantenerlo vivo mientras se procesen sus referencias.
2. Crear un tipo `EventoAuditoria` que represente vistas de los campos mediante `&str`; si almacena referencias, declarar de manera explícita el lifetime que las vincula con el texto de origen.
3. Implementar una función que analice una línea y devuelva un evento prestado o indique que la línea no tiene el formato esperado.
4. Detectar intentos de acceso denegados, cambios de rol y usuarios con múltiples intentos fallidos en una ventana simulada.
5. Generar un informe con conteos y las líneas que requieren revisión, manteniendo una referencia o número de línea que permita volver al registro original.
6. Documentar qué datos se mueven, cuáles se copian por ser tipos `Copy`, cuándo una referencia deja de ser válida y por qué no se debe devolver una referencia a una variable local.
7. Separar las fases de lectura, análisis y reporte para que una mutación del texto no ocurra mientras existan préstamos que la impidan.
8. Evitar usar `.clone()` como solución automática a conflictos de borrowing; justificar cada copia de datos que sí sea necesaria.

## Reglas de negocio

- Una línea válida debe contener exactamente los campos esperados y un resultado reconocido.
- Una línea incompleta se contabiliza como malformada; no puede provocar que el proceso completo falle por un índice inválido.
- Un registro con resultado `DENIED` se considera fallido, pero no todos los fallos representan por sí solos un incidente confirmado.
- Tres o más fallos del mismo usuario en el conjunto de prueba generan una alerta para revisión.
- Los eventos de cambio de rol deben aparecer en una sección de auditoría administrativa.
- El texto original debe seguir existiendo mientras haya `EventoAuditoria` que lo referencien.
- No se permite modificar el buffer original durante una fase en la que las referencias a su contenido sigan activas.

## Diseño técnico esperado

```text
auditoria_eventos/
├── Cargo.toml
├── README.md
├── data/
│   └── eventos_demo.log
└── src/
    ├── main.rs
    ├── evento.rs
    ├── parser.rs
    └── reporte.rs
```

Puedes empezar con un texto constante y luego pasar a un archivo local. Si usas `std::fs::read_to_string`, conserva el `String` dueño durante todo el análisis. No guardes referencias a un buffer que luego se destruya o se reasigne.

## Relación con los módulos del Nivel 3

| Módulo | Cómo se aplica en el proyecto |
|---|---|
| 1. Stack y heap | Distinguir el buffer principal y los datos temporales del análisis. |
| 2. Ownership | Decidir quién posee el texto y quién solo lo consulta. |
| 3. Move y Copy | Mover resultados a colecciones y copiar solo los tipos simples cuando corresponde. |
| 4. Borrowing | Analizar con `&str` y pasar referencias a funciones. |
| 5. Aliasing y mutabilidad | No mezclar préstamos incompatibles ni modificar mientras se lee. |
| 6. Slices | Extraer los campos de cada línea sin crear un `String` por campo. |
| 7. Lifetimes | Vincular la vida de cada vista de evento al texto fuente. |
| 8. Borrow checker | Corregir de raíz los conflictos de préstamos y acotar sus ámbitos. |

## Entregables

- Aplicación Cargo que procesa un archivo o fixture de logs.
- Parser que separa claramente líneas válidas y malformadas.
- Informe de seguridad con conteos y eventos para revisar.
- README con el formato de entrada, el significado de cada alerta y las decisiones de ownership.
- Pruebas de líneas válidas, líneas incompletas, campos vacíos y varias alertas del mismo usuario.

## Criterios de aceptación

- [ ] Los campos se representan con slices prestados durante el análisis principal.
- [ ] Los tipos que guardan referencias tienen lifetimes correctos y no pueden sobrevivir al buffer original.
- [ ] El sistema detecta accesos denegados y cambios administrativos.
- [ ] Las líneas malformadas se informan sin causar un `panic` por acceso fuera de rango.
- [ ] Se puede explicar por qué una operación mueve un valor o solo lo presta.
- [ ] No se usa `.clone()` para esconder los problemas de borrowing.
- [ ] Las mutaciones del buffer solo ocurren cuando ya no hay referencias activas.
- [ ] El mismo archivo produce los mismos conteos en ejecuciones repetidas.

## Pruebas de aceptación sugeridas

1. Procesar cuatro líneas válidas y comprobar los conteos por resultado.
2. Añadir una línea que contenga solo tres campos: debe reportarse como malformada.
3. Añadir tres accesos denegados de un mismo usuario: debe generarse una alerta.
4. Añadir un evento `CHANGE_ROLE`: debe aparecer en la sección administrativa.
5. Intentar diseñar una función que devuelva una referencia a una variable local y explicar por qué el compilador la rechaza.

## Retos opcionales

- Añadir filtro por intervalo de tiempo sin modificar los campos originales.
- Ordenar las alertas por cantidad de fallos.
- Comparar el consumo conceptual de una solución basada en muchos `String` con otra basada en slices.

## Fuera de alcance

No implementes todavía autenticación, conexión a una plataforma SIEM ni persistencia en base de datos. Trabaja con registros sintéticos y enfócate en seguridad de memoria, claridad del análisis y trazabilidad.


---

# Solución de referencia completa

Crea auditoria_eventos. Si se proporciona la ruta de un archivo como argumento, el programa lee ese archivo; si no, procesa los datos demostrativos incluidos.

~~~text
auditoria_eventos/
├── Cargo.toml
├── data/
│   └── eventos_demo.log
└── src/
    ├── main.rs
    ├── evento.rs
    ├── parser.rs
    └── reporte.rs
~~~

## Cargo.toml

~~~toml
[package]
name = "auditoria_eventos"
version = "0.1.0"
edition = "2021"

[dependencies]
~~~

## src/evento.rs

~~~rust
#[derive(Debug)]
pub struct Evento<'a> {
    pub numero_linea: usize,
    pub timestamp: &'a str,
    pub usuario: &'a str,
    pub accion: &'a str,
    pub recurso: &'a str,
    pub resultado: &'a str,
}
~~~

## src/parser.rs

~~~rust
use crate::evento::Evento;

pub fn normalizar(texto: &mut String) {
    // Se llama antes de crear referencias al buffer.
    *texto = texto.to_lowercase();
}

pub fn parsear_linea<'a>(
    linea: &'a str,
    numero_linea: usize,
) -> Result<Evento<'a>, &'static str> {
    let campos: Vec<&str> = linea.split('|').map(str::trim).collect();

    if campos.len() != 5 {
        return Err("se esperaban cinco campos separados por |");
    }
    if campos.iter().any(|campo| campo.is_empty()) {
        return Err("hay uno o mas campos vacios");
    }

    let resultado = campos[4];
    if resultado != "ok" && resultado != "denied" && resultado != "blocked" {
        return Err("resultado desconocido; use ok, denied o blocked");
    }

    Ok(Evento {
        numero_linea,
        timestamp: campos[0],
        usuario: campos[1],
        accion: campos[2],
        recurso: campos[3],
        resultado,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analiza_evento_valido() {
        let texto = String::from("2026-10-27t08:10:02z|usuario_17|login|portal|ok");
        let evento = parsear_linea(&texto, 1).unwrap();
        assert_eq!(evento.usuario, "usuario_17");
        assert_eq!(evento.resultado, "ok");
    }

    #[test]
    fn rechaza_una_linea_incompleta() {
        assert!(parsear_linea("fecha|usuario|login", 3).is_err());
    }
}
~~~

## src/reporte.rs

~~~rust
use std::collections::HashMap;

use crate::evento::Evento;

pub fn imprimir(eventos: &[Evento<'_>]) {
    println!("=== INFORME DE AUDITORIA ===");
    println!("Eventos validos: {}", eventos.len());

    let denegados = eventos.iter()
        .filter(|evento| evento.resultado == "denied")
        .count();
    let cambios_rol = eventos.iter()
        .filter(|evento| evento.accion == "change_role")
        .count();

    println!("Accesos denegados: {denegados}");
    println!("Cambios de rol: {cambios_rol}");

    println!("\nEventos que requieren revision:");
    for evento in eventos {
        if evento.resultado == "denied"
            || evento.resultado == "blocked"
            || evento.accion == "change_role"
        {
            println!(
                "linea {} | {} | usuario={} | accion={} | recurso={} | resultado={}",
                evento.numero_linea, evento.timestamp, evento.usuario,
                evento.accion, evento.recurso, evento.resultado
            );
        }
    }

    let mut fallos_por_usuario: HashMap<&str, usize> = HashMap::new();
    for evento in eventos.iter().filter(|evento| evento.resultado == "denied") {
        *fallos_por_usuario.entry(evento.usuario).or_insert(0) += 1;
    }

    let mut alertas: Vec<(&str, usize)> = fallos_por_usuario.into_iter()
        .filter(|(_, cantidad)| *cantidad >= 3)
        .collect();
    alertas.sort_by(|a, b| a.0.cmp(b.0));

    println!("\nAlertas por multiples fallos (tres o mas):");
    if alertas.is_empty() {
        println!("No se encontraron usuarios sobre el umbral.");
    } else {
        for (usuario, cantidad) in alertas {
            println!("ALERTA: {usuario} acumula {cantidad} accesos denegados");
        }
    }
}
~~~

## src/main.rs

~~~rust
mod evento;
mod parser;
mod reporte;

use std::{env, error::Error, fs};

use parser::{normalizar, parsear_linea};

const DATOS_DEMO: &str = "\
2026-10-27T08:10:02Z|usuario_17|LOGIN|portal|OK
2026-10-27T08:11:14Z|usuario_17|LOGIN|portal|DENIED
2026-10-27T08:11:42Z|usuario_17|RESET_PASSWORD|portal|DENIED
2026-10-27T08:12:02Z|usuario_17|LOGIN|portal|DENIED
2026-10-27T08:14:03Z|admin_2|CHANGE_ROLE|usuario_17|OK
2026-10-27T08:15:03Z|usuario_22|LOGIN|portal|OK
linea malformada
2026-10-27T08:17:03Z|usuario_23|LOGIN|portal|BLOCKED";

fn main() -> Result<(), Box<dyn Error>> {
    let argumento = env::args().nth(1);
    let mut texto = match argumento {
        Some(ruta) => fs::read_to_string(ruta)?,
        None => DATOS_DEMO.to_string(),
    };

    normalizar(&mut texto);

    {
        // Los eventos guardan referencias a segmentos de texto, no Strings propios.
        // El buffer original debe vivir e permanecer inmutable durante este bloque.
        let mut eventos = Vec::new();
        let mut lineas_invalidas = 0_usize;

        for (indice, linea) in texto.lines().enumerate() {
            if linea.trim().is_empty() {
                continue;
            }

            match parsear_linea(linea, indice + 1) {
                Ok(evento) => eventos.push(evento),
                Err(error) => {
                    lineas_invalidas += 1;
                    eprintln!("Linea {} ignorada: {}", indice + 1, error);
                }
            }
        }

        println!("Lineas invalidas: {lineas_invalidas}\n");
        reporte::imprimir(&eventos);
    } // Los préstamos terminan al salir del bloque.

    // Una vez terminados los préstamos, se puede modificar el String dueño.
    texto.push_str("\n# buffer reutilizado tras el analisis");
    println!("\nBuffer dueño disponible: {} bytes", texto.len());

    Ok(())
}
~~~

## data/eventos_demo.log

~~~text
2026-10-27T08:10:02Z|usuario_17|LOGIN|portal|OK
2026-10-27T08:11:14Z|usuario_17|LOGIN|portal|DENIED
2026-10-27T08:11:42Z|usuario_17|RESET_PASSWORD|portal|DENIED
2026-10-27T08:12:02Z|usuario_17|LOGIN|portal|DENIED
2026-10-27T08:14:03Z|admin_2|CHANGE_ROLE|usuario_17|OK
2026-10-27T08:15:03Z|usuario_22|LOGIN|portal|OK
linea malformada
2026-10-27T08:17:03Z|usuario_23|LOGIN|portal|BLOCKED
~~~

## Ejecutarlo

~~~bash
cargo run
cargo run -- data/eventos_demo.log
cargo test
~~~

La solución utiliza referencias y slices para los campos del evento. Los valores de Evento solo pueden existir mientras viva el String que contiene el texto. Los registros y las alertas son demostrativos, no son un sistema de seguridad de producción.


---
