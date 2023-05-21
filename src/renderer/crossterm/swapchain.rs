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
use std::io::{ Write, BufWriter, Stdout, stdout };
use std::sync::{ Mutex, Arc, Condvar };
use crate::renderer::crossterm::command_buffer::CommandBuffer;

pub struct Framebuffer(BufWriter<Stdout>);
impl Framebuffer {
    fn new() -> Framebuffer {
        Framebuffer(BufWriter::new(stdout()))
    }

    pub fn flush(&mut self) {
        self.0.flush().unwrap()
    }
}

pub struct Swapchain {
    pub framebuffers: Vec<Arc<Mutex<Framebuffer>>>,
    pub swapped: Arc<Condvar>,
    pub working_framebuffer: Arc<Mutex<usize>>,
}

impl Swapchain {
    pub fn new(framebuffer_count: usize) -> Swapchain {
        let mut buffers = Vec::new();
        buffers.resize_with(framebuffer_count, || Arc::new(Mutex::new(Framebuffer::new())));
        Swapchain {
            framebuffers: buffers,
            working_framebuffer: Arc::new(Mutex::new(0)),
            swapped: Arc::new(Condvar::new())
        }
    }

    pub fn queue_commands(&mut self, commands: &CommandBuffer) {
        let working_framebuffer = self.working_framebuffer.lock().unwrap();
        let mut framebuffer = self.framebuffers[*working_framebuffer].lock().unwrap();
        commands.execute(&mut framebuffer.0);
    }

    pub fn swap(&mut self) {
        let mut working_framebuffer = self.working_framebuffer.lock().unwrap();
        self.swapped.notify_all();
        *working_framebuffer = (*working_framebuffer + 1) % self.framebuffers.len();
    }
}
