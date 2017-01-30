use prelude::*;

use std::collections::hash_map::HashMap;
use world::block::BlockID;

pub struct Registry {
    ids_by_name: HashMap<String, BlockID>,
    uvs_by_id: HashMap<BlockID, Point2<u8>>,
}

impl Registry {
    pub fn new() -> Registry {
        let mut ret = Registry {
            ids_by_name: HashMap::new(),
            uvs_by_id: HashMap::new(),
        };

        ret.register_block("stone".into(), BlockID(1), point2(2, 0));
        ret.register_block("dirt".into(), BlockID(2), point2(0, 0));
        ret.register_block("grass".into(), BlockID(3), point2(1, 0));

        ret
    }

    pub fn lookup_id(&self, name: &String) -> Option<BlockID> {
        match self.ids_by_name.get(name) {
            Some(id) => Some(*id),
            None => None,
        }
    }

    pub fn lookup_texture(&self, id: BlockID) -> Option<Point2<u8>> {
        match self.uvs_by_id.get(&id) {
            Some(uv) => Some(*uv),
            None => None,
        }
    }

    pub fn register_block(&mut self, name: String, id: BlockID, uv_origin: Point2<u8>) {
        self.ids_by_name.insert(name, id);
        self.uvs_by_id.insert(id, uv_origin);
    }
}
