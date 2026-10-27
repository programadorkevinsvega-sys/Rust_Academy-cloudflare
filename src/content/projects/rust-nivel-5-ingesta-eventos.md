---
title: "Proyecto Nivel 5: Plataforma de ingesta de eventos de pedidos en tiempo real"
description: "Construye un servicio de procesamiento de eventos con trabajadores concurrentes, canales acotados, límites de concurrencia, timeouts y cierre ordenado."
language: rust
level: 5
levelLabel: "Nivel 05"
category: "Proyecto empresarial"
duration: "18–24 horas"
order: 5
slug: rust-nivel-5-ingesta-eventos
courseSlug: rust-nivel-5
company: "Mercado Andino (empresa de comercio electrónico simulada)"
role: "Desarrollador/a backend del equipo de plataforma"
moduleName: "Módulos 0–11: closures, concurrencia, threads, Send/Sync, Arc/Mutex/RwLock, canales, async/Future, Tokio, cancelación, timeouts, backpressure y límites"
---

# Misión del proyecto

## Contexto de empresa

Mercado Andino recibe eventos de pedidos provenientes de sus tiendas y operadores logísticos: pedido creado, pago confirmado, paquete despachado, entrega fallida y pedido entregado. En horas pico aparecen ráfagas de mensajes; si los consumidores son más lentos que los productores, la cola crece, aumenta la latencia y se vuelve difícil detener el servicio sin perder trazabilidad.

El equipo de plataforma necesita un prototipo de ingesta que procese eventos concurrentemente, mida resultados y aplique límites explícitos. El proyecto no necesita conectarse a servicios reales: los productores simularán el tráfico con datos deterministas para que el comportamiento sea repetible.

## Objetivo de negocio

Demostrar que el servicio puede procesar eventos en paralelo sin carreras de datos, resistir picos de trabajo con backpressure, aislar fallos por timeout y completar un apagado ordenado.

## Modelo de evento

Cada evento debe incluir, como mínimo:

- `event_id`: identificador único.
- `order_id`: pedido al que pertenece.
- `kind`: tipo de evento, representado por un `enum`.
- `occurred_at`: marca temporal de prueba.
- `source`: tienda u operador que lo produjo.

El modelo puede ampliarse para indicar resultado del procesamiento, duración y motivo de fallo. Usa datos sintéticos; no registres información personal real.

## Alcance funcional

### Fase A — primera versión con `std::thread`

1. Crear productores que generen lotes de eventos y trabajadores que los procesen.
2. Comunicar productores y consumidores usando canales `std::sync::mpsc`; transferir la propiedad de cada evento por el canal.
3. Implementar un grupo fijo de trabajadores y asegurar que los hilos necesarios terminen mediante `JoinHandle::join` o `thread::scope`.
4. Compartir métricas globales solo cuando haga falta, usando `Arc<Mutex<T>>` o una alternativa justificada.
5. Usar un `RwLock` para una configuración que se lea con frecuencia y cambie raramente, o documentar por qué no hace falta.
6. Definir un protocolo de mensajes tipado con `enum` y tratar los errores de los trabajadores.

### Fase B — versión asíncrona con Tokio

1. Crear un runtime con Tokio y migrar la canalización a tareas `async`.
2. Recibir eventos por un canal `mpsc` acotado para aplicar backpressure cuando el consumidor se retrasa.
3. Limitar las tareas activas con un `Semaphore` o un pool de tamaño fijo.
4. Agrupar y esperar tareas con `JoinSet`; registrar fallos sin perder el estado del resto del lote.
5. Añadir timeout por evento y, si aplica, un plazo total para un lote.
6. Usar `select!` y un mecanismo de cancelación para atender nuevas señales o cerrar el servicio.
7. Implementar apagado ordenado: detener productores, cerrar el canal, terminar o cancelar trabajadores según la política y esperar las tareas restantes.
8. Demostrar por pruebas que una cola llena frena al productor en lugar de crecer sin límite.
9. Comparar el diseño de threads con el diseño async e indicar qué parte del trabajo es CPU-bound y cuál está esperando E/S.

## Métricas operativas

- Eventos recibidos, procesados y rechazados.
- Conteo de cada tipo de evento.
- Eventos que excedieron el timeout.
- Máximo de trabajadores o tareas activas observado.
- Tiempo de procesamiento por evento y tiempo total del lote.
- Veces que un productor tuvo que esperar por capacidad disponible en la cola.
- Resultado del apagado: tareas terminadas, canceladas y errores registrados.

## Reglas operativas

