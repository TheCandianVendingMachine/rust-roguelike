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
use crate::engine_temp::game::state::State;
use crate::engine_temp::ecs::world::{ World, WorldComponents };
use crate::engine_temp::ecs::entity::Entity;

pub struct Game {
    world: World
}

impl Game {
    pub fn new() -> Game {
        let mut world = World::new();
        world.create_prefab("test", Box::new(|world: &mut WorldComponents, entity: &Entity| {
            world.transforms.create(entity);
        }));

        Game{
            world
        }
    }
}

impl State for Game {
    fn on_push(&mut self) {
        println!("push!");
        self.world.create_entity_from_prefab("test");
        println!("{:?}", self.world);
    }
}

