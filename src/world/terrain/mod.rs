use super::WorldPoint;
use super::chunk::Chunk;
use world::registry::Registry;

mod flat;

pub use self::flat::FlatGenerator;

pub trait ChunkGenerator {
    fn generate_chunk(&self, origin: WorldPoint, registry: &Registry) -> Chunk;
}
