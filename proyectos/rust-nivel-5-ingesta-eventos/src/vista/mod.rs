use crate::modelo::Metricas;

pub fn imprimir_metricas(titulo: &str, metricas: &Metricas) {
    println!("\n=== {titulo} ===");
    println!("Eventos procesados: {}", metricas.eventos_procesados);
    println!("Exitos: {}", metricas.exitos);
    println!("Fallos simulados: {}", metricas.fallos);
    println!("Timeouts: {}", metricas.timeouts);
    println!("Concurrencia maxima observada: {}", metricas.concurrencia_maxima);
}
