extern crate acorn;

struct AcornApplication {
    m_window: acorn::core::Window,
    m_shader: acorn::core::Shader
}

impl AcornApplication {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let window = acorn::core::Window::new(width, height, title);
        let shader = acorn::core::Shader::default();

        Self { m_window: window, m_shader: shader }
    }

    pub fn run(&mut self) {
        let vao = unsafe {
            let vertices: [f32; 9] = [
                -0.5, -0.5, 0.0,
                 0.5, -0.5, 0.0,
                 0.0,  0.5, 0.0
            ];

            let mut vbo = 0;
            let mut vao = 0;

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &vertices[0] as *const f32 as *const std::os::raw::c_void,
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3* std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            vao
        };

        while !self.m_window.should_close() {
            self.m_window.update();

            unsafe {
                gl::ClearColor(0.3, 0.5, 0.7, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                self.m_shader.bind();
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
        }
    }
}

impl Drop for AcornApplication {
    fn drop(&mut self) {
        println!("Application dropped!");
    }
}

fn main() {
    let mut app = AcornApplication::new(640, 480, "Acorn Application");
    app.run();
}
