use std::collections::{HashMap, HashSet};

use crate::modelo::{ErrorLinea, ErrorProceso, Factura, OrdenCompra};

pub fn parsear_linea(linea: &str) -> Result<Factura, ErrorLinea> {
    let campos: Vec<&str> = linea.split(';').map(str::trim).collect();
    if campos.len() != 6 {
        return Err(ErrorLinea::CamposIncorrectos(campos.len()));
    }
    let nombres = ["factura_id", "proveedor_id", "orden_id", "subtotal_cop", "impuesto_cop", "total_cop"];
    for (indice, valor) in campos.iter().enumerate() {
        if valor.is_empty() {
            return Err(ErrorLinea::CampoVacio(nombres[indice]));
        }
    }

    let parsear_importe = |valor: &str, nombre: &str| {
        valor.parse::<u64>().map_err(|_| ErrorLinea::ImporteInvalido(nombre.to_string()))
    };
    let subtotal_cop = parsear_importe(campos[3], "subtotal_cop")?;
    let impuesto_cop = parsear_importe(campos[4], "impuesto_cop")?;
    let total_cop = parsear_importe(campos[5], "total_cop")?;
    let esperado = subtotal_cop.checked_add(impuesto_cop).ok_or(ErrorLinea::Desbordamiento)?;
    if total_cop != esperado {
        return Err(ErrorLinea::TotalIncoherente { esperado, recibido: total_cop });
    }
    Ok(Factura {
        factura_id: campos[0].to_string(),
        proveedor_id: campos[1].to_string(),
        orden_id: campos[2].to_string(),
        subtotal_cop,
        impuesto_cop,
        total_cop,
    })
}

pub fn validar_conciliacion(
    factura: &Factura,
    ordenes: &HashMap<String, OrdenCompra>,
) -> Result<(), ErrorLinea> {
    let orden = ordenes.get(&factura.orden_id)
        .ok_or_else(|| ErrorLinea::OrdenNoEncontrada(factura.orden_id.clone()))?;
    if factura.proveedor_id != orden.proveedor_id {
        return Err(ErrorLinea::ProveedorNoCoincide);
    }
    if factura.subtotal_cop != orden.subtotal_cop {
        return Err(ErrorLinea::SubtotalNoCoincide {
            factura: factura.subtotal_cop,
            orden: orden.subtotal_cop,
        });
    }
    Ok(())
}

pub fn procesar_estricto(
    contenido: &str,
    ordenes: &HashMap<String, OrdenCompra>,
) -> Result<Vec<Factura>, ErrorProceso> {
    let mut aceptadas = Vec::new();
    let mut ids = HashSet::new();
    for (indice, linea) in contenido.lines().enumerate() {
        if linea.trim().is_empty() { continue; }
        let resultado = (|| {
            let factura = parsear_linea(linea)?;
            if !ids.insert(factura.factura_id.clone()) {
                return Err(ErrorLinea::FacturaDuplicada(factura.factura_id.clone()));
            }
            validar_conciliacion(&factura, ordenes)?;
            Ok(factura)
        })();
        match resultado {
            Ok(factura) => aceptadas.push(factura),
            Err(causa) => return Err(ErrorProceso { linea: indice + 1, causa }),
        }
    }
    Ok(aceptadas)
}

pub fn procesar_tolerante(
    contenido: &str,
    ordenes: &HashMap<String, OrdenCompra>,
) -> (Vec<Factura>, Vec<ErrorProceso>) {
    let mut aceptadas = Vec::new();
    let mut errores = Vec::new();
    let mut ids = HashSet::new();
    for (indice, linea) in contenido.lines().enumerate() {
        if linea.trim().is_empty() { continue; }
        let resultado = (|| {
            let factura = parsear_linea(linea)?;
            if !ids.insert(factura.factura_id.clone()) {
                return Err(ErrorLinea::FacturaDuplicada(factura.factura_id.clone()));
            }
            validar_conciliacion(&factura, ordenes)?;
            Ok(factura)
        })();
        match resultado {
            Ok(factura) => aceptadas.push(factura),
            Err(causa) => errores.push(ErrorProceso { linea: indice + 1, causa }),
        }
    }
    (aceptadas, errores)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ordenes() -> HashMap<String, OrdenCompra> {
        HashMap::from([
            ("OC-1".to_string(), OrdenCompra {
                proveedor_id: "P-1".to_string(),
                subtotal_cop: 100,
            }),
        ])
    }

    #[test]
    fn acepta_factura_con_total_correcto() {
        assert!(parsear_linea("F-1;P-1;OC-1;100;19;119").is_ok());
    }

    #[test]
    fn rechaza_total_incoherente() {
        assert!(matches!(
            parsear_linea("F-1;P-1;OC-1;100;19;120"),
            Err(ErrorLinea::TotalIncoherente { .. })
        ));
    }

    #[test]
    fn identifica_orden_inexistente() {
        let factura = parsear_linea("F-1;P-1;OC-X;100;19;119").unwrap();
        assert!(matches!(
            validar_conciliacion(&factura, &ordenes()),
            Err(ErrorLinea::OrdenNoEncontrada(_))
        ));
    }

    #[test]
    fn modo_tolerante_conserva_facturas_validas() {
        let contenido = "F-1;P-1;OC-1;100;19;119\nF-2;P-1;OC-1;100;19;120";
        let (validas, errores) = procesar_tolerante(contenido, &ordenes());
        assert_eq!(validas.len(), 1);
        assert_eq!(errores.len(), 1);
    }
}
