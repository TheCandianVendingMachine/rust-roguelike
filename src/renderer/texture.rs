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
use serde::ser::{ Serialize, Serializer, SerializeStruct };
use uuid::Uuid;
use std::sync::{ Arc, RwLock };
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TextureMetaData {
    uuid: Uuid,
    name: &'static str,
    filepath: PathBuf
}

pub struct Texture {
    meta: TextureMetaData
}

pub struct TextureManager {
    all_textures: HashMap<Uuid, Texture>,
    texture_names: HashMap<&'static str, Uuid>,
    texture_references: Arc<RwLock<TextureReferenceHandler>>,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {
            all_textures: HashMap::new(),
            texture_references: Arc::new(RwLock::new(TextureReferenceHandler::new())),
            texture_names: HashMap::new()
        }
    }

    fn create_handle(&mut self, uuid: Uuid) -> Handle {
        self.texture_references.write().unwrap().create(uuid);
        Handle {
            uuid,
            manager: self.texture_references.clone()
        }
    }

    fn create_texture(&mut self, meta_data: &TextureMetaData) -> Handle {
        let texture = Texture{
            meta: meta_data.clone()
        };

        self.texture_names.insert(meta_data.name, texture.meta.uuid);
        self.all_textures.insert(meta_data.uuid, texture);

        self.create_handle(meta_data.uuid)
    }

    fn destroy_texture(&mut self, texture: Uuid) {
        self.all_textures.remove(&texture);
        self.texture_references.write().unwrap().destroy(texture);
    }

    pub fn get_or_create_texture(&mut self, texture: &TextureMetaData) -> Handle {
        self.get_texture(texture).unwrap_or_else(|| self.create_texture(texture))
    }

    pub fn get_texture(&mut self, texture: &TextureMetaData) -> Option<Handle> {
        if self.all_textures.contains_key(&texture.uuid) {
            Some(self.create_handle(texture.uuid))
        } else {
            None
        }
    }

    /// Upkeeps internal state.
    /// - Remove all textures whose references are zero
    pub fn upkeep(&mut self) {
        // Purge zerod references
        let mut textures_to_purge = Vec::new();
        for (texture, ref_count) in self.texture_references.read().unwrap().textures.iter() {
            if *ref_count == 0 {
                textures_to_purge.push(*texture);
            }
        }

        for texture in textures_to_purge.iter() {
            if !self.all_textures.contains_key(texture) {
                panic!("Attempting to purge texture ({}) which does not exist in memory!", texture);
            }
            self.destroy_texture(*texture);
        }
    }
}

struct TextureReferenceHandler {
    textures: HashMap<Uuid, u64>
}

impl TextureReferenceHandler {
    fn new() -> TextureReferenceHandler {
        TextureReferenceHandler {
            textures: HashMap::new()
        }
    }

    fn create(&mut self, uuid: Uuid) {
        if self.textures.contains_key(&uuid) {
            let current_ref_count = self.textures.get(&uuid).unwrap();
            self.textures.insert(uuid, current_ref_count + 1);
        } else {
            self.textures.insert(uuid, 1);
        }
    }

    fn destroy(&mut self, uuid: Uuid) {
        if !self.textures.contains_key(&uuid) {
            panic!("Handle of texture ({}) does not exist when attempting to drop!", uuid);
        }

        if *self.textures.get(&uuid).unwrap() == 0 {
            panic!("Attempting to drop handle of texture ({}) when there are no active references!", uuid);
        }

        *self.textures.get_mut(&uuid).unwrap() -= 1;
    }

    fn remove(&mut self, uuid: Uuid) {
        if !self.textures.contains_key(&uuid) {
            panic!("Attempting to remove texture ({}) when it doesn't exist in the reference map!", uuid)
        }

        self.textures.remove(&uuid);
    }
}

/// A handle for a texture in memory. Reference counted
pub struct Handle {
    uuid: Uuid,
    manager: Arc<RwLock<TextureReferenceHandler>>
}

impl Drop for Handle {
    fn drop(&mut self) {
        let mut manager_lock = self.manager.write().unwrap();
        manager_lock.destroy(self.uuid);
    }
}

impl Serialize for Handle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where 
        S: Serializer {
        let mut state = serializer.serialize_struct("TextureHandle", 1)?;
        state.serialize_field("texture", &self.uuid)?;
        state.end()
    }
}


