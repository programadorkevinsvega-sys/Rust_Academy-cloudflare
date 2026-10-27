mod modelo;
mod service;
mod vista;
mod controlador;

#[tokio::main]
async fn main() {
    controlador::ejecutar().await;
}
