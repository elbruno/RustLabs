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
    let mascot = String::from("ferris");
    process(mascot.clone());
    process(mascot);
}

fn process(input: String) {}
fn process_number(input: u32) {}
