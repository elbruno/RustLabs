use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        println!("  >> Begin thread work, wait 2 seconds ...");
        thread::sleep(Duration::from_millis(2000));
        println!("  >> End thread work ...");
    });

    for i in 0..5 {
        println!("Loop 1 iteration: {}", i);
        thread::sleep(Duration::from_millis(500));
    }
}
