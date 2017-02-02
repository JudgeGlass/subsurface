use std::path::{Path, PathBuf};

use super::{WorldPoint, LocalPoint};
use super::block::{Block, BlockID, VISIBLE_NONE, SOLID_NO_LIGHT};

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};

pub const CHUNK_SIZE: i32 = 16;

pub struct Chunk {
    blocks: Vec<Block>,
    pub origin: WorldPoint,
}


fn local_loc_to_array_index(loc: LocalPoint) -> usize {
    loc.x as usize + loc.y as usize * CHUNK_SIZE as usize +
    loc.z as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize
}


fn origin_to_path(origin: WorldPoint, world_root: &Path) -> PathBuf {
    world_root.join(origin.x.to_string())
        .join(origin.y.to_string())
        .join(origin.z.to_string())
}

impl Chunk {
    pub fn write(&self, world_root: &Path) {
        use std::io::Write;
        use std::fs::File;
        use std::fs::DirBuilder;

        let path = origin_to_path(self.origin, world_root);
        DirBuilder::new().recursive(true).create(path.clone()).unwrap();
        let dest = path.join("chunk.bincode");
        debug!("Writing chunk to {:?}", dest);
        let encoded: Vec<u8> = encode(&self.blocks, SizeLimit::Infinite).unwrap();
        File::create(dest).unwrap().write_all(encoded.as_slice()).unwrap();
    }

    pub fn read(origin: WorldPoint, world_root: &Path) -> Option<Chunk> {
        use std::io::Read;
        use std::fs::File;

        let mut bytes = Vec::new();
        let path = origin_to_path(origin, world_root).join("chunk.bincode");
        debug!("Reading chunk from {:?}", path);
        match File::open(path) {
            Ok(mut file) => {
                file.read_to_end(&mut bytes).unwrap();
                Some(Chunk {
                    blocks: decode(bytes.as_slice()).unwrap(),
                    origin: origin,
                })
            }
            Err(_) => {
                debug!("Chunk was not found");
                None
            }
        }
    }

    pub fn new(origin: WorldPoint) -> Chunk {
        let mut ret = Chunk {
            blocks: Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize),
            origin: origin,
        };
        ret.blocks.resize((CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize,
                          Block::from_id(BlockID(0), VISIBLE_NONE, SOLID_NO_LIGHT));
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
