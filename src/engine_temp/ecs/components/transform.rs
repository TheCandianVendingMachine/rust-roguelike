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

use crate::engine_temp::math::vector2::Vector2i;
use crate::engine_temp::ecs::components::component::Component;
use uuid::Uuid;
use serde::Serialize;

/// Defines a position in a 2d grid for a given entity
#[derive(Serialize)]
pub struct TransformComponent {
    uuid: Uuid,
    pub position: Vector2i
}

impl TransformComponent {
    
}

impl Component for TransformComponent {
    const NAME: &'static str = "Transform";
    fn new() -> TransformComponent {
        TransformComponent {
            uuid: Uuid::new_v4(),
            position: Vector2i{ x: 0, y: 0 }
        }
    }

    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
}
