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
use crossterm::style;

#[derive(Clone, Copy, Debug)]
pub struct DrawInfo {
    pub colour: style::Color,
    pub draw_pos_x: u16,
    pub draw_pos_y: u16,
    pub character: char
}

/// What kind of clearing we want to do on a line. Can define from current cursor, or from 
/// a given position
#[derive(Clone, Copy, Debug)]
pub enum ClearInfo {
    /// Clear the entire visible screen. All draw commands before this are purged
    All,
    /// Clear everything past the cursor position. All draw commands after the cursor
    /// position are purged
    AfterCursor,
    /// Clear everything before the cursor position. All draw commands before the cursor 
    /// position are purged
    BeforeCursor,
    /// Clear everything on the current line. All draw commands on the current line are
    /// purged
    CurrentLine,
    /// Clear everything until a new line
    UntilNewLine,
    /// Clear everything after the given screen coordinate. All draw commands after 
    /// this position are purged
    After(u16, u16),
    /// Clear everything before the given screen coordinate. All draw commands before this 
    /// position are purged
    Before(u16, u16),
    /// Clear everything on the line given by the argument. All draw commands on this line 
    /// are purged
    Line(u16),
    /// Clear everything from the screen coordinate until a new line
    UntilNewLineFrom(u16, u16)
}

/// A single command to modify contents of the screen buffer
#[derive(Clone, Copy, Debug)]
pub enum Command {
    /// Clear the terminal
    Clear(ClearInfo),
    /// Draw to the terminal
    Draw(DrawInfo),
    /// Set the cursor position to the given screen coordinates
    SetCursorPosition(u16, u16),
    /// Flush the current buffer to the screen 
    Flush
}

