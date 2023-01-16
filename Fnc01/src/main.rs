fn main() {
    HolaR();
    holaM();
    show_message("123");
}

fn HolaR() {
    println!("Hola Reactor!");
}

fn holaM() {
    println!("Hello, world!");
}

// create a function that receives a string and prints it
fn show_message(message: &str) {
    println!("{}", message);
}
