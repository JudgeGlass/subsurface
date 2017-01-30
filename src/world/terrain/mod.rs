use super::WorldPoint;
use super::chunk::Chunk;
use world::registry::Registry;

mod flat;
mod simplex;

pub use self::flat::FlatGenerator;
pub use self::simplex::SimplexGenerator;

pub trait ChunkGenerator {
    fn generate_chunk(&self, origin: WorldPoint, registry: &Registry) -> Chunk;
}
