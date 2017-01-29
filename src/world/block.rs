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
