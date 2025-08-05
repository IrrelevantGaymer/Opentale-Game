use crate::world_generation::blocks::block_types::basic::Basic;
use crate::world_generation::blocks::block_types::BlockType;
use crate::world_generation::blocks::buildable::{Buildable, CanBuild, HasBuildVariants};

use crate::world_generation::blocks::face::{CullingFlag, Face};
use crate::world_generation::blocks::mesh_type::MeshType;

/// A Block where all faces use the same texture and model
pub struct BasicBuilder {
    /// The block's technical name
    name: &'static str,
    /// If no texture is defined, the default texture (a debug texture) will be used.
    texture: Option<&'static str>,
    // /// If no model is defined, the default model (a normal cube face) will be used.
    // model: Option<&'static str>,
    culling_flag: CullingFlag,
    /// a number that represents this block in memory
    id: usize,
    /// a value that represents the index 
    /// of this block's corresponding texture and material
    index: usize,
    /// defines what kind of meshes includes this block
    mesh_type: MeshType
}

impl BasicBuilder {
    pub const fn with_texture(mut self, texture: &'static str) -> Self {
        self.texture = Some(texture);
        self
    } 

    // pub const fn with_model(mut self, model: &'static str) -> Self {
    //     self.model = Some(model);
    //     self
    // } 

    pub const fn with_mesh_type(mut self, mesh_type: MeshType) -> Self {
        self.mesh_type = mesh_type;
        self
    }

    pub const fn with_culling_flag(
        mut self, 
        culling_flag: CullingFlag
    ) -> Self {
        self.culling_flag = culling_flag;
        self
    }
}

impl const Buildable for BasicBuilder {
    fn new_with_name(name: &'static str) -> Self {
        BasicBuilder {
            name,
            texture: None,
            //model: None,
            culling_flag: CullingFlag::Both,
            id: 0,
            index: 0,
            mesh_type: MeshType::Normal,
        }
    }
    fn get_texture_size() -> usize {1usize}
    fn with_index(mut self, idx: usize) -> Self {
        self.index = idx;
        self
    }
    fn set_index(&mut self, idx: usize) {
        self.index = idx;
    }
    fn with_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl HasBuildVariants for BasicBuilder {
    type Variants = ();
}

impl CanBuild for BasicBuilder {
    fn build(self) -> BlockType {
        //TODO: extract uvs and vertices from model
        //TODO: extract normals, material, depth map, etc.
        BlockType::Basic(Basic::new(
            self.name, 
            Face::new(
                self.index, 
                self.culling_flag
            ), 
            self.mesh_type
        ))
    }
}