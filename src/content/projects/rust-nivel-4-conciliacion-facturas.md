---
title: "Proyecto Nivel 4: Conciliador seguro de facturas de proveedores"
description: "Procesa lotes de facturas, valida importes y órdenes de compra, y entrega errores claros sin que un dato defectuoso tumbe el proceso."
language: rust
level: 4
levelLabel: "Nivel 04"
category: "Proyecto empresarial"
duration: "12–16 horas"
order: 4
slug: rust-nivel-4-conciliacion-facturas
courseSlug: rust-nivel-4
company: "Logística Andina (empresa simulada)"
role: "Desarrollador/a del equipo de finanzas y controles"
moduleName: "Módulos 1–9: Option, Result, patrones, operador ?, panic, unwrap/expect, errores personalizados y diseño de código seguro"
---

# Misión del proyecto

## Código fuente ejecutable

El código de referencia explicado en este documento también está disponible como una aplicación Rust independiente, separada en las carpetas `modelo/`, `service/`, `controlador/` y `vista/`.

- [Abrir el proyecto y sus archivos fuente en GitHub](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/tree/main/proyectos/rust-nivel-4-conciliacion-facturas)
- Ejecutar desde la raíz del repositorio: `cargo run --manifest-path proyectos/rust-nivel-4-conciliacion-facturas/Cargo.toml`
- Ejecutar sus pruebas: `cargo test --manifest-path proyectos/rust-nivel-4-conciliacion-facturas/Cargo.toml`

Esta versión utiliza datos simulados; no conecta con sistemas empresariales reales.

## Contexto de empresa

Logística Andina recibe cada día archivos de facturas de distintos proveedores. Los archivos llegan con campos vacíos, identificadores repetidos, importes mal digitados y órdenes de compra que no coinciden. El proceso actual depende de hojas de cálculo y falla de forma poco clara cuando encuentra una fila incorrecta.

El equipo de finanzas necesita un procesador de lotes que valide cada registro, concilie los importes y entregue un informe auditable. Tu responsabilidad es diseñar el flujo para que los errores esperables sean valores explícitos, no caídas inesperadas del programa.

## Objetivo de negocio

Reducir las facturas aceptadas con información inválida, identificar diferencias con las órdenes de compra y permitir que el equipo corrija registros con mensajes accionables.

## Formato de entrada

Recibirás un archivo delimitado por punto y coma con los campos:

```text
factura_id;proveedor_id;orden_id;subtotal_cop;impuesto_cop;total_cop
FAC-1001;PROV-10;OC-8001;250000;47500;297500
FAC-1002;PROV-12;OC-8002;180000;34200;214200
FAC-1003;PROV-10;OC-8001;250000;47500;310000
FAC-1004;PROV-14;OC-9999;90000;17100;107100
FAC-1005;PROV-10;OC-8003;abc;0;0
```

El conjunto es sintético. Los valores de impuesto son datos de prueba y no deben interpretarse como asesoría tributaria ni como una regla fiscal universal.

## Alcance funcional

1. Definir tipos de dominio para una factura, una orden de compra y el resultado de conciliación.
2. Parsear cada línea y validar el número de campos, los identificadores y los importes enteros no negativos.
3. Comprobar la suma de subtotal e impuesto usando operaciones aritméticas verificadas, por ejemplo `checked_add`, para detectar desbordamientos.
4. Buscar la orden de compra correspondiente y comparar proveedor, orden, subtotal y total según las reglas configuradas.
5. Detectar facturas duplicadas dentro del lote.
6. Implementar dos modos: estricto, que se detiene ante el primer error; y tolerante, que conserva las filas válidas y recopila errores con número de línea y contexto.
7. Definir errores propios con variantes claras; implementar `Display` y `Error` y conversiones con `From` donde simplifiquen el uso de `?`.
8. Usar `Option` para búsquedas que pueden no encontrar una orden y `Result` para operaciones que pueden fallar.
9. Separar errores de formato, reglas comerciales y fallos de operación; no reemplazarlos por mensajes genéricos.
10. Añadir pruebas unitarias para entradas válidas y caminos de error. No uses `unwrap()` sobre el contenido de entrada.

## Reglas de negocio

- Todos los identificadores requeridos deben estar presentes.
- Los campos monetarios se representan como enteros en COP para no introducir errores binarios de punto flotante en sumas de dinero.
- El total registrado debe coincidir con subtotal más impuesto según la regla del proyecto.
- Una factura con un identificador ya procesado se marca como duplicada y no se vuelve a contabilizar.
- Una orden inexistente no se puede considerar conciliada.
- Una diferencia de importe se reporta con el identificador de factura y los valores esperados y recibidos.
- En modo tolerante, una línea defectuosa no elimina del informe las demás líneas válidas.
- Los errores de datos externos no deben provocar `panic!`; el uso de `expect` solo se permite si existe una invariante documentada y demostrable.

