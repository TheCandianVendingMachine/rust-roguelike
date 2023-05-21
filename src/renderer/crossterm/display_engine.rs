/*
    a roguelike game created for a fun exercise
    copyright (c) 2023  bailey danyluk

    this program is free software: you can redistribute it and/or modify
    it under the terms of the gnu general public license as published by
    the free software foundation, either version 3 of the license, or
    (at your option) any later version.

    this program is distributed in the hope that it will be useful,
    but without any warranty; without even the implied warranty of
    merchantability or fitness for a particular purpose.  see the
    gnu general public license for more details.

    you should have received a copy of the gnu general public license
    along with this program.  if not, see <https://www.gnu.org/licenses/>.
*/
use crate::renderer::crossterm::swapchain::Framebuffer;
use std::sync::{
    Arc, Mutex, Condvar,
    atomic::{ AtomicBool, Ordering }
};
use std::thread;

pub struct DisplayEngine {
    framebuffers: Vec<Arc<Mutex<Framebuffer>>>,
    ready: Arc<Condvar>,
    render_thread: Option<thread::JoinHandle<()>>,
    running: Arc<AtomicBool>
}

impl DisplayEngine {
    pub fn new(framebuffers: Vec<Arc<Mutex<Framebuffer>>>, ready: Arc<Condvar>) -> DisplayEngine {
        DisplayEngine {
            framebuffers: framebuffers.clone(),
            ready,
            render_thread: None,
            running: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn run(&mut self) {
        let running = self.running.clone();
        let ready = self.ready.clone();
        let framebuffers = self.framebuffers.clone();

        self.render_thread = Some(thread::spawn(move || {
            running.store(true, Ordering::SeqCst);
            while running.load(Ordering::SeqCst) {
                // we wait until we are signalled to render. Until then we can't assume there 
                // is data in the framebuffer
                let working_index = Mutex::new(0);
                let mut locked_index = working_index.lock().unwrap();
                locked_index = ready.wait(locked_index).unwrap();
                framebuffers[*locked_index].lock().unwrap().flush()
            }
        }));
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }
}
