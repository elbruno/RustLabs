use std::collections::HashMap;

fn main() {
    let mut pets = HashMap::new();
    pets.insert("Ace", "dog");
    pets.insert("Goku", "car");
    pets.insert("Jim", "squirrel");
    println!("{:#?}", pets);

    let net = "Net";
    let net_animal = pets.get(net);
    println!("{} is a {:?}", net, net_animal);
}
