use crate::world_generation::blocks::block_types::pillar::Pillar;
use crate::world_generation::blocks::block_types::BlockType;
use crate::world_generation::blocks::buildable::{Buildable, CanBuild, HasBuildVariants};

use crate::world_generation::blocks::face::{CullingFlag, Face};
use crate::world_generation::blocks::mesh_type::MeshType;
use crate::world_generation::blocks::texture_builder::TextureBuilder;
use crate::world_generation::blocks::DEFAULT_TEXTURE;

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
    /// defines what kind of meshes includes this block
    mesh_types: PillarFaces<MeshType>
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

    pub const fn with_mesh_type_all(mut self, mesh_type: MeshType) -> Self {
        self.mesh_types = PillarFaces::new_all(mesh_type);
        self
    }

    pub const fn with_mesh_types(
        mut self, 
        mesh_types: PillarFaces<MeshType>
    ) -> Self {
        self.mesh_types = mesh_types;
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
            mesh_types: PillarFaces::new_all(MeshType::Normal),
        }
    }
    fn get_texture_size() -> usize {1usize}
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
    type BuildTo = BlockType;

    fn build(&self, texture_builder: &mut TextureBuilder) -> BlockType {
        //TODO: extract uvs and vertices from model
        //TODO: extract normals, material, depth map, etc.
        BlockType::Pillar(Pillar::new(
            self.name, 
            [
                Face::new(
                    texture_builder.get_index_from_texture(
                        self.textures
                            .get_texture(PillarFace::Up)
                            .unwrap_or(DEFAULT_TEXTURE)
                    ),
                    self.culling_flags.up
                ),
                Face::new(
                    texture_builder.get_index_from_texture(
                        self.textures
                            .get_texture(PillarFace::Sides)
                            .unwrap_or(DEFAULT_TEXTURE)
                    ), 
                    self.culling_flags.sides
                ),
                Face::new(
                    texture_builder.get_index_from_texture(
                        self.textures
                            .get_texture(PillarFace::Down)
                            .unwrap_or(DEFAULT_TEXTURE)
                    ), 
                    self.culling_flags.down
                ),
            ],
            [
                self.mesh_types.up,
                self.mesh_types.sides,
                self.mesh_types.down
            ]
        ))
    }
}

pub struct PillarFaces<T> {
    pub up: T,
    pub sides: T,
    pub down: T
}

impl<T> PillarFaces<T> {
    const fn new_all(all: T) -> Self where T: Copy {
        Self {
            up: all,
            sides: all,
            down: all
        }
    }
}

pub enum PillarFace {
    Up, Sides, Down
}

trait GetTexture {
    type Output;

    fn get_texture(&self, face: PillarFace) -> Self::Output;
}

impl<T> GetTexture for Option<PillarFaces<T>> where T: Copy {
    type Output = Option<T>;

    fn get_texture(&self, face: PillarFace) -> Self::Output {
        match face {
            PillarFace::Up => self.as_ref().map(|faces| faces.up),
            PillarFace::Sides => self.as_ref().map(|faces| faces.sides),
            PillarFace::Down => self.as_ref().map(|faces| faces.down),
        }
    }
}