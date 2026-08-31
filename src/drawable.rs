use glium;
use glium::Surface;
pub struct DrawInfo<V, I> where V: glium::vertex::Vertex, I: glium::index::Index {
    pub vb: glium::VertexBuffer<V>,
    pub ib: glium::IndexBuffer<I>,
    pub diffuse_texture: glium::texture::Texture2d,
    pub program: glium::Program,
}

pub struct WorldInfo {
    pub target: glium::Frame, 
    pub perspective: [[f32; 4]; 4], 
    pub u_light: [f32; 3], 
    pub view: [[f32; 4]; 4],
}

pub trait Drawable<V, I> where V: glium::vertex::Vertex, I: glium::index::Index {
    fn on_draw(
        &self,
        draw_info: &DrawInfo<V, I>,
        mut world_info: WorldInfo,
    ) {
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            u_light: world_info.u_light,
            perspective: world_info.perspective,
            view: world_info.view,
            diffuse_texture: &draw_info.diffuse_texture
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        world_info.target.draw(&draw_info.vb, &draw_info.ib, &draw_info.program, &uniforms, &params).unwrap();
        world_info.target.finish().unwrap();
    }

    fn draw(&self, mut world_info: WorldInfo) {}
}

pub struct TestObject<V, I> where V: glium::vertex::Vertex, I: glium::index::Index  {
   pub draw_info: DrawInfo<V, I>, 
}

impl<V: glium::vertex::Vertex, I: glium::index::Index> TestObject<V, I> {}

impl<V: glium::vertex::Vertex, I: glium::index::Index> Drawable<V, I> for TestObject<V, I> {
    fn draw(&self, mut world_info: WorldInfo) {
        self.on_draw(&self.draw_info, world_info);
    }
}
