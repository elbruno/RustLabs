fn main() {
    // {
    //     let mascot = String::from("ferris");
    // }
    // println!("{}", mascot);

    // error
    // let mascot = String::from("ferris");
    // let ferris = mascot;
    // println!("{}", mascot);

    // works ferris is a constant
    // let mascot = "ferris";
    // let ferris = mascot;
    // println!("{}", mascot);

    // error, string no se copia hace valor por referencia
    // let mascot = String::from("ferris");
    // process(mascot);
    // process(mascot);

    // works, u32 implementa copy
    // let n:u32 = 1;
    // process_number(n);
    // process_number(n);

    // works, copia explicita del valor
    // let mascot = String::from("ferris");
    // process(mascot.clone());
    // process(mascot);

    // works, se pasa referencia
    // let mascot = String::from("ferris");
    // let ferris = &mascot;
    // println!("1 - {}", mascot);
    // println!("2 - {}", ferris);

    // works, se pasa referencia
    // let mascot = String::from("ferris");
    // print_mascot(&mascot);
    // print_mascot(&mascot);

    // // error, no se puede modificar una referencia
    // let mascot = String::from("ferris");
    // change_mascot_err(&mascot);

    // works, se pasa referencia mutable
    let mut mascot = String::from("ferris");
    change_mascot_ok(&mut mascot);
}

// fn change_mascot_err(mascot: &String) {
//     mascot.push_str(" the crab");
// }

fn change_mascot_ok(mascot: &mut String) {
    mascot.push_str(" the crab");
}

fn print_mascot(mascot: &String) {
    println!("pet - {}", mascot);
}
fn process(input: String) {}
fn process_number(input: u32) {}
