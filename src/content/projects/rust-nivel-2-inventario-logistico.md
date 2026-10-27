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

Esta solución separa modelo, inventario y reportes en archivos de Rust. Crea el proyecto con los siguientes comandos:

```bash
cargo new inventario_logistico
cd inventario_logistico
```

La estructura final será:

```text
inventario_logistico/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── producto.rs
    ├── inventario.rs
    └── reportes.rs
```

## Cargo.toml

```toml
[package]
name = "inventario_logistico"
version = "0.1.0"
edition = "2021"

[dependencies]
```

## src/producto.rs

```rust
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Categoria {
    Bebida,
    Comida,
    Aseo,
    Tecnologia,
}

impl fmt::Display for Categoria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let texto = match self {
            Categoria::Bebida => "Bebida",
            Categoria::Comida => "Comida",
            Categoria::Aseo => "Aseo",
            Categoria::Tecnologia => "Tecnologia",
        };
        write!(f, "{texto}")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Movimiento {
    Entrada(u32),
    Salida(u32),
}

#[derive(Debug, Clone)]
pub struct Producto {
    id: u32,
    sku: String,
    nombre: String,
    categoria: Categoria,
    precio_cop: u64,
    stock: u32,
    stock_minimo: u32,
    activo: bool,
}

pub trait Reportable {
    fn linea_reporte(&self) -> String;
}

impl Producto {
    pub fn nuevo(
        id: u32,
        sku: &str,
        nombre: &str,
        categoria: Categoria,
        precio_cop: u64,
        stock: u32,
        stock_minimo: u32,
    ) -> Result<Self, String> {
        if sku.trim().is_empty() {
            return Err("El SKU no puede estar vacío".to_string());
        }
        if nombre.trim().is_empty() {
            return Err("El nombre del producto no puede estar vacío".to_string());
        }
        if precio_cop == 0 {
            return Err(format!("El precio de {nombre} debe ser mayor que cero"));
        }

        Ok(Self {
            id,
            sku: sku.trim().to_string(),
            nombre: nombre.trim().to_string(),
            categoria,
            precio_cop,
            stock,
            stock_minimo,
            activo: true,
        })
    }

    pub fn sku(&self) -> &str { &self.sku }
    pub fn nombre(&self) -> &str { &self.nombre }
    pub fn categoria(&self) -> Categoria { self.categoria }
    pub fn stock(&self) -> u32 { self.stock }
    pub fn stock_minimo(&self) -> u32 { self.stock_minimo }
    pub fn esta_activo(&self) -> bool { self.activo }

    pub fn valor_stock_cop(&self) -> Result<u64, String> {
        self.precio_cop
            .checked_mul(u64::from(self.stock))
            .ok_or_else(|| format!("Desbordamiento al calcular el valor de {}", self.sku))
    }

    pub fn aplicar_movimiento(&mut self, movimiento: Movimiento) -> Result<(), String> {
        match movimiento {
            Movimiento::Entrada(cantidad) => {
                self.stock = self.stock.checked_add(cantidad)
                    .ok_or_else(|| format!("El stock excedería el límite para {}", self.sku))?;
            }
            Movimiento::Salida(cantidad) => {
                if cantidad > self.stock {
                    return Err(format!(
                        "Stock insuficiente para {}: hay {}, se solicitan {}",
                        self.sku, self.stock, cantidad
                    ));
                }
                self.stock -= cantidad;
            }
        }
        Ok(())
    }
}

impl Reportable for Producto {
    fn linea_reporte(&self) -> String {
        format!(
            "#{:03} | {:<12} | {:<20} | {:<7} | stock {:>4} | mínimo {:>4} | ${} | {}",
            self.id, self.sku, self.nombre, self.categoria,
            self.stock, self.stock_minimo, self.precio_cop,
            if self.activo { "Activo" } else { "Inactivo" }
        )
    }
}
```

## src/inventario.rs

