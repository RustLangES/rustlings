// clippy1.rs
//
// La herramienta Clippy es una colección de lints para analizar tu código para
// que puedas detectar errores comunes y mejorar tu código Rust.
//
// Para estos ejercicios, el código no se compilará cuando haya advertencias de
// clippy. Verifica las sugerencias de clippy desde la salida para resolver el
// ejercicio.
//
// Ejecuta `rustlings hint clippy1` o usa el subcomando `hint` para obtener una
// pista.

// NO ESTOY HECHO

use std::f32;

fn main() {
    let pi = 3.14f32;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "El área de un círculo de radio {:.2} es {:.5}!",
        radius, area
    )
}
