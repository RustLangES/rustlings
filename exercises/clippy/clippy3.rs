// clippy3.rs
//
// Aquí hay un par de correcciones de Clippy más fáciles, para que puedas ver su utilidad.
// Sin pistas.

// NO ESTOY HECHO

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        my_option.unwrap();
    }

    let my_arr = &[
        -1, -2, -3
        -4, -5, -6
    ];
    println!("Mi array! Aquí está: {:?}", my_arr);

    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("Este Vector está vacío, ¿Lo ves? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    value_a = value_b;
    value_b = value_a;
    println!("valor a: {}; valor b: {}", value_a, value_b);
}
