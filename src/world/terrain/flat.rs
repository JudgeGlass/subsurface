use prelude::*;
use world::block::{Block, BlockID};
use world::chunk::{Chunk, CHUNK_SIZE};
use world::WorldPoint;
use world::registry::Registry;
use super::ChunkGenerator;

pub struct FlatGenerator {
    high: i32,
    low: i32,
    block: String,
}

impl FlatGenerator {
    pub fn new(high: i32, low: i32, block: String) -> FlatGenerator {
        FlatGenerator {
            high: high,
            low: low,
            block: block,
        }
    }
}

impl ChunkGenerator for FlatGenerator {
    fn generate_chunk(&self, origin: WorldPoint, registry: &Registry) -> Chunk {
        let mut chunk = Chunk::new(origin);
        let id = registry.lookup_id(&self.block).expect("Could not find block ID");

        for y in origin.y..(origin.y + CHUNK_SIZE) {
            if y <= self.high && y >= self.low {
                for x in origin.x..(origin.x + CHUNK_SIZE) {
                    for z in origin.z..(origin.z + CHUNK_SIZE) {
                        chunk.set_block_immediate(point3(x, y, z), Block::from_id_only(id));
                    }
                }
            }
        }

        chunk
    }
}
