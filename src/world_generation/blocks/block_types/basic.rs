use crate::world_generation::blocks::{block::IsBlock, face::{Face, FaceDir, Index, Uv, Vertex}, mesh_type::MeshType};

pub struct Basic {
    name: &'static str,
    face: Face,
    mesh_type: MeshType
}

impl Basic {
    pub fn new(name: &'static str, face: Face, mesh_type: MeshType) -> Self {
        Self { name, face, mesh_type }
    }
}

impl IsBlock for Basic {
    fn name(&self) -> &'static str {
        self.name
    }

    fn get_uvs(&self, _dir: FaceDir) -> &[Uv] {
        self.face.uvs()
    }

    fn get_vertices(&self, dir: FaceDir) -> Box<[Vertex]> {
        self.face.vertices(dir)
    }

    fn num_vertices(&self, dir: FaceDir) -> usize {
        self.face.vertices(dir).len()
    }

    fn get_indices(&self, _dir: FaceDir, offset: usize) -> Box<[Index]> {
        self.face.indices().iter().map(|i| *i + offset).collect()
    }

    fn mesh_type(&self, _dir: FaceDir) -> MeshType {
        self.mesh_type
    }
}