```rust
use std::collections::{HashMap, HashSet};

use crate::producto::{Categoria, Movimiento, Producto};

pub struct Inventario {
    productos: Vec<Producto>,
    skus: HashSet<String>,
}

impl Inventario {
    pub fn nuevo() -> Self {
        Self { productos: Vec::new(), skus: HashSet::new() }
    }

    pub fn agregar(&mut self, producto: Producto) -> Result<(), String> {
        if self.skus.contains(producto.sku()) {
            return Err(format!("Ya existe el SKU {}", producto.sku()));
        }
        self.skus.insert(producto.sku().to_string());
        self.productos.push(producto);
        Ok(())
    }

    pub fn buscar(&self, sku: &str) -> Option<&Producto> {
        self.productos.iter().find(|producto| producto.sku() == sku)
    }

    pub fn aplicar_movimiento(
        &mut self,
        sku: &str,
        movimiento: Movimiento,
    ) -> Result<(), String> {
        let producto = self.productos.iter_mut()
            .find(|producto| producto.sku() == sku)
            .ok_or_else(|| format!("No existe el producto con SKU {sku}"))?;
        producto.aplicar_movimiento(movimiento)
    }

    pub fn productos(&self) -> &[Producto] { &self.productos }

    pub fn productos_bajo_minimo(&self) -> Vec<&Producto> {
        self.productos.iter()
            .filter(|p| p.esta_activo() && p.stock() <= p.stock_minimo())
            .collect()
    }

    pub fn valor_total_cop(&self) -> Result<u64, String> {
        let mut total = 0_u64;
        for producto in &self.productos {
            total = total.checked_add(producto.valor_stock_cop()?)
                .ok_or_else(|| "Desbordamiento al sumar el valor total".to_string())?;
        }
        Ok(total)
    }

    pub fn stock_por_categoria(&self) -> HashMap<Categoria, u64> {
        let mut resumen = HashMap::new();
        for producto in &self.productos {
            *resumen.entry(producto.categoria()).or_insert(0_u64) += u64::from(producto.stock());
        }
        resumen
    }
}

#[cfg(test)]
mod tests {
    use super::Inventario;
    use crate::producto::{Categoria, Movimiento, Producto};

    fn inventario_con_producto() -> Inventario {
        let mut inventario = Inventario::nuevo();
        let producto = Producto::nuevo(
            1, "SKU-TEST", "Agua", Categoria::Bebida, 2_500, 10, 3
        ).expect("los datos de prueba son validos");
        inventario.agregar(producto).expect("el SKU de prueba es unico");
        inventario
    }

    #[test]
    fn rechaza_sku_duplicado() {
        let mut inventario = inventario_con_producto();
        let duplicado = Producto::nuevo(
            2, "SKU-TEST", "Otra agua", Categoria::Bebida, 1_000, 1, 0
        ).expect("el producto duplicado tiene datos validos");
        assert!(inventario.agregar(duplicado).is_err());
        assert_eq!(inventario.productos().len(), 1);
    }

    #[test]
    fn salida_mayor_que_el_stock_no_modifica_el_inventario() {
        let mut inventario = inventario_con_producto();
        assert!(inventario.aplicar_movimiento("SKU-TEST", Movimiento::Salida(20)).is_err());
        assert_eq!(inventario.buscar("SKU-TEST").unwrap().stock(), 10);
    }

    #[test]
    fn movimiento_valido_actualiza_stock_y_valor() {
        let mut inventario = inventario_con_producto();
        inventario.aplicar_movimiento("SKU-TEST", Movimiento::Entrada(5)).unwrap();
        assert_eq!(inventario.buscar("SKU-TEST").unwrap().stock(), 15);
        assert_eq!(inventario.valor_total_cop().unwrap(), 37_500);
    }
}
```

## src/reportes.rs

```rust
use crate::inventario::Inventario;
use crate::producto::Reportable;

// Función genérica: sirve con cualquier tipo que cumpla el predicado.
pub fn buscar_primero<T, F>(elementos: &[T], mut coincide: F) -> Option<&T>
where
    F: FnMut(&T) -> bool,
{
    elementos.iter().find(|elemento| coincide(elemento))
}

pub fn imprimir_catalogo(inventario: &Inventario) {
    println!("CATÁLOGO DE INVENTARIO");
    println!("---------------------------------------------------------------");
    for producto in inventario.productos() {
        println!("{}", producto.linea_reporte());
    }
}

pub fn imprimir_reposicion(inventario: &Inventario) {
    println!("ALERTAS DE REPOSICIÓN");
    let alertas = inventario.productos_bajo_minimo();
    if alertas.is_empty() {
        println!("No hay productos bajo el mínimo.");
        return;
    }
    for producto in alertas {
        println!(
            "{}: stock={}, mínimo={}",
            producto.sku(), producto.stock(), producto.stock_minimo()
        );
    }
}
```

