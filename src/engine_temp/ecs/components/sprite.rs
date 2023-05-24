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
use crate::engine_temp::ecs::components::component::Component;
use crate::renderer::texture::Handle;
use uuid::Uuid;
use serde::Serialize;

/// Defines a component which allows this sprite to be rendered
#[derive(Serialize)]
pub struct SpriteComponent {
    uuid: Uuid,
    pub texture: Option<Handle>
}

impl SpriteComponent {
    
}

impl Component for SpriteComponent {
    const NAME: &'static str = "Sprite";
    fn new() -> SpriteComponent {
        SpriteComponent {
            uuid: Uuid::new_v4(),
            texture: None
        }
    }

    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
}
