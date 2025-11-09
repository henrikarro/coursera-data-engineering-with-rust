use std::{sync::MutexGuard, thread, time::Duration};

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
        if self.algorithm == WaiterAlgorithm::Greedy {
            return self.greedily_get_forks(philosopher);
        }
        let (left, right) = (philosopher.left_fork_id, philosopher.right_fork_id);
        let comparison = match self.algorithm {
            WaiterAlgorithm::IdBased => philosopher.id % 2 == 0,
            WaiterAlgorithm::LeftRight => left < right,
            WaiterAlgorithm::Deadlock => true,
            WaiterAlgorithm::Greedy => panic!("Greedy should already have been handled"),
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
        log::info!("{} trying to pick up fork {}", philosopher.name, fork_id);
        let fork = self.forks.get(fork_id).unwrap().get();
        log::info!("{} picked up fork {}", philosopher.name, fork.id);
        fork
    }

    fn greedily_get_forks(&self, philosopher: &Philosopher) -> (MutexGuard<'_, Fork>, MutexGuard<'_, Fork>) {
        loop {
            if let Some((first_fork, second_fork)) = self.find_any_free_forks() {
                log::info!(
                    "{} greedily picked up forks {} and {}",
                    philosopher.name, first_fork.id, second_fork.id
                );
                return (first_fork, second_fork);
            };
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn find_any_free_forks(&self) -> Option<(MutexGuard<'_, Fork>, MutexGuard<'_, Fork>)> {
        if let Some(first_fork) = self.find_any_free_fork() {
            if let Some(second_fork) = self.find_any_free_fork() {
                return Some((first_fork, second_fork));
            }
        }
        None
    }

    fn find_any_free_fork(&self) -> Option<MutexGuard<'_, Fork>> {
        for fork in &self.forks {
            if let Ok(locked_fork) = fork.try_get() {
                return Some(locked_fork);
            }
        }
        None
    }
}

/// Different ways the [`Waiter`] can pick forks.
#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum WaiterAlgorithm {
    /// For even philosophers; pick up the left fork first, then the right. For odd philosophers, start with the right fork instaed.
    IdBased,

    /// Pick up the left fork first, then the right, except for philosophers whose right fork has a lower id than the left.
    LeftRight,

    /// Always pick up the left fork first, then the right.
    Deadlock,

    /// Find any free fork, or wait until one becomes free. This ignores the philosophers' requested fork ids.
    Greedy,
}
