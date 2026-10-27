---
title: "Proyecto Nivel 2: Inventario y abastecimiento para una cadena retail"
description: "Diseña una aplicación de consola modular para controlar productos, existencias, movimientos de inventario y alertas de reposición."
language: rust
level: 2
levelLabel: "Nivel 02"
category: "Proyecto empresarial"
duration: "12–16 horas"
order: 2
slug: rust-nivel-2-inventario-logistico
courseSlug: rust-nivel-2
company: "Norte Distribución (empresa simulada)"
role: "Desarrollador/a junior del equipo de logística"
moduleName: "Módulos 0–8: ownership y referencias, structs, enums, impl, módulos, Cargo, colecciones, genéricos y traits"
---

# Misión del proyecto

## Código fuente ejecutable

El código de referencia explicado en este documento también está disponible como una aplicación Rust independiente, separada en las carpetas `modelo/`, `service/`, `controlador/` y `vista/`.

- [Abrir el proyecto y sus archivos fuente en GitHub](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/tree/main/proyectos/rust-nivel-2-inventario-logistico)
- Ejecutar desde la raíz del repositorio: `cargo run --manifest-path proyectos/rust-nivel-2-inventario-logistico/Cargo.toml`
- Ejecutar sus pruebas: `cargo test --manifest-path proyectos/rust-nivel-2-inventario-logistico/Cargo.toml`

Esta versión utiliza datos simulados; no conecta con sistemas empresariales reales.

## Contexto de empresa

Norte Distribución abastece cuatro tiendas y necesita una herramienta interna para mantener un catálogo único de productos. El equipo reporta problemas frecuentes: referencias duplicadas, ventas que dejan el stock negativo y productos por debajo del mínimo de reposición que no aparecen en el informe diario.

Te incorporas al equipo que desarrollará un prototipo de gestión de inventario. La primera entrega será una aplicación de consola, con datos en memoria y una arquitectura que permita sumar almacenamiento persistente más adelante. No se requiere base de datos ni servicio web en este nivel.

## Objetivo de negocio

Proporcionar una fuente de verdad para el catálogo y las existencias, rechazar movimientos inválidos y entregar informes de reposición que el equipo de bodega pueda revisar.

## Alcance funcional

1. Modelar `Producto` con identificador, SKU, nombre, categoría, precio, stock actual y stock mínimo.
2. Modelar las categorías y los estados del producto con `enum`; no representar estados importantes con números mágicos.
3. Implementar métodos para construir productos válidos, calcular valor de inventario, recibir unidades, registrar salidas y determinar si se necesita reposición.
4. Mantener el catálogo en un `Vec<Producto>` y usar `HashMap` para consultas por SKU o acumulados por categoría cuando tenga sentido.
5. Usar `HashSet` para detectar SKU duplicados durante una carga inicial.
6. Diseñar módulos separados para modelo, inventario, reportes y punto de entrada.
7. Aplicar `Option` cuando una búsqueda puede no encontrar producto y `Result` cuando una operación puede fallar, como vender más unidades de las disponibles.
8. Definir un trait de presentación o reporte que permita mostrar productos con un formato consistente.
9. Crear una función o struct genérico pequeño y útil, con límites de traits explícitos; evita genéricos añadidos solo para cumplir una lista.
10. Añadir pruebas unitarias para reglas de inventario críticas.

## Historias de usuario

- Como auxiliar de bodega, quiero registrar un producto para que tenga SKU único y datos válidos.
- Como vendedor, quiero registrar una salida para no permitir que el stock quede negativo.
- Como responsable de compras, quiero consultar los productos por debajo del stock mínimo para preparar pedidos de reposición.
- Como jefe de tienda, quiero ver el valor total del inventario y el total de unidades por categoría.
- Como auditor, quiero distinguir un producto no encontrado de una venta rechazada por falta de stock.

## Reglas de negocio

- El SKU no puede estar vacío y no puede repetirse.
- El precio debe ser mayor que cero.
- No se permite una salida superior al stock actual.
- Una entrada incrementa el stock; una salida lo reduce solo después de validar que es posible.
- La ausencia de un producto es un resultado normal de búsqueda y se representa explícitamente.
- Los errores deben informar qué operación falló; no se deben ocultar con `unwrap()` en la lógica del negocio.
- Los reportes deben poder probarse con un conjunto pequeño y conocido de productos.

## Diseño técnico esperado

```text
inventario_logistico/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── producto.rs
    ├── inventario.rs
    └── reportes.rs
```

El proyecto debe compilar con Cargo y aprovechar la visibilidad de módulos: los campos internos que no deban modificarse libremente deben permanecer privados y exponerse mediante métodos.

## Relación con los módulos del Nivel 2

