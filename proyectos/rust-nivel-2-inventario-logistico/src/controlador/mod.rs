use crate::{
    modelo::{Categoria, Movimiento, Producto},
    service::{buscar_primero, Inventario},
    vista,
};

pub fn ejecutar() {
    let mut inventario = Inventario::nuevo();
    let resultados = [
        Producto::nuevo(1, "SKU-AGUA", "Agua", Categoria::Bebida, 3_000, 40, 10),
        Producto::nuevo(2, "SKU-CAFE", "Cafe molido", Categoria::Bebida, 18_000, 7, 8),
        Producto::nuevo(3, "SKU-ARROZ", "Arroz", Categoria::Comida, 4_200, 25, 6),
        Producto::nuevo(4, "SKU-JABON", "Jabon", Categoria::Aseo, 3_800, 15, 5),
        Producto::nuevo(5, "SKU-PAN", "Pan tajado", Categoria::Comida, 6_500, 4, 5),
        Producto::nuevo(6, "SKU-TECLADO", "Teclado", Categoria::Tecnologia, 65_000, 7, 2),
    ];

    let mut productos: Vec<Producto> = resultados
        .into_iter()
        .filter_map(|resultado| match resultado {
            Ok(producto) => Some(producto),
            Err(error) => {
                println!("Producto invalido: {error}");
                None
            }
        })
        .collect();

    // Demuestra el control de duplicados de la carga inicial.
    productos.push(
        Producto::nuevo(99, "SKU-AGUA", "Agua duplicada", Categoria::Bebida, 2_800, 5, 2)
            .expect("el producto duplicado tiene datos validos"),
    );
    for error in inventario.cargar_unicos(productos) {
        println!("Alta rechazada: {error}");
    }

    if let Err(error) = inventario.aplicar_movimiento("SKU-AGUA", Movimiento::Salida(10)) {
        println!("No se registro la venta: {error}");
    }
    if let Err(error) = inventario.aplicar_movimiento("SKU-AGUA", Movimiento::Entrada(5)) {
        println!("No se registro la recepcion: {error}");
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
    if let Some(producto) = buscar_primero(
        inventario.productos(),
        |producto| producto.stock() <= producto.stock_minimo()
    ) {
        println!(
            "Primera referencia para reponer: id={} SKU={} producto={}",
            producto.id(), producto.sku(), producto.nombre()
        );
    }
}
