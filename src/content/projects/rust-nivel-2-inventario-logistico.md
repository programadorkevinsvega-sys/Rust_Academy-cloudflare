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

Crea el proyecto Cargo inventario_logistico:

~~~text
inventario_logistico/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── producto.rs
    ├── inventario.rs
    └── reportes.rs
~~~

## Cargo.toml

~~~toml
[package]
name = "inventario_logistico"
version = "0.1.0"
edition = "2021"

[dependencies]
~~~

## src/producto.rs

~~~rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Categoria {
    Bebida,
    Comida,
    Aseo,
    Tecnologia,
}

impl fmt::Display for Categoria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nombre = match self {
            Categoria::Bebida => "Bebida",
            Categoria::Comida => "Comida",
            Categoria::Aseo => "Aseo",
            Categoria::Tecnologia => "Tecnologia",
        };
        write!(f, "{nombre}")
    }
}

#[derive(Debug, Clone)]
pub struct Producto {
    pub sku: String,
    pub nombre: String,
    pub precio_cop: u64,
    pub stock: u32,
    pub stock_minimo: u32,
    pub categoria: Categoria,
}

impl Producto {
    pub fn new(
        sku: &str,
        nombre: &str,
        precio_cop: u64,
        stock: u32,
        stock_minimo: u32,
        categoria: Categoria,
    ) -> Result<Self, String> {
        let sku = sku.trim();
        let nombre = nombre.trim();

        if sku.is_empty() {
            return Err("El SKU no puede estar vacio".to_string());
        }
        if nombre.is_empty() {
            return Err("El nombre no puede estar vacio".to_string());
        }
        if precio_cop == 0 {
            return Err(format!("El precio de {nombre} debe ser mayor que cero"));
        }

        Ok(Self {
            sku: sku.to_string(),
            nombre: nombre.to_string(),
            precio_cop,
            stock,
            stock_minimo,
            categoria,
        })
    }

    pub fn valor_inventario(&self) -> u64 {
        self.precio_cop * u64::from(self.stock)
    }

    pub fn necesita_reposicion(&self) -> bool {
        self.stock <= self.stock_minimo
    }
}

pub trait Describible {
    fn describir(&self) -> String;
}

impl Describible for Producto {
    fn describir(&self) -> String {
        format!(
            "{} | {} | {} | {} COP | stock {} (minimo {})",
            self.sku, self.nombre, self.categoria, self.precio_cop,
            self.stock, self.stock_minimo
        )
    }
}

impl fmt::Display for Producto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describir())
    }
}
~~~

## src/inventario.rs

~~~rust
use std::collections::{HashMap, HashSet};

use crate::producto::{Categoria, Producto};

pub struct Inventario {
    productos: Vec<Producto>,
    skus: HashSet<String>,
}

impl Inventario {
    pub fn new() -> Self {
        Self {
            productos: Vec::new(),
            skus: HashSet::new(),
        }
    }

    pub fn agregar(&mut self, producto: Producto) -> Result<(), String> {
        if self.skus.contains(&producto.sku) {
            return Err(format!("Ya existe el SKU {}", producto.sku));
        }

        self.skus.insert(producto.sku.clone());
        self.productos.push(producto);
        Ok(())
    }

    pub fn buscar(&self, sku: &str) -> Option<&Producto> {
        self.productos.iter().find(|producto| producto.sku == sku)
    }

    pub fn recibir(&mut self, sku: &str, cantidad: u32) -> Result<(), String> {
        if cantidad == 0 {
            return Err("La cantidad recibida debe ser mayor que cero".to_string());
        }

        let producto = self.productos.iter_mut()
            .find(|producto| producto.sku == sku)
            .ok_or_else(|| format!("No existe el SKU {sku}"))?;

        producto.stock += cantidad;
        Ok(())
    }

    pub fn registrar_salida(&mut self, sku: &str, cantidad: u32) -> Result<(), String> {
        if cantidad == 0 {
            return Err("La cantidad de salida debe ser mayor que cero".to_string());
        }

        let producto = self.productos.iter_mut()
            .find(|producto| producto.sku == sku)
            .ok_or_else(|| format!("No existe el SKU {sku}"))?;

        if producto.stock < cantidad {
            return Err(format!(
                "Stock insuficiente para {}: hay {} y se solicitan {}",
                producto.nombre, producto.stock, cantidad
            ));
        }

        producto.stock -= cantidad;
        Ok(())
    }

    pub fn valor_total(&self) -> u64 {
        self.productos.iter().map(Producto::valor_inventario).sum()
    }

    pub fn unidades_por_categoria(&self) -> HashMap<String, u64> {
        let mut resumen = HashMap::new();

        for producto in &self.productos {
            let nombre = match &producto.categoria {
                Categoria::Bebida => "Bebida",
                Categoria::Comida => "Comida",
                Categoria::Aseo => "Aseo",
                Categoria::Tecnologia => "Tecnologia",
            };
            *resumen.entry(nombre.to_string()).or_insert(0) += u64::from(producto.stock);
        }

        resumen
    }

