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
use crate::engine_temp::ecs::world::{ World, WorldComponents };
use crate::engine_temp::ecs::components::{
    sprite::SpriteComponent, transform::TransformComponent
};
use crate::engine_temp::containers::bit_set::BitSet;
use crate::engine_temp::fence::FenceRC;

/// Operates on all entities which can be rendered.
/// Renders sprites to the screen and transforms as needed
pub struct RenderSystem {
    sync_fence: FenceRC
}

impl RenderSystem {
    pub fn new(fence: FenceRC) -> RenderSystem {
        RenderSystem {
            sync_fence: fence
        }
    }

    pub fn update(&self, components: &WorldComponents, world: &World) {
        // Step 1) Fetch all entities which have desired components
        // We do this by doing a bitset comparison.
        // Bit index = entity 
        // 1 -> Has desired component 
        // 0 -> Does not have desired component
        let mut transform_entities = BitSet::new(world.entity_count);
        for entity in components.transforms.entities_with_components().iter() {
            transform_entities.set(*entity);
        }

        let mut sprite_entities = BitSet::new(world.entity_count);
        for entity in components.sprites.entities_with_components().iter() {
            sprite_entities.set(*entity);
        }

        let entities_to_work_on = transform_entities & sprite_entities;
        for entity in entities_to_work_on.get_set_indices() {
            // Operate on these entities 
        }
    }
}
