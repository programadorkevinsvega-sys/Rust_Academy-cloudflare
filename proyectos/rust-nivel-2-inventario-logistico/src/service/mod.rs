use std::collections::{HashMap, HashSet};

use crate::modelo::{Categoria, Movimiento, Producto};

#[derive(Default)]
pub struct Inventario {
    productos: Vec<Producto>,
}

impl Inventario {
    pub fn nuevo() -> Self { Self::default() }

    pub fn agregar(&mut self, producto: Producto) -> Result<(), String> {
        if self.productos.iter().any(|actual| actual.sku() == producto.sku()) {
            return Err(format!("SKU duplicado: {}", producto.sku()));
        }
        self.productos.push(producto);
        Ok(())
    }

    pub fn cargar_unicos(&mut self, productos: Vec<Producto>) -> Vec<String> {
        let mut vistos = HashSet::new();
        let mut errores = Vec::new();
        for producto in productos {
            if !vistos.insert(producto.sku().to_string()) {
                errores.push(format!("SKU duplicado en carga: {}", producto.sku()));
                continue;
            }
            if let Err(error) = self.agregar(producto) {
                errores.push(error);
            }
        }
        errores
    }

    pub fn buscar(&self, sku: &str) -> Option<&Producto> {
        self.productos.iter().find(|producto| producto.sku() == sku)
    }

    pub fn aplicar_movimiento(&mut self, sku: &str, movimiento: Movimiento) -> Result<(), String> {
        let producto = self.productos.iter_mut()
            .find(|producto| producto.sku() == sku)
            .ok_or_else(|| format!("producto no encontrado: {sku}"))?;
        match movimiento {
            Movimiento::Entrada(cantidad) => producto.recibir(cantidad),
            Movimiento::Salida(cantidad) => producto.despachar(cantidad),
        }
    }

    pub fn productos(&self) -> &[Producto] { &self.productos }

    pub fn valor_total_cop(&self) -> Result<u64, String> {
        self.productos.iter().try_fold(0_u64, |total, producto| {
            let valor = producto.precio_cop()
                .checked_mul(u64::from(producto.stock()))
                .ok_or("desbordamiento calculando valor de producto")?;
            total.checked_add(valor).ok_or_else(|| "desbordamiento del inventario".to_string())
        })
    }

    pub fn productos_bajo_minimo(&self) -> Vec<&Producto> {
        self.productos.iter()
            .filter(|producto| producto.stock() <= producto.stock_minimo())
            .collect()
    }

    pub fn stock_por_categoria(&self) -> HashMap<Categoria, u64> {
        let mut resumen = HashMap::new();
        for producto in &self.productos {
            *resumen.entry(producto.categoria()).or_insert(0_u64) += u64::from(producto.stock());
        }
        resumen
    }
}

pub fn buscar_primero<T, F>(elementos: &[T], mut coincide: F) -> Option<&T>
where
    F: FnMut(&T) -> bool,
{
    elementos.iter().find(|elemento| coincide(elemento))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::{Categoria, Movimiento, Producto};

    fn inventario_demo() -> Inventario {
        let mut inventario = Inventario::nuevo();
        let producto = Producto::nuevo(1, "SKU-TEST", "Agua", Categoria::Bebida, 2_500, 10, 3)
            .expect("datos de prueba validos");
        inventario.agregar(producto).expect("SKU de prueba unico");
        inventario
    }

    #[test]
    fn rechaza_sku_duplicado() {
        let mut inventario = inventario_demo();
        let duplicado = Producto::nuevo(2, "SKU-TEST", "Otra agua", Categoria::Bebida, 1_000, 1, 0)
            .expect("datos de producto validos");
        assert!(inventario.agregar(duplicado).is_err());
        assert_eq!(inventario.productos().len(), 1);
    }

    #[test]
    fn salida_invalida_no_modifica_stock() {
        let mut inventario = inventario_demo();
        assert!(inventario.aplicar_movimiento("SKU-TEST", Movimiento::Salida(20)).is_err());
        assert_eq!(inventario.buscar("SKU-TEST").unwrap().stock(), 10);
    }

    #[test]
    fn entrada_actualiza_stock_y_valor() {
        let mut inventario = inventario_demo();
        inventario.aplicar_movimiento("SKU-TEST", Movimiento::Entrada(5)).unwrap();
        assert_eq!(inventario.buscar("SKU-TEST").unwrap().stock(), 15);
        assert_eq!(inventario.valor_total_cop().unwrap(), 37_500);
    }
}
