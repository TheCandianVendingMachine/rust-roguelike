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

    pub fn create(&mut self, entity: &Entity) {
        self.entity_component_set.push(entity.index, T::new());
    }

    pub fn remove(&mut self, entity: &Entity) {
        self.entity_component_set.remove(entity.index);
    }
}

