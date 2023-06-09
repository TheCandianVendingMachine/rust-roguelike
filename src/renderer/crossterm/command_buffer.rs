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
use std::io::Write;
use crossterm::{ QueueableCommand, cursor };
use crossterm::terminal::{ Clear, ClearType };
use crossterm::style::{ self, Print, SetForegroundColor };
use crate::renderer::crossterm::command::{ self, Command, ClearInfo };

/// A meta-draw buffer which represents a consecutive set of draw buffers with colours
#[derive(Clone)]
struct ConsecutiveBuffer {
    end: usize,
    colour: style::Color,
    draw_commands: Vec<command::DrawInfo>
}

impl ConsecutiveBuffer {
    fn new() -> ConsecutiveBuffer {
        ConsecutiveBuffer {
            end: usize::MAX,
            colour: style::Color::White,
            draw_commands: Vec::new()
        }
    }

    fn execute<'a, T>(&self, stdout: &'a mut T) -> &'a mut T where T: Write {
        let mut stdout_queue = stdout;
        stdout_queue = stdout_queue.queue(SetForegroundColor(self.colour)).unwrap();
        for draw_command in self.draw_commands.iter() {
            stdout_queue = stdout_queue
                .queue(cursor::MoveTo(draw_command.draw_pos_x, draw_command.draw_pos_y)).unwrap()
                .queue(Print(draw_command.character)).unwrap()
        }
        stdout_queue
    }

    fn prepare(&mut self, width: u16) {
        // Sort by 1d position to ensure we are drawing consecutive characters
        self.draw_commands.sort_by_key(|k| k.draw_pos_x + k.draw_pos_y * width);
    }
}

/// A draw buffer is a set of draw commands ended by a single clear command 
/// We can order draw commands within a draw buffer to give us more control over when
/// certain characters are drawn, such as ordering by x or y position, colour, etc.
#[derive(Clone)]
struct DrawBuffer {
    draw_commands: Vec<Command>,
    consecutive_buffers: Vec<ConsecutiveBuffer>,
    clear_command: Option<ClearInfo>,
    ready: bool
}

impl DrawBuffer {
    fn new() -> DrawBuffer {
        DrawBuffer {
            draw_commands: Vec::new(),
            consecutive_buffers: Vec::new(),
            clear_command: None,
            ready: false 
        }
    }

    /// Prepare an internal buffer for drawing. Order the draw commands in the requested
    /// ordering
    fn prepare(&mut self) {
        // If a draw buffer is ready, we wont recompile it.
        // To compile a draw buffer, we sort every command such that x0<x1. Afterward,
        // we find consecutive colours and add that to a meta-array that will store the
        // colour and cursor position that represents it. We then will draw the characters 
        // until we end the buffer
        if !self.ready {
            if self.draw_commands.is_empty() {
                self.ready = true;
                return;
            }
            // Algorithm:
            // - Get all draw commands 
            // - Sort draw commands by 1 dimensional index
            // - Iterate through sorted commands 
            // - While the current colour = last seen colour, add to a consecutive buffer 
            // - If we see a new colour, push current consecutive buffer to queue and 
            //      allocate new one 

            // Get all draw commands
            let mut max_width = 0;
            let mut draw_commands: Vec<(&command::DrawInfo, usize)> = self.draw_commands
                .iter()
                .enumerate()
                .filter_map(|(idx, c)| match c {
                    Command::Draw(d) => {
                        max_width = u16::max(max_width, d.draw_pos_x);
                        Some((d, idx))
                    },
                    _ => None 
                })
                .collect();
                
            // Sort by 1d index
            draw_commands.sort_by_key(|c| c.0.draw_pos_x + c.0.draw_pos_y * max_width);

            // Iterate through sorted commands
            let mut last_colour_seen = draw_commands[0].0.colour;
            let mut working_consecutive_buffer = ConsecutiveBuffer::new();
            working_consecutive_buffer.colour = last_colour_seen;
            for (c, idx) in draw_commands.iter() {
                // If our colour hasn't been seen, push this consecutive buffer to the vec
                if c.colour != last_colour_seen {
                    // end the current consecutive buffer
                    working_consecutive_buffer.end = *idx;
                    self.consecutive_buffers.push(working_consecutive_buffer);
                    working_consecutive_buffer = ConsecutiveBuffer::new();

                    // initialise state
                    last_colour_seen = c.colour;
                    working_consecutive_buffer.colour = last_colour_seen;
                }

                working_consecutive_buffer.draw_commands.push(**c);
            }

            // if we have a draw command, we wouldnt have pushed this to our draw buffers.
            // so we push it now
            if let Some((_, idx)) = draw_commands.last() {
                working_consecutive_buffer.end = *idx;
                working_consecutive_buffer.prepare(max_width);
                self.consecutive_buffers.push(working_consecutive_buffer);
            }

            self.ready = true;
        }
    }

