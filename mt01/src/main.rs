use std::thread;
use std::time::Duration;

fn main() {

    // create a thread
    let handle = thread::spawn(|| {

        // everything in here runs
        // in its own separate thread
        for i in 0..10 {

            println!("Loop 2 iteration: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // main thread
    for i in 0..5 {

        println!("Loop 1 iteration: {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    handle.join().unwrap();
}