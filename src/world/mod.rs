use dot_vox;

use std::collections::hash_map::HashMap;
use prelude::*;
use graphics;
use glium;

type WorldPoint = Point3<i32>;
type LocalPoint = Point3<u8>;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BlockID(pub u32);

bitflags!(
    pub flags FaceVisibility: u8 {
        const VISIBLE_NONE   = 0b00000000,
        const VISIBLE_UNSET  = 0b11111111,

        const VISIBLE_TOP    = 0b00000001,
        const VISIBLE_BOTTOM = 0b00000010,
        const VISIBLE_RIGHT  = 0b00000100,
        const VISIBLE_LEFT   = 0b00001000,
        const VISIBLE_FRONT  = 0b00010000,
        const VISIBLE_BACK   = 0b00100000,
    }
);

#[derive(Copy, Clone)]
pub struct Block {
    pub id: BlockID,
    pub visibility: FaceVisibility,
}

pub const CHUNK_SIZE: i32 = 16;

pub struct Chunk {
    blocks: Vec<Block>,
    pub origin: WorldPoint,
}

pub struct World {
    chunks: HashMap<WorldPoint, Chunk>,
}


#[inline]
fn find_chunk_origin(point: WorldPoint) -> WorldPoint {
    point3(point.x & (-CHUNK_SIZE), point.y & (-CHUNK_SIZE), point.z & (-CHUNK_SIZE))
}

#[test]
fn origins() {
    assert_eq!(find_chunk_origin(point3(0,0,0)), point3(0,0,0));
    assert_eq!(find_chunk_origin(point3(4,4,5)), point3(0,0,0));
    assert_eq!(find_chunk_origin(point3(-1,-1,-1)), point3(-CHUNK_SIZE,-CHUNK_SIZE,-CHUNK_SIZE));
}


impl World {
    pub fn from_vox(data: dot_vox::DotVoxData) -> World {
        let mut world = World { chunks: HashMap::new() };

        for model in data.models.iter() {
            for voxel in model.voxels.iter() {
                let loc = point3(voxel.y as i32, voxel.z as i32, voxel.x as i32);
                let color32 = data.pallete[voxel.i as usize];

                world.set_block_immediate(loc, Block::from_id(BlockID(color32), VISIBLE_NONE));
            }
        }

        world.fix_visibility();

        world
    }

    pub fn make_models<F: glium::backend::Facade>(&self, facade: &F) -> Vec<graphics::Model> {
        self.chunks.iter().map(|(_, chunk)| graphics::Model::new(facade, chunk)).collect()
    }

    pub fn get_block(&self, loc: WorldPoint) -> Block {
        let chunk_origin = find_chunk_origin(loc);
        match self.chunks.get(&chunk_origin) {
            Some(chunk) => chunk.get_block(loc),
            None => Block::from_id(BlockID(0), VISIBLE_NONE),
        }
    }

    fn fix_visibility(&mut self) {
        let chunk_keys: Vec<WorldPoint> = self.chunks.keys().map(|x| x.clone()).collect();
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

        match new_chunk {
            Some(mut chunk) => {
                chunk.set_block_immediate(loc, block);
                self.chunks.insert(chunk_origin, chunk);
            }
            _ => (),
        }
    }
}

fn local_loc_to_array_index(loc: LocalPoint) -> usize {
    loc.x as usize + loc.y as usize * CHUNK_SIZE as usize +
    loc.z as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize
}


impl Chunk {
    fn new(origin: WorldPoint) -> Chunk {
        let mut ret = Chunk {
            blocks: Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize),
            origin: origin,
        };
        ret.blocks.resize((CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize,
                          Block::from_id(BlockID(0), VISIBLE_NONE));
        ret
    }

    fn set_block_immediate(&mut self, loc: WorldPoint, block: Block) {
        let index = self.loc_to_array_index(loc);
        self.blocks[index] = block;
    }

    fn is_loc_local(&self, loc: WorldPoint) -> bool {
        let offset = loc - self.origin;
        offset.x < CHUNK_SIZE && offset.y < CHUNK_SIZE && offset.z < CHUNK_SIZE &&
        offset.x >= 0 && offset.y >= 0 && offset.z >= 0
    }

    fn loc_to_array_index(&self, loc: WorldPoint) -> usize {
        let offset = loc - self.origin;
        if !self.is_loc_local(loc) {
            println!("Offset: {} {} {}", loc.x, loc.y, loc.z);
        }
        assert!(self.is_loc_local(loc));

        let index = (offset.x + offset.y * CHUNK_SIZE +
                     offset.z * CHUNK_SIZE * CHUNK_SIZE) as usize;
        assert!(index < self.blocks.len());
        index
    }

    fn get_block(&self, loc: WorldPoint) -> Block {
        let index = self.loc_to_array_index(loc);
        self.blocks[index]
    }

    pub fn get_block_local(&self, loc: LocalPoint) -> Block {
        let index = local_loc_to_array_index(loc);
        self.blocks[index]
    }
}

impl Block {
    #[inline]
    pub fn from_id(id: BlockID, visibility: FaceVisibility) -> Block {
        Block {
            id: id,
            visibility: visibility,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.id == BlockID(0)
    }
}