## Entregables

- Proyecto Cargo `conciliador_facturas`.
- Parser y modelo de dominio separados de la lógica de conciliación.
- Fixture de órdenes de compra aprobadas y archivo de facturas para pruebas.
- Informe con facturas conciliadas, duplicadas, no encontradas, con diferencias y con errores de formato.
- Pruebas automatizadas y README con ejemplos de ejecución y los límites del sistema.

## Relación con los módulos del Nivel 4

| Módulo | Cómo se aplica en el proyecto |
|---|---|
| 1. Filosofía de errores | Los problemas de entrada se modelan y comunican, no se esconden. |
| 2. `Option<T>` | Una orden puede no existir; una consulta puede no tener resultado. |
| 3. `Result<T, E>` | El parseo y las operaciones de negocio notifican fallos recuperables. |
| 4. Pattern matching | Se desestructuran resultados y se tratan explícitamente variantes de error. |
| 5. `?` | Se propagan fallos al nivel que tiene contexto para decidir. |
| 6. `panic!` | Se evita ante archivos externos y se reserva para situaciones realmente irrecuperables. |
| 7. `unwrap` y `expect` | Se eliminan de rutas con datos externos o se justifican mediante invariantes en pruebas. |
| 8. Errores personalizados | Tipos propios, `Display`, `Error`, `From` y contexto de línea. |
| 9. Código seguro | Validación en el borde, aritmética comprobada, acceso seguro y pruebas negativas. |

## Criterios de aceptación

- [ ] Una factura válida se concilia si coincide con su orden aprobada.
- [ ] Las diferencias y órdenes inexistentes aparecen con un mensaje accionable.
- [ ] Las facturas duplicadas se identifican sin contabilizarlas dos veces.
- [ ] El modo tolerante informa todas las filas defectuosas y conserva las válidas.
- [ ] El modo estricto devuelve el primer error con número de línea.
- [ ] Una suma que desborda el tipo numérico se informa como error.
- [ ] Ninguna entrada defectuosa provoca `panic` por un `unwrap()` o un índice fuera de rango.
- [ ] Los errores se prueban y se muestran con `Display`, no solo con la representación de depuración.
- [ ] Las pruebas cubren por lo menos cinco caminos negativos.

## Plan de trabajo sugerido

1. Define las estructuras, enums y errores del dominio.
2. Implementa y prueba el parser de una sola línea.
3. Agrega validación de sumas y duplicados.
4. Implementa conciliación contra las órdenes de compra.
5. Añade los modos estricto y tolerante.
6. Escribe pruebas de casos límite y documenta cómo se recupera el equipo de cada error.

## Retos opcionales

- Generar un archivo de salida con registros listos para revisión humana.
- Añadir un umbral de diferencia permitido configurable para conciliaciones, dejando clara la regla aplicada.
- Añadir un resumen por proveedor sin perder trazabilidad de la línea de origen.

## Fuera de alcance

No automatices pagos ni conectes sistemas bancarios. El sistema entrega una decisión de conciliación auditable; la autorización de pago sigue siendo un proceso separado.


---

# Solución de referencia completa

Crea el proyecto conciliador_facturas:

~~~text
conciliador_facturas/
├── Cargo.toml
└── src/
    └── main.rs
~~~

## Cargo.toml

~~~toml
[package]
name = "conciliador_facturas"
version = "0.1.0"
edition = "2021"

[dependencies]
~~~

## src/main.rs

~~~rust
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
struct OrdenCompra {
    proveedor_id: String,
    subtotal_cop: u64,
}

#[derive(Debug, Clone)]
struct Factura {
    factura_id: String,
    proveedor_id: String,
    orden_id: String,
    subtotal_cop: u64,
    impuesto_cop: u64,
    total_cop: u64,
}

