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
use std::rc::Rc;

use crate::game::state_machine::StateMachine;
use std::time::{ Instant, Duration };

struct GameHandler {
    state_machine: StateMachine,
    // Timings to update the game state at a fixed rate
    last_update: Instant,
    accumulator: Duration,
    simulation_rate: Duration, 
    runtime: Duration
}

impl GameHandler {
    pub fn new() -> GameHandler {
        GameHandler {
            state_machine: StateMachine::new(),
            last_update: Instant::now(),
            accumulator: Duration::new(0, 0),
            simulation_rate: Duration::from_secs_f64(1.0 / 60.0),
            runtime: Duration::new(0, 0)
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta_time = now - self.last_update;
        self.last_update = now;

        self.accumulator += delta_time;
        self.runtime += delta_time;

        self.state_machine.pre_update();
        self.state_machine.update();

        while self.accumulator >= self.simulation_rate {
            self.accumulator -= self.simulation_rate;
            self.state_machine.update_fixed(self.simulation_rate.as_secs_f64());
        }

        self.state_machine.post_update();
    }
}

/// Events that the manages the engine itself
pub enum EngineEvent {
    Stop,
}

/// A wrapper around a mpsc Reciever and Sender so we don't pollute the engine namespace
pub struct EngineEventHandler {
    event_queue: mpsc::Receiver<EngineEvent>,
    sender_base: Rc<mpsc::Sender<EngineEvent>>
}

impl EngineEventHandler {
    pub fn new() -> EngineEventHandler {
        let (send, recv) = mpsc::channel();
        EngineEventHandler {
            event_queue: recv,
            sender_base: Rc::new(send)
        }
    }

    pub fn get_publisher(&mut self) -> Rc<mpsc::Sender<EngineEvent>> {
        self.sender_base.clone()
    }

    fn pop(&mut self) -> Option<EngineEvent> {
        match self.event_queue.try_recv() {
            Ok(event) => Some(event),
            Err(_) => None
        }
    }
}

/// The way the game is initialised and ran.
///
/// The engine will setup a render fence and input queue, and will update the game state
/// as needed. If initialised with a render fence, it will synchronise the game state
/// with the renderer as needed
pub struct Engine {
    render_fence: Option<FenceRC>,
    input_queue: mpsc::Receiver<Input>,
    engine_event_handler: EngineEventHandler,
    running: bool,
    game_handler: GameHandler
}

impl Engine {
    pub fn new(input_queue: mpsc::Receiver<Input>, render_fence: Option<FenceRC>) -> Engine {
        let engine_event_handler = EngineEventHandler::new();

        Engine {
            render_fence,
            input_queue,
            engine_event_handler,
            running: true,
            game_handler: GameHandler::new()
        }
    }

    fn handle_input(&mut self) {
        loop {
            let input = match self.input_queue.try_recv() {
                Ok(input) => input,
                _ => { break }
            };

            // Handle input
            match input {
                Input::CloseGame => self.running = false,
            }
        }
    }

    fn handle_engine_events(&mut self) {
        while let Some(event) = self.engine_event_handler.pop() {
            match event {
                EngineEvent::Stop => self.running = false,
            }
        }
    }

    fn update_state(&mut self) {
        self.game_handler.tick();
    }

    fn sync(&self) {
        if let Some(render_fence) = &self.render_fence {
            render_fence.0.close().pass();
        }
    }

    /// Start and run the engine until program halts
    pub fn run(&mut self) {
        while self.running {
            self.handle_input();
            self.update_state();
            self.handle_engine_events();
            self.sync();
        }
    }
}
