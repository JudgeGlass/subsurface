use prelude::*;

use std::collections::hash_map::HashMap;
use world::block::{BlockID, Face};

pub struct Registry {
    ids_by_name: HashMap<String, BlockID>,
    uvs_by_id: HashMap<BlockID, BlockUV>,
}

#[derive(Clone, Copy)]
pub struct BlockUV {
    uvs: [Point2<u8>; 6],
}

impl BlockUV {
    pub fn one_face(uv_origin: Point2<u8>) -> BlockUV {
        BlockUV { uvs: [uv_origin, uv_origin, uv_origin, uv_origin, uv_origin, uv_origin] }
    }

    pub fn get_face(&self, face: Face) -> Point2<u8> {
        self.uvs[face.to_index()]
    }
}

impl Registry {
    pub fn new() -> Registry {
        let mut ret = Registry {
            ids_by_name: HashMap::new(),
            uvs_by_id: HashMap::new(),
        };

        ret.register_block("stone".into(), BlockID(1), BlockUV::one_face(point2(2, 0)));
        ret.register_block("dirt".into(), BlockID(2), BlockUV::one_face(point2(0, 0)));
        ret.register_block("grass".into(),
                           BlockID(3),
                           BlockUV {
                               uvs: [point2(1, 0),
                                     point2(0, 0),
                                     point2(3, 0),
                                     point2(3, 0),
                                     point2(3, 0),
                                     point2(3, 0)],
                           });

        ret
    }

    pub fn lookup_id(&self, name: &String) -> Option<BlockID> {
        match self.ids_by_name.get(name) {
            Some(id) => Some(*id),
            None => None,
        }
    }

    pub fn lookup_texture(&self, id: BlockID) -> Option<BlockUV> {
        match self.uvs_by_id.get(&id) {
            Some(uv) => Some(*uv),
            None => None,
        }
    }

    pub fn register_block(&mut self, name: String, id: BlockID, uv: BlockUV) {
        self.ids_by_name.insert(name, id);
        self.uvs_by_id.insert(id, uv);
    }
}
