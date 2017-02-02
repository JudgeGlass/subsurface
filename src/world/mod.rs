use prelude::*;
use cgmath;
use num;

pub mod chunk;
pub mod block;
pub mod world;
pub mod terrain;
pub mod registry;

type WorldPoint = Point3<i32>;
type LocalPoint = Point3<u8>;

pub use self::world::World;

pub type RegionIter = PointIter<i32>;
pub type LocalIter = PointIter<u8>;

pub struct PointIter<I: cgmath::BaseNum + num::One> {
    current: Point3<I>,
    start: Point3<I>,
    stop: Point3<I>,
    started: bool,
}

impl<I: cgmath::BaseNum + num::One> PointIter<I> {
    pub fn new(start: Point3<I>, stop: Point3<I>) -> PointIter<I> {
        PointIter {
            current: start,
            start: start,
            stop: stop,
            started: false,
        }
    }
}

impl<I: cgmath::BaseNum + num::One> Iterator for PointIter<I> {
    type Item = Point3<I>;

    fn next(&mut self) -> Option<Point3<I>> {
        if !self.started {
            self.current = self.start;
            self.started = true;
            Some(self.current)
        } else {
            if self.current.x < self.stop.x {
                self.current.x += I::one();
                Some(self.current)
            } else {
                if self.current.y < self.stop.y {
                    self.current.y += I::one();
                    self.current.x = self.start.x;
                    Some(self.current)
                } else {
                    if self.current.z < self.stop.z {
                        self.current.z += I::one();
                        self.current.y = self.start.y;
                        self.current.x = self.start.x;
                        Some(self.current)
                    } else {
                        None
                    }
                }
            }
        }
    }
}
