use std::collections::HashMap;

fn main() {
    let mut mascotas = HashMap::new();
    mascotas.insert("Ace", "perro");
    mascotas.insert("Goku", "gato");
    mascotas.insert("Jim", "ardilla");
    mascotas.insert("Net", "gato");
    mascotas.insert("Val", "pez");
    println!("1 - {:?}", mascotas);

    let goku = "Goku";
    let goku_animal = mascotas.get(goku);
    println!("{} es un {:?}", goku, goku_animal);

    // borrar un elemento
    // let jim = "Jim";
    // mascotas.remove(jim);
    // println!("2 - {:?}", mascotas);

    // let jim_animal = mascotas.get(jim);
    // println!("{} es un {:?}", jim, jim_animal);

    // ejemplo de un bucle
    println!("");
    println!("==================");
    println!("Ejemplo de Bucle");
    println!("==================");
    for (key, value) in &mascotas {
        if value != &"pez" {
            println!("{} es un {}", key, value);
        } else {
            println!("{} es un {} y no nos gustan los peces!", key, value);
        }
    }

    println!("");
    println!("==================");
    println!("Ejemplo de LOOP");
    println!("==================");

    let mut counter = 1;
    loop {
        println!("We loop forever! {}", counter);
        if(counter == 5) {
            println!("terminamos el loop !");
            break;
        }
        counter += 1;
    }

    println!("");
    println!("==================");
    println!("Ejemplo de WHILE");
    println!("==================");

    counter = 1;
    while counter > 0 {
        println!("We loop a while... {}", counter);
        counter = counter + 1;
    }
}
