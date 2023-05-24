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
use crate::engine_temp::containers::sparse_set::SparseSet;
use crate::engine_temp::ecs::components::component::Component; 
use crate::engine_temp::ecs::entity::Entity;
use serde::Serialize;
use std::fmt;

pub struct ComponentManager<T> where 
    T: Component {
    entity_component_set: SparseSet<T>
}

impl<T> ComponentManager<T> where 
    T: Component {
    pub fn new(count: usize) -> ComponentManager<T> {
        ComponentManager{
            entity_component_set: SparseSet::new(count),
        }
    }

    pub fn create(&mut self, entity: &Entity) -> &mut T {
        self.entity_component_set.push(entity.index, T::new())
    }

    pub fn remove(&mut self, entity: &Entity) {
        self.entity_component_set.remove(entity.index);
    }

    pub fn get(&self, entity: &Entity) -> Option<&T> {
        self.entity_component_set.get(entity.index)
    }

    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.entity_component_set.get_mut(entity.index)
    }

    pub fn entities_with_components(&self) -> Vec<usize> {
        self.entity_component_set.get_all_elements()
    }
}

impl<T> fmt::Debug for ComponentManager<T> where 
    T: Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("ComponentManager<{}>", T::NAME).as_str())
            .field("component count", &self.entity_component_set.get_all_elements().len())
        .finish_non_exhaustive()
    }
}
