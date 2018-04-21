use core::sync::atomic::{fence, Ordering};

// This is meant to be used to create slightly unsafe global variables where you would like
// them to be updated if possible, but are not able to use a lock to control access and will
// probably be using usafe options on a global. Intended use case is for a panic function
// that would like to query some options on what to do and clearly cannot wait on a lock
// To allow it to be used in globals the internal member is public, but if you access through
// the public member beyond construction you will of course not actually get the benefits of
// the fencing.
pub struct FencedVar<T: Copy>{ pub construction_value: T }

impl<T: Copy> FencedVar<T> {
    pub fn get(&self) -> T {
        fence(Ordering::Acquire);
        self.construction_value
    }
    pub fn set(&mut self, v: T) {
        self.construction_value = v;
        fence(Ordering::Release);
    }
}
