use bevy::app::Plugin;

use crate::{
    table, 
    world_generation::blocks::{
        block::Block, block_builders::{basic::BasicBuilder, pillar::PillarBuilder}, block_types::BlockType, buildable::CanBuild, face::CullingFlag
    }
};        

pub mod as_id;
pub mod block_builders;
pub mod block_types;
pub mod buildable;
pub mod block;
pub mod define;
pub mod face;
pub mod mesh_type;
pub mod table;
pub mod texture_builder;

pub const DEFAULT_TEXTURE: &'static str = "debug_texture.png";

table!(
    CanBuild<BuildTo = BlockType>, BlockType, 
    enum BlockId, 
    static BLOCK_BUILDERS = {
        let Dirt: BasicBuilder = Block::new_basic("dirt");
        let Grass: BasicBuilder = Block::new_basic("grass_block");
        let Stone: BasicBuilder = Block::new_basic("stone");
        let Log: PillarBuilder = Block::new_pillar("log");
        let Snow: BasicBuilder = Block::new_basic("snow_block");
        let Leaf: BasicBuilder = Block::new_basic("leaf")
            .with_culling_flag(CullingFlag::None);
    }
);

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, _app: &mut bevy::app::App) {
        
    }
}