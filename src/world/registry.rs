use prelude::*;

use std::collections::hash_map::HashMap;
use world::block::BlockID;

pub struct Registry {
    ids_by_name: HashMap<String, BlockID>,
    // TODO: UV map here instead
    colors_by_id: HashMap<BlockID, Color>,
}

impl Registry {
    pub fn new() -> Registry {
        let mut ret = Registry {
            ids_by_name: HashMap::new(),
            colors_by_id: HashMap::new(),
        };

        ret.register_block("stone".into(), BlockID(1), color(64, 64, 64));
        ret.register_block("dirt".into(), BlockID(2), color(122, 48, 0));
        ret.register_block("grass".into(), BlockID(2), color(0, 127, 14));

        ret
    }

    pub fn lookup_id(&self, name: &String) -> Option<BlockID> {
        match self.ids_by_name.get(name) {
            Some(id) => Some(*id),
            None => None,
        }
    }

    pub fn lookup_color(&self, id: BlockID) -> Option<Color> {
        match self.colors_by_id.get(&id) {
            Some(color) => Some(*color),
            None => None,
        }
    }

    pub fn register_block(&mut self, name: String, id: BlockID, color: Color) {
        self.ids_by_name.insert(name, id);
        self.colors_by_id.insert(id, color);
    }
}
