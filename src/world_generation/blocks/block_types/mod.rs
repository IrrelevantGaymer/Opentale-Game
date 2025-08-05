use crate::world_generation::blocks::block_types::{basic::Basic, pillar::Pillar};

pub mod basic;
pub mod custom;
pub mod pillar;

pub enum BlockType {
    Basic(Basic),
    Pillar(Pillar)
}