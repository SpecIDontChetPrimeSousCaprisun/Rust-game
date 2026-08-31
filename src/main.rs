use glium::Surface;
use glium::Program;
use glium::backend::Facade;
use glium::backend::glutin::SimpleWindowBuilder;
use glium::uniform;
use glium::winit::event::{Event, WindowEvent};
use glium::winit::event_loop::{ControlFlow, EventLoop};
use obj::{Obj, load_obj, TexturedVertex};
mod vector;
mod drawable;
use drawable::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, normal, tex_coords);

#[macro_use]
extern crate glium;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);
    window.set_cursor_grab(glium::winit::window::CursorGrabMode::Locked);
    window.set_cursor_visible(false);

    let image = image::load(std::io::Cursor::new(&include_bytes!("/home/chevre/.config/fastfetch/logo2.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let input = include_bytes!("../Monkey.obj");
    let obj: Obj<TexturedVertex, u16> = load_obj(&input[..])?;

    let vb = obj.vertex_buffer(display.get_context())?;
    let ib = obj.index_buffer(display.get_context())?;

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;
        in vec3 texture;

        out vec3 v_normal;
        out vec3 v_position;
        out vec3 v_tex_coords;

        uniform sampler2D tex;
        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 matrix;

        void main() {
            v_tex_coords = texture;
            mat4 modelview = view * matrix;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            v_position = gl_Position.xyz / gl_Position.w;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 v_normal;
        in vec3 v_position;
        in vec3 v_tex_coords;

        out vec4 color;

        uniform vec3 u_light;
        uniform sampler2D diffuse_texture;

        const vec3 specular_color = vec3(1.0, 1.0, 1.0);

        void main() {
            vec3 diffuse_color = texture(diffuse_texture, vec2(v_tex_coords.x, v_tex_coords.y)).rgb;
            vec3 ambient_color = diffuse_color * 0.1;

            float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

            vec3 camera_dir = normalize(-v_position);
            vec3 half_direction = normalize(normalize(u_light) + camera_dir);
            float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

            color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap(); 
    let obj = TestObject{
        draw_info: DrawInfo {
            vb: vb,
            ib: ib,
            diffuse_texture: texture,
            program: program,
            position: vector::new_vector(&[0.0, 0.0, 0.0]),
            direction: vector::new_vector(&[0.0, 0.0, -1.0]),
            up: vector::new_vector(&[0.0, 1.0, 0.0]),
        },
    };

    let mut pitch = 0.0;
    let mut yaw = 90.0;
    let mut cam_pos = vector::new_vector(&[0.0, 0.0, -3.0]);
    let mut cam_up = vector::new_vector(&[0.0, 1.0, 0.0]);
    let mut cam_front = vector::new_vector(&[0.0, 0.0, 1.0]);
    let mut cam_vel = vector::new_vector(&[0.0, 0.0, 0.0]);
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::DeviceEvent { event, .. } => {
                let new_values = on_mouse_movement(&event, &mut cam_front, yaw, pitch);

                if new_values.2 {
                    yaw = new_values.0;
                    pitch = new_values.1;
                }
            },
            glium::winit::event::Event::WindowEvent { event, .. } => {
                cam_vel = on_kb_inp(&event, &cam_vel);

                match event {
                    glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    glium::winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    },
                    glium::winit::event::WindowEvent::RedrawRequested => {
                        let mut cam_right = cam_front.cross_product(&cam_up).normal(); 

                        let mut f_add = vector::Vector::clone(&cam_front);
                        f_add.mult(&vector::new_vector(&[cam_vel.z; 3]));
                        cam_pos.add(&f_add);

                        let mut r_add = vector::Vector::clone(&cam_right);
                        r_add.mult(&vector::new_vector(&[cam_vel.x; 3]));
                        cam_pos.add(&r_add);

                        let mut target = display.draw();
                        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
                        let perspective = {
                            let (width, height) = target.get_dimensions();
                            let aspect_ratio = height as f32 / width as f32;

                            let fov: f32 = 3.141592 / 3.0;
                            let zfar = 1024.0;
                            let znear = 0.1;

                            let f = 1.0 / (fov / 2.0).tan();

                            [
                                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                                [         0.0         ,     f ,              0.0              ,   0.0],
                                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                            ]
                        };

                        let view = vector::Vector::get_view_matrix(&cam_pos, &cam_front, &cam_up);

                        obj.draw(
                            WorldInfo {
                                target: target,
                                perspective: perspective,
                                u_light: [0.5, 0.5, 0.5f32],
                                view: view
                            }
                        ); 
                    },
                    _ => (),
                }

            },
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });

    Ok(())
}

fn on_kb_inp(event: &glium::winit::event::WindowEvent, cam_vel: &vector::Vector) -> vector::Vector {
    let mut new_vel = vector::Vector::clone(cam_vel);

    match event {
        glium::winit::event::WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
            glium::winit::keyboard::PhysicalKey::Code(key_code) => match key_code {
                glium::winit::keyboard::KeyCode::KeyW => {
                    if event.state == glium::winit::event::ElementState::Pressed {
                        new_vel.z = 1.0;
                    } else if new_vel.z == 1.0 {
                        new_vel.z = 0.0;
                    }
                },
                glium::winit::keyboard::KeyCode::KeyS => {
                    if event.state == glium::winit::event::ElementState::Pressed {
                        new_vel.z = -1.0;
                    } else if new_vel.z == -1.0 {
                        new_vel.z = 0.0;
                    }
                },
                glium::winit::keyboard::KeyCode::KeyA => {
                    if event.state == glium::winit::event::ElementState::Pressed {
                        new_vel.x = 1.0;
                    } else if new_vel.x == 1.0 {
                        new_vel.x = 0.0;
                    }
                },
                glium::winit::keyboard::KeyCode::KeyD => {
                    if event.state == glium::winit::event::ElementState::Pressed {
                        new_vel.x = -1.0;
                    } else if new_vel.x == -1.0 {
                        new_vel.x = 0.0;
                    }
                },
                _ => (),
            },
            _ => (),
        },
        _ => (),
    };

    new_vel
}

fn on_mouse_movement(event: &glium::winit::event::DeviceEvent, cam_front: &mut vector::Vector, yaw: f32, pitch: f32) -> (f32, f32, bool) {
    let mut new_yaw: f32 = 0.0;
    let mut new_pitch: f32 = 0.0;
    let mut goodEvent = false;

    match event {
        glium::winit::event::DeviceEvent::MouseMotion { delta } => {
            new_yaw = yaw + (-delta.0 as f32 / 1000.0);
            new_pitch = pitch + (-delta.1 as f32 / 1000.0);
            goodEvent = true;

            let mut direction = vector::new_vector(&[0.0; 3]);

            direction.x = yaw.cos() * pitch.cos();
            direction.y = pitch.sin();
            direction.z = yaw.sin() * pitch.cos();
            
            direction = direction.normal();

            cam_front.replace(&direction);
        },
        _ => ()
    };

    (new_yaw, new_pitch, goodEvent)
}
