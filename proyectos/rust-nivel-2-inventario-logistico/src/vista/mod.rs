use crate::{
    modelo::Reportable,
    service::Inventario,
};

pub fn imprimir_catalogo(inventario: &Inventario) {
    println!("CATALOGO DE INVENTARIO");
    println!("---------------------------------------------------------------");
    for producto in inventario.productos() {
        println!("{}", producto.linea_reporte());
    }
}

pub fn imprimir_alertas(inventario: &Inventario) {
    println!("\nALERTAS DE REPOSICION");
    let alertas = inventario.productos_bajo_minimo();
    if alertas.is_empty() {
        println!("No hay productos bajo el minimo.");
        return;
    }
    for producto in alertas {
        println!("{}: stock={}, minimo={}", producto.sku(), producto.stock(), producto.stock_minimo());
    }
}
