use crate::{
    modelo::{Venta, META_DIARIA_COP},
    service::{estado_cierre, ingreso_linea, ingreso_total, linea_mayor_ingreso, promedio_por_linea, unidades_totales},
};

pub fn imprimir_cierre(ventas: &[Venta]) {
    println!("==============================================");
    println!("CIERRE DIARIO - CAFE CENTRAL");
    println!("==============================================");
    for (indice, &(producto, precio, unidades)) in ventas.iter().enumerate() {
        println!(
            "{:>2}. {:<20} {:>3} uds. x {} COP = {} COP",
            indice + 1, producto, unidades, precio, ingreso_linea(precio, unidades)
        );
    }

    let total = ingreso_total(ventas);
    let (promedio, hay_promedio) = promedio_por_linea(ventas);
    let (producto, ingreso, hay_producto) = linea_mayor_ingreso(ventas);
    println!("----------------------------------------------");
    println!("Unidades vendidas: {}", unidades_totales(ventas));
    println!("Ingreso total: {} COP", total);
    println!("Meta diaria: {} COP", META_DIARIA_COP);
    if hay_promedio {
        println!("Promedio por linea: {:.2} COP", promedio);
    } else {
        println!("Promedio por linea: no disponible");
    }
    if hay_producto {
        println!("Mayor ingreso: {} ({} COP)", producto, ingreso);
    } else {
        println!("Mayor ingreso: no disponible");
    }
    println!("Resultado: {}", estado_cierre(total));
}
