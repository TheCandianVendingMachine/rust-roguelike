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

mod fence;
mod engine;
mod input;

use crate::engine::Engine;
use crate::fence::FenceRC;

use std::sync::mpsc;

fn main() {
    let render_fence = FenceRC::new();

    let (_send, recv) = mpsc::channel();
    let mut engine = Engine::new(recv, Some(render_fence.clone()));

    loop {
        engine.tick();
    }

}
