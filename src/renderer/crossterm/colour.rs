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
use crossterm::style::Color;
use crate::renderer::colours::Colour;

/// Map from RGB8 to a limited colour set of 16 colours
pub fn map_to_limited_colours(original: &Colour) -> Color {
    struct ColourMapping {
        generic: Colour,
        mapped: Color 
    }

    let potential_colours = [
        ColourMapping { generic: Colour::black(), mapped: Color::Black },
        ColourMapping { generic: Colour::dark_red(), mapped: Color::DarkRed },
        ColourMapping { generic: Colour::dark_green(), mapped: Color::DarkGreen },
        ColourMapping { generic: Colour::dark_yellow(), mapped: Color::DarkYellow },
        ColourMapping { generic: Colour::dark_blue(), mapped: Color::DarkBlue },
        ColourMapping { generic: Colour::dark_purple(), mapped: Color::DarkMagenta },
        ColourMapping { generic: Colour::dark_cyan(), mapped: Color::DarkCyan },
        ColourMapping { generic: Colour::gray(), mapped: Color::Grey },
        ColourMapping { generic: Colour::dark_gray(), mapped: Color::DarkGrey },
        ColourMapping { generic: Colour::red(), mapped: Color::Red },
        ColourMapping { generic: Colour::green(), mapped: Color::Green },
        ColourMapping { generic: Colour::yellow(), mapped: Color::Yellow },
        ColourMapping { generic: Colour::blue(), mapped: Color::Blue },
        ColourMapping { generic: Colour::purple(), mapped: Color::Magenta },
        ColourMapping { generic: Colour::cyan(), mapped: Color::Cyan },
        ColourMapping { generic: Colour::white(), mapped: Color::White },

    ];

    let mut smallest_distance = std::u32::MAX;
    let mut closest_colour: Color = potential_colours[0].mapped;
    for colour in potential_colours {
        let distance: u32;
        let r_distance_1 = i32::unsigned_abs(original.r as i32 - colour.generic.r as i32);
        let g_distance_1 = i32::unsigned_abs(original.g as i32 - colour.generic.g as i32);
        let b_distance_1 = i32::unsigned_abs(original.b as i32 - colour.generic.b as i32);

        distance = r_distance_1 * r_distance_1 + g_distance_1 * g_distance_1 + b_distance_1 * b_distance_1;

        if distance < smallest_distance {
            smallest_distance = distance;
            closest_colour = colour.mapped;
        }
    }

    closest_colour
}

pub fn map_to_crossterm_rgb(original: &Colour) -> Color {
    Color::Rgb { r: original.r, g: original.g, b: original.b }
}

