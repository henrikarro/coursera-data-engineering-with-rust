use std::{thread, time::Instant};

use dining_philosophers::{
    kitchen::{Kitchen, Waiter, WaiterAlgorithm},
    philosopher::{Philosopher, create_philosophers},
};

const NUM_FORKS: usize = 4;

fn main() {
    let algorithm = WaiterAlgorithm::IdBased;
    let kitchen = Kitchen::new(NUM_FORKS, algorithm);
    let philosophers = create_philosophers(NUM_FORKS);

    println!(
        "We have {} philosophers and {} forks, using the {:?} waiter algorithm",
        philosophers.len(),
        NUM_FORKS,
        algorithm
    );

    let start = Instant::now();

    feast(&philosophers, &kitchen.waiter());

    println!("Total time: {:?}", start.elapsed());
}

fn feast(philosophers: &Vec<Philosopher>, waiter: &Waiter) {
    thread::scope(|scope| {
        for philosopher in philosophers {
            scope.spawn(|| philosopher.eat(waiter));
        }
    });
}
