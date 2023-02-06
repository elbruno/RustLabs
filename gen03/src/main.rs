use std::fmt;

#[derive(Debug, PartialEq)]
struct Mascota {
    nombre: String,
    edad: u32,
}

impl fmt::Display for Mascota {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mascota: {} - Edad: {}", self.nombre, self.edad)
    }
}

fn main() {
    let ace = Mascota {
        nombre: String::from("Ace the Puppy"),
        edad: 1,
    };

    let goku = Mascota {
        nombre: String::from("Goku"),
        edad: 4,
    };

    println!("ace: {:?}", ace);
    println!("goku: {:?}", goku);

    if ace == goku {
        println!("equal!");
    } else {
        println!("not equal!");
    }
}
