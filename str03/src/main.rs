/*
   Copyright (c) 2023
   Author      : Bruno Capuano
   Create Time : 2023 January
   Change Log  :
   - Working with strings with strucs and named arguments

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

struct Pet {
    name: String,
    age: u8,
}

fn main() {
    let pet_ace = Pet {
        name: String::from("ACE"),
        age: 1,
    };

    // println!("{pet_ace.name} the pet is {pet_ace.age} years old.");
    // Trigger this error:
    // println!("{pet_ace.name} the pet is {pet_ace.age} years old.");
    //            ^^^^^^^^^^^^ not supported in format string

    println!("{} the pet is {} years old.", pet_ace.name, pet_ace.age);
    // Expected output: ACE the pet is 1 years old.
}
