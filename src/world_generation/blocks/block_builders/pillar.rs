use crate::world_generation::blocks::block_types::pillar::Pillar;
use crate::world_generation::blocks::block_types::BlockType;
use crate::world_generation::blocks::buildable::{Buildable, CanBuild, HasBuildVariants};

use crate::world_generation::blocks::face::{CullingFlag, Face};
use crate::world_generation::blocks::mesh_type::MeshType;

/// A Block where all faces use the same texture and model
pub struct PillarBuilder {
    /// The block's technical name
    name: &'static str,
    /// If no texture is defined, the default texture (a debug texture) will be used.
    textures: Option<PillarFaces<&'static str>>,
    // /// If no model is defined, the default model (a normal cube face) will be used.
    // model: Option<&'static str>,
    culling_flags: PillarFaces<CullingFlag>,
    /// a number that represents this block in memory
    id: usize,
    /// a value that represents the index 
    /// of this block's corresponding texture and material
    index: usize,
    /// defines what kind of meshes includes this block
    mesh_type: MeshType
}

impl PillarBuilder {
    pub const fn with_textures(mut self, textures: PillarFaces<&'static str>) -> Self {
        self.textures = Some(textures);
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

    pub const fn with_culling_flags(
        mut self, 
        culling_flags: PillarFaces<CullingFlag>
    ) -> Self {
        self.culling_flags = culling_flags;
        self
    }
}

impl const Buildable for PillarBuilder {
    fn new_with_name(name: &'static str) -> Self {
        PillarBuilder {
            name,
            textures: None,
            //model: None,
            culling_flags: PillarFaces {
                up: CullingFlag::Both,
                sides: CullingFlag::Both,
                down: CullingFlag::Both
            },
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

impl HasBuildVariants for PillarBuilder {
    type Variants = ();
}

impl CanBuild for PillarBuilder {
    fn build(self) -> BlockType {
        //TODO: extract uvs and vertices from model
        //TODO: extract normals, material, depth map, etc.
        BlockType::Pillar(Pillar::new(
            self.name, 
            [
                Face::new(self.index + 0, self.culling_flags.up),
                Face::new(self.index + 1, self.culling_flags.sides),
                Face::new(self.index + 2, self.culling_flags.down),
            ],
            self.mesh_type
        ))
    }
}

pub struct PillarFaces<T> {
    up: T,
    sides: T,
    down: T
}