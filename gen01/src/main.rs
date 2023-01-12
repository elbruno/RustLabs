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

use std::collections::HashMap;

fn main() {

    let mut names_and_ages: HashMap<&str, i32> = HashMap::new();

    names_and_ages.insert("Jeff the Squirrel", 2);
    names_and_ages.insert("Ace the Puppy", 1);
    names_and_ages.insert("Net the Cat", 4);

    // print the names and ages
    for (name, age) in &names_and_ages {
        println!("{} is {} years old", name, age);
    }
}