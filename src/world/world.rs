use std::collections::hash_map::HashMap;
use std::path::{Path, PathBuf};
use prelude::*;
use graphics;
use gfx;

use super::WorldPoint;
use super::chunk::{Chunk, CHUNK_SIZE, CHUNK_EXTENTS_LESS_ONE};
use super::block::*;
use super::terrain::ChunkGenerator;
use super::registry::Registry;

pub struct World {
    chunks: HashMap<WorldPoint, Chunk>,
    world_root: PathBuf,
    chunk_gen: Box<ChunkGenerator>,
    pub registry: Registry,

    dirty_chunks: Vec<WorldPoint>,
}

#[inline]
fn find_chunk_origin(point: WorldPoint) -> WorldPoint {
    point3(point.x & (-CHUNK_SIZE),
           point.y & (-CHUNK_SIZE),
           point.z & (-CHUNK_SIZE))
}

#[test]
fn origins() {
    assert_eq!(find_chunk_origin(point3(0, 0, 0)), point3(0, 0, 0));
    assert_eq!(find_chunk_origin(point3(4, 4, 5)), point3(0, 0, 0));
    assert_eq!(find_chunk_origin(point3(-1, -1, -1)),
               point3(-CHUNK_SIZE, -CHUNK_SIZE, -CHUNK_SIZE));
}


impl World {
    pub fn from_path(world_root: &Path,
                     extents: (Vector3<i32>, Vector3<i32>),
                     chunk_gen: Box<ChunkGenerator>)
                     -> World {
        use num_iter::range_step;

        let mut world = World {
            chunks: HashMap::new(),
            world_root: world_root.into(),
            chunk_gen: chunk_gen,
            registry: Registry::new(),

            dirty_chunks: Vec::new(),
        };
        for x in range_step(extents.0.x, extents.1.x, CHUNK_SIZE) {
            for y in range_step(extents.1.y - CHUNK_SIZE,
                                extents.0.y - CHUNK_SIZE,
                                -CHUNK_SIZE) {
                for z in range_step(extents.0.z, extents.1.z, CHUNK_SIZE) {
                    world.load_chunk(point3(x, y, z));
                }
            }
        }

        world.fix_visibility();
        world
    }

    fn load_chunk(&mut self, chunk_origin: WorldPoint) {
        match Chunk::read(chunk_origin, &self.world_root) {
            Some(chunk) => {
                self.chunks.insert(chunk_origin, chunk);
            }
            None => {
                self.chunks.insert(chunk_origin,
                                   self.chunk_gen.generate_chunk(chunk_origin, &self.registry));
            }
        }
    }

    pub fn make_models<R, F: gfx::traits::FactoryExt<R>>(&self,
                                                         factory: &mut F)
                                                         -> Vec<(WorldPoint, graphics::Model<R>)>
        where R: gfx::Resources
    {
        self.chunks
            .iter()
            .map(|(origin, chunk)| (origin, graphics::Model::new(factory, chunk, &self.registry)))
            .filter(|o| o.1.is_some())
            .map(|o| (*o.0, o.1.unwrap()))
            .collect()
    }

    pub fn clean_chunk<R, F>(&mut self, factory: &mut F) -> Option<(WorldPoint, graphics::Model<R>)>
        where R: gfx::Resources,
              F: gfx::traits::FactoryExt<R>
    {
        match self.dirty_chunks.pop() {
            Some(origin) => {
                let chunk = self.chunks.get_mut(&origin).unwrap();
                chunk.dirty = false;
                match graphics::Model::new(factory, chunk, &self.registry) {
                    Some(model) => Some((origin, model)),
                    None => None,
                }
            }
            None => None,
        }
    }

    pub fn get_block(&self, loc: WorldPoint) -> Block {
        let chunk_origin = find_chunk_origin(loc);
        match self.chunks.get(&chunk_origin) {
            Some(chunk) => chunk.get_block(loc),
            None => Block::from_id(BlockID(0), VISIBLE_NONE, SOLID_NO_LIGHT),
        }
    }

