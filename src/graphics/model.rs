use glium;

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

impl Model {
    pub fn new<F: glium::backend::Facade>(facade: &F, translate: Vector3<f32>) -> Model {
        let mut verts = Vec::new();

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    Model::make_bottom(point3(i * 2, j * 2, k * 2), &mut verts);
                    Model::make_top(point3(i * 2, j * 2, k * 2), &mut verts);
                    Model::make_front(point3(i * 2, j * 2, k * 2), &mut verts);
                    Model::make_back(point3(i * 2, j * 2, k * 2), &mut verts);
                    Model::make_left(point3(i * 2, j * 2, k * 2), &mut verts);
                    Model::make_right(point3(i * 2, j * 2, k * 2), &mut verts);
                }
            }
        }

        Model {
            vbo: glium::VertexBuffer::new(facade, &verts).unwrap(),
            ibo: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            model: Matrix4::from_translation(translate).into(),
        }
    }

    fn make_bottom(origin: Point3<u8>, vert_out: &mut Vec<Vertex>) {
        let c = [255, 255, 255];
        let v = [vnew(point3(origin.x, origin.y, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c),

                 vnew(point3(origin.x, origin.y, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c)];

        vert_out.extend_from_slice(&v);
    }

    fn make_top(origin: Point3<u8>, vert_out: &mut Vec<Vertex>) {
        let c = [0, 0, 0];
        let v = [vnew(point3(origin.x, origin.y + 1, origin.z + 1), &c),
                 vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z), &c),

                 vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1), &c),
                 vnew(point3(origin.x + 1, origin.y + 1, origin.z), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z), &c)];

        vert_out.extend_from_slice(&v);
    }

    fn make_back(origin: Point3<u8>, vert_out: &mut Vec<Vertex>) {
        let c = [255, 0, 255];
        let v = [vnew(point3(origin.x, origin.y, origin.z), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z), &c),

                 vnew(point3(origin.x, origin.y + 1, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y + 1, origin.z), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z), &c)];

        vert_out.extend_from_slice(&v);
    }

    fn make_front(origin: Point3<u8>, vert_out: &mut Vec<Vertex>) {
        let c = [0, 255, 0];
        let v = [vnew(point3(origin.x, origin.y, origin.z + 1), &c),
                 vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z + 1), &c),

                 vnew(point3(origin.x + 1, origin.y, origin.z + 1), &c),
                 vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z + 1), &c)];

        vert_out.extend_from_slice(&v);
    }

    fn make_left(origin: Point3<u8>, vert_out: &mut Vec<Vertex>) {
        let c = [0, 255, 255];
        let v = [vnew(point3(origin.x, origin.y, origin.z), &c),
                 vnew(point3(origin.x, origin.y, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z), &c),

                 vnew(point3(origin.x, origin.y, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z + 1), &c),
                 vnew(point3(origin.x, origin.y + 1, origin.z), &c)];

        vert_out.extend_from_slice(&v);
    }

    fn make_right(origin: Point3<u8>, vert_out: &mut Vec<Vertex>) {
        let c = [255, 0, 0];
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
