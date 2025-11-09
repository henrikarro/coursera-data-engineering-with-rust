use std::{thread, time::Duration};

use crate::kitchen::Waiter;

pub struct Philosopher {
    pub id: usize,
    pub left_fork_id: usize,
    pub right_fork_id: usize,
    pub name: String,
}

impl Philosopher {
    fn new(id: usize, left_fork_id: usize, right_fork_id: usize, name: &str) -> Self {
        Philosopher {
            id: id,
            left_fork_id: left_fork_id,
            right_fork_id: right_fork_id,
            name: name.to_string(),
        }
    }

    pub fn eat(&self, waiter: &Waiter) {
        let forks = waiter.get_forks(self);

        println!("{} eating...", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} finished eating", self.name);

        println!("{} putting down fork {}", self.name, forks.0.id);
        println!("{} putting down fork {}", self.name, forks.1.id);
    }
}

static PHILOSOPHER_NAMES: &[&str] = &[
    "Jürgen Habermas",
    "Friedrich Engels",
    "Karl Marx",
    "Thomas Piketty",
    "Michel Foucault",
    "Socrates",
    "Plato",
    "Aristotle",
    "Pythagoras",
    "Heraclitus",
    "Democritus",
    "Diogenes",
    "Epicurus",
    "Zeno of Citium",
    "Thales of Miletus",
];

pub static NUM_PHILOSOPHERS: usize = PHILOSOPHER_NAMES.len();

pub fn create_philosophers(num_philosophers: usize, num_forks: usize) -> Vec<Philosopher> {
    PHILOSOPHER_NAMES
        .iter()
        .enumerate()
        .filter(|(id, _name)| id < &num_philosophers)
        .map(|(id, name)| create_philosopher(num_forks, id, *name))
        .collect()
}

fn create_philosopher(num_forks: usize, id: usize, name: &str) -> Philosopher {
    let left_fork_id = id % num_forks;
    let right_fork_id = (left_fork_id + 1) % num_forks;
    Philosopher::new(id, left_fork_id, right_fork_id, name)
}
