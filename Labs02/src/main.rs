use std::collections::HashMap;

fn main() {
    let mut mascotas = HashMap::new();

    mascotas.insert("Ace", "dog");
    mascotas.insert("Goku", "cat");
    mascotas.insert("Val", "fish");
    mascotas.insert("Jim", "squirrel");

    println!("1 {:?}", mascotas);

    // get element from hashmap based on key
    let mascota_nombre = "Goku";
    let mascota_edad = mascotas.get(&mascota_nombre);
    println!("{:?} is {:?} yo", mascota_nombre, mascota_edad);
    println!("2 {:?}", mascotas);

    // // delete element using the key "Jim"
    // let mascota_borrar = "Jim";
    // mascotas.remove(&mascota_borrar);
    // println!("3 {:?}", mascotas);

    // // get deleted element
    // let mascota = mascotas.get(&mascota_borrar);
    // println!("{:?} is {:?} yo", mascota_borrar, mascota);
    // println!("4 {:?}", mascotas);

    // loop to show mascotas
    println!("loop to show mascotas");
    for (key, value) in &mascotas {
        println!("{} is a {}", key, value);
    }

    // loop to show mascotas but not fish
    println!("loop to show mascotas but not fish");
    for (key, value) in &mascotas {
        println!("{} is a {}", key, value);
        if value == &"fish" {
            break;
        }
    }

    // use a while loop to show mascotas
    println!("While Loop Demo");
    let mut contador = 0;
    while contador < mascotas.len() {
        for (key, value) in &mascotas {
            println!("{} is a {}", key, value);
            contador += 1;
        }
    }


}
