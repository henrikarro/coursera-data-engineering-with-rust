use std::{thread, time::Instant};

use clap::Parser;
use clap_num::number_range;
use env_logger;
use log;

use dining_philosophers::{
    kitchen::{Kitchen, Waiter, WaiterAlgorithm},
    philosopher::{NUM_PHILOSOPHERS, Philosopher, create_philosophers},
};

fn parse_num_philosophers(s: &str) -> Result<usize, String> {
    number_range(s, 1, NUM_PHILOSOPHERS)
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(short='p', long, default_value_t=NUM_PHILOSOPHERS, value_parser=parse_num_philosophers)]
    num_philosphers: usize,

    #[arg(short = 'f', long, default_value = "4")]
    num_forks: usize,

    #[arg(short, long, default_value = "id-based")]
    algorithm: WaiterAlgorithm,
}

fn main() {
    configure_logging();

    let args = Args::parse();

    let num_forks = args.num_forks;
    let algorithm = args.algorithm;
    let kitchen = Kitchen::new(num_forks, algorithm);
    let philosophers = create_philosophers(args.num_philosphers, num_forks);

    log::info!(
        "We have {} philosophers and {} forks, using the {:?} waiter algorithm",
        philosophers.len(),
        num_forks,
        algorithm
    );

    let start = Instant::now();

    feast(&philosophers, &kitchen.waiter());

    log::info!("Total time: {:?}", start.elapsed());
}

fn configure_logging() {
    let env = env_logger::Env::new().filter_or("RUST_LOG", "info");
    env_logger::Builder::from_env(env).init();
}

fn feast(philosophers: &Vec<Philosopher>, waiter: &Waiter) {
    thread::scope(|scope| {
        for philosopher in philosophers {
            scope.spawn(|| philosopher.eat(waiter));
        }
    });
}