- La capacidad de la cola debe ser configurable y finita en la versión async.
- El número de trabajadores concurrentes debe tener un límite explícito.
- Ningún hilo o tarea que deba finalizar puede quedar olvidado sin `join`, sin seguimiento o sin una política de cancelación.
- Los datos compartidos deben estar protegidos por el mecanismo apropiado; no se debe usar un lock por costumbre si un canal resuelve mejor el flujo.
- Los locks no deben mantenerse durante esperas async ni durante trabajo lento que no requiera el estado protegido.
- Una tarea lenta o fallida no debe bloquear indefinidamente a todo el sistema.
- Los errores, timeouts y cancelaciones se cuentan por separado.
- Los cierres deben poder probarse en un escenario reproducible, sin depender de Ctrl+C ni de servicios externos.

## Diseño técnico esperado

```text
ingesta_eventos/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── evento.rs
    ├── productor.rs
    ├── worker.rs
    ├── metricas.rs
    ├── threading.rs
    └── async_pipeline.rs
```

Usa la dependencia Tokio con las features necesarias para runtime, macros, sincronización, tareas y temporizadores. Mantén separada la lógica del evento de los detalles de ejecución concurrente.

## Relación con los módulos del Nivel 5

| Módulo | Cómo se aplica en el proyecto |
|---|---|
| 0. Closures | Capturas y propiedad de los datos enviados a hilos y tareas. |
| 1. Concurrencia, paralelismo y async | Justificar los dos diseños y distinguir CPU-bound de I/O-bound. |
| 2. Threads | Creación, espera y terminación de hilos. |
| 3. `Send` y `Sync` | Garantías requeridas por los datos que cruzan hilos. |
| 4. `Arc`, `Mutex`, `RwLock` | Compartir métricas y configuración con sincronización explícita. |
| 5. Canales estándar | Protocolo de mensajes, varios productores y trabajadores. |
| 6. Async y `.await` | Operaciones asíncronas que permiten avanzar mientras otra tarea espera. |
| 7. Futures y runtime | Entender quién ejecuta y planifica los futuros. |
| 8. Tokio: tareas | Crear, agrupar y recoger resultados de tareas. |
| 9. Tokio: canales y sincronización | Canal acotado, métricas y comunicación entre tareas. |
| 10. Cancelación | Detener el procesamiento y cerrar los recursos con orden. |
| 11. Timeouts y límites | Presupuesto por evento, backpressure y concurrencia limitada. |

## Entregables

- Dos ejecuciones demostrables: una implementación de threads y otra con Tokio.
- Datos sintéticos suficientes para mostrar carga normal, pico de tráfico, evento lento y evento fallido.
- Métricas al final del lote y logs que distingan error, timeout y cancelación.
- Pruebas para la cola acotada, el límite de concurrencia, el fallo de un trabajador y el cierre ordenado.
- README con diagrama de la canalización, parámetros de configuración, comandos para ejecutar y resultados de pruebas.

## Criterios de aceptación

- [ ] El sistema procesa múltiples eventos en más de un trabajador.
- [ ] La versión estándar espera correctamente a que los hilos finalicen.
- [ ] Los datos compartidos están protegidos y no se utiliza `unsafe` para evitar las reglas de Rust.
- [ ] La versión async usa un canal acotado y demuestra backpressure.
- [ ] El límite de concurrencia se cumple incluso bajo un lote grande.
- [ ] Los timeouts se registran sin confundirlos con errores de negocio.
- [ ] Una tarea fallida se detecta y no deja el coordinador esperando para siempre.
- [ ] El apagado ordenado cierra productores y consumidores y deja métricas finales coherentes.
- [ ] Se explican las diferencias de diseño entre threads y async según el trabajo ejecutado.

## Pruebas de aceptación sugeridas

1. Enviar un lote pequeño y verificar que todos los eventos terminan procesados.
2. Enviar más eventos que la capacidad del canal: comprobar que el productor experimenta espera y la memoria de la cola no crece ilimitadamente.
3. Simular un evento que tarda más que el timeout: debe contarse como timeout, sin congelar el resto.
4. Hacer que un trabajador termine con error y verificar que el coordinador lo observa.
5. Solicitar cierre mientras hay eventos pendientes: el sistema debe aplicar la política documentada y terminar.
6. Registrar el número máximo de tareas activas y comprobar que nunca supera el límite configurado.

## Plan de trabajo sugerido

1. Define el evento y una función de procesamiento determinista.
2. Construye la versión con threads y canales de la biblioteca estándar.
3. Agrega métricas compartidas y pruebas para las reglas de sincronización.
4. Implementa la versión Tokio con canal acotado y límite de concurrencia.
5. Agrega timeout, cancelación y cierre ordenado.
6. Ejecuta pruebas de carga simulada y documenta los resultados.