#[derive(Debug)]
enum ErrorLinea {
    CamposIncorrectos(usize),
    CampoVacio(&'static str),
    NumeroInvalido { campo: &'static str, valor: String },
    Desbordamiento,
    TotalIncoherente { esperado: u64, recibido: u64 },
    FacturaDuplicada(String),
    OrdenNoEncontrada(String),
    ProveedorNoCoincide { factura: String, orden: String },
    SubtotalNoCoincide { factura: u64, orden: u64 },
}

impl fmt::Display for ErrorLinea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorLinea::CamposIncorrectos(n) =>
                write!(f, "se esperaban 6 campos y llegaron {n}"),
            ErrorLinea::CampoVacio(campo) =>
                write!(f, "el campo {campo} esta vacio"),
            ErrorLinea::NumeroInvalido { campo, valor } =>
                write!(f, "{campo} no es un entero valido: {valor}"),
            ErrorLinea::Desbordamiento =>
                write!(f, "subtotal mas impuesto excede el rango permitido"),
            ErrorLinea::TotalIncoherente { esperado, recibido } =>
                write!(f, "total incoherente: se esperaba {esperado} y llego {recibido}"),
            ErrorLinea::FacturaDuplicada(id) =>
                write!(f, "la factura {id} esta duplicada"),
            ErrorLinea::OrdenNoEncontrada(id) =>
                write!(f, "no existe la orden de compra {id}"),
            ErrorLinea::ProveedorNoCoincide { factura, orden } =>
                write!(f, "el proveedor de la factura ({factura}) no coincide con el de la orden ({orden})"),
            ErrorLinea::SubtotalNoCoincide { factura, orden } =>
                write!(f, "subtotal de factura {factura} no coincide con el de la orden {orden}"),
        }
    }
}

impl Error for ErrorLinea {}

#[derive(Debug)]
struct ErrorProceso {
    linea: usize,
    causa: ErrorLinea,
}

impl fmt::Display for ErrorProceso {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "linea {}: {}", self.linea, self.causa)
    }
}

impl Error for ErrorProceso {}

fn leer_numero(campo: &'static str, valor: &str) -> Result<u64, ErrorLinea> {
    valor.trim().parse::<u64>().map_err(|_| ErrorLinea::NumeroInvalido {
        campo,
        valor: valor.trim().to_string(),
    })
}

fn parsear_linea(linea: &str) -> Result<Factura, ErrorLinea> {
    let campos: Vec<&str> = linea.split(';').map(str::trim).collect();
    if campos.len() != 6 {
        return Err(ErrorLinea::CamposIncorrectos(campos.len()));
    }

    let nombres = [
        "factura_id",
        "proveedor_id",
        "orden_id",
        "subtotal_cop",
        "impuesto_cop",
        "total_cop",
    ];
    for (indice, valor) in campos.iter().enumerate() {
        if valor.is_empty() {
            return Err(ErrorLinea::CampoVacio(nombres[indice]));
        }
    }

    let subtotal_cop = leer_numero("subtotal_cop", campos[3])?;
    let impuesto_cop = leer_numero("impuesto_cop", campos[4])?;
    let total_cop = leer_numero("total_cop", campos[5])?;
    let esperado = subtotal_cop
        .checked_add(impuesto_cop)
        .ok_or(ErrorLinea::Desbordamiento)?;

    if total_cop != esperado {
        return Err(ErrorLinea::TotalIncoherente {
            esperado,
            recibido: total_cop,
        });
    }

    Ok(Factura {
        factura_id: campos[0].to_string(),
        proveedor_id: campos[1].to_string(),
        orden_id: campos[2].to_string(),
        subtotal_cop,
        impuesto_cop,
        total_cop,
    })
}

fn validar_conciliacion(
    factura: &Factura,
    ordenes: &HashMap<String, OrdenCompra>,
) -> Result<(), ErrorLinea> {
    let orden = ordenes
        .get(&factura.orden_id)
        .ok_or_else(|| ErrorLinea::OrdenNoEncontrada(factura.orden_id.clone()))?;

    if factura.proveedor_id != orden.proveedor_id {
        return Err(ErrorLinea::ProveedorNoCoincide {
            factura: factura.proveedor_id.clone(),
            orden: orden.proveedor_id.clone(),
        });
    }

    if factura.subtotal_cop != orden.subtotal_cop {
        return Err(ErrorLinea::SubtotalNoCoincide {
            factura: factura.subtotal_cop,
            orden: orden.subtotal_cop,
        });
    }

    Ok(())
}

fn procesar_estricto(
    contenido: &str,
    ordenes: &HashMap<String, OrdenCompra>,
) -> Result<Vec<Factura>, ErrorProceso> {
    let mut aceptadas = Vec::new();
    let mut ids = HashSet::new();

    for (indice, linea) in contenido.lines().enumerate() {
        if linea.trim().is_empty() {
            continue;
        }

        let resultado = (|| {
            let factura = parsear_linea(linea)?;
            if !ids.insert(factura.factura_id.clone()) {
                return Err(ErrorLinea::FacturaDuplicada(factura.factura_id.clone()));
            }
            validar_conciliacion(&factura, ordenes)?;
            Ok(factura)
        })();

        match resultado {
            Ok(factura) => aceptadas.push(factura),
            Err(causa) => return Err(ErrorProceso { linea: indice + 1, causa }),
        }
    }

    Ok(aceptadas)
}

