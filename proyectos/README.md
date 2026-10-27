# Soluciones Rust ejecutables

Esta carpeta contiene cinco aplicaciones Rust independientes, una por nivel del curso. Cada una tiene su propio Cargo.toml, punto de entrada y arquitectura por carpetas:

- modelo/: entidades y tipos del dominio.
- service/: reglas de negocio, cálculos, validaciones y procesamiento.
- controlador/: coordina el flujo de la aplicación.
- vista/: presenta resultados para el usuario.

## Ejecutar todos los proyectos

Desde la raíz del repositorio:

    cargo test --manifest-path proyectos/Cargo.toml

## Ejecutar un proyecto específico

Por ejemplo, para el nivel 5:

    cargo run --manifest-path proyectos/rust-nivel-5-ingesta-eventos/Cargo.toml
    cargo test --manifest-path proyectos/rust-nivel-5-ingesta-eventos/Cargo.toml

Los datos son simulados. No se conecta a bases de datos, plataformas de pago ni sistemas empresariales reales.

Los cinco proyectos comparten las carpetas modelo, service, controlador y vista. La complejidad interna crece por nivel: el nivel 1 usa tipos básicos y funciones; el nivel 5 agrega canales, hilos, Tokio, límites de concurrencia, timeouts y cancelación.
