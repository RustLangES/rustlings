// from_str.rs
//
// Esto es similar a from_into.rs, pero esta vez implementaremos `FromStr` y
// devolverá errores en lugar de un valor por default. Además, al
// implementar FromStr, puedes usar el método `parse` en Strings para generar
// un objeto del tipo del implementador. puedes leer más sobre esto en
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Ejecuta `rustlings hint from_str` o usa el subcomando watch `hint` para una
// pista.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// Usaremos este tipo de error para la implementación de `FromStr`.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // String de entada vacía
    Empty,
    // Número de campos incorrecto
    BadLen,
    // Campo nombre vacío
    NoName,
    // Error envuelto de parse::<usize>()
    ParseInt(ParseIntError),
}

// I AM NOT DONE

// Pasos:
// 1. Si la longitud del String proporcionado es 0, se devolverá un error
// 2. Dividir el String dado en las comas que contenga
// 3. Solo 2 elementos deben ser devueltos de la división, de lo contrario
//    devolver un error
// 4. Extraer el primer elemento de la operación de división y usarlo como nombre
// 5. Extraer el otro elemento de la operación de división y analizarlo en un
//    `usize` como la edad con algo como `"4".parse::<usize>()`
// 6. Si algo sale mal al extraer el nombre y la edad, se debe devolver un error
// Si todo sale bien, entonces devolver un Result de un objeto Person
//
// Como apunte: `Box<dyn Error>` implementa `From<&'_ str>`. Esto significa que
// si quieres devolver un mensaje de error de tipo String, puedes hacerlo a
// través de `return Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
