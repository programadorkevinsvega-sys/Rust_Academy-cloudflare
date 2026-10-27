// En el nivel 1 conservamos arrays y tuplas; no hace falta crear una struct.
pub type Venta = (&'static str, u64, u32);

pub const META_DIARIA_COP: u64 = 330_000;

pub const VENTAS: [Venta; 6] = [
    ("Cafe americano", 4_500, 18),
    ("Capuchino", 6_500, 12),
    ("Chocolate", 5_000, 8),
    ("Croissant", 4_000, 10),
    ("Sandwich", 11_000, 7),
    ("Agua", 3_000, 9),
];
