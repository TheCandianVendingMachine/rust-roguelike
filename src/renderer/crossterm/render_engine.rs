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
use crate::renderer::crossterm::{
    colour,
    device_settings::DeviceSettings,
    swapchain::Swapchain,
    display_engine::DisplayEngine
};
use crossterm::style::Color;

struct Display {
    engine: DisplayEngine,
    width: u16,
    height: u16
}

pub struct RenderEngine {
    swapchain: Swapchain,
    display: Display,
    clear_colour: Color
}

impl RenderEngine {
    pub fn new(settings: DeviceSettings) -> RenderEngine {
        let swapchain = Swapchain::new(settings.swapchain_count);
        RenderEngine {
            display: Display {
                engine: DisplayEngine::new(swapchain.framebuffers.clone(), swapchain.swapped.clone()),
                width: settings.display_length_x,
                height: settings.display_length_y
            },
            swapchain,
            clear_colour: colour::map_to_limited_colours(&settings.clear_colour)
        }
    }

    pub fn init(&mut self) {
        self.display.engine.run();
    }
}
