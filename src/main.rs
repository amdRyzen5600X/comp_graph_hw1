#[macro_use]
extern crate glium;
use glium::Surface;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #4")
        .build(&event_loop);

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    let shape = vec![
        Vertex {
            position: [0.0, 1.0],
        },
        Vertex {
            position: [0.5, 0.5],
        },
        Vertex {
            position: [0.0, 0.0],
        },
        Vertex {
            position: [0.0, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5],
        },
        Vertex {
            position: [0.0, -1.0],
        },
        Vertex {
            position: [0.0, 0.0],
        },
        Vertex {
            position: [1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5],
        },
        Vertex {
            position: [0.0, 0.0],
        },
        Vertex {
            position: [-1.0, 0.0],
        },
        Vertex {
            position: [-0.5, 0.5],
        },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = 0.0;

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                glium::winit::event::WindowEvent::RedrawRequested => {
                    t += 0.12;
                    let x = t.sin();
                    let y = t.cos();

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);

                    let uniforms = uniform! {
                        matrix: [
                            [y, -x, 0.0, 0.0],
                            [x, y, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ]
                        // matrix: [
                        //     [1.0, 0.0, 0.0, 0.0],
                        //     [0.0, 1.0, 0.0, 0.0],
                        //     [0.0, 0.0, 1.0, 0.0],
                        //     [0.0, 0.0, 0.0, 1.0f32],
                        // ]
                    };

                    target
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();
                    target.finish().unwrap();
                }
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                }
                _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}
