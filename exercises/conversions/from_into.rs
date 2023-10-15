// from_into.rs
//
// El Trait From se utiliza para conversiones de valor a valor. Si From está implementado
// correctamente para un tipo, el Trait Into debería funcionar a la inversa. Puedes leer
// más información en https://doc.rust-lang.org/std/convert/trait.From.html
//
// Ejecuta `rustlings hint from_into` o usa el subcomando watch `hint` para una
// pista.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// implementamos el Trait Default para utilizarlo como fallback
// cuando el string proporcionado no es convertible en un objeto Person
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Tu tarea es completar esta implementación para que la línea `let p =
// Person::from("Mark,20")` compile Ten en cuenta que tendrá que parsear el
// componente age en un `usize` con algo como `"4".parse::<usize>()`. El
// resultado de esto necesita ser manejado apropiadamente.
//
// Pasos:
// 1. Si la longitud del String proporcionado es 0, entonces devuelve el valor Default de
//    Person.
// 2. Dividir el String dado en las comas presentes en él.
// 3. Extraer el primer elemento de la operación de división y utilizarlo como nombre.
// 4. Si el nombre está vacío, devuelve el Default de Person.
// 5. Extraer el otro elemento de la operación de división y pasarlo en un
//    `usize` como el age.
// Si al parsear el age, algo va mal, entonces ddevuelve el valor Default de
// Person De lo contrario, devuelve un objeto Person instanciado con los resultados

// I AM NOT DONE

impl From<&str> for Person {
    fn from(s: &str) -> Person {
    }
}

fn main() {
    // Usa la función `from`
    let p1 = Person::from("Mark,20");
    // Dado que From está implementado para Person, deberíamos poder utilizar Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Prueba que por defecto es Juan de 30 años
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Comprueba que se devuelve Jhon cuando se proporciona el String incorrecto
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Prueba de que "Mark,20" funciona
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Prueba que "Mark,twenty" devolverá la persona por default debido a un
        // error en el parseo de la edad
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "Mike");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "Mike");
        assert_eq!(p.age, 32);
    }
}
