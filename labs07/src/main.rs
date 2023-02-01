#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    genre: char,
}

// implement Clone for Person struct
impl Clone for Person {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            age: self.age,
            genre: self.genre,
        }
    }
}

fn main() {
    // Vector of family members
    let mut family: Vec<Person> = Vec::new();
    family.push(Person {
        name: "Alice".to_string(),
        age: 20,
        genre: 'F',
    });

    family.push(Person {
        name: "Carol".to_string(),
        age: 25,
        genre: 'F',
    });

    family.push(Person {
        name: "Edwin".to_string(),
        age: 34,
        genre: 'M',
    });

    // Show current info
    println!("{}", "-".repeat(50));
    println!("Show current information");
    println!("{}", "-".repeat(50));
    for individual in &family {
        println!("{:#?}", individual);
    }
    println!("{}", "-".repeat(50));

    // Iterate through Family and add a family lastname to the field name
    for individual in &mut family {
        individual.name = format!("{} {}", individual.name, "Thompson");
    }

    println!("{}", "-".repeat(50));
    println!("Show new information after adding the last name");
    println!("{}", "-".repeat(50));
    for individual in &family {
        println!("{:#?}", individual);
    }
    println!("{}", "-".repeat(50));

    // rest of the code goes here ...
}
