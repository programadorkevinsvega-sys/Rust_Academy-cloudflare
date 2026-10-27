use crate::modelo::{ErrorProceso, Factura};

pub fn imprimir_resultados(facturas: &[Factura], errores: &[ErrorProceso]) {
    println!("=== REPORTE DE CONCILIACION ===");
    println!("Facturas aceptadas: {}", facturas.len());
    let total: u64 = facturas.iter().map(|factura| factura.total_cop).sum();
    println!("Total conciliado: {total} COP");
    for factura in facturas {
        println!(
            "OK {} | proveedor={} | orden={} | total={} COP",
            factura.factura_id, factura.proveedor_id, factura.orden_id, factura.total_cop
        );
    }
    println!("\nIncidencias para corregir: {}", errores.len());
    for error in errores {
        println!("- {error}");
    }
}
