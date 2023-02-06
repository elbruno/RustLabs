fn main() {
    // create new empty vector with generic datatype
    let vChar = vec!['A','c','e'];
    let vNum = vec![1,2];

    print_variable_type(&vChar[0]);
    print_variable_type(&vNum[0]);
    
}

fn print_variable_type<K>(_: &K) {
    println!("{}", std::any::type_name::<K>())
}
