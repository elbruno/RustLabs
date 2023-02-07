fn main() {
    let mascotas = vec!["Red", "Yellow", "Green"];

    // iterator
    let mut mascotas_iterator = mascotas.iter();
    println!("mascotas iterator = {:?}", mascotas_iterator);

    // fetch values from iterator one by one using next() method
    println!("{:?}", mascotas_iterator.next());
    println!("{:?}", mascotas_iterator.next());
    println!("{:?}", mascotas_iterator.next());
    println!("{:?}", mascotas_iterator.next());
    
    println!("{}", "-".repeat(20));

    // using iter() to iterate through a collection
    for color in mascotas.iter() {
        // reference to the items in the iterator
        println!("{}", color);
    }

    println!("{}", "-".repeat(20));
    // using iter() to iterate through a collection
    for color in mascotas.iter() {
        // reference to the items in the iterator
        println!("{}", color);
    }    
}
