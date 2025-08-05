use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};

pub type Uv = [f32; 2];
pub type Vertex = [f32; 3];
pub type Index = usize;

pub const DEFAULT_UVS: &[Uv] = &[
    [0., 0.,], 
    [1., 0.,], 
    [0., 1.,], 
    [1., 1.,],
];
pub const DEFAULT_VERTICES: &[Vertex] = &[
    [0., 0., 0.,], 
    [1., 0., 0.,], 
    [0., 1., 0.,], 
    [1., 1., 0.,], 
];
pub const DEFAULT_INDICES: &[Index] = &[0, 1, 2, 1, 3, 2];

//TODO: Make Face into a trait, and create 
//TODO: NormalFace, ConnectedFace, RandomFace, etc. 
//TODO: structs to support different kinds of blocks and textures
pub struct Face {
    texture_index: usize,
    texture_uvs: Box<[Uv]>,
    /// Vertices have a bias in the xy plane, facing the -z direction
    vertices: Box<[Vertex]>,
    indices: Box<[Index]>,
    culling_flag: CullingFlag
}

impl Face {
    /// the amount of textures associated with this face
    pub const SIZE: usize = 1;

    pub fn new(index: usize, culling_flag: CullingFlag) -> Self {
        Self {
            texture_index: index,
            texture_uvs: Box::from(DEFAULT_UVS),
            vertices: Box::from(DEFAULT_VERTICES),
            indices: Box::from(DEFAULT_INDICES),
            culling_flag
        }
    }

    pub fn index(&self) -> Index {
        self.texture_index
    }

    pub fn uvs(&self) -> &[Uv] {
        &*self.texture_uvs
    }

    pub fn vertices(&self, dir: FaceDir) -> Box<[Vertex]> {
        let q = match dir {
            FaceDir::Up => Quat::from_axis_angle(Vec3::X, -PI / 2.),
            FaceDir::North => Quat::from_axis_angle(Vec3::Y, PI),
            FaceDir::West => Quat::from_axis_angle(Vec3::Y, -PI / 2.),
            FaceDir::East => Quat::from_axis_angle(Vec3::Y, PI / 2.),
            FaceDir::South => Quat::IDENTITY,
            FaceDir::Down => Quat::from_axis_angle(Vec3::X, PI / 2.),
        };
        self.vertices.iter()
            .map(|vertex| {
                (
                    q.mul_vec3(Vec3::from_array(*vertex) - Vec3::ONE / 2.) 
                        + Vec3::ONE / 2.
                ).to_array()
            }).collect()
    }

    pub fn indices(&self) -> &[Index] {
        &*self.indices
    }

    pub fn culling_flag(&self) -> CullingFlag {
        self.culling_flag
    }
}

#[derive(Clone, Copy)]
/// The direction of a face.
/// 
/// Used for relative indexing from a texture index
pub enum FaceDir {
    Up, North, West, East, South, Down
}

#[derive(Clone, Copy)]
/// A flag representing when a texture is culled
pub enum CullingFlag {
    /// This face can not be culled nor does it cull other faces. 
    /// 
    /// Useful for transparent blocks, or blocks that aren't the size of a full block.
    /// 
    /// For example: glass, fences, water, and torches
    None = 0,
    /// Can cull other faces, but can't be culled itself.
    /// 
    /// Useful for blocks whose models' bounding box 
    /// is larger than a standard block.
    Cullable = 1,
    /// Can be culled itself, but can't cull other faces.
    /// 
    /// Useful for blocks whose models' bounding box
    /// is smaller than a standard block
    /// 
    /// For example: stairs, slopes, and slabs
    Culling = 2,
    /// Can cull and be culled by other faces.
    ///
    /// Useful for most solid blocks
    Both = 3
}