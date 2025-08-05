use crate::world_generation::blocks::{as_id::AsId, block_types::BlockType};

#[const_trait]
pub trait Buildable: HasBuildVariants + CanBuild {
    fn new_with_name(name: &'static str) -> Self;
    fn get_texture_size() -> usize;
    /// This is useful for blocks with multiple blockstates
    /// such as stairs, slabs, waterloggable blocks, rotateable blocks,
    /// connecting blocks, etc.  Most blocks don't have blockstates,
    /// so the default is 1.
    fn get_id_span() -> usize { 1usize }
    fn with_index(self, idx: usize) -> Self;
    fn set_index(&mut self, idx: usize);
    fn with_id(self, id: usize) -> Self;
    fn set_id(&mut self, id: usize);
}

pub trait HasBuildVariants {
    type Variants: AsId;
}

pub trait CanBuild: Send + Sync {
    fn build(self) -> BlockType;
}