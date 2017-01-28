use prelude::*;

use cgmath::{Deg, perspective, SquareMatrix};
use gfx;
use gfx::format::U16Norm;

use graphics::model::Model;
use graphics::Camera;

// If this is > 12 bytes, indexed drawing is has better space efficiency
gfx_vertex_struct!{
    Vertex {
        position: [u8; 4] = "position",
        color: Color = "color",
        uv: [U16Norm; 2] = "uv",
    }
}

gfx_pipeline!{
    pipe {
        vbo: gfx::VertexBuffer<Vertex> = (),
        out_color: gfx::RenderTarget<gfx::format::Srgba8> = "ocolor",
        out_depth_stencil: gfx::DepthTarget<gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,

        model: gfx::Global<[[f32; 4]; 4]> = "model",
        view: gfx::Global<[[f32; 4]; 4]> = "view",
        projection: gfx::Global<[[f32; 4]; 4]> = "projection",
    }
}

pub struct Renderer<R: gfx::Resources> {
    pso: gfx::pso::PipelineState<R, pipe::Meta>,
    models: Vec<Model<R>>,
    data: pipe::Data<R>,
    pub camera: Camera,
}


impl<R: gfx::Resources> Renderer<R> {
    pub fn new<F>(factory: &mut F,
                  color_target: gfx::handle::RenderTargetView<R, gfx::format::Srgba8>,
                  depth_stencil_target: gfx::handle::DepthStencilView<R,
                                                                      (gfx::format::D24_S8,
                                                                       gfx::format::Unorm)>)
                  -> Renderer<R>
        where F: gfx::traits::FactoryExt<R>
    {
        let vertex_shader_src = include_bytes!("../../resources/vertex.glsl");
        let fragment_shader_src = include_bytes!("../../resources/fragment.glsl");
        let program = factory.link_program(vertex_shader_src, fragment_shader_src).unwrap();
        let mut rasterizer = gfx::state::Rasterizer::new_fill();
        rasterizer.front_face = gfx::state::FrontFace::CounterClockwise;
        rasterizer.cull_face = gfx::state::CullFace::Back;
        let pso = factory.create_pipeline_from_program(&program,
                                          gfx::Primitive::TriangleList,
                                          rasterizer,
                                          pipe::new())
            .unwrap();

        let data = pipe::Data {
            vbo: factory.create_vertex_buffer(&[Vertex {
                                                    position: [0, 0, 0, 0],
                                                    color: [U8Norm(0), U8Norm(0), U8Norm(0),
                                                            U8Norm(0)],
                                                    uv: [U16Norm(0), U16Norm(0)],
                                                }]),
            out_color: color_target,
            out_depth_stencil: depth_stencil_target,

            model: Matrix4::identity().into(),
            view: Matrix4::identity().into(),
            projection: Matrix4::identity().into(),
        };

        Renderer {
            pso: pso,
            models: Vec::new(),
            camera: Camera::new(point3(-1.0, 7.0, -1.0), 0.0, 0.0),
            data: data,
        }
    }

    pub fn add_models(&mut self, mut models: Vec<Model<R>>) {
        self.models.append(&mut models);
    }

    pub fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        let (width, height) = (1024, 768);
        let projection = perspective(Deg(90.0), width as f32 / height as f32, 0.1, 1000.0).into();

        encoder.clear(&self.data.out_color, [0.0, 0.0, 0.0, 1.0]);
        encoder.clear_depth(&self.data.out_depth_stencil, 1.0);
        encoder.clear_stencil(&self.data.out_depth_stencil, 0);

        for model in self.models.iter() {
            self.data.model = model.model;
            self.data.view = *self.camera.get_view_matrix();
            self.data.projection = projection;
            self.data.vbo = model.vbo.clone();

            encoder.draw(&model.slice, &self.pso, &self.data);
        }
    }
}