fn procesar_tolerante(
    contenido: &str,
    ordenes: &HashMap<String, OrdenCompra>,
) -> (Vec<Factura>, Vec<ErrorProceso>) {
    let mut aceptadas = Vec::new();
    let mut errores = Vec::new();
    let mut ids = HashSet::new();

    for (indice, linea) in contenido.lines().enumerate() {
        if linea.trim().is_empty() {
            continue;
        }

        let resultado = (|| {
            let factura = parsear_linea(linea)?;
            if !ids.insert(factura.factura_id.clone()) {
                return Err(ErrorLinea::FacturaDuplicada(factura.factura_id.clone()));
            }
            validar_conciliacion(&factura, ordenes)?;
            Ok(factura)
        })();

        match resultado {
            Ok(factura) => aceptadas.push(factura),
            Err(causa) => errores.push(ErrorProceso { linea: indice + 1, causa }),
        }
    }

    (aceptadas, errores)
}

const DATOS: &str = "\
FAC-1001;PROV-10;OC-8001;250000;47500;297500
FAC-1002;PROV-12;OC-8002;180000;34200;214200
FAC-1001;PROV-10;OC-8001;250000;47500;297500
FAC-1004;PROV-14;OC-9999;90000;17100;107100
FAC-1005;PROV-10;OC-8001;abc;0;0
FAC-1006;PROV-10;OC-8001;250000;47500;310000
FAC-1007;PROV-12;OC-8001;250000;47500;297500
FAC-1008;PROV-10;OC-8001;240000;45600;285600";

fn ordenes_demo() -> HashMap<String, OrdenCompra> {
    HashMap::from([
        ("OC-8001".to_string(), OrdenCompra {
            proveedor_id: "PROV-10".to_string(),
            subtotal_cop: 250_000,
        }),
        ("OC-8002".to_string(), OrdenCompra {
            proveedor_id: "PROV-12".to_string(),
            subtotal_cop: 180_000,
        }),
    ])
}

fn main() -> Result<(), Box<dyn Error>> {
    let ordenes = ordenes_demo();

    match procesar_estricto(DATOS, &ordenes) {
        Ok(facturas) => println!("Modo estricto: {} facturas conciliadas", facturas.len()),
        Err(error) => println!("Modo estricto se detuvo: {error}"),
    }

    let (facturas, errores) = procesar_tolerante(DATOS, &ordenes);
    println!("\n=== REPORTE TOLERANTE ===");
    println!("Facturas conciliadas: {}", facturas.len());
    println!("Filas rechazadas: {}", errores.len());

    let total_validado: u64 = facturas.iter().map(|f| f.total_cop).sum();
    println!("Total de facturas conciliadas: {} COP", total_validado);

    for factura in &facturas {
        println!(
            "OK {} | proveedor={} | orden={} | subtotal={} COP | impuesto={} COP | total={} COP",
            factura.factura_id, factura.proveedor_id, factura.orden_id,
            factura.subtotal_cop, factura.impuesto_cop, factura.total_cop
        );
    }

    println!("\nIncidencias para corregir:");
    for error in errores {
        println!("  - {error}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acepta_una_factura_con_total_correcto() {
        let factura = parsear_linea("FAC-1;PROV-1;OC-1;100;19;119");
        assert!(factura.is_ok());
    }

    #[test]
    fn rechaza_un_total_incoherente() {
        let error = parsear_linea("FAC-1;PROV-1;OC-1;100;19;120");
        assert!(matches!(error, Err(ErrorLinea::TotalIncoherente { .. })));
    }

    #[test]
    fn rechaza_importe_no_numerico() {
        assert!(parsear_linea("FAC-1;PROV-1;OC-1;abc;19;119").is_err());
    }

    #[test]
    fn detecta_una_orden_inexistente() {
        let factura = parsear_linea("FAC-1;PROV-1;OC-X;100;19;119").unwrap();
        let error = validar_conciliacion(&factura, &HashMap::new());
        assert!(matches!(error, Err(ErrorLinea::OrdenNoEncontrada(_))));
    }
}
~~~

## Ejecutarlo

~~~bash
cargo run
cargo test
~~~

La lista de órdenes de compra está simulada en memoria. En la práctica, la empresa debería validar sus reglas tributarias y conectar una fuente fiable de órdenes antes de usar un procesador así con información real.


---
