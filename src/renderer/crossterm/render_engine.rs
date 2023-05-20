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
use crate::renderer::crossterm::device_settings::DeviceSettings;
use crate::renderer::crossterm::colour;
use crossterm::style::Color;

struct Display {
    width: u16,
    height: u16
}

pub struct RenderEngine {
    display: Display,
    clear_colour: Color
}

impl RenderEngine {
    pub fn new(settings: DeviceSettings) -> RenderEngine {
        RenderEngine {
            display: Display {
                width: settings.display_length_x,
                height: settings.display_length_y
            },
            clear_colour: colour::map_to_limited_colours(&settings.clear_colour)
        }
    }

    /// Buffer a draw at the given coordinates
    pub fn draw_at(&self, x: u16, y: u16, character: char) {
        
    }
}
