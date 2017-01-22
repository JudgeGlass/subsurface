use glium;
use dot_vox;

use prelude::*;

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
    pub fn new<F: glium::backend::Facade>(facade: &F, translate: Vector3<f32>) -> Model {
        let mut verts = Vec::new();

        let data = dot_vox::load("resources/menger.vox").unwrap();

        for model in data.models.iter() {
            for voxel in model.voxels.iter() {
                let loc = point3(voxel.y, voxel.z, voxel.x);
                let color32 = data.pallete[voxel.i as usize];
                let color = [(color32 & 0xFF) as u8, ((color32>>8) & 0xFF) as u8, ((color32>>16) & 0xFF) as u8];

                Model::make_bottom(loc, color, &mut verts);
                Model::make_top(loc, color, &mut verts);
                Model::make_front(loc, color, &mut verts);
                Model::make_back(loc, color, &mut verts);
                Model::make_left(loc, color, &mut verts);
                Model::make_right(loc, color, &mut verts);
            }
        }

        Model {
            vbo: glium::VertexBuffer::new(facade, &verts).unwrap(),
            ibo: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            model: Matrix4::from_translation(translate).into(),
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