    pub fn reposicion_pendiente(&self) -> Vec<&Producto> {
        self.productos.iter()
            .filter(|producto| producto.necesita_reposicion())
            .collect()
    }

    pub fn productos(&self) -> &[Producto] {
        &self.productos
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::producto::Categoria;

    fn producto_prueba() -> Producto {
        Producto::new("SKU-1", "Cafe", 4500, 10, 3, Categoria::Bebida)
            .expect("los datos fijos de la prueba son validos")
    }

    #[test]
    fn rechaza_sku_duplicado() {
        let mut inventario = Inventario::new();
        inventario.agregar(producto_prueba()).unwrap();
        let resultado = inventario.agregar(producto_prueba());
        assert!(resultado.is_err());
        assert_eq!(inventario.productos().len(), 1);
    }

    #[test]
    fn salida_invalida_no_cambia_el_stock() {
        let mut inventario = Inventario::new();
        inventario.agregar(producto_prueba()).unwrap();
        assert!(inventario.registrar_salida("SKU-1", 11).is_err());
        assert_eq!(inventario.buscar("SKU-1").unwrap().stock, 10);
    }

    #[test]
    fn entrada_y_salida_actualizan_el_stock() {
        let mut inventario = Inventario::new();
        inventario.agregar(producto_prueba()).unwrap();
        inventario.recibir("SKU-1", 5).unwrap();
        inventario.registrar_salida("SKU-1", 3).unwrap();
        assert_eq!(inventario.buscar("SKU-1").unwrap().stock, 12);
    }
}
~~~

## src/reportes.rs

~~~rust
use crate::producto::Describible;

pub fn imprimir_lista<T: Describible>(titulo: &str, elementos: &[T]) {
    println!();
    println!("=== {titulo} ===");

    if elementos.is_empty() {
        println!("Sin registros.");
        return;
    }

    for elemento in elementos {
        println!("{}", elemento.describir());
    }
}
~~~

## src/main.rs

~~~rust
mod inventario;
mod producto;
mod reportes;

use inventario::Inventario;
use producto::{Categoria, Producto};
use reportes::imprimir_lista;

fn main() -> Result<(), String> {
    let mut inventario = Inventario::new();

    let iniciales = vec![
        Producto::new("SKU-100", "Agua", 2500, 40, 8, Categoria::Bebida)?,
        Producto::new("SKU-200", "Arroz", 4200, 25, 5, Categoria::Comida)?,
        Producto::new("SKU-300", "Jabon", 3800, 3, 6, Categoria::Aseo)?,
        Producto::new("SKU-400", "Cafe", 4500, 12, 4, Categoria::Bebida)?,
        Producto::new("SKU-500", "Galletas", 2800, 2, 10, Categoria::Comida)?,
        Producto::new("SKU-600", "Teclado", 65000, 7, 2, Categoria::Tecnologia)?,
    ];

    for producto in iniciales {
        inventario.agregar(producto)?;
    }

    imprimir_lista("Catalogo de productos", inventario.productos());

    match Producto::new("SKU-100", "SKU repetido", 1000, 1, 0, Categoria::Aseo)
        .and_then(|producto| inventario.agregar(producto))
    {
        Ok(()) => println!("Producto adicional registrado."),
        Err(error) => println!("Alta rechazada: {error}"),
    }

    match inventario.registrar_salida("SKU-100", 10) {
        Ok(()) => println!("Salida registrada: SKU-100, 10 unidades."),
        Err(error) => println!("No fue posible registrar la salida: {error}"),
    }

    match inventario.registrar_salida("SKU-200", 100) {
        Ok(()) => println!("Salida registrada: SKU-200, 100 unidades."),
        Err(error) => println!("Salida rechazada: {error}"),
    }

    inventario.recibir("SKU-300", 10)?;
    println!("\nValor total del inventario: {} COP", inventario.valor_total());

    let mut categorias: Vec<_> = inventario.unidades_por_categoria().into_iter().collect();
    categorias.sort_by(|a, b| a.0.cmp(&b.0));
    println!("\nUnidades por categoria:");
    for (categoria, unidades) in categorias {
        println!("{categoria}: {unidades}");
    }

    println!("\nProductos para reposicion:");
    let reposicion = inventario.reposicion_pendiente();
    if reposicion.is_empty() {
        println!("No hay productos pendientes.");
    } else {
        for producto in reposicion {
            println!(
                "{} | stock actual {} | stock minimo {}",
                producto.sku, producto.stock, producto.stock_minimo
            );
        }
    }

    if let Some(producto) = inventario.buscar("SKU-400") {
        println!("\nConsulta por SKU: {}", producto);
    } else {
        println!("\nNo existe el SKU solicitado.");
    }

    Ok(())
}
~~~

## Ejecutarlo

~~~bash
cargo run
cargo test
~~~

La implementación trabaja en memoria; la separación de módulos permite incorporar persistencia después.