    fn execute_clear<'a, T>(&self, stdout: &'a mut T, command: &ClearInfo) -> &'a mut T where
        T: Write {
        match command {
            ClearInfo::All => stdout.queue(Clear(ClearType::All)).unwrap(),
            ClearInfo::AfterCursor => stdout.queue(Clear(ClearType::FromCursorDown)).unwrap(),
            ClearInfo::BeforeCursor => stdout.queue(Clear(ClearType::FromCursorUp)).unwrap(),
            ClearInfo::CurrentLine => stdout.queue(Clear(ClearType::CurrentLine)).unwrap(),
            ClearInfo::UntilNewLine => stdout.queue(Clear(ClearType::UntilNewLine)).unwrap(),
            ClearInfo::After(x, y) => {
                stdout
                    .queue(cursor::SavePosition).unwrap()
                    .queue(cursor::MoveTo(*x, *y)).unwrap()
                    .queue(Clear(ClearType::FromCursorDown)).unwrap()
                    .queue(cursor::RestorePosition).unwrap()
            },
            ClearInfo::Before(x, y) => {
                stdout
                    .queue(cursor::SavePosition).unwrap()
                    .queue(cursor::MoveTo(*x, *y)).unwrap()
                    .queue(Clear(ClearType::FromCursorUp)).unwrap()
                    .queue(cursor::RestorePosition).unwrap()
            },
            ClearInfo::Line(row) => {
                stdout
                    .queue(cursor::SavePosition).unwrap()
                    .queue(cursor::MoveToRow(*row)).unwrap()
                    .queue(Clear(ClearType::CurrentLine)).unwrap()
                    .queue(cursor::RestorePosition).unwrap()
            },
            ClearInfo::UntilNewLineFrom(x, y) => {
                stdout 
                    .queue(cursor::SavePosition).unwrap()
                    .queue(cursor::MoveTo(*x, *y)).unwrap()
                    .queue(Clear(ClearType::UntilNewLine)).unwrap()
                    .queue(cursor::RestorePosition).unwrap()
            }
        }
    }

    fn execute_draw<'a, T>(&self, stdout: &'a mut T, command: &command::DrawInfo) -> &'a mut T where 
        T: Write {
        stdout
            .queue(cursor::MoveTo(command.draw_pos_x, command.draw_pos_y)).unwrap()
            .queue(Print(command.character)).unwrap()
    }

    fn execute_cursor_move<'a, T>(&self, stdout: &'a mut T, command: (&u16, &u16)) -> &'a mut T where 
        T: Write {
        stdout.queue(cursor::MoveTo(*command.0, *command.1)).unwrap()
    }

    fn execute_flush<'a, T>(&self, stdout: &'a mut T) -> &'a mut T where
        T: Write {
        stdout.flush().unwrap();
        stdout
    }

    fn execute<T>(&self, stdout: &mut T) where T: Write {
        if !self.ready {
            panic!("Attempting to use draw buffer when it has not been prepared!");
        }
        let mut stdout_queued = stdout;

        if let Some(clear_info) = self.clear_command {
            stdout_queued = self.execute_clear(stdout_queued, &clear_info);
        }

        let working_consecutive_buffer = &mut self.consecutive_buffers.iter();
        for command in &self.draw_commands {
            stdout_queued = match command {
                Command::Draw(_) => {
                    if let Some(buffer) = working_consecutive_buffer.next() {
                        stdout_queued = buffer.execute(stdout_queued);
                    }
                    stdout_queued
                },
                Command::SetCursorPosition(x, y) => self.execute_cursor_move(stdout_queued, (x, y)),
                Command::Flush => self.execute_flush(stdout_queued),
                _ => stdout_queued
            }
        }
    }
}

