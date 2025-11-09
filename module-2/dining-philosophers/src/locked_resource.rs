use std::sync::{Mutex, MutexGuard, TryLockResult};

#[derive(Debug)]
pub struct LockedResource<T> {
    lock: Mutex<T>,
}

impl<T> LockedResource<T> {
    pub fn new(data: T) -> Self {
        LockedResource { lock: Mutex::new(data) }
    }

    pub fn get(&self) -> MutexGuard<'_, T> {
        self.lock.lock().unwrap()
    }

    pub fn try_get(&self) -> TryLockResult<MutexGuard<'_, T>> {
        self.lock.try_lock()
    }
}
