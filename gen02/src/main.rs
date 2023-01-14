/*
   Copyright (c) 2023
   Author      : Bruno Capuano
   Create Time : 2023 January
   Change Log  :
   - Demos working with generics in Rust
   
   The MIT License (MIT)
   Permission is hereby granted, free of charge, to any person obtaining a copy
   of this software and associated documentation files (the "Software"), to deal
   in the Software without restriction, including without limitation the rights
   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
   copies of the Software, and to permit persons to whom the Software is
   furnished to do so, subject to the following conditions:
   The above copyright notice and this permission notice shall be included in
   all copies or substantial portions of the Software.
   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
   THE SOFTWARE.
*/

use std::fmt::Display;

// Generic Function to print data to the console
fn console_log<T: Display>(x: T) {
    println!("{}", x);
}

fn main() {
    // create a new Pet named Ace
    let ace = Pet {
        name: String::from("Ace the Puppy"),
        age: 1,
    };

    // print different types of data
    console_log("Hello World");
    console_log(123);
    console_log(ace.log_pet_info());
}

struct Pet {
    name: String,
    age: u32,
}

// Player specific methods
impl Pet {
    fn log_pet_info(self) -> String {
        return format!("Pet Name: {} - Age: {}", self.name, self.age);
    }
}
