use crate::{modelo::Evento, service::fallos_por_usuario};

pub fn imprimir_informe(eventos: &[Evento<'_>], lineas_invalidas: usize) {
    println!("=== INFORME DE AUDITORIA ===");
    println!("Eventos validos: {}", eventos.len());
    println!("Lineas invalidas: {lineas_invalidas}");
    let denegados = eventos.iter()
        .filter(|evento| evento.resultado.eq_ignore_ascii_case("DENIED"))
        .count();
    let cambios_rol = eventos.iter()
        .filter(|evento| evento.accion.eq_ignore_ascii_case("CHANGE_ROLE"))
        .count();
    println!("Accesos denegados: {denegados}");
    println!("Cambios de rol: {cambios_rol}");
    println!("\nEventos que requieren revision:");
    for evento in eventos {
        if evento.resultado.eq_ignore_ascii_case("DENIED")
            || evento.resultado.eq_ignore_ascii_case("BLOCKED")
            || evento.accion.eq_ignore_ascii_case("CHANGE_ROLE")
        {
            println!(
                "linea {} | {} | usuario={} | accion={} | recurso={} | resultado={}",
                evento.numero_linea, evento.timestamp, evento.usuario,
                evento.accion, evento.recurso, evento.resultado
            );
        }
    }
    println!("\nAlertas por multiples fallos:");
    let alertas = fallos_por_usuario(eventos);
    if alertas.is_empty() {
        println!("No se encontraron usuarios sobre el umbral.");
    }
    for (usuario, cantidad) in alertas {
        println!("ALERTA: {usuario} acumula {cantidad} accesos denegados");
    }
}
