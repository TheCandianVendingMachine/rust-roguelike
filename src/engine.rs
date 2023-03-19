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

enum EngineEvent {
    Stop
}

struct EngineEventHandler {
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

    pub fn pop(&mut self) -> Option<EngineEvent> {
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
    running: bool
}

impl Engine {
    pub fn new(input_queue: mpsc::Receiver<Input>, render_fence: Option<FenceRC>) -> Engine {
        let engine_event_handler = EngineEventHandler::new();

        Engine {
            render_fence,
            input_queue,
            engine_event_handler,
            running: true
        }
    }

    fn handle_input(&mut self) {
        loop {
            let input = match self.input_queue.try_recv() {
                Ok(input) => input,
                _ => { break }
            };

            // Handle input
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

    }

    fn sync(&self) {
        if let Some(render_fence) = &self.render_fence {
            render_fence.0.close().pass();
        }
    }

    /// Simulate one tick of the game state
    pub fn tick(&mut self) {
        while self.running {
            self.handle_input();
            self.update_state();
            self.sync();
            self.handle_engine_events();
        }
    }
}
