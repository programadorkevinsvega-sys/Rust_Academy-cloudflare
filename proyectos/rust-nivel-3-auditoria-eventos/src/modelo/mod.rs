#[derive(Debug)]
pub struct Evento<'a> {
    pub numero_linea: usize,
    pub timestamp: &'a str,
    pub usuario: &'a str,
    pub accion: &'a str,
    pub recurso: &'a str,
    pub resultado: &'a str,
}