/// A set of commands that will be executed at draw time to create images on the terminal
#[derive(Clone)]
pub struct CommandBuffer {
    draw_buffers: Vec<DrawBuffer>
}

impl CommandBuffer {
    pub fn new() -> CommandBuffer {
        CommandBuffer {
            draw_buffers: Vec::new()
        }
    }

    pub fn execute<T>(&self, stdout: &mut T) where T: Write {
        for draw_buffer in self.draw_buffers.iter() {
            draw_buffer.execute(stdout)
        }
    }
}

pub struct CommandBufferCreator<'a> {
    command_buffer: &'a mut CommandBuffer,
    draw_buffers: Vec<DrawBuffer>,
    working_draw_buffer: DrawBuffer,
    first_command: bool
}

impl<'a> CommandBuffer {
    pub fn add_commands(&'a mut self) -> CommandBufferCreator<'a> {
        CommandBufferCreator {
            command_buffer: self,
            draw_buffers: Vec::new(),
            working_draw_buffer: DrawBuffer::new(),
            first_command: true
        }
    }
}

impl<'a> CommandBufferCreator<'a> {
    fn handle_clear_command(&mut self, clear_info: command::ClearInfo) {
    }

    fn handle_draw_command(&mut self, draw_info: command::DrawInfo) {
    }

    fn handle_flush(&mut self) {
    }

    fn handle_cursor_move(&mut self, cursor_coordinates: (u16, u16)) {
    }

    /// Add a command to the command buffer for execution
    pub fn execute(mut self, command: Command) -> CommandBufferCreator<'a> {
        match command {
            Command::Clear(clear_info) => self.handle_clear_command(clear_info),
            Command::Draw(draw_info) => self.handle_draw_command(draw_info),
            Command::Flush => self.handle_flush(),
            Command::SetCursorPosition(x, y) => self.handle_cursor_move((x, y))
        }

        if let Command::Clear(clear_info) = command {
            if !self.first_command {
                // If not the first command in this buffer, push the buffer to the queue
                // and get ready to create a new one
                self.draw_buffers.push(self.working_draw_buffer.clone());
                self.working_draw_buffer = DrawBuffer::new();
            }
            // Push the clear command to the working draw buffer
            self.working_draw_buffer.clear_command = Some(clear_info);
        } else {
            self.working_draw_buffer.draw_commands.push(command);
        }
        self.first_command = false;

        self
    }

    /// Compile the command buffer for execution
    pub fn compile(mut self) {
        // Step 1) Push working draw buffer onto queue 
        if !self.first_command {
            self.draw_buffers.push(self.working_draw_buffer);
        }

        // Step 2) Clear all draw buffers before an "All" clear 
        let mut last_all_draw = 0;
        for (idx, draw_buffer) in self.draw_buffers.iter().enumerate() {
            if let Some(ClearInfo::All) = draw_buffer.clear_command {
                last_all_draw = idx;
            }
        }
        if last_all_draw > 0 {
            self.draw_buffers = self.draw_buffers.drain(0..last_all_draw).collect();
        }

        self.command_buffer.draw_buffers = self.draw_buffers;

        // Step 3) Compile remaining draw buffers
        for draw_buffer in self.command_buffer.draw_buffers.iter_mut() {
            draw_buffer.prepare();
        }

    }
}
