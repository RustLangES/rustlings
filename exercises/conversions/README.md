# Conversiones de tipos

Rust ofrece una multitud de formas de convertir un valor de un tipo dado en otro tipo.

La forma más simple de conversión de tipos es una expresión de conversión de tipos. Se denota con el operador binario `as`. Por ejemplo, `println!("{}", 1 + 1.0);` no se compilaría, ya que `1` es un entero mientras que `1.0` es un flotante. Sin embargo, `println!("{}", 1 as f32 + 1.0)` debería compilar. El ejercicio [`using_as`](using_as.rs) intenta cubrir esto.

Rust también ofrece traits que facilitan las conversiones de tipos al implementarlos. Estos traits se pueden encontrar en el módulo [`convert`](https://doc.rust-lang.org/std/convert/index.html).
Los Traits son los siguientes:

- `From` e `Into` cubiertos en [`from_into`](from_into.rs)
- `TryFrom` y `TryInto` cubiertos en [`try_from_into`](try_from_into.rs)
- `AsRef` y `AsMut` cubiertos en [`as_ref_mut`](as_ref_mut.rs)

Además, el módulo `std::str` ofrece un trait llamado [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) que ayuda a convertir strings en tipos de destino a través del método `parse` en strings. Si se implementa correctamente para un tipo dado `Person`, entonces `let p: Person = "Mark,20".parse().unwrap()` debería compilar y ejecutarse sin provocar un pánico.

Éstas deberían ser las formas principales ***dentro de la biblioteca estándar*** de convertir datos en los tipos deseados.

## Más información

No se tratan directamente en el libro, pero la biblioteca estándar tiene una gran documentación al respecto.

- [Conversiones](https://doc.rust-lang.org/std/convert/index.html)
- [El Trait `FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)
