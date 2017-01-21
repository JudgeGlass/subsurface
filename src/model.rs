use glium;
use cgmath::{Vector3, Matrix4};

// If this is > 12 bytes, indexed drawing is has better space efficiency
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [u8; 3],
    color: [u8; 3],
}

implement_vertex!(Vertex, position normalize(false), color normalize(true));

pub struct Model {
    pub vbo: glium::VertexBuffer<Vertex>,
    pub ibo: glium::index::NoIndices,
    pub model: [[f32; 4]; 4],
}

type RVec3 = [u8; 3];

impl Model {
    pub fn new<F: glium::backend::Facade>(facade: &F, translate: Vector3<f32>) -> Model {
        let mut verts = Vec::new();

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    Model::make_bottom(&[i * 2, j * 2, k * 2], &mut verts);
                    Model::make_top(&[i * 2, j * 2, k * 2], &mut verts);
                    Model::make_front(&[i * 2, j * 2, k * 2], &mut verts);
                    Model::make_back(&[i * 2, j * 2, k * 2], &mut verts);
                    Model::make_left(&[i * 2, j * 2, k * 2], &mut verts);
                    Model::make_right(&[i * 2, j * 2, k * 2], &mut verts);
                }
            }
        }

        Model {
            vbo: glium::VertexBuffer::new(facade, &verts).unwrap(),
            ibo: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            model: Matrix4::from_translation(translate).into(),
        }
    }

    fn make_bottom(origin: &RVec3, vert_out: &mut Vec<Vertex>) {
        vert_out.extend_from_slice(&[Vertex::new(&[origin[0], origin[1], origin[2] + 1],
                                                 &[255, 255, 255]),
                                     Vertex::new(&[origin[0], origin[1], origin[2]],
                                                 &[255, 255, 255]),
                                     Vertex::new(&[origin[0] + 1, origin[1], origin[2] + 1],
                                                 &[255, 255, 255])]);

        vert_out.extend_from_slice(&[Vertex::new(&[origin[0], origin[1], origin[2]],
                                                 &[255, 255, 255]),
                                     Vertex::new(&[origin[0] + 1, origin[1], origin[2]],
                                                 &[255, 255, 255]),
                                     Vertex::new(&[origin[0] + 1, origin[1], origin[2] + 1],
                                                 &[255, 255, 255])]);
    }

    fn make_top(origin: &RVec3, vert_out: &mut Vec<Vertex>) {
        vert_out.extend_from_slice(&[Vertex::new(&[origin[0], origin[1]+1, origin[2] + 1],
                                                 &[0, 0, 0]),
                                     Vertex::new(&[origin[0]+1, origin[1]+1, origin[2]+1],
                                                 &[0, 0, 0]),
                                     Vertex::new(&[origin[0], origin[1]+1, origin[2]],
                                                 &[0, 0, 0])]);

        vert_out.extend_from_slice(&[Vertex::new(&[origin[0]+1, origin[1]+1, origin[2]+1],
                                                 &[0, 0, 0]),
                                     Vertex::new(&[origin[0] + 1, origin[1]+1, origin[2]],
                                                 &[0, 0, 0]),
                                     Vertex::new(&[origin[0], origin[1]+1, origin[2]],
                                                 &[0, 0, 0])]);
    }

    fn make_back(origin: &RVec3, vert_out: &mut Vec<Vertex>) {
        vert_out.extend_from_slice(&[Vertex::new(&[origin[0], origin[1], origin[2]],
                                                 &[255, 0, 255]),
                                     Vertex::new(&[origin[0], origin[1]+1, origin[2]],
                                                 &[255, 0, 255]),
                                     Vertex::new(&[origin[0]+1, origin[1], origin[2]],
                                                 &[255, 0, 255])]);

        vert_out.extend_from_slice(&[Vertex::new(&[origin[0], origin[1]+1, origin[2]],
                                                 &[255, 0, 255]),
                                     Vertex::new(&[origin[0]+1, origin[1]+1, origin[2]],
                                                 &[255, 0, 255]),
                                     Vertex::new(&[origin[0]+1, origin[1], origin[2]],
                                                 &[255, 0, 255])]);
    }

    fn make_front(origin: &RVec3, vert_out: &mut Vec<Vertex>) {
        vert_out.extend_from_slice(&[Vertex::new(&[origin[0], origin[1], origin[2]+1],
                                                 &[0, 255, 0]),
                                     Vertex::new(&[origin[0]+1, origin[1], origin[2]+1],
                                                 &[0, 255, 0]),
                                     Vertex::new(&[origin[0], origin[1]+1, origin[2]+1],
                                                 &[0, 255, 0])]);

        vert_out.extend_from_slice(&[Vertex::new(&[origin[0]+1, origin[1], origin[2]+1],
                                                 &[0, 255, 0]),
                                     Vertex::new(&[origin[0]+1, origin[1]+1, origin[2]+1],
                                                 &[0, 255, 0]),
                                     Vertex::new(&[origin[0], origin[1]+1, origin[2]+1],
                                                 &[0, 255, 0])]);
    }

    fn make_left(origin: &RVec3, vert_out: &mut Vec<Vertex>) {
        vert_out.extend_from_slice(&[Vertex::new(&[origin[0], origin[1], origin[2]],
                                                 &[0, 255, 255]),
                                     Vertex::new(&[origin[0], origin[1], origin[2]+1],
                                                 &[0, 255, 255]),
                                     Vertex::new(&[origin[0], origin[1]+1, origin[2]],
                                                 &[0, 255, 255])]);

        vert_out.extend_from_slice(&[Vertex::new(&[origin[0], origin[1], origin[2]+1],
                                                 &[0, 255, 255]),
                                     Vertex::new(&[origin[0], origin[1]+1, origin[2]+1],
                                                 &[0, 255, 255]),
                                     Vertex::new(&[origin[0], origin[1]+1, origin[2]],
                                                 &[0, 255, 255])]);
    }

    fn make_right(origin: &RVec3, vert_out: &mut Vec<Vertex>) {
        vert_out.extend_from_slice(&[Vertex::new(&[origin[0]+1, origin[1], origin[2]],
                                                 &[255, 0, 0]),
                                     Vertex::new(&[origin[0]+1, origin[1]+1, origin[2]],
                                                 &[255, 0, 0]),
                                     Vertex::new(&[origin[0]+1, origin[1], origin[2]+1],
                                                 &[255, 0, 0])]);

        vert_out.extend_from_slice(&[Vertex::new(&[origin[0]+1, origin[1]+1, origin[2]],
                                                 &[255, 0, 0]),
                                     Vertex::new(&[origin[0]+1, origin[1]+1, origin[2]+1],
                                                 &[255, 0, 0]),
                                     Vertex::new(&[origin[0]+1, origin[1], origin[2]+1],
                                                 &[255, 0, 0])]);
    }

}

impl Vertex {
    fn new(position: &RVec3, color: &RVec3) -> Vertex {
        Vertex {
            position: position.clone(),
            color: color.clone(),
        }
    }
}
