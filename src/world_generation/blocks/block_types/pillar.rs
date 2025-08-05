use crate::world_generation::blocks::{block::IsBlock, face::{Face, FaceDir, Index, Uv, Vertex}, mesh_type::MeshType};

pub struct Pillar {
    name: &'static str,
    faces: [Face; 3],
    mesh_type: MeshType
}

impl Pillar {
    pub fn new(
        name: &'static str, 
        faces: [Face; 3], 
        mesh_type: MeshType
    ) -> Self {
        Self { name, faces, mesh_type }
    }

    pub fn face_index(dir: FaceDir) -> usize {
        match dir {
            FaceDir::Up      => 0,
            FaceDir::North 
            | FaceDir::West 
            | FaceDir::East 
            | FaceDir::South => 1,
            FaceDir::Down    => 2
        }
    }
}

impl IsBlock for Pillar {
    fn name(&self) -> &'static str {
        self.name
    }

    fn get_uvs(&self, dir: FaceDir) -> &[Uv] {
        self.faces[Self::face_index(dir)].uvs()
    }

    fn get_vertices(&self, dir: FaceDir) -> Box<[Vertex]> {
        self.faces[Self::face_index(dir)].vertices(dir)
    }

    fn num_vertices(&self, dir: FaceDir) -> usize {
        self.faces[Self::face_index(dir)].vertices(dir).len()
    }

    fn get_indices(&self, dir: FaceDir, offset: usize) -> Box<[Index]> {
        self.faces[Self::face_index(dir)]
            .indices()
            .iter()
            .map(|i| *i + offset)
            .collect()
    }

    fn mesh_type(&self) -> MeshType {
        self.mesh_type
    }
}