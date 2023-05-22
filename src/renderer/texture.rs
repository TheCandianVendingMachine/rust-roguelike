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
use std::path::{ PathBuf, Path };
use std::time::{ Instant, Duration };

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TextureMetaData {
    uuid: Uuid,
    name: &'static str,
    path: PathBuf
}

pub struct Texture {
    meta: TextureMetaData
}

pub struct TextureManager {
    all_textures: HashMap<Uuid, Texture>,
    texture_paths: HashMap<PathBuf, Uuid>,
    texture_names: HashMap<&'static str, Uuid>,
    texture_references: Arc<RwLock<TextureReferenceHandler>>,
}

impl TextureManager {
    /// Creates a new, empty, TextureManager
    pub fn new() -> TextureManager {
        TextureManager {
            all_textures: HashMap::new(),
            texture_paths: HashMap::new(),
            texture_names: HashMap::new(),
            texture_references: Arc::new(RwLock::new(TextureReferenceHandler::new())),
        }
    }

    /// Create a new handle to return to the user
    fn create_handle(&mut self, uuid: Uuid) -> Handle {
        self.texture_references.write().unwrap().create(uuid);
        Handle {
            uuid,
            manager: self.texture_references.clone()
        }
    }

    /// Create a brand new texture. Loads whatever data needed into memory as specified 
    /// by TextureMetaData
    fn create_texture(&mut self, meta_data: &TextureMetaData) -> Handle {
        let texture = Texture{
            meta: meta_data.clone()
        };

        self.texture_names.insert(meta_data.name, texture.meta.uuid);
        self.texture_paths.insert(meta_data.path.clone(), texture.meta.uuid);
        self.all_textures.insert(meta_data.uuid, texture);

        self.create_handle(meta_data.uuid)
    }

    /// Destroy a texture completely from memory. Drops all references, removes anything 
    /// in memory. After this function, we will need to load the texture into memory again
    fn destroy_texture(&mut self, texture: Uuid) {
        if !self.all_textures.contains_key(&texture) {
            panic!("Attempting to destroy texture ({}) when it doesn't exist!", texture);
        }
        let texture_obj = self.all_textures.get(&texture).unwrap();


        self.texture_names.remove(texture_obj.meta.name);
        self.texture_paths.remove(&texture_obj.meta.path);
        self.texture_references.write().unwrap().destroy(texture);
        self.all_textures.remove(&texture);
    }

    /// Gets a texture with the associated meta data. If the texture does not exist,
    /// we will create a new one
    pub fn get_or_create_texture(&mut self, texture: &TextureMetaData) -> Handle {
        self.get_texture(texture).unwrap_or_else(|| self.create_texture(texture))
    }

    /// Gets a texture with the associated meta data
    pub fn get_texture(&mut self, texture: &TextureMetaData) -> Option<Handle> {
        self.get_texture_by_uuid(texture.uuid)
    }

    /// Gets a handle to a texture by name
    pub fn get_texture_by_name<T: AsRef<str>>(&mut self, name: T) -> Option<Handle> {
        if !self.texture_names.contains_key(name.as_ref()) {
            None
        } else {
            self.get_texture_by_uuid(*self.texture_names.get(name.as_ref()).unwrap())
        }
    }

    /// Get a handle to a texture by filepath 
    pub fn get_texture_by_path<T: AsRef<Path>>(&mut self, path: T) -> Option<Handle> {
        if !self.texture_paths.contains_key(path.as_ref()) {
            None
        } else {
            self.get_texture_by_uuid(*self.texture_paths.get(path.as_ref()).unwrap())
        }
    }

    /// Get a handle to a texture by UUID
    pub fn get_texture_by_uuid(&mut self, texture: Uuid) -> Option<Handle> {
        if self.all_textures.contains_key(&texture) {
            Some(self.create_handle(texture))
        } else {
            None 
        }
    }

    /// Upkeeps internal state.
    /// - Remove all textures whose references are zero
    pub fn upkeep(&mut self) {
        // Purge zerod references
        let textures_to_purge = self.texture_references.
            read().unwrap().
            get_textures_for_destruction();

        for texture in textures_to_purge.iter() {
            if !self.all_textures.contains_key(texture) {
                panic!("Attempting to purge texture ({}) which does not exist in memory!", texture);
            }
            self.destroy_texture(*texture);
        }
    }
}

/// An internal handler for reference counting. Every texture handle holds a reference 
/// to this
struct TextureReferenceHandler {
    textures: HashMap<Uuid, u64>,
    textures_when_lost_reference: HashMap<Uuid, Instant>
}

impl TextureReferenceHandler {
    /// How much time has to have elapsed before a texture with zero references is marked 
    /// for destruction
    const TIME_WITHOUT_REFERENCE_UNTIL_DESTRUCTION: Duration = Duration::from_secs(60);

    /// Return a new reference counter
    fn new() -> TextureReferenceHandler {
        TextureReferenceHandler {
            textures: HashMap::new(),
            textures_when_lost_reference: HashMap::new()
        }
    }

    /// Create a new texture with the UUID. If the texture has already been created, 
    /// increment the reference counter
    fn create(&mut self, uuid: Uuid) {
        if self.textures.contains_key(&uuid) {
            let current_ref_count = self.textures.get(&uuid).unwrap();
            self.textures.insert(uuid, current_ref_count + 1);
        } else {
            self.textures.insert(uuid, 1);
        }
    }

    /// Drop a reference to the associated texture from the internal map 
    /// If the reference count drops to 0, start a timer so we can destroy the texture 
    /// when we elapse a certain amount
    fn destroy(&mut self, uuid: Uuid) {
        if !self.textures.contains_key(&uuid) {
            panic!("Handle of texture ({}) does not exist when attempting to drop!", uuid);
        }

        if *self.textures.get(&uuid).unwrap() == 0 {
            panic!("Attempting to drop handle of texture ({}) when there are no active references!", uuid);
        }

        *self.textures.get_mut(&uuid).unwrap() -= 1;
        if *self.textures.get(&uuid).unwrap() == 0 {
            self.textures_when_lost_reference.insert(uuid, Instant::now());
        }
    }

    /// Remove a texture UUID from the reference map
    /// Removes it completely, no records will be recorded
    fn remove(&mut self, uuid: Uuid) {
        if !self.textures.contains_key(&uuid) {
            panic!("Attempting to remove texture ({}) when it doesn't exist in the reference map!", uuid)
        }

        self.textures.remove(&uuid);
    }

    /// Return a vector of texture UUID's which are marked for destruction
    /// A texture is marked for destruction when:
    ///  - There are no references to it 
    ///  - There has been more than TIME_WITHOUT_REFERENCE_UNTIL_DESTRUCTION since no 
    ///    references were pointed at it
    fn get_textures_for_destruction(&self) -> Vec<Uuid> {
        let mut textures = Vec::new();
        for (texture, ref_count) in self.textures.iter() {
            if *ref_count == 0 {
                let time_since_texture_lost = self.textures_when_lost_reference.get(texture)
                    .unwrap_or(&Instant::now())
                    .elapsed();

                if time_since_texture_lost > TextureReferenceHandler::TIME_WITHOUT_REFERENCE_UNTIL_DESTRUCTION {
                    textures.push(texture.clone());
                }
            }
        }
        textures 
    }
}

/// A handle for a texture in memory. A texture is reference counted, and there is no 
/// way to retrieve a texture from a handle alone: You need a TextureManager instance 
/// to work with the texture
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


