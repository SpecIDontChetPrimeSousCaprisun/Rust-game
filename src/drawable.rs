use num_quaternion::{Quaternion, UnitQuaternion, Q32, Q64, UQ32, UQ64};
use glium;
use glium::Surface;
use crate::vector::*;

pub struct DrawInfo<V, I> where V: glium::vertex::Vertex, I: glium::index::Index {
    pub vb: glium::VertexBuffer<V>,
    pub ib: glium::IndexBuffer<I>,
    pub diffuse_texture: glium::texture::Texture2d,
    pub program: glium::Program,
    pub position: Vector,
    pub rotation: Vector,
}

impl<V: glium::vertex::Vertex, I: glium::index::Index> DrawInfo<V, I> {
    pub fn get_matrix (&self) -> [[f32; 4]; 4] {
        let uq = UnitQuaternion::from_euler_angles(self.rotation.x * 0.01745329, self.rotation.y * 0.01745329, self.rotation.z * 0.01745329);
        return view_matrix(self.position.as_3d_array(), uq.rotate_vector([0.0, 0.0, 1.0]), uq.rotate_vector([0.0, 1.0, 0.0]));
    }
}

pub struct WorldInfo<'a> {
    pub perspective: &'a[[f32; 4]; 4], 
    pub u_light: [f32; 3], 
    pub view: [[f32; 4]; 4],
}

pub trait Drawable<V, I> where V: glium::vertex::Vertex, I: glium::index::Index {
    fn on_draw(
        &self,
        draw_info: &DrawInfo<V, I>,
        mut world_info: WorldInfo,
        mut target: &mut glium::Frame
    ) {
        let uniforms = uniform! {
            matrix: draw_info.get_matrix(),
            u_light: world_info.u_light,
            perspective: *world_info.perspective,
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

        target.draw(&draw_info.vb, &draw_info.ib, &draw_info.program, &uniforms, &params).unwrap();
    }

    fn draw(&self, mut world_info: WorldInfo, mut target: &mut glium::Frame) {}
}
