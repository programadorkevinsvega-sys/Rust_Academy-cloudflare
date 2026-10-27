use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use tokio::{
    sync::{mpsc, watch, Mutex, Semaphore},
    task::JoinSet,
    time::{sleep, timeout},
};

use crate::modelo::{Estado, Evento, Metricas, Resultado};

const NUM_TRABAJADORES: usize = 5;
const MAX_CONCURRENCIA: usize = 3;
const CAPACIDAD_CANAL: usize = 2;
const TIMEOUT_EVENTO: Duration = Duration::from_millis(180);
const PLAZO_TOTAL: Duration = Duration::from_secs(3);

pub async fn ejecutar(eventos: Vec<Evento>) -> Result<Metricas, String> {
    let (tx, rx) = mpsc::channel::<Evento>(CAPACIDAD_CANAL);
    let receptor = Arc::new(Mutex::new(rx));
    let metricas = Arc::new(Mutex::new(Metricas::default()));
    let semaforo = Arc::new(Semaphore::new(MAX_CONCURRENCIA));
    let activos = Arc::new(AtomicUsize::new(0));
    let maximos = Arc::new(AtomicUsize::new(0));
    let (cancelar_tx, cancelar_rx) = watch::channel(false);
    let mut trabajadores = JoinSet::new();

    for _ in 0..NUM_TRABAJADORES {
        let receptor = Arc::clone(&receptor);
        let metricas = Arc::clone(&metricas);
        let semaforo = Arc::clone(&semaforo);
        let activos = Arc::clone(&activos);
        let maximos = Arc::clone(&maximos);
        let mut cancelar = cancelar_rx.clone();

        trabajadores.spawn(async move {
            loop {
                let recibido = { receptor.lock().await.recv().await };
                let Some(evento) = recibido else { break };

                let permiso = match Arc::clone(&semaforo).acquire_owned().await {
                    Ok(permiso) => permiso,
                    Err(_) => break,
                };
                let actual = activos.fetch_add(1, Ordering::SeqCst) + 1;
                maximos.fetch_max(actual, Ordering::SeqCst);

                let resultado = tokio::select! {
                    resultado = timeout(TIMEOUT_EVENTO, async {
                        sleep(Duration::from_millis(evento.demora_ms)).await;
                        if evento.id % 7 == 0 {
                            Estado::Fallo("fallo simulado de validacion".to_string())
                        } else {
                            Estado::Exito
                        }
                    }) => {
                        let estado = match resultado {
                            Ok(estado) => estado,
                            Err(_) => Estado::Timeout,
                        };
                        Resultado { evento_id: evento.id, estado }
                    }
                    _ = cancelar.changed() => Resultado {
                        evento_id: evento.id,
                        estado: Estado::Timeout,
                    },
                };

                metricas.lock().await.registrar(&resultado);
                activos.fetch_sub(1, Ordering::SeqCst);
                drop(permiso);
            }
        });
    }

    // La cola tiene capacidad 2: send().await espera cuando los consumidores no dan abasto.
    for evento in eventos {
        if tx.send(evento).await.is_err() {
            return Err("todos los trabajadores cerraron el canal".into());
        }
    }
    drop(tx);

    let espera_trabajadores = async {
        while let Some(resultado) = trabajadores.join_next().await {
            if let Err(error) = resultado {
                eprintln!("Tarea de trabajador fallida: {error}");
            }
        }
    };

    if timeout(PLAZO_TOTAL, espera_trabajadores).await.is_err() {
        let _ = cancelar_tx.send(true);
        trabajadores.abort_all();
        while trabajadores.join_next().await.is_some() {}
    }

    let mut resumen = metricas.lock().await.clone();
    resumen.concurrencia_maxima = maximos.load(Ordering::SeqCst);
    Ok(resumen)
}
