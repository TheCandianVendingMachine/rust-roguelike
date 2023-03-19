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

use crate::input::Input;
use crate::fence::FenceRC;
use std::sync::mpsc;

/// The way the game is initialised and ran.
///
/// The engine will setup a render fence and input queue, and will update the game state
/// as needed. If initialised with a render fence, it will synchronise the game state
/// with the renderer as needed
pub struct Engine {
    render_fence: Option<FenceRC>,
    input_queue: mpsc::Receiver<Input>,
    running: bool
}

impl Engine {
    pub fn new(input_queue: mpsc::Receiver<Input>, render_fence: Option<FenceRC>) -> Engine {
        Engine {
            render_fence,
            input_queue,
            running: true
        }
    }

    /// Simulate one tick of the game state
    pub fn tick(&mut self) {
        while self.running {
            loop {
                let input = match self.input_queue.try_recv() {
                    Ok(input) => input,
                    _ => { break }
                };

                // Handle input
            }

            if let Some(render_fence) = &self.render_fence {
                render_fence.0.close().pass();
            }
        }
    }
}
