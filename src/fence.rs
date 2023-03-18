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
    open: AtomicBool
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
            open: AtomicBool::new(true)
        }
    }

    pub fn open(&self) {
        self.open.store(true, Ordering::Relaxed)
    }

    pub fn close(&self) {
        self.open.store(false, Ordering::Relaxed);
    }

    pub fn wait(&self) {
        while self.open.load(Ordering::Relaxed) {
            hint::spin_loop();
        }
    }
}

