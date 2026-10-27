use std::collections::HashMap;

use crate::{
    modelo::OrdenCompra,
    service::{procesar_estricto, procesar_tolerante},
    vista,
};

const DATOS: &str = "\
FAC-1001;PROV-10;OC-8001;250000;47500;297500
FAC-1002;PROV-12;OC-8002;180000;34200;214200
FAC-1001;PROV-10;OC-8001;250000;47500;297500
FAC-1004;PROV-14;OC-9999;90000;17100;107100
FAC-1005;PROV-10;OC-8001;abc;0;0";

fn ordenes_demo() -> HashMap<String, OrdenCompra> {
    HashMap::from([
        ("OC-8001".to_string(), OrdenCompra { proveedor_id: "PROV-10".to_string(), subtotal_cop: 250_000 }),
        ("OC-8002".to_string(), OrdenCompra { proveedor_id: "PROV-12".to_string(), subtotal_cop: 180_000 }),
    ])
}

pub fn ejecutar() {
    let ordenes = ordenes_demo();
    match procesar_estricto(DATOS, &ordenes) {
        Ok(facturas) => println!("Modo estricto: {} facturas", facturas.len()),
        Err(error) => println!("Modo estricto se detuvo: {error}"),
    }
    let (facturas, errores) = procesar_tolerante(DATOS, &ordenes);
    vista::imprimir_resultados(&facturas, &errores);
}