## Retos opcionales

- Añadir deduplicación por `event_id` y documentar cómo se sincroniza.
- Implementar una respuesta con `oneshot` para consultar el estado de un lote.
- Emitir notificaciones de métricas con `watch` o `broadcast` y justificar cuál semántica corresponde.
- Introducir reintentos limitados con espera exponencial, timeout total y cancelación respetada durante la espera.

## Fuera de alcance

No conectes todavía una cola de producción, una base de datos ni servicios de logística reales. El objetivo es validar la arquitectura de concurrencia y las garantías operativas con cargas simuladas y resultados reproducibles.


---

# Solución de referencia completa

Esta solución entrega dos implementaciones para la misma carga simulada: una con hilos de la biblioteca estándar y otra con Tokio. Ambas cuentan éxitos, fallos y timeouts. La versión Tokio utiliza un canal acotado, un semáforo, tareas agrupadas y cancelación por plazo total.

Crea el proyecto ingesta_eventos con esta estructura:

~~~text
ingesta_eventos/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── evento.rs
    ├── common.rs
    ├── threading.rs
    └── async_pipeline.rs
~~~

## Cargo.toml

~~~toml
[package]
name = "ingesta_eventos"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
~~~

## src/evento.rs

~~~rust
#[derive(Debug, Clone)]
pub enum TipoEvento {
    PedidoCreado,
    PagoConfirmado,
    PaqueteDespachado,
    EntregaFallida,
    PedidoEntregado,
}

#[derive(Debug, Clone)]
pub struct Evento {
    pub id: u32,
    pub pedido_id: u32,
    pub tipo: TipoEvento,
    pub fuente: &'static str,
    pub demora_ms: u64,
}

pub fn datos_demo() -> Vec<Evento> {
    use TipoEvento::*;

    vec![
        Evento { id: 1, pedido_id: 9001, tipo: PedidoCreado, fuente: "tienda-medellin", demora_ms: 40 },
        Evento { id: 2, pedido_id: 9002, tipo: PagoConfirmado, fuente: "pasarela-pagos", demora_ms: 75 },
        Evento { id: 3, pedido_id: 9003, tipo: PaqueteDespachado, fuente: "operador-logistico", demora_ms: 250 },
        Evento { id: 4, pedido_id: 9004, tipo: EntregaFallida, fuente: "operador-logistico", demora_ms: 60 },
        Evento { id: 5, pedido_id: 9005, tipo: PedidoEntregado, fuente: "tienda-bogota", demora_ms: 90 },
        Evento { id: 6, pedido_id: 9006, tipo: PagoConfirmado, fuente: "pasarela-pagos", demora_ms: 220 },
        Evento { id: 7, pedido_id: 9007, tipo: PedidoCreado, fuente: "tienda-cali", demora_ms: 55 },
        Evento { id: 8, pedido_id: 9008, tipo: PaqueteDespachado, fuente: "operador-logistico", demora_ms: 80 },
        Evento { id: 9, pedido_id: 9009, tipo: PedidoEntregado, fuente: "tienda-medellin", demora_ms: 240 },
        Evento { id: 10, pedido_id: 9010, tipo: PagoConfirmado, fuente: "pasarela-pagos", demora_ms: 100 },
    ]
}
~~~

## src/common.rs

~~~rust
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Debug)]
pub enum Estado {
    Exito,
    Fallo(String),
    Timeout,
}

#[derive(Debug)]
pub struct Resultado {
    pub evento_id: u32,
    pub estado: Estado,
}

#[derive(Debug, Default)]
pub struct Metricas {
    pub procesados: usize,
    pub exitos: usize,
    pub errores: usize,
    pub timeouts: usize,
    pub max_concurrencia: usize,
}

impl Metricas {
    pub fn registrar(&mut self, resultado: &Resultado) {
        self.procesados += 1;
        match &resultado.estado {
            Estado::Exito => self.exitos += 1,
            Estado::Fallo(_) => self.errores += 1,
            Estado::Timeout => self.timeouts += 1,
        }
        println!("Evento #{} -> {:?}", resultado.evento_id, resultado.estado);
    }

    pub fn imprimir(&self, titulo: &str) {
        println!("\n=== {titulo} ===");
        println!("Eventos finalizados: {}", self.procesados);
        println!("Exitos: {}", self.exitos);
        println!("Errores de procesamiento: {}", self.errores);
        println!("Timeouts: {}", self.timeouts);
        println!("Maximo de trabajos simultaneos observado: {}", self.max_concurrencia);
    }
}

