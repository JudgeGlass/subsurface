use prelude::*;
use world::block::Block;
use world::chunk::{Chunk, CHUNK_SIZE};
use world::WorldPoint;
use world::registry::Registry;
use super::ChunkGenerator;
use noise;

pub struct SimplexGenerator {
    high: i32,
    low: i32,

    seed: noise::Seed,
}

impl SimplexGenerator {
    pub fn new(high: i32, low: i32) -> SimplexGenerator {
        SimplexGenerator {
            high: high,
            low: low,

            seed: noise::Seed::new(87),
        }
    }
}

const FLAT_SCALING_FACTOR: f32 = 0.07;
const HOLE_SCALING_FACTOR: f32 = 0.15;

impl ChunkGenerator for SimplexGenerator {
    fn generate_chunk(&self, origin: WorldPoint, registry: &Registry) -> Chunk {
        let mut chunk = Chunk::new(origin);
        let grass_id = registry.lookup_id(&"grass".into()).expect("Could not find block ID");
        let dirt_id = registry.lookup_id(&"dirt".into()).expect("Could not find block ID");
        let stone_id = registry.lookup_id(&"stone".into()).expect("Could not find block ID");


        for x in origin.x..(origin.x + CHUNK_SIZE) {
            for z in origin.z..(origin.z + CHUNK_SIZE) {
                let sample = noise::open_simplex2(&self.seed,
                                                  &[x as f32 * FLAT_SCALING_FACTOR,
                                                    z as f32 * FLAT_SCALING_FACTOR]);
                let sample_0_1 = (sample + 1.0) / 2.0;
                let sampled_height = ((self.high - self.low) as f32 * sample_0_1) as i32 + self.low;

                for y in origin.y..(origin.y + CHUNK_SIZE) {
                    if y <= sampled_height &&
                       noise::open_simplex3(&self.seed,
                                            &[x as f32 * HOLE_SCALING_FACTOR,
                                              y as f32 * HOLE_SCALING_FACTOR,
                                              z as f32 * HOLE_SCALING_FACTOR]) <
                       0.25 {
                        let id = if y == sampled_height {
                            grass_id
                        } else if y <= sampled_height - 1 && y >= sampled_height - 3 {
                            dirt_id
                        } else {
                            stone_id
                        };

                        chunk.set_block_immediate(point3(x, y, z), Block::from_id_only(id));
                    }
                }
            }
        }

        chunk
    }
}