## src/main.rs

```rust
mod inventario;
mod producto;
mod reportes;

use inventario::Inventario;
use producto::{Categoria, Movimiento, Producto};
use reportes::{buscar_primero, imprimir_catalogo, imprimir_reposicion};

fn main() {
    let mut inventario = Inventario::nuevo();

    let productos = [
        Producto::nuevo(1, "SKU-AGUA", "Agua", Categoria::Bebida, 3_000, 40, 10),
        Producto::nuevo(2, "SKU-CAFE", "Café molido", Categoria::Bebida, 18_000, 7, 8),
        Producto::nuevo(3, "SKU-ARROZ", "Arroz", Categoria::Comida, 4_200, 25, 6),
        Producto::nuevo(4, "SKU-JABON", "Jabón", Categoria::Aseo, 3_800, 15, 5),
        Producto::nuevo(5, "SKU-PAN", "Pan tajado", Categoria::Comida, 6_500, 4, 5),
        Producto::nuevo(6, "SKU-TECLADO", "Teclado", Categoria::Tecnologia, 65_000, 7, 2),
    ];

    for resultado in productos {
        match resultado {
            Ok(producto) => {
                if let Err(error) = inventario.agregar(producto) {
                    println!("No se pudo agregar: {error}");
                }
            }
            Err(error) => println!("Producto inválido: {error}"),
        }
    }

    let duplicado = Producto::nuevo(
        99, "SKU-AGUA", "Agua duplicada", Categoria::Bebida, 2_800, 5, 2
    );
    match duplicado {
        Ok(producto) => match inventario.agregar(producto) {
            Ok(()) => println!("Producto agregado."),
            Err(error) => println!("Alta rechazada correctamente: {error}"),
        },
        Err(error) => println!("Datos inválidos: {error}"),
    }

    println!();
    imprimir_catalogo(&inventario);

    println!();
    match inventario.aplicar_movimiento("SKU-AGUA", Movimiento::Salida(10)) {
        Ok(()) => println!("Venta registrada: 10 unidades de SKU-AGUA."),
        Err(error) => println!("Venta rechazada: {error}"),
    }
    match inventario.aplicar_movimiento("SKU-ARROZ", Movimiento::Salida(100)) {
        Ok(()) => println!("Salida registrada."),
        Err(error) => println!("Venta rechazada correctamente: {error}"),
    }
    match inventario.aplicar_movimiento("SKU-AGUA", Movimiento::Entrada(5)) {
        Ok(()) => println!("Recepción registrada: 5 unidades de SKU-AGUA."),
        Err(error) => println!("Recepción rechazada: {error}"),
    }

    match inventario.buscar("SKU-JABON") {
        Some(producto) => println!("Búsqueda: encontrado {}", producto.nombre()),
        None => println!("Búsqueda: producto no encontrado"),
    }
    match inventario.buscar("SKU-NO-EXISTE") {
        Some(producto) => println!("Encontrado: {}", producto.nombre()),
        None => println!("Búsqueda controlada: el SKU no existe."),
    }

    println!();
    match inventario.valor_total_cop() {
        Ok(total) => println!("Valor total del stock: ${total} COP"),
        Err(error) => println!("No se pudo calcular el valor total: {error}"),
    }

    println!("Stock por categoría:");
    for (categoria, unidades) in inventario.stock_por_categoria() {
        println!("  {categoria}: {unidades} unidades");
    }

    println!();
    imprimir_reposicion(&inventario);

    if let Some(producto) = buscar_primero(
        inventario.productos(),
        |p| p.stock() <= p.stock_minimo()
    ) {
        println!("Primera referencia que requiere reposición: {}", producto.sku());
    }
}
```

## Ejecutar y probar

```bash
cargo fmt
cargo check
cargo run
cargo test
```

La solución usa Vec como catálogo, HashSet para rechazar SKU duplicados y HashMap para agrupar unidades. Los campos del producto son privados y las modificaciones pasan por métodos que validan las reglas de negocio. La función genérica y el trait Reportable están separados del modelo.