| Módulo | Cómo se aplica en el proyecto |
|---|---|
| 0. Puente | Ownership, referencias, slices, `Option` y `Result` para evitar copias innecesarias y modelar fallos. |
| 1. Structs | Tipos del dominio para producto, movimiento y resumen. |
| 2. Enums | Categorías, tipo de movimiento y resultado de operaciones. |
| 3. Métodos e `impl` | Constructores validados y operaciones de inventario. |
| 4. Módulos | Separación del modelo, reglas del inventario y reportes. |
| 5. Cargo | Paquete, comandos de build y pruebas. |
| 6. Colecciones | `Vec`, `HashMap` y `HashSet` para catálogo, búsquedas y duplicados. |
| 7. Genéricos | Una utilidad genérica útil para filtrar, resumir o presentar elementos. |
| 8. Traits | Comportamiento de presentación o exportación implementado por los tipos del dominio. |

## Entregables

- Código fuente con los módulos indicados o una estructura equivalente justificada.
- Datos semilla para al menos 12 productos distribuidos en varias categorías.
- Informe de inventario, informe de reposición y resumen por categoría.
- Pruebas unitarias y README con decisiones de diseño, comandos y ejemplos de uso.
- Registro claro de operaciones rechazadas y motivo del rechazo.

## Criterios de aceptación

- [ ] No se pueden registrar dos productos con el mismo SKU.
- [ ] Los productos con precio inválido son rechazados.
- [ ] No se puede realizar una salida que exceda el stock disponible.
- [ ] Una búsqueda inexistente devuelve `None` o una respuesta explícita equivalente.
- [ ] Los informes reflejan correctamente las entradas y salidas registradas.
- [ ] `cargo test` cubre como mínimo alta válida, SKU duplicado, stock insuficiente y reposición.
- [ ] La lógica de dominio no depende de `println!` para comunicar resultados a otras funciones.
- [ ] Los módulos exponen una API pequeña y coherente.

## Pruebas de aceptación sugeridas

1. Crear una referencia `SKU-100` con 25 unidades y mínimo 8.
2. Registrar una salida de 7 unidades: el stock debe quedar en 18.
3. Intentar una salida de 30 unidades: debe rechazarse y conservarse el stock anterior.
4. Recibir 5 unidades y validar el nuevo stock.
5. Intentar registrar otra referencia con `SKU-100`: debe fallar sin modificar el catálogo.
6. Crear un producto con stock inferior al mínimo y comprobar que aparece en reposición.

## Retos opcionales

- Crear un historial de movimientos con fecha simulada y usuario responsable.
- Añadir ordenamiento determinista por SKU o valor de inventario.
- Preparar una capa de almacenamiento detrás de un trait, sin implementar una base de datos todavía.

## Fuera de alcance

No implementes una API HTTP, base de datos ni sincronización entre tiendas. La meta es un dominio bien modelado, con reglas comprobables y un código que otro desarrollador pueda ampliar.

---

# Solución de referencia completa

La implementación ejecutable está separada en capas. No concentres las reglas del negocio en `src/main.rs`: el punto de entrada solo declara los módulos y llama al controlador.

## Arquitectura requerida

```text
rust-nivel-2-inventario-logistico/
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

- **Modelo:** representa productos, categorías, movimientos de inventario y datos de cada SKU.
- **Service:** valida operaciones, controla el stock, rechaza SKU duplicados y calcula indicadores de inventario.
- **Controlador:** prepara los datos de ejemplo, coordina las operaciones y envía el resultado a la vista.
- **Vista:** muestra el inventario, los movimientos y las alertas de stock.
- **main.rs:** declara las capas y ejecuta el controlador.

## Código fuente por archivo

- [Cargo.toml](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-2-inventario-logistico/Cargo.toml)
- [src/main.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-2-inventario-logistico/src/main.rs)
- [Modelo: src/modelo/mod.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-2-inventario-logistico/src/modelo/mod.rs)
- [Service: src/service/mod.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-2-inventario-logistico/src/service/mod.rs)
- [Vista: src/vista/mod.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-2-inventario-logistico/src/vista/mod.rs)
- [Controlador: src/controlador/mod.rs](https://github.com/programadorkevinsvega-sys/Rust_Academy-cloudflare/blob/main/proyectos/rust-nivel-2-inventario-logistico/src/controlador/mod.rs)

## Ejecutar y probar

Desde la raíz del repositorio:

```bash
cargo run --manifest-path proyectos/rust-nivel-2-inventario-logistico/Cargo.toml
cargo test --manifest-path proyectos/rust-nivel-2-inventario-logistico/Cargo.toml
```

Los productos y movimientos iniciales son datos demostrativos. En producción, cada movimiento debe quedar registrado con fecha, responsable y referencia para permitir auditoría y reconstrucción del stock.
