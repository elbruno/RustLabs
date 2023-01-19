/*
   Copyright (c) 2023
   Author      : Bruno Capuano
   Create Time : 2023 January
   Change Log  :
   - Demo working with arrays and println! macro
   
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
fn main() {
    // Declare array with weekdays
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    // print days triggers an error
    // ^^^^ `[&str; 7]` cannot be formatted with the default formatter
    // println!("{}", days);

    // print days in debug mode
    println!("{:?}", days);

    // print days in debug mode with a beautifier
    println!("{:#?}", days);

    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];

    // print bytes in debug mode
    println!("{:?}", bytes);

    // print bytes in debug mode with a beautifier
    println!("{:#?}", bytes);
}
