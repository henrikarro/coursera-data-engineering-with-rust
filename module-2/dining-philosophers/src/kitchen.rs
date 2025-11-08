use std::sync::MutexGuard;

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
        Waiter { forks: locked_forks, algorithm: algorithm }
    }
    pub fn get_forks(&self, philosopher: &Philosopher) -> (MutexGuard<'_, Fork>, MutexGuard<'_, Fork>) {
        let (left, right) = (philosopher.left_fork_id, philosopher.right_fork_id);
        let comparison = match self.algorithm {
            WaiterAlgorithm::IdBased => philosopher.id % 2 == 0,
            WaiterAlgorithm::LeftRight => left < right,
            WaiterAlgorithm::Deadlock => true,
        };
        if comparison {
            let left_fork = self.forks.get(left).unwrap().get();
            println!("{} picked up fork {}", philosopher.name, left_fork.id);
            let right_fork = self.forks.get(right).unwrap().get();
            println!("{} picked up fork {}", philosopher.name, right_fork.id);
            (left_fork, right_fork)
        } else {
            let right_fork = self.forks.get(right).unwrap().get();
            println!("{} picked up fork {}", philosopher.name, right_fork.id);
            let left_fork = self.forks.get(left).unwrap().get();
            println!("{} picked up fork {}", philosopher.name, left_fork.id);
            (left_fork, right_fork)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum WaiterAlgorithm {
    IdBased,
    LeftRight,
    Deadlock,
}
