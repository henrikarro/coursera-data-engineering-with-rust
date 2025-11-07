use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::{Duration, SystemTime},
};

use rand::Rng;

const NUM_THREADS: usize = 10;

fn main() {
    let now = SystemTime::now();
    let data = create_data(NUM_THREADS);
    print_reference_counts(&data);
    print_data("Before expensive computation", &data);
    assert_vector(&data, |i| i + 1);
    let handles = start_threads(&data);
    for handle in handles {
        handle.join().unwrap();
    }
    assert_vector(&data, |i| i + 2);
    print_data("After expensive computation", &data);
    print_reference_counts(&data);
    assert!(
        now.elapsed().unwrap().as_secs() < 3,
        "This took too long! Elapsed time: {:?}",
        now.elapsed().unwrap()
    );
}

fn start_threads(data: &Arc<Mutex<Vec<i32>>>) -> Vec<thread::JoinHandle<()>> {
    let handles = (0..data.lock().unwrap().len())
        .map(|i| {
            let handle = {
                let my_data = data.clone();
                print_reference_counts(&my_data);
                thread::spawn(move || {
                    expensive_computation(i, my_data);
                })
            };
            handle
        })
        .collect::<Vec<_>>();
    handles
}

// "Computation" functions

fn expensive_computation(i: usize, data: Arc<Mutex<Vec<i32>>>) {
    println!("Thread {} starting...", i);
    let now = SystemTime::now();
    let mut rng = rand::rng();
    sleep(Duration::from_millis(rng.random_range(250..1000)));
    lock_and_perform_computation(i, data);
    sleep(Duration::from_millis(rng.random_range(250..1000)));
    println!("Thread {} exiting after {:?}", i, now.elapsed().unwrap());
}

fn lock_and_perform_computation(i: usize, data: Arc<Mutex<Vec<i32>>>) {
    println!("Thread {} working on shared vector", i);
    let mut data = data.lock().unwrap();
    data[i] += 1;
}

// Utility functions

fn create_data(size: usize) -> Arc<Mutex<Vec<i32>>> {
    let mut data = Vec::new();
    for i in 1..=size {
        data.push(i as i32);
    }
    Arc::new(Mutex::new(data))
}

fn print_data(heading: &str, data: &Arc<Mutex<Vec<i32>>>) {
    let data = data.lock().unwrap();
    let string_data = (*data).iter().map(|i| i.to_string()).collect::<Vec<String>>();
    println!("{}: {}", heading, string_data.join(", "));
}

fn print_reference_counts(data: &Arc<Mutex<Vec<i32>>>) {
    println!("Number of references to data: {}", Arc::strong_count(data));
}

fn assert_vector(data: &Arc<Mutex<Vec<i32>>>, f: fn(usize) -> usize) {
    let data = data.lock().unwrap();
    for i in 0..data.len() {
        assert_eq!(f(i), data[i] as usize);
    }
}
