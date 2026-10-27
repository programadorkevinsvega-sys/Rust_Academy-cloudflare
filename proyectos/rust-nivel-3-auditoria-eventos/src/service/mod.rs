use std::collections::HashMap;

use crate::modelo::Evento;

pub fn parsear_linea<'a>(
    linea: &'a str,
    numero_linea: usize,
) -> Result<Evento<'a>, &'static str> {
    let campos: Vec<&str> = linea.split('|').map(str::trim).collect();
    if campos.len() != 5 {
        return Err("se esperaban cinco campos separados por |");
    }
    if campos.iter().any(|campo| campo.is_empty()) {
        return Err("hay campos vacios");
    }
    let resultado = campos[4];
    if !["OK", "DENIED", "BLOCKED"].iter().any(|valor| resultado.eq_ignore_ascii_case(valor)) {
        return Err("resultado desconocido");
    }
    Ok(Evento {
        numero_linea,
        timestamp: campos[0],
        usuario: campos[1],
        accion: campos[2],
        recurso: campos[3],
        resultado,
    })
}

pub fn analizar<'a>(texto: &'a str) -> (Vec<Evento<'a>>, usize) {
    let mut eventos = Vec::new();
    let mut invalidas = 0;
    for (indice, linea) in texto.lines().enumerate() {
        if linea.trim().is_empty() { continue; }
        match parsear_linea(linea, indice + 1) {
            Ok(evento) => eventos.push(evento),
            Err(error) => {
                invalidas += 1;
                eprintln!("Linea {} ignorada: {}", indice + 1, error);
            }
        }
    }
    (eventos, invalidas)
}

pub fn fallos_por_usuario(eventos: &[Evento<'_>]) -> Vec<(String, usize)> {
    let mut conteo: HashMap<&str, usize> = HashMap::new();
    for evento in eventos.iter().filter(|evento| evento.resultado.eq_ignore_ascii_case("DENIED")) {
        *conteo.entry(evento.usuario).or_insert(0) += 1;
    }
    let mut alertas: Vec<(String, usize)> = conteo.into_iter()
        .filter(|(_, cantidad)| *cantidad >= 3)
        .map(|(usuario, cantidad)| (usuario.to_string(), cantidad))
        .collect();
    alertas.sort_by(|a, b| a.0.cmp(&b.0));
    alertas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsea_un_evento_sin_copiar_los_campos() {
        let texto = "fecha|usuario_1|LOGIN|portal|OK";
        let evento = parsear_linea(texto, 1).expect("linea valida");
        assert_eq!(evento.usuario, "usuario_1");
        assert_eq!(evento.resultado, "OK");
    }

    #[test]
    fn rechaza_linea_malformada() {
        assert!(parsear_linea("fecha|usuario|LOGIN", 2).is_err());
    }

    #[test]
    fn detecta_tres_fallos_del_mismo_usuario() {
        let texto = "t1|u1|LOGIN|portal|DENIED\nt2|u1|LOGIN|portal|DENIED\nt3|u1|LOGIN|portal|DENIED";
        let (eventos, invalidas) = analizar(texto);
        assert_eq!(invalidas, 0);
        assert_eq!(fallos_por_usuario(&eventos), vec![("u1".to_string(), 3)]);
    }
}
