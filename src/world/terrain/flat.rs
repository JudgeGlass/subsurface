use world::block::Block;
use world::chunk::{Chunk, CHUNK_EXTENTS_LESS_ONE};
use world::WorldPoint;
use world::registry::Registry;
use world::RegionIter;
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

        for loc in RegionIter::new(origin, origin + CHUNK_EXTENTS_LESS_ONE) {
            if loc.y <= self.high && loc.y >= self.low {
                chunk.set_block_immediate(loc, Block::from_id_only(id));
            }
        }

        chunk
    }
}
