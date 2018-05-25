extern crate md5;

use md5::compute;

use std::sync::Arc;
use std::sync::atomic::{Ordering, ATOMIC_BOOL_INIT, ATOMIC_USIZE_INIT};
use std::thread;
use std::time::{SystemTime};

const NUM_THREADS: u32 = 20;

fn time_since_start(start: SystemTime) {
    match start.elapsed() {
        Ok(elapsed) => {
            println!("Total running time: ~ {} seconds", elapsed.as_secs());
        }
        Err(_) => {
            println!("OUCH!");
        }
    }
}

fn main() {
    let start = SystemTime::now();
    let target: Vec<char> = vec!['0', '0', '0', '0', '0', '0'];

    let mut handles = vec![];
    let counter = Arc::new(ATOMIC_USIZE_INIT);
    let is_found = Arc::new(ATOMIC_BOOL_INIT);


    for _ in 0u32..NUM_THREADS {
        let counter_clone = counter.clone();
        let is_found_clone = is_found.clone();
        let target_clone = target.clone();

        let handle = thread::spawn(move || {
            let mut inner_found = is_found_clone.load(Ordering::Relaxed);

            while !inner_found {
                let inner_counter = counter_clone.fetch_add(1, Ordering::Relaxed);
                let string = format!("{}{}", "ckczppom", inner_counter);
                let digest = compute(string);
                let current_digest = format!("{:x}", digest);
                
                let first_x = current_digest.chars().take(6).collect::<Vec<char>>();

                if first_x == target_clone {
                    println!("Counter: {} -- {:?}", inner_counter, first_x);
                    is_found_clone.store(true, Ordering::Relaxed);
                }

                inner_found = is_found_clone.load(Ordering::Relaxed);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", counter);

    time_since_start(start);
}
