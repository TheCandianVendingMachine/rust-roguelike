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
use crate::game::state_machine::StateMachine;

pub trait State {
    fn on_push(&mut self) {}
    fn on_pop(&mut self) {}

    fn init(&mut self) {}
    fn deinit(&mut self) {}

    fn update(&mut self) {}
    fn update_fixed(&mut self, delta_time: f64) {}
    fn pre_update(&mut self) {}
    fn post_update(&mut self) {}
}

