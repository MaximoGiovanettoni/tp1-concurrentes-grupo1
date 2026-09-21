use almacen_inteligente::{mantenimiento, recepcion};

fn main() {
    println!("=== Zona de Recepción ===");
    recepcion::ejecutar();

    println!("\n=== Estación de Mantenimiento ===");
    mantenimiento::ejecutar();

    println!("\nSimulación finalizada correctamente.");
}