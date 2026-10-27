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

La implementación ejecutable está separada en capas. No concentres las reglas del negocio en `src/main.rs`: el punto de entrada solo declara los módulos y llama al controlador.

## Arquitectura requerida

```text
rust-nivel-3-auditoria-eventos/
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

- **Modelo:** define la estructura de un evento de auditoría y los campos que deben estar presentes.
- **Service:** analiza las líneas, valida los campos, clasifica los eventos y calcula estadísticas de fallos.
- **Controlador:** obtiene la entrada de demostración o el archivo indicado y coordina el análisis.
- **Vista:** presenta el resumen de eventos y las incidencias detectadas.
- **main.rs:** declara las capas y ejecuta el controlador.

## Código fuente por archivo

- [Cargo.toml](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-3-auditoria-eventos/Cargo.toml)
- [src/main.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-3-auditoria-eventos/src/main.rs)
- [Modelo: src/modelo/mod.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-3-auditoria-eventos/src/modelo/mod.rs)
- [Service: src/service/mod.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-3-auditoria-eventos/src/service/mod.rs)
- [Vista: src/vista/mod.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-3-auditoria-eventos/src/vista/mod.rs)
- [Controlador: src/controlador/mod.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-3-auditoria-eventos/src/controlador/mod.rs)

## Ejecutar y probar

Desde la raíz del repositorio:

```bash
cargo run --manifest-path proyectos/rust-nivel-3-auditoria-eventos/Cargo.toml
cargo test --manifest-path proyectos/rust-nivel-3-auditoria-eventos/Cargo.toml
```

Los registros incluidos son de demostración. En producción, los eventos deben proceder de una fuente controlada y conservarse de forma íntegra; el parser debe informar las líneas inválidas sin ocultar errores.
