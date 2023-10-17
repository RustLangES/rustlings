// as_ref_mut.rs
//
// AsRef y AsMut permiten realizar conversiones sin costo de referencia a referencia.
// Mas información en https://doc.rust-lang.org/std/convert/trait.AsRef.html y
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectivamente.
//
// Ejecute `rustlings hint as_ref_mut` o utilice el subcomando watch `hint`
// para una sugerencia.

// I AM NOT DONE

// Obtenga el número de bytes (no caracteres) del argumento dado.
// TODO: Añadir el Trait AsRef apropiadamente como un Trait Bound.
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtenga el número de caracteres (no bytes) en el argumento dado.
// TODO: Añadir el Trait AsRef apropiadamente como un Trait Bound.
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Eleva al cuadrado un número usando as_mut().
// TODO: Añadir el Trait Bound apropiado.
fn num_sq<T>(arg: &mut T) {
    // TODO: Implementar el cuerpo de la función.
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