    fn fix_visibility(&mut self) {
        let chunk_keys: Vec<WorldPoint> = self.chunks.keys().cloned().collect();
        for chunk_key in chunk_keys {
            for current_loc in super::RegionIter::new(chunk_key,
                                                      chunk_key + CHUNK_EXTENTS_LESS_ONE) {
                let current_block = self.get_block(current_loc);
                if !current_block.is_empty() {
                    let mut visibility = VISIBLE_NONE;

                    for face in Face::iter() {
                        visibility |= if self.get_block(current_loc + face.normal())
                            .is_empty() {
                            face.to_visible_mask()
                        } else {
                            VISIBLE_NONE
                        };
                    }

                    // let light = LightKind::source((current_loc.z.abs() % 16) as u8,
                    //                               (current_loc.x.abs() % 16) as u8);
                    let light = LightKind::source(15, 15);
                    let block = Block::from_id(current_block.id, visibility, light);
                    self.set_block_immediate(current_loc, block);
                }
            }
        }
    }

    fn set_block_immediate(&mut self, loc: WorldPoint, block: Block) {
        let chunk_origin = find_chunk_origin(loc);
        let new_chunk = match self.chunks.get_mut(&chunk_origin) {
            Some(chunk) => {
                chunk.set_block_immediate(loc, block);
                if !chunk.dirty {
                    chunk.dirty = true;
                    self.dirty_chunks.push(chunk_origin);
                }
                None
            }
            None => Some(Chunk::new(chunk_origin)),
        };

        if let Some(mut chunk) = new_chunk {
            chunk.set_block_immediate(loc, block);
            self.chunks.insert(chunk_origin, chunk);
        }
    }

    pub fn place_block(&mut self, loc: WorldPoint, id: BlockID) {
        let mut visibility = VISIBLE_NONE;
        for face in Face::iter() {
            let neighbor = self.get_block(loc + face.normal());
            if neighbor.is_empty() {
                visibility |= face.to_visible_mask();
            } else {
                let neighbor_face = face.opposite();
                let new_neighbor = Block::from_id(neighbor.id,
                                                  neighbor.visibility &
                                                  (!neighbor_face.to_visible_mask()),
                                                  neighbor.light);
                self.set_block_immediate(loc + face.normal(), new_neighbor);
            }
        }

        let block = Block::from_id(id, visibility, LightKind::source(15, 15));
        self.set_block_immediate(loc, block);
    }

    pub fn break_block(&mut self, loc: WorldPoint) {
        let block = Block::from_id(BlockID(0), VISIBLE_NONE, LightKind::source(15, 15));
        self.set_block_immediate(loc, block);

        for face in Face::iter() {
            let neighbor = self.get_block(loc + face.normal());
            if !neighbor.is_empty() {
                let neighbor_face = face.opposite();
                let new_neighbor = Block::from_id(neighbor.id,
                                                  neighbor.visibility |
                                                  neighbor_face.to_visible_mask(),
                                                  neighbor.light);
                self.set_block_immediate(loc + face.normal(), new_neighbor);
            }
        }

    }

    pub fn write_all_chunks(&self) {
        for (_, chunk) in &self.chunks {
            chunk.write(&self.world_root);
        }
    }

    pub fn cast_ray(&self,
                    origin: Point3<f32>,
                    direction: Vector3<f32>)
                    -> Option<(WorldPoint, Face)> {
        use num::signum;
        use cgmath::prelude::*;

        let mut origin_vox = point3(origin.x as i32, origin.y as i32, origin.z as i32);
        let step = vec3(signum(direction.x),
                        signum(direction.y),
                        signum(direction.z));
        let mut max = point3(distance_from_edge(origin.x, direction.x),
                             distance_from_edge(origin.y, direction.y),
                             distance_from_edge(origin.z, direction.z));
        let delta = step.div_element_wise(direction);

        loop {
            let normal = if max.x < max.y && max.x < max.z {
                if max.x.abs() > direction.x.abs() {
                    break;
                }

                origin_vox.x += step.x as i32;
                max.x += delta.x;
                point3(-step.x as i32, 0, 0)
            } else if max.y < max.x && max.y < max.z {
                if max.y.abs() > direction.y.abs() {
                    break;
                }

                origin_vox.y += step.y as i32;
                max.y += delta.y;
                point3(0, -step.y as i32, 0)
            } else {
                assert!(max.z <= max.x && max.z <= max.y);
                if max.z.abs() > direction.z.abs() {
                    break;
                }

                origin_vox.z += step.z as i32;
                max.z += delta.z;
                point3(0, 0, -step.z as i32)
            };

            let block = self.get_block(origin_vox);
            if !block.is_empty() {
                return Some((origin_vox, Face::from_normal(normal)));
            }
        }

        None
    }
}

fn distance_from_edge(value: f32, direction: f32) -> f32 {
    (if direction > 0.0 {
        value.ceil() - value
    } else {
        value - value.floor()
    }) / direction.abs()
}
