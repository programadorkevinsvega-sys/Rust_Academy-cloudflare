use crate::{modelo::datos_demo, service, vista};

pub async fn ejecutar() {
    let eventos = datos_demo();
    println!("Mercado Andino - ingesta de eventos");
    println!("Lote de eventos de prueba: {}", eventos.len());

    match service::hilos::ejecutar(&eventos) {
        Ok(metricas) => vista::imprimir_metricas("Procesamiento con std::thread", &metricas),
        Err(error) => eprintln!("Fallo en la version con hilos: {error}"),
    }

    match service::async_pipeline::ejecutar(eventos).await {
        Ok(metricas) => vista::imprimir_metricas("Procesamiento async con Tokio", &metricas),
        Err(error) => eprintln!("Fallo en la version async: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use crate::modelo::datos_demo;

    #[test]
    fn el_lote_demo_tiene_eventos_sinteticos() {
        let eventos = datos_demo();
        assert_eq!(eventos.len(), 9);
        assert!(eventos.iter().all(|evento| evento.pedido_id.starts_with("PED-")));
    }
}
