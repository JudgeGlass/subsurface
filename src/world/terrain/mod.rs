use super::WorldPoint;
use super::chunk::Chunk;

mod flat;

pub use self::flat::FlatGenerator;

pub trait ChunkGenerator {
    fn generate_chunk(&self, origin: WorldPoint) -> Chunk;
}
