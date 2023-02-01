#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    genre: char,
}

fn main() {
    // Vector of family members
    let mut family: Vec<Person> = Vec::new();

    family.push(Person {
        name: "Frank".to_string(),
        age: 23,
        genre: 'M',
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
        println!("name : {:#?}", individual.name);
        println!("age : {:#?}", individual.age);
        println!("name : {:#?}", individual.genre);
    }
    println!("{}", "-".repeat(50));

    // // Add family lastname
    // for member in &family {
    //     //member.name = member.name.pus  + "Thompson";
    //     member.name.push_str("Thompson");
    // }

    // // Increase age by 2
    // for adult in &family {
    //     adult.age += 2;
    // }

    // // Show updated info
    // println!("{}", "-".repeat(50));
    // for individual in &family {
    //     match individual.genre {
    //         'M' => {
    //             println!("{} is a man {} years old", individual.name, individual.age);
    //         }
    //         'F' => {
    //             println!(
    //                 "{} is a woman {} years old",
    //                 individual.name, individual.age
    //             );
    //         }
    //         _ => { // all cases goes here
    //         }
    //     }
    // }
}