// Este guardia reduce el contador aunque un hilo termine o una tarea async se cancele.
pub struct GuardiaActiva {
    contador: Arc<AtomicUsize>,
}

impl GuardiaActiva {
    pub fn entrar(
        contador: Arc<AtomicUsize>,
        maximo: Arc<AtomicUsize>,
    ) -> Self {
        let actual = contador.fetch_add(1, Ordering::SeqCst) + 1;
        maximo.fetch_max(actual, Ordering::SeqCst);
        Self { contador }
    }
}

impl Drop for GuardiaActiva {
    fn drop(&mut self) {
        self.contador.fetch_sub(1, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registra_los_tres_resultados() {
        let mut metricas = Metricas::default();
        metricas.registrar(&Resultado { evento_id: 1, estado: Estado::Exito });
        metricas.registrar(&Resultado {
            evento_id: 2,
            estado: Estado::Fallo("evento no valido".to_string()),
        });
        metricas.registrar(&Resultado { evento_id: 3, estado: Estado::Timeout });

        assert_eq!(metricas.procesados, 3);
        assert_eq!(metricas.exitos, 1);
        assert_eq!(metricas.errores, 1);
        assert_eq!(metricas.timeouts, 1);
    }
}
~~~

## src/threading.rs

~~~rust
use std::{
    sync::{
        Arc, Mutex,
        atomic::AtomicUsize,
        mpsc,
    },
    thread,
    time::Duration,
};

use crate::{
    common::{Estado, GuardiaActiva, Metricas, Resultado},
    evento::Evento,
};

fn procesar(evento: Evento) -> Resultado {
    println!(
        "[thread] pedido={} tipo={:?} fuente={}",
        evento.pedido_id, evento.tipo, evento.fuente
    );
    thread::sleep(Duration::from_millis(evento.demora_ms));

    let estado = if evento.id % 7 == 0 {
        Estado::Fallo("fallo simulado de validacion".to_string())
    } else {
        Estado::Exito
    };

    Resultado { evento_id: evento.id, estado }
}

pub fn ejecutar(eventos: &[Evento]) -> Result<(), String> {
    const TRABAJADORES: usize = 3;

    let (tx, rx) = mpsc::channel::<Evento>();
    let receptor = Arc::new(Mutex::new(rx));
    let metricas = Arc::new(Mutex::new(Metricas::default()));
    let activos = Arc::new(AtomicUsize::new(0));
    let maximos = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    for numero in 0..TRABAJADORES {
        let receptor = Arc::clone(&receptor);
        let metricas = Arc::clone(&metricas);
        let activos = Arc::clone(&activos);
        let maximos = Arc::clone(&maximos);

        let handle = thread::Builder::new()
            .name(format!("worker-{numero}"))
            .spawn(move || {
                loop {
                    // El lock se mantiene solo mientras se recibe el siguiente mensaje.
                    let recibido = {
                        let rx = receptor.lock().unwrap_or_else(|p| p.into_inner());
                        rx.recv()
                    };

                    let evento = match recibido {
                        Ok(evento) => evento,
                        Err(_) => break,
                    };

                    let _guardia = GuardiaActiva::entrar(
                        Arc::clone(&activos),
                        Arc::clone(&maximos),
                    );
                    let resultado = procesar(evento);

                    let mut datos = metricas.lock().unwrap_or_else(|p| p.into_inner());
                    datos.registrar(&resultado);
                }
            })
            .map_err(|error| format!("no se pudo crear un trabajador: {error}"))?;

        handles.push(handle);
    }

    for evento in eventos {
        tx.send(evento.clone())
            .map_err(|error| format!("no se pudo enviar el evento: {error}"))?;
    }
    // Al cerrar el canal, cada worker sale de recv y puede finalizar.
    drop(tx);

    for handle in handles {
        if handle.join().is_err() {
            eprintln!("Un trabajador terminó por panic.");
        }
    }

    let mut resumen = metricas.lock().unwrap_or_else(|p| p.into_inner());
    resumen.max_concurrencia = maximos.load(std::sync::atomic::Ordering::SeqCst);
    resumen.imprimir("Procesamiento con std::thread");
    Ok(())
}
~~~

## src/async_pipeline.rs

~~~rust
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use tokio::{
    sync::{mpsc, Semaphore},
    task::JoinSet,
    time::{sleep, timeout},
};

use crate::{
    common::{Estado, GuardiaActiva, Metricas, Resultado},
    evento::Evento,
};

async fn procesar(evento: Evento) -> Resultado {
    println!(
        "[async] pedido={} tipo={:?} fuente={}",
        evento.pedido_id, evento.tipo, evento.fuente
    );
    sleep(Duration::from_millis(evento.demora_ms)).await;

    let estado = if evento.id % 7 == 0 {
        Estado::Fallo("fallo simulado de validacion".to_string())
    } else {
        Estado::Exito
    };

    Resultado { evento_id: evento.id, estado }
}

pub async fn ejecutar(eventos: Vec<Evento>) -> Result<(), String> {
    const LIMITE_CONCURRENCIA: usize = 3;
    const CAPACIDAD_CANAL: usize = 2;
    const TIMEOUT_POR_EVENTO: Duration = Duration::from_millis(180);
    const PLAZO_TOTAL: Duration = Duration::from_secs(3);

    let (tx, mut rx) = mpsc::channel::<Resultado>(CAPACIDAD_CANAL);

    // Un consumidor recoge resultados y simula que el sistema de monitoreo es más lento.
    let recolector = tokio::spawn(async move {
        let mut metricas = Metricas::default();
        while let Some(resultado) = rx.recv().await {
            metricas.registrar(&resultado);
            sleep(Duration::from_millis(15)).await;
        }
        metricas
    });

    let semaforo = Arc::new(Semaphore::new(LIMITE_CONCURRENCIA));
    let activos = Arc::new(AtomicUsize::new(0));
    let maximos = Arc::new(AtomicUsize::new(0));
    let mut tareas = JoinSet::new();

    for evento in eventos {
        // Si se alcanzó el límite, el productor espera hasta que se libere un permiso.
        let permiso = Arc::clone(&semaforo)
            .acquire_owned()
            .await
            .map_err(|error| format!("semaforo cerrado inesperadamente: {error}"))?;
        let tx_evento = tx.clone();
        let activos_tarea = Arc::clone(&activos);
        let maximos_tarea = Arc::clone(&maximos);

        tareas.spawn(async move {
            let _permiso = permiso;
            let _guardia = GuardiaActiva::entrar(activos_tarea, maximos_tarea);
            let id = evento.id;

            // select! cancela el futuro de procesamiento si vence el plazo de este evento.
            let resultado = tokio::select! {
                resultado = procesar(evento) => resultado,
                _ = sleep(TIMEOUT_POR_EVENTO) => Resultado {
                    evento_id: id,
                    estado: Estado::Timeout,
                },
            };

            // Al estar lleno el canal, send().await espera: esto aplica backpressure.
            if tx_evento.send(resultado).await.is_err() {
                eprintln!("El recolector terminó antes de recibir el resultado.");
            }
        });
    }

    drop(tx); // Cuando todas las tareas cierren su emisor, el canal termina.

    let espera_tareas = async {
        while let Some(resultado) = tareas.join_next().await {
            if let Err(error) = resultado {
                eprintln!("Una tarea falló al unirse: {error}");
            }
        }
    };

    if timeout(PLAZO_TOTAL, espera_tareas).await.is_err() {
        println!("Plazo total agotado: se cancelan las tareas pendientes.");
        tareas.abort_all();
        while tareas.join_next().await.is_some() {}
    }

    let mut metricas = recolector
        .await
        .map_err(|error| format!("falló el recolector: {error}"))?;
    metricas.max_concurrencia = maximos.load(Ordering::SeqCst);
    metricas.imprimir("Procesamiento async con Tokio");
    Ok(())
}
~~~

## src/main.rs

~~~rust
mod async_pipeline;
mod common;
mod evento;
mod threading;

#[tokio::main]
async fn main() {
    let eventos = evento::datos_demo();

    println!("Eventos de entrada: {}", eventos.len());
    if let Err(error) = threading::ejecutar(&eventos) {
        eprintln!("Fallo en la versión con hilos: {error}");
    }

    println!("\n==============================================");
    println!("Comienza la versión asíncrona");
    println!("==============================================");

    if let Err(error) = async_pipeline::ejecutar(eventos).await {
        eprintln!("Fallo en la versión async: {error}");
    }
}
~~~

## Ejecutarlo

~~~bash
cargo run
cargo test
~~~

La versión de hilos usa un canal estándar, un grupo fijo de trabajadores y espera sus handles. La versión async limita el trabajo con Semaphore y un canal acotado; select! aplica un timeout por evento, mientras que el plazo global permite abortar tareas pendientes. Las demoras y fallos son simulados; no se contacta ningún servicio real.

