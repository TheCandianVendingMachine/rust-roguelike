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
use crate::engine_temp::game::state::State;
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
            let mut state = self.state_stack.pop().unwrap();
            state.on_pop();
            if let Some(stack_state) = self.state_stack.last_mut() {
                stack_state.init();
            }
            self.pop_count -= 1;
        }

        while let Some(state) = self.queued_states.pop_front() {
            if let Some(stack_state) = self.state_stack.last_mut() {
                stack_state.deinit();
            }
            self.state_stack.push(state);
            self.state_stack.last_mut().unwrap().on_push();
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

    pub fn current_state(&self) -> Option<&Box<dyn State>> {
        self.state_stack.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestState{
        counter: usize,
        on_push_counter: usize,
        on_pop_counter: usize,
        init_counter: usize,
        deinit_counter: usize,
        pre_update_counter: usize,
        update_counter: usize,
        update_fixed_counter: usize,
        post_update_counter: usize
    }

    impl TestState {
        fn new(on_push_counter: usize, on_pop_counter: usize, init_counter: usize, deinit_counter: usize, pre_update_counter: usize, update_counter: usize, update_fixed_counter: usize, post_update_counter: usize) -> TestState {
            TestState {
                counter: 0,
                on_push_counter,
                on_pop_counter,
                init_counter,
                deinit_counter, 
                pre_update_counter,
                update_counter,
                update_fixed_counter,
                post_update_counter
            }
        }

        fn increment(&mut self) -> usize {
            self.counter += 1;
            self.counter
        }
    }

    impl State for TestState {
        fn on_push(&mut self) {
            let i = self.increment();
            assert_eq!(self.on_push_counter, i);
        }

        fn on_pop(&mut self) {
            let i = self.increment();
            assert_eq!(self.on_pop_counter, i);
        }

        fn init(&mut self) {
            let i = self.increment();
            assert_eq!(self.init_counter, i);
        }

        fn deinit(&mut self) {
            let i = self.increment();
            assert_eq!(self.deinit_counter, i);
        }

        fn pre_update(&mut self) {
            let i = self.increment();
            assert_eq!(self.pre_update_counter, i);
        }

        fn update(&mut self) {
            let i = self.increment();
            assert_eq!(self.update_counter, i);
        }

        fn update_fixed(&mut self, _delta_time: f64) {
            let i = self.counter;
            assert_eq!(self.update_fixed_counter, i);
        }

        fn post_update(&mut self) {
            let i = self.increment();
            assert_eq!(self.post_update_counter, i - 1);
        }
    }

    #[test]
    fn test_empty() {
        let mut sm = StateMachine::new();
        sm.pre_update();
        sm.update();
        sm.update_fixed(0.0);
        sm.post_update();
    }

    #[test]
    fn test_single_push() {
        let mut sm = StateMachine::new();
        let ts = TestState::new(
            1,
            0,
            2,
            0,
            3,
            0,
            0,
            0
        );
        sm.queue_push(Box::new(ts));
        assert_eq!(sm.current_state().is_none(), true);
        sm.pre_update();
        assert_eq!(sm.current_state().is_none(), false);
    }

    #[test]
    fn test_pop() {
        let mut sm = StateMachine::new();
        let ts = TestState::new(
            1,
            5,
            2,
            4,
            3,
            0,
            0,
            0
        );

        sm.queue_push(Box::new(ts));
        sm.pre_update();
        sm.queue_pop();
        sm.pre_update();
    }

    #[test]
    fn test_full_tick() {
        let mut sm = StateMachine::new();
        let ts = TestState::new(
            1,
            7,
            2,
            6,
            3,
            4,
            4,
            4
        );

        sm.queue_push(Box::new(ts));
        sm.pre_update();
        sm.update();
        sm.update_fixed(0.0);
        sm.post_update();
    }

    #[test]
    fn test_multi_push() {
        {
            let mut sm = StateMachine::new();
            let ts0 = TestState::new(
                1,
                0,
                2,
                4,
                3,
                0,
                0,
                0
            );
            let ts1 = TestState::new(
                1,
                0,
                2,
                0,
                3,
                0,
                0,
                0
            );

            sm.queue_push(Box::new(ts0));
            sm.pre_update();
            sm.queue_push(Box::new(ts1));
            sm.pre_update();
        }
        {
            let mut sm = StateMachine::new();
            let ts0 = TestState::new(
                1,
                0,
                2,
                3,
                0,
                0,
                0,
                0
            );
            let ts1 = TestState::new(
                1,
                0,
                2,
                0,
                3,
                4,
                4,
                4
            );

            sm.queue_push(Box::new(ts0));
            sm.queue_push(Box::new(ts1));
            sm.pre_update();
            sm.update();
            sm.update_fixed(0.0);
            sm.post_update();
        }
    }

}

