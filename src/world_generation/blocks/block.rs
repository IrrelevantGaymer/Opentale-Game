use crate::world_generation::blocks::{
    block_builders::{basic::BasicBuilder, pillar::PillarBuilder}, 
    buildable::Buildable, 
    face::{FaceDir, Index, Uv, Vertex}, 
    mesh_type::MeshType};


pub struct Block;

impl Block {
    pub const fn new_basic(name: &'static str) -> BasicBuilder {
        BasicBuilder::new_with_name(name)
    }

    pub const fn new_pillar(name: &'static str) -> PillarBuilder {
        PillarBuilder::new_with_name(name)
    }
}

pub trait IsBlock: Send + Sync {
    fn name(&self) -> &'static str;
    fn get_uvs(&self, dir: FaceDir) -> &[Uv];
    fn get_vertices(&self, dir: FaceDir) -> Box<[Vertex]>;
    fn num_vertices(&self, dir: FaceDir) -> usize;
    fn get_indices(&self, dir: FaceDir, offset: usize) -> Box<[Index]>;
    fn mesh_type(&self) -> MeshType;
}