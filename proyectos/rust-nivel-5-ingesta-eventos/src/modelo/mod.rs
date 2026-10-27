use std::fmt;

#[derive(Clone, Debug)]
pub enum TipoEvento {
    PedidoCreado,
    PagoConfirmado,
    PaqueteDespachado,
    EntregaFallida,
}

impl fmt::Display for TipoEvento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nombre = match self {
            TipoEvento::PedidoCreado => "pedido_creado",
            TipoEvento::PagoConfirmado => "pago_confirmado",
            TipoEvento::PaqueteDespachado => "paquete_despachado",
            TipoEvento::EntregaFallida => "entrega_fallida",
        };
        write!(f, "{nombre}")
    }
}

#[derive(Clone, Debug)]
pub struct Evento {
    pub id: u64,
    pub pedido_id: String,
    pub tipo: TipoEvento,
    pub fuente: String,
    pub demora_ms: u64,
}

#[derive(Clone, Debug)]
pub enum Estado {
    Exito,
    Fallo(String),
    Timeout,
}

#[derive(Clone, Debug)]
pub struct Resultado {
    pub evento_id: u64,
    pub estado: Estado,
}

#[derive(Clone, Debug, Default)]
pub struct Metricas {
    pub eventos_procesados: usize,
    pub exitos: usize,
    pub fallos: usize,
    pub timeouts: usize,
    pub concurrencia_maxima: usize,
}

impl Metricas {
    pub fn registrar(&mut self, resultado: &Resultado) {
        self.eventos_procesados += 1;
        match &resultado.estado {
            Estado::Exito => {
                self.exitos += 1;
                println!("Evento {} procesado correctamente", resultado.evento_id);
            }
            Estado::Fallo(motivo) => {
                self.fallos += 1;
                eprintln!("Evento {} rechazado: {}", resultado.evento_id, motivo);
            }
            Estado::Timeout => {
                self.timeouts += 1;
                eprintln!("Evento {} supero el timeout configurado", resultado.evento_id);
            }
        }
    }
}

pub fn datos_demo() -> Vec<Evento> {
    let demoras = [40, 80, 220, 60, 30, 240, 70, 100, 50];
    demoras.iter().enumerate().map(|(indice, demora)| Evento {
        id: (indice + 1) as u64,
        pedido_id: format!("PED-{:04}", indice + 100),
        tipo: match indice % 4 {
            0 => TipoEvento::PedidoCreado,
            1 => TipoEvento::PagoConfirmado,
            2 => TipoEvento::PaqueteDespachado,
            _ => TipoEvento::EntregaFallida,
        },
        fuente: if indice % 2 == 0 { "tienda-norte" } else { "operador-logistico" }.to_string(),
        demora_ms: *demora,
    }).collect()
}
