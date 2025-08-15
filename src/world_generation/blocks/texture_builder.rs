use std::collections::HashMap;

use crate::world_generation::blocks::DEFAULT_TEXTURE;

pub type TexturePath = &'static str;
pub type TextureIndex = usize;

pub struct TextureBuilder {
    textures: HashMap<TexturePath, TextureIndex>
}

impl TextureBuilder {
    pub fn new() -> Self {
        let mut textures = HashMap::new();
        textures.insert(DEFAULT_TEXTURE, 0);
        
        Self { textures }
    }

    pub fn get_index_from_texture(
        &mut self, 
        path: TexturePath
    ) -> TextureIndex {
        if let Some(index) = self.textures.get(&path) {
            return *index;
        }
        let index = self.textures.len();
        self.textures.insert(path, index);
        return index;
    }
}