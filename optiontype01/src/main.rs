use std::collections::HashMap;

fn main() {
    let mut pets = HashMap::new();
    pets.insert("Ace", "perro");
    pets.insert("Goku", "gato");
    pets.insert("Jim", "ardilla");
    println!("1 - {:?}", pets);

    // pick ace
    let ace = pets.get("Ace");
    println!("2 - {:?}", ace);

    // for loop iterating in pets with the keys "ace" and "jim"
    for pet_name in ["Ace", "Jim", "Bruno"].iter() {
        let pet = pets.get(pet_name);
        match pets.get(pet_name) {
            Some(pet_type) => println!("{} is a {}!", pet_name, pet_type),
            None => println!("There is no pet type for {:?}! :(", &pet),
        }
    }
}

fn vector_demo() {
    // create a pets vector
    let pets = vec!["ace", "goku", "jim"];

    // pick the first pet:
    let first = pets.get(0);
    println!("1 - {:?}", first);

    // pick the third pet:
    let third = pets.get(2);
    println!("2 - {:?}", third);

    // pick the 99th pet, which is non-existent:
    let non_existent = pets.get(99);
    println!("3 - {:?}", non_existent);
}

fn hasmap_demo() {
    let mut pets = HashMap::new();
    pets.insert("Ace", "perro");
    pets.insert("Goku", "gato");
    pets.insert("Jim", "ardilla");
    println!("1 - {:?}", pets);

    // pick ace
    let ace = pets.get("Ace");
    println!("2 - {:?}", ace);

    // pick the 99th pet, which is non-existent:
    let non_existent = pets.get("non_existent");
    println!("3 - {:?}", non_existent);
}
