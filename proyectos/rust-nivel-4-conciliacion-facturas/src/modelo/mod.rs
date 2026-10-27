use std::fmt;

#[derive(Clone, Debug)]
pub struct Factura {
    pub factura_id: String,
    pub proveedor_id: String,
    pub orden_id: String,
    pub subtotal_cop: u64,
    pub impuesto_cop: u64,
    pub total_cop: u64,
}

#[derive(Clone, Debug)]
pub struct OrdenCompra {
    pub proveedor_id: String,
    pub subtotal_cop: u64,
}

#[derive(Debug, PartialEq)]
pub enum ErrorLinea {
    CamposIncorrectos(usize),
    CampoVacio(&'static str),
    ImporteInvalido(String),
    Desbordamiento,
    TotalIncoherente { esperado: u64, recibido: u64 },
    OrdenNoEncontrada(String),
    ProveedorNoCoincide,
    SubtotalNoCoincide { factura: u64, orden: u64 },
    FacturaDuplicada(String),
}

impl fmt::Display for ErrorLinea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorLinea::CamposIncorrectos(n) => write!(f, "se esperaban 6 campos y llegaron {n}"),
            ErrorLinea::CampoVacio(campo) => write!(f, "el campo {campo} esta vacio"),
            ErrorLinea::ImporteInvalido(campo) => write!(f, "importe no numerico: {campo}"),
            ErrorLinea::Desbordamiento => write!(f, "desbordamiento calculando subtotal + impuesto"),
            ErrorLinea::TotalIncoherente { esperado, recibido } => write!(f, "total incoherente: esperado {esperado}, recibido {recibido}"),
            ErrorLinea::OrdenNoEncontrada(id) => write!(f, "orden de compra no encontrada: {id}"),
            ErrorLinea::ProveedorNoCoincide => write!(f, "el proveedor de la factura no coincide con la orden"),
            ErrorLinea::SubtotalNoCoincide { factura, orden } => write!(f, "subtotal de factura {factura} distinto del subtotal de orden {orden}"),
            ErrorLinea::FacturaDuplicada(id) => write!(f, "factura duplicada: {id}"),
        }
    }
}
impl std::error::Error for ErrorLinea {}

#[derive(Debug)]
pub struct ErrorProceso {
    pub linea: usize,
    pub causa: ErrorLinea,
}

impl fmt::Display for ErrorProceso {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "linea {}: {}", self.linea, self.causa)
    }
}
