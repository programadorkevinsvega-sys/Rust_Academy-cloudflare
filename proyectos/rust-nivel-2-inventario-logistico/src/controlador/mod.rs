use crate::{
    modelo::{Categoria, Movimiento, Producto},
    service::Inventario,
    vista,
};

pub fn ejecutar() {
    let mut inventario = Inventario::nuevo();
    let carga = [
        Producto::nuevo(1, "SKU-AGUA", "Agua", Categoria::Bebida, 3_000, 40, 10),
        Producto::nuevo(2, "SKU-CAFE", "Cafe molido", Categoria::Bebida, 18_000, 7, 8),
        Producto::nuevo(3, "SKU-ARROZ", "Arroz", Categoria::Comida, 4_200, 25, 6),
        Producto::nuevo(4, "SKU-JABON", "Jabon", Categoria::Aseo, 3_800, 15, 5),
        Producto::nuevo(5, "SKU-PAN", "Pan tajado", Categoria::Comida, 6_500, 4, 5),
    ];

    for resultado in carga {
        match resultado {
            Ok(producto) => if let Err(error) = inventario.agregar(producto) {
                println!("Alta rechazada: {error}");
            },
            Err(error) => println!("Producto invalido: {error}"),
        }
    }

    if let Err(error) = inventario.aplicar_movimiento("SKU-AGUA", Movimiento::Salida(10)) {
        println!("No se registro la venta: {error}");
    }
    if let Err(error) = inventario.aplicar_movimiento("SKU-ARROZ", Movimiento::Salida(100)) {
        println!("Venta rechazada correctamente: {error}");
    }

    vista::imprimir_catalogo(&inventario);
    vista::imprimir_alertas(&inventario);
    match inventario.valor_total_cop() {
        Ok(total) => println!("\nValor total en stock: {total} COP"),
        Err(error) => println!("\nNo se pudo calcular el valor: {error}"),
    }
    println!("Stock por categoria: {:?}", inventario.stock_por_categoria());
}
