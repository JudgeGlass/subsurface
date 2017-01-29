use std::collections::hash_map::HashMap;
use prelude::*;
use dot_vox;
use graphics;
use gfx;

use super::WorldPoint;
use super::chunk::{Chunk, CHUNK_SIZE};
use super::block::*;

pub struct World {
    chunks: HashMap<WorldPoint, Chunk>,
}


#[inline]
fn find_chunk_origin(point: WorldPoint) -> WorldPoint {
    point3(point.x & (-CHUNK_SIZE),
           point.y & (-CHUNK_SIZE),
           point.z & (-CHUNK_SIZE))
}

#[test]
fn origins() {
    assert_eq!(find_chunk_origin(point3(0, 0, 0)), point3(0, 0, 0));
    assert_eq!(find_chunk_origin(point3(4, 4, 5)), point3(0, 0, 0));
    assert_eq!(find_chunk_origin(point3(-1, -1, -1)),
               point3(-CHUNK_SIZE, -CHUNK_SIZE, -CHUNK_SIZE));
}


impl World {
    pub fn from_vox(data: dot_vox::DotVoxData) -> World {
        debug!("Loading world from MagicaVoxel data...");
        let mut world = World { chunks: HashMap::new() };

        for model in &data.models {
            for voxel in &model.voxels {
                let loc = point3(voxel.y as i32, voxel.z as i32, voxel.x as i32);
                let color32 = data.pallete[voxel.i as usize];

                world.set_block_immediate(loc, Block::from_id(BlockID(color32), VISIBLE_NONE));
            }
        }

        world.fix_visibility();

        world
    }

    pub fn make_models<R, F: gfx::traits::FactoryExt<R>>(&self,
                                                         factory: &mut F)
                                                         -> Vec<graphics::Model<R>>
        where R: gfx::Resources
    {
        self.chunks.iter().map(|(_, chunk)| graphics::Model::new(factory, chunk)).collect()
    }

    pub fn get_block(&self, loc: WorldPoint) -> Block {
        let chunk_origin = find_chunk_origin(loc);
        match self.chunks.get(&chunk_origin) {
            Some(chunk) => chunk.get_block(loc),
            None => Block::from_id(BlockID(0), VISIBLE_NONE),
        }
    }

    fn fix_visibility(&mut self) {
        let chunk_keys: Vec<WorldPoint> = self.chunks.keys().cloned().collect();
        for chunk_key in chunk_keys {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let current_loc = chunk_key + vec3(x, y, z);
                        let current_block = self.get_block(current_loc);
                        if !current_block.is_empty() {
                            let mut visibility = VISIBLE_NONE;

                            visibility |= if self.get_block(current_loc + vec3(0, 1, 0))
                                .is_empty() {
                                VISIBLE_TOP
                            } else {
                                VISIBLE_NONE
                            };

                            visibility |= if self.get_block(current_loc + vec3(0, -1, 0))
                                .is_empty() {
                                VISIBLE_BOTTOM
                            } else {
                                VISIBLE_NONE
                            };

                            visibility |= if self.get_block(current_loc + vec3(1, 0, 0))
                                .is_empty() {
                                VISIBLE_RIGHT
                            } else {
                                VISIBLE_NONE
                            };

                            visibility |= if self.get_block(current_loc + vec3(-1, 0, 0))
                                .is_empty() {
                                VISIBLE_LEFT
                            } else {
                                VISIBLE_NONE
                            };

                            visibility |= if self.get_block(current_loc + vec3(0, 0, 1))
                                .is_empty() {
                                VISIBLE_FRONT
                            } else {
                                VISIBLE_NONE
                            };

                            visibility |= if self.get_block(current_loc + vec3(0, 0, -1))
                                .is_empty() {
                                VISIBLE_BACK
                            } else {
                                VISIBLE_NONE
                            };

                            self.set_block_immediate(current_loc,
                                                     Block {
                                                         id: current_block.id,
                                                         visibility: visibility,
                                                     });
                        }
                    }
                }
            }
        }
    }

    fn set_block_immediate(&mut self, loc: WorldPoint, block: Block) {
        let chunk_origin = find_chunk_origin(loc);
        let new_chunk = match self.chunks.get_mut(&chunk_origin) {
            Some(chunk) => {
                chunk.set_block_immediate(loc, block);
                None
            }
            None => Some(Chunk::new(chunk_origin)),
        };

        if let Some(mut chunk) = new_chunk {
            chunk.set_block_immediate(loc, block);
            self.chunks.insert(chunk_origin, chunk);
        }
    }
}
