// using_as.rs
//
// La conversión de tipos en Rust se realiza mediante el uso del operador `as`. Ten en cuenta
// que el operador `as` no solo se utiliza cuando se hace una conversión de tipos. También ayuda
// a renombrar imports.
//
// El objetivo es asegurarse de que la división no falla al compilar y
// devuelva el tipo correcto.
//
// Ejecuta `rustlings hint using_as` o utilica el subcomando watch `hint` para obtener una
// pista.

// I AM NOT DONE

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
