use prelude::*;

use cgmath::{Deg, perspective, SquareMatrix};
use gfx;
use image;
use std::collections::hash_map::HashMap;

use graphics::model::Model;
use graphics::Camera;
use world::WorldPoint;

// If this is > 12 bytes, indexed drawing is has better space efficiency
gfx_vertex_struct!{
    Vertex {
        position: [u8; 4] = "position",
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

        block_texture: gfx::TextureSampler<[f32; 4]> = "block_texture",
    }
}

pub struct Renderer<R: gfx::Resources> {
    pso: gfx::pso::PipelineState<R, pipe::Meta>,
    models: HashMap<WorldPoint, Model<R>>,
    data: pipe::Data<R>,
    pub camera: Camera,
    pub projection: [[f32; 4]; 4],
}

// Borrowed from a gfx example
fn load_texture<R, F>(factory: &mut F,
                      data: &[u8])
                      -> Result<gfx::handle::ShaderResourceView<R, [f32; 4]>, String>
    where R: gfx::Resources,
          F: gfx::Factory<R>
{
    use std::io::Cursor;
    use gfx::texture as t;
    let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);
    let (_, view) = factory.create_texture_immutable_u8::<gfx::format::Srgba8>(kind, &[&img])
        .unwrap();
    Ok(view)
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

        let sampler_info = gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Scale,
                                                          gfx::texture::WrapMode::Clamp);

        let data = pipe::Data {
            vbo: factory.create_vertex_buffer(&[Vertex {
                                                    position: [0, 0, 0, 0],
                                                    uv: [U16Norm(0), U16Norm(0)],
                                                }]),
            out_color: color_target,
            out_depth_stencil: depth_stencil_target,

            model: Matrix4::identity().into(),
            view: Matrix4::identity().into(),
            projection: Matrix4::identity().into(),

            block_texture: (load_texture(factory,
                                         &include_bytes!("../../resources/textures/blocks.png")
                                              [..])
                                .unwrap(),
                            factory.create_sampler(sampler_info)),
        };

        Renderer {
            pso: pso,
            models: HashMap::new(),
            camera: Camera::new(point3(-1.0, 40.0, -1.0), 0.0, 0.0),
            data: data,
            projection: perspective(Deg(90.0), 1024 as f32 / 768 as f32, 0.1, 1000.0).into(),
        }
    }

    pub fn set_model(&mut self, origin: WorldPoint, models: Model<R>) {
        self.models.insert(origin, models);
    }

    pub fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        encoder.clear(&self.data.out_color, [0.0, 0.0, 0.0, 1.0]);
        encoder.clear_depth(&self.data.out_depth_stencil, 1.0);
        encoder.clear_stencil(&self.data.out_depth_stencil, 0);

        for model in &self.models {
            self.data.model = model.1.model;
            self.data.view = *self.camera.get_view_matrix();
            self.data.projection = self.projection;
            self.data.vbo = model.1.vbo.clone();

            encoder.draw(&model.1.slice, &self.pso, &self.data);
        }
    }
}
