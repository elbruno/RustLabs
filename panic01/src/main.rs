fn main() {
    // let v = vec![0, 1, 2, 3];
    // println!("{}", v[6]); // this will cause a panic!

    loop {
        let mut line = String::new();
        println!("Ingrese su nombre o 'q' para salor :");
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "q" {
            //break;
            panic!("Error");
        }
        println!("Hola , {}", line);
        println!("nro de bytes leidos , {}", b1);
    }
}
