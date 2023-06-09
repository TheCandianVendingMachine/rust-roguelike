/*
    A roguelike game created for a fun exercise
    Copyright (C) 2023  Bailey Danyluk

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::{Arc};
use std::hint;
use std::clone::Clone;

/// A way to synchronise between multiple threads.
///
/// A fence has two states, open and closed. If the fence is open, when we synchronise
/// the program won't block while if the fence is closed we will have to wait for it to 
/// open again before continuing.
///
/// If all references of a Fence wait when it is closed, we will run into a deadlock
pub struct Fence {
    is_open: AtomicBool
}

pub struct FenceRC(pub Arc<Fence>);
impl FenceRC {
    pub fn new() -> FenceRC {
        FenceRC(Arc::new(Fence::new()))
    }
}

impl Clone for FenceRC {
    fn clone(&self) -> Self {
        FenceRC(self.0.clone())
    }
}

impl Fence {
    pub fn new() -> Fence {
        Fence {
            is_open: AtomicBool::new(true)
        }
    }

    /// Open the fence and let any blocking thread through
    pub fn open(&self) {
        self.is_open.store(true, Ordering::Relaxed)
    }

    /// Close the fence and block any thread that attemps to pass
    pub fn close(&self) -> &Self {
        self.is_open.store(false, Ordering::Relaxed);
        self
    }

    // Pass through the fence. If fence is closed, we will wait until it is open again.
    // If it is open, we pass through
    pub fn pass(&self) {
        while !self.is_open.load(Ordering::Relaxed) {
            hint::spin_loop();
        }
    }

    // If the fence is closed, we run func_on_closed. Afterward we attempt to pass 
    // through the fence
    pub fn pass_or_execute<F>(&self, func_on_closed: F) 
        where F: FnOnce() 
        {
        if !self.is_open.load(Ordering::Relaxed) {
            func_on_closed();
        }
        self.pass();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::{ Duration, Instant };

    macro_rules! assert_timeout{
        ($left:expr, $timeout:expr, $delay:expr) => {
            assert_eq!(thread::spawn(move || {
                let start_time = Instant::now();
                while !$left.is_finished() {
                    if Instant::now() - start_time >= Duration::from_millis($timeout) {
                        panic!("Did not terminate in time");
                    }
                    thread::sleep(Duration::from_millis($delay));
                }
                $left.join()
            }).join().is_ok(), true);
        }
    }

    #[test]
    fn test_fence() {
        {
            let fence = FenceRC::new();
            let f0 = fence.0.clone();
            let t0 = thread::spawn(move || {
                f0.pass();
            });
            assert_timeout!(t0, 1000, 100);
        }

        {
            let fence = FenceRC::new();
            let f0 = fence.0.clone();
            let f1 = fence.0.clone();

            fence.0.close();

            let t0 = thread::spawn(move || {
                f0.pass()
            });

            thread::spawn(move || {
                thread::sleep(Duration::from_millis(500));
                f1.open()
            }).join().unwrap();

            assert_timeout!(t0, 1000, 250);
        }

        {
            let fence = FenceRC::new();
            let f0 = fence.0.clone();
            fence.0.close();

            let t0 = thread::spawn(move || {
                f0.pass_or_execute(|| { thread::sleep(Duration::from_millis(100)); f0.open() });
            });
            assert_timeout!(t0, 500, 100);
        }
    }

}

