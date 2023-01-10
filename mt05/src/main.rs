/*
   Copyright (c) 2023
   Author      : Bruno Capuano
   Create Time : 2023 January
   Change Log  :
   - Demos working with threads in Rust
   
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

use std::thread;
use std::time::Duration;

fn main() {
    let sec_thread =  thread::spawn(|| {
        println!("  >> Begin thread work, wait 5 seconds ...");
        thread::sleep(Duration::from_millis(5000));
        println!("  >> End thread work ...");
    });

    for i in 0..5 {
        println!("Loop 1 iteration: {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    sec_thread.join().unwrap();
}
