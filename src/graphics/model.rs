use prelude::*;

use gfx;
use gfx::format::U16Norm;
use super::renderer::Vertex;

use world::chunk::{Chunk, CHUNK_SIZE};
use world::block::*;
use world::registry::Registry;

pub struct Model<R: gfx::Resources> {
    pub vbo: gfx::handle::Buffer<R, Vertex>,
    pub slice: gfx::Slice<R>,
    pub model: TransformMatrix,
}

fn darken(color: &mut Color, amount: u8) {
    for i in 0..3 {
        color[i] = U8Norm(if amount <= color[i].0 {
            color[i].0 - amount
        } else {
            0
        })
    }
}

impl<R: gfx::Resources> Model<R> {
    pub fn new<F: gfx::traits::FactoryExt<R>>(factory: &mut F,
                                              chunk: &Chunk,
                                              registry: &Registry)
                                              -> Model<R> {
        let mut verts = Vec::new();

        for x in 0..CHUNK_SIZE as u8 {
            for y in 0..CHUNK_SIZE as u8 {
                for z in 0..CHUNK_SIZE as u8 {
                    let loc = point3(x, y, z);
                    let block = chunk.get_block_local(loc);
                    if !block.is_empty() {
                        let color = registry.lookup_color(block.id)
                            .expect("Could not find color for block id");

                        if block.visibility.contains(VISIBLE_BOTTOM) {
                            make_bottom(loc, color, &mut verts);
                        }
                        if block.visibility.contains(VISIBLE_TOP) {
                            make_top(loc, color, &mut verts);
                        }
                        if block.visibility.contains(VISIBLE_FRONT) {
                            make_front(loc, color, &mut verts);
                        }
                        if block.visibility.contains(VISIBLE_BACK) {
                            make_back(loc, color, &mut verts);
                        }
                        if block.visibility.contains(VISIBLE_LEFT) {
                            make_left(loc, color, &mut verts);
                        }
                        if block.visibility.contains(VISIBLE_RIGHT) {
                            make_right(loc, color, &mut verts);
                        }
                    }
                }
            }
        }

        let (vbo, slice) = factory.create_vertex_buffer_with_slice(verts.as_slice(), ());
        Model {
            vbo: vbo,
            slice: slice,
            model: Matrix4::from_translation(vec3(chunk.origin.x as f32,
                                                  chunk.origin.y as f32,
                                                  chunk.origin.z as f32))
                .into(),
        }
    }
}


fn make_bottom(origin: Point3<u8>, color: Color, vert_out: &mut Vec<Vertex>) {
    let mut c = color; //[255, 255, 255];
    darken(&mut c, 30);
    let v = [vnew(point3(origin.x, origin.y, origin.z + 1), &c),
             vnew(point3(origin.x, origin.y, origin.z), &c),
             vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c),

             vnew(point3(origin.x, origin.y, origin.z), &c),
             vnew(point3(origin.x + 1, origin.y, origin.z), &c),
             vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c)];

    vert_out.extend_from_slice(&v);
}

fn make_top(origin: Point3<u8>, color: Color, vert_out: &mut Vec<Vertex>) {
    let c = color; //[0, 0, 0];
    let v = [vnew(point3(origin.x, origin.y + 1, origin.z + 1), &c),
             vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1), &c),
             vnew(point3(origin.x, origin.y + 1, origin.z), &c),

             vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1), &c),
             vnew(point3(origin.x + 1, origin.y + 1, origin.z), &c),
             vnew(point3(origin.x, origin.y + 1, origin.z), &c)];

    vert_out.extend_from_slice(&v);
}

fn make_back(origin: Point3<u8>, color: Color, vert_out: &mut Vec<Vertex>) {
    let mut c = color; //[255, 0, 255];
    darken(&mut c, 10);
    let v = [vnew(point3(origin.x, origin.y, origin.z), &c),
             vnew(point3(origin.x, origin.y + 1, origin.z), &c),
             vnew(point3(origin.x + 1, origin.y, origin.z), &c),

             vnew(point3(origin.x, origin.y + 1, origin.z), &c),
             vnew(point3(origin.x + 1, origin.y + 1, origin.z), &c),
             vnew(point3(origin.x + 1, origin.y, origin.z), &c)];

    vert_out.extend_from_slice(&v);
}

fn make_front(origin: Point3<u8>, color: Color, vert_out: &mut Vec<Vertex>) {
    let mut c = color; //[0, 255, 0];
    darken(&mut c, 10);
    let v = [vnew(point3(origin.x, origin.y, origin.z + 1), &c),
             vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c),
             vnew(point3(origin.x, origin.y + 1, origin.z + 1), &c),

             vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c),
             vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1), &c),
             vnew(point3(origin.x, origin.y + 1, origin.z + 1), &c)];

    vert_out.extend_from_slice(&v);
}

fn make_left(origin: Point3<u8>, color: Color, vert_out: &mut Vec<Vertex>) {
    let mut c = color; //[0, 255, 255];
    darken(&mut c, 5);
    let v = [vnew(point3(origin.x, origin.y, origin.z), &c),
             vnew(point3(origin.x, origin.y, origin.z + 1), &c),
             vnew(point3(origin.x, origin.y + 1, origin.z), &c),

             vnew(point3(origin.x, origin.y, origin.z + 1), &c),
             vnew(point3(origin.x, origin.y + 1, origin.z + 1), &c),
             vnew(point3(origin.x, origin.y + 1, origin.z), &c)];

    vert_out.extend_from_slice(&v);
}

fn make_right(origin: Point3<u8>, color: Color, vert_out: &mut Vec<Vertex>) {
    let mut c = color; //[255, 0, 0];
    darken(&mut c, 5);
    let v = [vnew(point3(origin.x + 1, origin.y, origin.z), &c),
             vnew(point3(origin.x + 1, origin.y + 1, origin.z), &c),
             vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c),

             vnew(point3(origin.x + 1, origin.y + 1, origin.z), &c),
             vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1), &c),
             vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c)];

    vert_out.extend_from_slice(&v);
}

fn vnew(position: Point3<u8>, color: &Color) -> Vertex {
    Vertex {
        position: [position.x, position.y, position.z, 1],
        color: *color,
        uv: [U16Norm(0), U16Norm(0)],
    }
}
