use glium;
use dot_vox;

use prelude::*;

use world;

// If this is > 12 bytes, indexed drawing is has better space efficiency
#[derive(Copy, Clone)]
pub struct Vertex {
    position: SVector3<u8>,
    color: SVector3<u8>,
}

implement_vertex!(Vertex, position normalize(false), color normalize(true));

pub struct Model {
    pub vbo: glium::VertexBuffer<Vertex>,
    pub ibo: glium::index::NoIndices,
    pub model: SMatrix4<f32>,
}

fn darken(color: &mut SVector3<u8>, ammount: u8) {
    for i in 0..3 {
        color[i] = if ammount <= color[i] {
            color[i] - ammount
        } else {
            0
        }
    }
}

impl Model {
    pub fn new<F: glium::backend::Facade>(facade: &F, chunk: &world::Chunk) -> Model {
        let mut verts = Vec::new();

        for x in 0..world::CHUNK_SIZE as u8 {
            for y in 0..world::CHUNK_SIZE as u8 {
                for z in 0..world::CHUNK_SIZE as u8 {
                    let loc = point3(x, y, z);
                    let block = chunk.get_block_local(loc);
                    if !block.is_empty() {
                        let color32 = block.id.0;
                        let color = [(color32 & 0xFF) as u8,
                                     ((color32 >> 8) & 0xFF) as u8,
                                     ((color32 >> 16) & 0xFF) as u8];

                        if block.visibility.contains(world::VISIBLE_BOTTOM) {
                            Model::make_bottom(loc, color, &mut verts);
                        }
                        if block.visibility.contains(world::VISIBLE_TOP) {
                            Model::make_top(loc, color, &mut verts);
                        }
                        if block.visibility.contains(world::VISIBLE_FRONT) {
                            Model::make_front(loc, color, &mut verts);
                        }
                        if block.visibility.contains(world::VISIBLE_BACK) {
                            Model::make_back(loc, color, &mut verts);
                        }
                        if block.visibility.contains(world::VISIBLE_LEFT) {
                            Model::make_left(loc, color, &mut verts);
                        }
                        if block.visibility.contains(world::VISIBLE_RIGHT) {
                            Model::make_right(loc, color, &mut verts);
                        }
                    }
                }
            }
        }

        Model {
            vbo: glium::VertexBuffer::new(facade, &verts).unwrap(),
            ibo: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            model: Matrix4::from_translation(vec3(chunk.origin.x as f32,
                                                  chunk.origin.y as f32,
                                                  chunk.origin.z as f32))
                .into(),
        }
    }

    fn make_bottom(origin: Point3<u8>, color: SVector3<u8>, vert_out: &mut Vec<Vertex>) {
        let mut c = color;//[255, 255, 255];
        darken(&mut c, 30);
        let v = [vnew(point3(origin.x, origin.y, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c),

                 vnew(point3(origin.x, origin.y, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c)];

        vert_out.extend_from_slice(&v);
    }

    fn make_top(origin: Point3<u8>, color: SVector3<u8>, vert_out: &mut Vec<Vertex>) {
        let c = color;//[0, 0, 0];
        let v = [vnew(point3(origin.x, origin.y + 1, origin.z + 1), &c),
                 vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z), &c),

                 vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1), &c),
                 vnew(point3(origin.x + 1, origin.y + 1, origin.z), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z), &c)];

        vert_out.extend_from_slice(&v);
    }

    fn make_back(origin: Point3<u8>, color: SVector3<u8>, vert_out: &mut Vec<Vertex>) {
        let mut c = color;//[255, 0, 255];
        darken(&mut c, 10);
        let v = [vnew(point3(origin.x, origin.y, origin.z), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z), &c),

                 vnew(point3(origin.x, origin.y + 1, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y + 1, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z), &c)];

        vert_out.extend_from_slice(&v);
    }

    fn make_front(origin: Point3<u8>, color: SVector3<u8>, vert_out: &mut Vec<Vertex>) {
        let mut c = color;//[0, 255, 0];
        darken(&mut c, 10);
        let v = [vnew(point3(origin.x, origin.y, origin.z + 1), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z + 1), &c),

                 vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c),
                 vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z + 1), &c)];

        vert_out.extend_from_slice(&v);
    }

    fn make_left(origin: Point3<u8>, color: SVector3<u8>, vert_out: &mut Vec<Vertex>) {
        let mut c = color;//[0, 255, 255];
        darken(&mut c, 5);
        let v = [vnew(point3(origin.x, origin.y, origin.z), &c),
                 vnew(point3(origin.x, origin.y, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z), &c),

                 vnew(point3(origin.x, origin.y, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z), &c)];

        vert_out.extend_from_slice(&v);
    }

    fn make_right(origin: Point3<u8>, color: SVector3<u8>, vert_out: &mut Vec<Vertex>) {
        let mut c = color;//[255, 0, 0];
        darken(&mut c, 5);
        let v = [vnew(point3(origin.x + 1, origin.y, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y + 1, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c),

                 vnew(point3(origin.x + 1, origin.y + 1, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c)];

        vert_out.extend_from_slice(&v);
    }
}

fn vnew(position: Point3<u8>, color: &SVector3<u8>) -> Vertex {
    Vertex {
        position: position.into(),
        color: color.clone(),
    }
}
