use std::sync::MutexGuard;

use clap::ValueEnum;

use crate::{locked_resource::LockedResource, philosopher::Philosopher};

#[derive(Debug, Copy, Clone)]
pub struct Fork {
    pub id: usize,
}

#[derive(Debug)]
pub struct Kitchen {
    waiter: Waiter,
}

impl Kitchen {
    pub fn new(num_forks: usize, algorithm: WaiterAlgorithm) -> Self {
        let forks = Kitchen::create_forks(num_forks);
        let waiter = Waiter::new(&forks, algorithm);
        Kitchen { waiter: waiter }
    }

    fn create_forks(num_forks: usize) -> Vec<Fork> {
        let mut forks: Vec<Fork> = vec![];
        for i in 0..num_forks {
            forks.push(Fork { id: i })
        }
        forks
    }

    pub fn waiter(&self) -> &Waiter {
        &self.waiter
    }
}

#[derive(Debug)]
pub struct Waiter {
    forks: Vec<LockedResource<Fork>>,
    algorithm: WaiterAlgorithm,
}

impl Waiter {
    fn new(forks: &[Fork], algorithm: WaiterAlgorithm) -> Self {
        let mut locked_forks = Vec::<LockedResource<Fork>>::new();
        for fork in forks {
            locked_forks.push(LockedResource::new(*fork));
        }
        Waiter {
            forks: locked_forks,
            algorithm: algorithm,
        }
    }

    pub fn get_forks(&self, philosopher: &Philosopher) -> (MutexGuard<'_, Fork>, MutexGuard<'_, Fork>) {
        let (left, right) = (philosopher.left_fork_id, philosopher.right_fork_id);
        let comparison = match self.algorithm {
            WaiterAlgorithm::IdBased => philosopher.id % 2 == 0,
            WaiterAlgorithm::LeftRight => left < right,
            WaiterAlgorithm::Deadlock => true,
        };
        if comparison {
            let left_fork = self.pick_up_fork(philosopher, left);
            let right_fork = self.pick_up_fork(philosopher, right);
            (left_fork, right_fork)
        } else {
            let right_fork = self.pick_up_fork(philosopher, right);
            let left_fork = self.pick_up_fork(philosopher, left);
            (left_fork, right_fork)
        }
    }

    fn pick_up_fork(&self, philosopher: &Philosopher, fork_id: usize) -> MutexGuard<'_, Fork> {
        println!("{} trying to pick up fork {}", philosopher.name, fork_id);
        let fork = self.forks.get(fork_id).unwrap().get();
        println!("{} picked up fork {}", philosopher.name, fork.id);
        fork
    }
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum WaiterAlgorithm {
    IdBased,
    LeftRight,
    Deadlock,
}
