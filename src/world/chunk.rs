use super::{WorldPoint, LocalPoint};
use super::block::{Block, BlockID, VISIBLE_NONE};

pub const CHUNK_SIZE: i32 = 16;

pub struct Chunk {
    blocks: Vec<Block>,
    pub origin: WorldPoint,
}


fn local_loc_to_array_index(loc: LocalPoint) -> usize {
    loc.x as usize + loc.y as usize * CHUNK_SIZE as usize +
    loc.z as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize
}


impl Chunk {
    pub fn new(origin: WorldPoint) -> Chunk {
        let mut ret = Chunk {
            blocks: Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize),
            origin: origin,
        };
        ret.blocks.resize((CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize,
                          Block::from_id(BlockID(0), VISIBLE_NONE));
        ret
    }

    pub fn set_block_immediate(&mut self, loc: WorldPoint, block: Block) {
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

        let index = (offset.x + offset.y * CHUNK_SIZE + offset.z * CHUNK_SIZE * CHUNK_SIZE) as
                    usize;
        assert!(index < self.blocks.len());
        index
    }

    pub fn get_block(&self, loc: WorldPoint) -> Block {
        let index = self.loc_to_array_index(loc);
        self.blocks[index]
    }

    pub fn get_block_local(&self, loc: LocalPoint) -> Block {
        let index = local_loc_to_array_index(loc);
        self.blocks[index]
    }
}
