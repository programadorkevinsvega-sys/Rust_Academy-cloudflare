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

La solución ejecutable está organizada por capas. **No debes crear el proyecto monolítico con todo el código dentro de `src/main.rs` que se muestra en ejemplos antiguos**. La implementación vigente está en la carpeta `proyectos/rust-nivel-4-conciliacion-facturas/` del repositorio.

## Arquitectura requerida

```text
rust-nivel-4-conciliacion-facturas/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── modelo/
    │   └── mod.rs
    ├── vista/
    │   └── mod.rs
    ├── controlador/
    │   └── mod.rs
    └── service/
        └── mod.rs
```

- **Modelo:** representa facturas, órdenes de compra y errores del dominio.
- **Service:** parsea las líneas, valida importes, detecta duplicados y concilia cada factura contra su orden de compra.
- **Controlador:** prepara los datos de demostración y coordina el procesamiento estricto y tolerante.
- **Vista:** imprime el reporte de facturas aceptadas y las incidencias.
- **main.rs:** declara los módulos e inicia el controlador; no contiene las reglas del negocio.

## Código fuente por archivo

- [Cargo.toml](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-4-conciliacion-facturas/Cargo.toml)
- [src/main.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-4-conciliacion-facturas/src/main.rs)
- [Modelo: src/modelo/mod.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-4-conciliacion-facturas/src/modelo/mod.rs)
- [Service: src/service/mod.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-4-conciliacion-facturas/src/service/mod.rs)
- [Vista: src/vista/mod.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-4-conciliacion-facturas/src/vista/mod.rs)
- [Controlador: src/controlador/mod.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-4-conciliacion-facturas/src/controlador/mod.rs)

## Ejecutar y probar

Desde la raíz del repositorio:

```bash
cargo run --manifest-path proyectos/rust-nivel-4-conciliacion-facturas/Cargo.toml
cargo test --manifest-path proyectos/rust-nivel-4-conciliacion-facturas/Cargo.toml
```

Las órdenes de compra son datos de demostración en memoria. Antes de utilizar el conciliador en producción, la empresa debe conectar una fuente fiable de órdenes y verificar sus reglas tributarias.
