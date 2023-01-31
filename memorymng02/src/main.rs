// error, utlizar valor que salido del ambito
// let x;
// {
//     let y = 42;
//     x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
// }
// println!("x: {}", x); // `x` refers to `y` but `y has been dropped!

// error
// let x;
// {
//     let y = 42;
//     x = &y;
// }
// println!("x: {}", x);

#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn erase(_: String) {}

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    erase(text);

    println!("{:?}", fox);
    println!("{:?}", dog);
}
