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

use crate::ecs::entity::Entity;
use std::collections::HashMap;
use crate::ecs::component_manager::ComponentManager;
use crate::ecs::components::{
    transform::TransformComponent,
};

pub struct WorldComponents {
    pub transforms: ComponentManager<TransformComponent>
}

impl WorldComponents {
    pub fn new() -> WorldComponents {
        WorldComponents {
            transforms: ComponentManager::new(128)
        }
    }
}

pub struct World {
    components: WorldComponents,
    entity_count: usize,
    entity_prefabs: HashMap<String, Box<dyn FnMut(&mut WorldComponents, &Entity)>>
}

impl World {
    pub fn new() -> World {
        World {
            components: WorldComponents::new(),
            entity_count: 0,
            entity_prefabs: HashMap::new()
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_count += 1;
        Entity::new(self.entity_count - 1)
    }

    pub fn create_entity_from_prefab<S>(&mut self, prefab: S) -> Option<Entity> where 
        S: Into<String> {
        let s = &prefab.into();
        if !self.entity_prefabs.contains_key(s) {
            return None
        }
        let e = self.create_entity();
        let prefab_closure = self.entity_prefabs.get_mut(s).unwrap();
        prefab_closure(&mut self.components, &e);
        Some(e)
    }

    pub fn create_prefab<S>(&mut self, prefab: S, on_create: Box<dyn FnMut(&mut WorldComponents, &Entity)>) where 
        S: Into<String> {
        self.entity_prefabs.insert(prefab.into(), on_create);
    }
}

pub fn process_unfixed_tick(world: &mut World) {

}

pub fn process_fixed_tick(world: &mut World, delta_time: f64) {

}

