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

mod engine_temp;
mod renderer;
mod game;

use crate::engine_temp::engine::Engine;
use crate::engine_temp::fence::FenceRC;

use game::game::Game;

use std::sync::mpsc;

use simplelog::*;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();

    loop {

    }

    let (_send, recv) = mpsc::channel();
    let mut engine = Engine::new(recv, None);
    engine.game_handler.state_machine.queue_push(Box::new(Game::new()));
    engine.run();
}
