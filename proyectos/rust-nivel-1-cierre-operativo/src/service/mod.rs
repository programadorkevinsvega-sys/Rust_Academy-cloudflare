use crate::modelo::{Venta, META_DIARIA_COP};

pub fn ingreso_linea(precio_cop: u64, unidades: u32) -> u64 {
    precio_cop * u64::from(unidades)
}

pub fn ingreso_total(ventas: &[Venta]) -> u64 {
    let mut total = 0_u64;
    for &(_, precio, unidades) in ventas {
        total += ingreso_linea(precio, unidades);
    }
    total
}

pub fn unidades_totales(ventas: &[Venta]) -> u32 {
    let mut total = 0_u32;
    for &(_, _, unidades) in ventas {
        total += unidades;
    }
    total
}

pub fn promedio_por_linea(ventas: &[Venta]) -> (f64, bool) {
    if ventas.is_empty() {
        return (0.0, false);
    }
    (ingreso_total(ventas) as f64 / ventas.len() as f64, true)
}

pub fn linea_mayor_ingreso(ventas: &[Venta]) -> (&'static str, u64, bool) {
    if ventas.is_empty() {
        return ("Sin datos", 0, false);
    }
    let (mut nombre_mayor, precio, unidades) = ventas[0];
    let mut mayor = ingreso_linea(precio, unidades);
    for &(nombre, precio, unidades) in &ventas[1..] {
        let ingreso = ingreso_linea(precio, unidades);
        if ingreso > mayor {
            nombre_mayor = nombre;
            mayor = ingreso;
        }
    }
    (nombre_mayor, mayor, true)
}

pub fn estado_cierre(total_cop: u64) -> &'static str {
    if total_cop == 0 {
        "Sin ventas"
    } else if total_cop >= META_DIARIA_COP {
        "Meta cumplida"
    } else {
        "Por debajo de la meta"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::VENTAS;

    #[test]
    fn calcula_ingreso_y_unidades() {
        assert_eq!(ingreso_total(&VENTAS), 343_000);
        assert_eq!(unidades_totales(&VENTAS), 64);
    }

    #[test]
    fn evita_dividir_por_cero() {
        let vacias: [Venta; 0] = [];
        assert_eq!(promedio_por_linea(&vacias), (0.0, false));
        assert_eq!(estado_cierre(ingreso_total(&vacias)), "Sin ventas");
    }

    #[test]
    fn encuentra_la_linea_de_mayor_ingreso() {
        assert_eq!(linea_mayor_ingreso(&VENTAS), ("Cafe americano", 81_000, true));
    }
}
