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
use crate::game::state::State;
use std::collections::VecDeque;

pub struct StateMachine {
    queued_states: VecDeque<Box<dyn State>>,
    state_stack: Vec<Box<dyn State>>,
    pop_count: usize,
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            queued_states: VecDeque::new(),
            state_stack: Vec::new(),
            pop_count: 0
        }
    }

    pub fn queue_push(&mut self, state: Box<dyn State>) {
        self.queued_states.push_back(state);
    }

    pub fn queue_pop(&mut self) {
        self.pop_count += 1
    }

    pub fn pre_update(&mut self) {
        while self.pop_count > 0 {
            self.state_stack.last_mut().unwrap().deinit();
            self.state_stack.pop();
            self.pop_count -= 1;
        }

        while let Some(state) = self.queued_states.pop_front() {
            self.state_stack.push(state);
            self.state_stack.last_mut().unwrap().init();
        }


        if let Some(state) = self.state_stack.last_mut() {
            state.pre_update();
        }
    }

    pub fn update(&mut self) {
        if let Some(state) = self.state_stack.last_mut() {
            state.update();
        }
    }

    pub fn update_fixed(&mut self, delta_time: f64) {
        if let Some(state) = self.state_stack.last_mut() {
            state.update_fixed(delta_time);
        }
 
    }

    pub fn post_update(&mut self) {
        if let Some(state) = self.state_stack.last_mut() {
            state.post_update();
        }
    }
}

