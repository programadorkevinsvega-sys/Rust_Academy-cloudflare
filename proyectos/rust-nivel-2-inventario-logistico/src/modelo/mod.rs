use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Categoria {
    Bebida,
    Comida,
    Aseo,
    Tecnologia,
}

impl fmt::Display for Categoria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nombre = match self {
            Categoria::Bebida => "Bebida",
            Categoria::Comida => "Comida",
            Categoria::Aseo => "Aseo",
            Categoria::Tecnologia => "Tecnologia",
        };
        write!(f, "{nombre}")
    }
}

#[derive(Clone, Debug)]
pub struct Producto {
    id: u64,
    sku: String,
    nombre: String,
    categoria: Categoria,
    precio_cop: u64,
    stock: u32,
    stock_minimo: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum Movimiento {
    Entrada(u32),
    Salida(u32),
}

pub trait Reportable {
    fn linea_reporte(&self) -> String;
}

impl Producto {
    pub fn nuevo(
        id: u64,
        sku: &str,
        nombre: &str,
        categoria: Categoria,
        precio_cop: u64,
        stock: u32,
        stock_minimo: u32,
    ) -> Result<Self, String> {
        if sku.trim().is_empty() {
            return Err("el SKU no puede estar vacio".into());
        }
        if nombre.trim().is_empty() {
            return Err("el nombre no puede estar vacio".into());
        }
        if precio_cop == 0 {
            return Err("el precio debe ser mayor que cero".into());
        }
        Ok(Self {
            id,
            sku: sku.trim().to_string(),
            nombre: nombre.trim().to_string(),
            categoria,
            precio_cop,
            stock,
            stock_minimo,
        })
    }

    pub fn id(&self) -> u64 { self.id }
    pub fn sku(&self) -> &str { &self.sku }
    pub fn nombre(&self) -> &str { &self.nombre }
    pub fn categoria(&self) -> Categoria { self.categoria }
    pub fn precio_cop(&self) -> u64 { self.precio_cop }
    pub fn stock(&self) -> u32 { self.stock }
    pub fn stock_minimo(&self) -> u32 { self.stock_minimo }

    pub fn recibir(&mut self, cantidad: u32) -> Result<(), String> {
        self.stock = self.stock.checked_add(cantidad).ok_or("desbordamiento de stock")?;
        Ok(())
    }

    pub fn despachar(&mut self, cantidad: u32) -> Result<(), String> {
        if cantidad > self.stock {
            return Err(format!(
                "stock insuficiente para {}: disponible {}, solicitado {}",
                self.sku, self.stock, cantidad
            ));
        }
        self.stock -= cantidad;
        Ok(())
    }
}

impl Reportable for Producto {
    fn linea_reporte(&self) -> String {
        format!(
            "{:<12} {:<20} categoria={:<12} stock={:>3} minimo={:>3} valor={} COP",
            self.sku,
            self.nombre,
            self.categoria,
            self.stock,
            self.stock_minimo,
            self.precio_cop * u64::from(self.stock)
        )
    }
}
