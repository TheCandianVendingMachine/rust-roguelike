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

pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Colour {
    pub fn rgb(r: u8, g: u8, b: u8) -> Colour {
        Colour { r, g, b, a: 0xFF }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Colour {
        Colour { r, g, b, a }
    }

    pub fn hex(hex: &str) -> Colour {
        if hex.len() < 6 {
            return Colour::black()
        }

        if hex.chars().find(|c| -> bool { c.is_ascii_hexdigit() }).is_some() {
            return Colour::black()
        }

        let r_slice = &hex[0..1];
        let g_slice = &hex[2..3];
        let b_slice = &hex[4..5];

        Colour {
            r: u8::from_str_radix(r_slice, 16).unwrap() as u8,
            g: u8::from_str_radix(g_slice, 16).unwrap() as u8,
            b: u8::from_str_radix(b_slice, 16).unwrap() as u8,
            a: 0xFF
        }
    }

    pub fn hex_alpha(hex: &str) -> Colour {
        if hex.len() < 8 {
            return Colour::black()
        }

        if hex.chars().find(|c| -> bool { c.is_ascii_hexdigit() }).is_some() {
            return Colour::black()
        }

        let r_slice = &hex[0..1];
        let g_slice = &hex[2..3];
        let b_slice = &hex[4..5];
        let a_slice = &hex[6..7];

        Colour {
            r: u8::from_str_radix(r_slice, 16).unwrap() as u8,
            g: u8::from_str_radix(g_slice, 16).unwrap() as u8,
            b: u8::from_str_radix(b_slice, 16).unwrap() as u8,
            a: u8::from_str_radix(a_slice, 16).unwrap() as u8
        }
    }

    pub fn black() -> Colour        { Colour::rgb(0x2e, 0x34, 0x36) }
    pub fn dark_red() -> Colour     { Colour::rgb(0xcc, 0x00, 0x00) }
    pub fn dark_green() -> Colour   { Colour::rgb(0x4e, 0x9a, 0x06) }
    pub fn dark_yellow() -> Colour  { Colour::rgb(0xc4, 0xa0, 0x00) }
    pub fn dark_blue() -> Colour    { Colour::rgb(0x34, 0x65, 0xa4) }
    pub fn dark_purple() -> Colour  { Colour::rgb(0x75, 0x50, 0x7b) }
    pub fn dark_cyan() -> Colour    { Colour::rgb(0x06, 0x98, 0x9a) }
    pub fn gray() -> Colour         { Colour::rgb(0xd3, 0xd7, 0xcf) }
    pub fn dark_gray() -> Colour    { Colour::rgb(0x55, 0x57, 0x53) }
    pub fn red() -> Colour          { Colour::rgb(0xef, 0x29, 0x29) }
    pub fn green() -> Colour        { Colour::rgb(0x8a, 0xe2, 0x34) }
    pub fn yellow() -> Colour       { Colour::rgb(0xfc, 0xe9, 0x4f) }
    pub fn blue() -> Colour         { Colour::rgb(0x72, 0x9f, 0xcf) }
    pub fn purple() -> Colour       { Colour::rgb(0xad, 0x7f, 0xa8) }
    pub fn cyan() -> Colour         { Colour::rgb(0x34, 0xe2, 0xe2) }
    pub fn white() -> Colour        { Colour::rgb(0xee, 0xee, 0xec) }
}

