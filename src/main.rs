use glium::Surface;
mod teapot;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, normal, tex_coords);

#[macro_use]
extern crate glium;
fn main() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    window.set_cursor_grab(glium::winit::window::CursorGrabMode::Locked);
    window.set_cursor_visible(false);

    let image = image::load(std::io::Cursor::new(&include_bytes!("/home/chevre/.config/fastfetch/logo2.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    
    let shape = glium::vertex::VertexBuffer::new(&display, &[
        Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
    ]).unwrap();

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                        &teapot::INDICES).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;
        in vec2 tex_coords;

        out vec3 v_normal;
        out vec3 v_position;
        out vec2 v_tex_coords;

        uniform sampler2D tex;
        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
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
        in vec2 v_tex_coords;

        out vec4 color;

        uniform vec3 u_light;
        uniform sampler2D diffuse_texture;

        const vec3 specular_color = vec3(1.0, 1.0, 1.0);

        void main() {
            vec3 diffuse_color = texture(diffuse_texture, v_tex_coords).rgb;
            vec3 ambient_color = diffuse_color * 0.1;

            float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

            vec3 camera_dir = normalize(-v_position);
            vec3 half_direction = normalize(normalize(u_light) + camera_dir);
            float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

            color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut pitch = 0.0;
    let mut yaw = 90.0;
    let mut cam_pos = [0.0, 0.0, -3.0];
    let mut cam_up = [0.0, 1.0, 0.0];
    let mut cam_front = [0.0, 0.0, 1.0];
    let mut cam_vel = [0.0, 0.0, 0.0];
    let mut t: f32 = 0.0;
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
                        cam_pos[0] += cam_front[0] * cam_vel[2];
                        cam_pos[1] += cam_front[1] * cam_vel[2];
                        cam_pos[2] += cam_front[2] * cam_vel[2];

                        let mut cam_right = normal(&cross_product(&cam_front, &cam_up)); 

                        cam_pos[0] += cam_front[0] * cam_vel[2];
                        cam_pos[1] += cam_front[1] * cam_vel[2];
                        cam_pos[2] += cam_front[2] * cam_vel[2];

                        cam_pos[0] += cam_right[0] * cam_vel[0];
                        cam_pos[1] += cam_right[1] * cam_vel[0];
                        cam_pos[2] += cam_right[2] * cam_vel[0];

                        t += 0.02;

                        let x_off = t.sin() * 3.0;

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

                        let view = view_matrix(&cam_pos, &cam_front, &cam_up);

                        let uniforms = uniform! {
                            matrix: [
                                [1.0, 0.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [0.0, 0.0, 0.0, 1.0f32],
                            ],
                            u_light: [1.4, 0.4, 0.7f32],
                            perspective: perspective,
                            view: view,
                            tex: &texture,
                        };

                        let params = glium::DrawParameters {
                            depth: glium::Depth {
                                test: glium::draw_parameters::DepthTest::IfLess,
                                write: true,
                                .. Default::default()
                            },
                            .. Default::default()
                        };

                        target.draw(&shape, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
                        &uniforms,
                        &params).unwrap();
                        target.finish().unwrap();
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
}

fn cross_product(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0]
    ]
}

fn normal(v: &[f32; 3]) -> [f32; 3] {
    let len = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    let len = len.sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = normal(direction);

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn on_kb_inp(event: &glium::winit::event::WindowEvent, cam_vel: &[f32; 3]) -> [f32; 3]{
    let mut new_vel = [0.0; 3];

    new_vel[0] = cam_vel[0];
    new_vel[1] = cam_vel[1];
    new_vel[2] = cam_vel[2];

    match event {
        glium::winit::event::WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
            glium::winit::keyboard::PhysicalKey::Code(key_code) => match key_code {
                glium::winit::keyboard::KeyCode::KeyW => {
                    if event.state == glium::winit::event::ElementState::Pressed {
                        new_vel[2] = 1.0;
                    } else if new_vel[2] == 1.0 {
                        new_vel[2] = 0.0;
                    }
                },
                glium::winit::keyboard::KeyCode::KeyS => {
                    if event.state == glium::winit::event::ElementState::Pressed {
                        new_vel[2] = -1.0;
                    } else if new_vel[2] == -1.0 {
                        new_vel[2] = 0.0;
                    }
                },
                glium::winit::keyboard::KeyCode::KeyA => {
                    if event.state == glium::winit::event::ElementState::Pressed {
                        new_vel[0] = 1.0;
                    } else if new_vel[0] == 1.0 {
                        new_vel[0] = 0.0;
                    }
                },
                glium::winit::keyboard::KeyCode::KeyD => {
                    if event.state == glium::winit::event::ElementState::Pressed {
                        new_vel[0] = -1.0;
                    } else if new_vel[0] == -1.0 {
                        new_vel[0] = 0.0;
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

fn on_mouse_movement(event: &glium::winit::event::DeviceEvent, cam_front: &mut [f32; 3], yaw: f32, pitch: f32) -> (f32, f32, bool) {
    let mut new_yaw: f32 = 0.0;
    let mut new_pitch: f32 = 0.0;
    let mut goodEvent = false;

    match event {
        glium::winit::event::DeviceEvent::MouseMotion { delta } => {
            new_yaw = yaw + (-delta.0 as f32 / 1000.0);
            new_pitch = pitch + (-delta.1 as f32 / 1000.0);
            goodEvent = true;

            let mut direction: [f32; 3] = [0.0, 0.0, 0.0];

            direction[0] = yaw.cos() * pitch.cos();
            direction[1] = pitch.sin();
            direction[2] = yaw.sin() * pitch.cos();
            
            direction = normal(&direction);

            cam_front[0] = direction[0];
            cam_front[1] = direction[1];
            cam_front[2] = direction[2];
        },
        _ => ()
    };

    (new_yaw, new_pitch, goodEvent)
}
