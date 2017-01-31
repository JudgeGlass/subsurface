use prelude::*;

use std::collections::hash_map::HashMap;
use world::block::BlockID;

pub struct Registry {
    ids_by_name: HashMap<String, BlockID>,
    uvs_by_id: HashMap<BlockID, BlockUV>,
}

#[derive(Clone, Copy)]
pub struct BlockUV {
    pub top: Point2<u8>,
    pub bottom: Point2<u8>,
    pub left: Point2<u8>,
    pub right: Point2<u8>,
    pub front: Point2<u8>,
    pub back: Point2<u8>,
}

impl BlockUV {
    pub fn one_face(uv_origin: Point2<u8>) -> BlockUV {
        BlockUV {
            top: uv_origin,
            bottom: uv_origin,
            left: uv_origin,
            right: uv_origin,
            front: uv_origin,
            back: uv_origin,
        }
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
                               top: point2(1, 0),
                               bottom: point2(0, 0),
                               left: point2(3, 0),
                               right: point2(3, 0),
                               front: point2(3, 0),
                               back: point2(3, 0),
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
