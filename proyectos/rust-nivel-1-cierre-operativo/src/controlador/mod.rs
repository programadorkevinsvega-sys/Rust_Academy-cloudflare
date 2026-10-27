use crate::{modelo::VENTAS, service::ingreso_total, vista};

pub fn ejecutar() {
    vista::imprimir_cierre(&VENTAS);
    let ventas_vacias = [];
    println!(
        "Prueba sin ventas: total={} COP y sin division por cero.",
        ingreso_total(&ventas_vacias)
    );
}
