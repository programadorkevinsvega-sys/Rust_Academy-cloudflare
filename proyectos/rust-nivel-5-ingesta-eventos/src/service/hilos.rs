use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc, Arc, Mutex, RwLock,
    },
    thread,
    time::Duration,
};

use crate::modelo::{Estado, Evento, Metricas, Resultado};

const NUM_TRABAJADORES: usize = 3;
const DEMORA_MAXIMA_MS: u64 = 180;

pub fn ejecutar(eventos: &[Evento]) -> Result<Metricas, String> {
    let (tx, rx) = mpsc::channel::<Evento>();
    let receptor = Arc::new(Mutex::new(rx));
    let metricas = Arc::new(Mutex::new(Metricas::default()));
    let activos = Arc::new(AtomicUsize::new(0));
    let maximos = Arc::new(AtomicUsize::new(0));
    let limite_demora = Arc::new(RwLock::new(DEMORA_MAXIMA_MS));
    let mut handles = Vec::new();

    for numero in 0..NUM_TRABAJADORES {
        let receptor = Arc::clone(&receptor);
        let metricas = Arc::clone(&metricas);
        let activos = Arc::clone(&activos);
        let maximos = Arc::clone(&maximos);
        let limite_demora = Arc::clone(&limite_demora);

        let handle = thread::Builder::new()
            .name(format!("worker-{numero}"))
            .spawn(move || loop {
                let recibido = {
                    let guardia = receptor.lock().unwrap_or_else(|p| p.into_inner());
                    guardia.recv()
                };
                let evento = match recibido {
                    Ok(evento) => evento,
                    Err(_) => break,
                };

                println!(
                    "[thread] pedido={} tipo={} fuente={}",
                    evento.pedido_id, evento.tipo, evento.fuente
                );
                let actual = activos.fetch_add(1, Ordering::SeqCst) + 1;
                maximos.fetch_max(actual, Ordering::SeqCst);
                let limite = *limite_demora.read().unwrap_or_else(|p| p.into_inner());
                thread::sleep(Duration::from_millis(evento.demora_ms));
                let estado = if evento.demora_ms > limite {
                    Estado::Timeout
                } else if evento.id % 7 == 0 {
                    Estado::Fallo("fallo simulado de validacion".to_string())
                } else {
                    Estado::Exito
                };
                let resultado = Resultado { evento_id: evento.id, estado };
                metricas.lock().unwrap_or_else(|p| p.into_inner()).registrar(&resultado);
                activos.fetch_sub(1, Ordering::SeqCst);
            })
            .map_err(|error| format!("no se pudo crear el trabajador: {error}"))?;
        handles.push(handle);
    }

    for evento in eventos {
        tx.send(evento.clone()).map_err(|error| format!("no se pudo enviar evento: {error}"))?;
    }
    drop(tx);

    for handle in handles {
        handle.join().map_err(|_| "un trabajador termino por panic".to_string())?;
    }

    let mut resumen = metricas.lock().unwrap_or_else(|p| p.into_inner()).clone();
    resumen.concurrencia_maxima = maximos.load(Ordering::SeqCst);
    Ok(resumen)
}
