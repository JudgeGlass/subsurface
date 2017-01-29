use prelude::*;

pub mod chunk;
pub mod block;
pub mod world;
pub mod terrain;

type WorldPoint = Point3<i32>;
type LocalPoint = Point3<u8>;

pub use self::world::World;
