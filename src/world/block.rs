use std;
use prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable, Hash)]
pub struct BlockID(pub u32);

bitflags!(
    #[derive(RustcEncodable, RustcDecodable)]
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

#[derive(Copy, Clone, Debug)]
pub enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

static FACE_LIST: [Face; 6] =
    [Face::Top, Face::Bottom, Face::Left, Face::Right, Face::Front, Face::Back];

#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct LightLevel(pub u8);

#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct SunLightLevel(pub u8);

pub type TotalLightLevel = (SunLightLevel, LightLevel);

#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct FaceLightLevels {
    levels: [TotalLightLevel; 6],
}

#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub enum LightKind {
    Source(TotalLightLevel),
    Solid(FaceLightLevels),
}

pub const SOLID_NO_LIGHT: LightKind =
    LightKind::Solid(FaceLightLevels { levels: [(SunLightLevel(0), LightLevel(0)); 6] });

#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct Block {
    pub id: BlockID,
    pub visibility: FaceVisibility,
    pub light: LightKind,
}

impl LightKind {
    pub fn source(sun: u8, block: u8) -> LightKind {
        LightKind::Source((SunLightLevel(sun), LightLevel(block)))
    }
}

impl Face {
    pub fn from_normal(normal: Point3<i32>) -> Face {
        if normal.x > 0 {
            Face::Right
        } else if normal.x < 0 {
            Face::Left
        } else if normal.y > 0 {
            Face::Top
        } else if normal.y < 0 {
            Face::Bottom
        } else if normal.z > 0 {
            Face::Front
        } else if normal.z < 0 {
            Face::Back
        } else {
            panic!("Normal is invalid {:?}", normal);
        }
    }

    pub fn to_index(&self) -> usize {
        match *self {
            Face::Top => 0,
            Face::Bottom => 1,
            Face::Left => 2,
            Face::Right => 3,
            Face::Front => 4,
            Face::Back => 5,
        }
    }

    #[inline]
    pub fn iter() -> std::slice::Iter<'static, Face> {
        FACE_LIST.iter()
    }

    #[inline]
    pub fn to_visible_mask(&self) -> FaceVisibility {
        match *self {
            Face::Top => VISIBLE_TOP,
            Face::Bottom => VISIBLE_BOTTOM,
            Face::Left => VISIBLE_LEFT,
            Face::Right => VISIBLE_RIGHT,
            Face::Front => VISIBLE_FRONT,
            Face::Back => VISIBLE_BACK,
        }
    }

    #[inline]
    pub fn normal(&self) -> Vector3<i32> {
        match *self {
            Face::Top => vec3(0, 1, 0),
            Face::Bottom => vec3(0, -1, 0),
            Face::Left => vec3(-1, 0, 0),
            Face::Right => vec3(1, 0, 0),
            Face::Front => vec3(0, 0, 1),
            Face::Back => vec3(0, 0, -1),
        }
    }
}

impl Block {
    #[inline]
    pub fn from_id(id: BlockID, visibility: FaceVisibility, light: LightKind) -> Block {
        Block {
            id: id,
            visibility: visibility,
            light: light,
        }
    }

    #[inline]
    pub fn from_id_only(id: BlockID) -> Block {
        Block {
            id: id,
            visibility: VISIBLE_UNSET,
            light: LightKind::source(0, 0),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.id == BlockID(0)
    }

    #[inline]
    pub fn face_light(&self, face: Face) -> TotalLightLevel {
        match self.light {
            LightKind::Source(light) => light,
            LightKind::Solid(lights) => lights.levels[face.to_index()],
        }
    }

    #[inline]
    pub fn is_visible(&self, face: Face) -> bool {
        match face {
            Face::Top => self.visibility.contains(VISIBLE_TOP),
            Face::Bottom => self.visibility.contains(VISIBLE_BOTTOM),
            Face::Left => self.visibility.contains(VISIBLE_LEFT),
            Face::Right => self.visibility.contains(VISIBLE_RIGHT),
            Face::Front => self.visibility.contains(VISIBLE_FRONT),
            Face::Back => self.visibility.contains(VISIBLE_BACK),
        }
    }
}
