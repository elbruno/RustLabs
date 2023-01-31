fn main() {
    let fruits = vec!["coco", "banana"];
    for &index in [1, 0, 5].iter() {
        match fruits.get(index) {
            Some(&"coco") => println!("Coco!!!"),
            Some(fruit_name) => println!("{}", fruit_name),
            None => println!("No fruta en {}", index),
        }
    }
}