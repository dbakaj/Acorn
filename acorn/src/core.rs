extern crate glfw;
use self::glfw::{Context, Key, Action};
extern crate gl;

pub struct Window {
    m_window: glfw::PWindow,
    m_events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    m_glfw: glfw::Glfw
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);

        gl::load_with(|s| window.get_proc_address(s));

        Self { m_window: window, m_events: events, m_glfw: glfw }
    }

    pub fn update(&mut self) -> () {
        self.m_window.swap_buffers();
        self.m_glfw.poll_events();

        for(_, event) in glfw::flush_messages(&self.m_events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.m_window.set_should_close(true);
                },

                _ => {}
            }
        }
    }

    pub fn should_close(&self) -> bool {
        self.m_window.should_close()
    }
}

const VERTSOURCE: &str =r#"
    #version 330 core
    layout (location = 0) in vec3 pos;
    
    void main() 
    {
        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    }
"#;

const FRAGSOURCE: &str = r#"
    #version 330 core
    out vec4 color;
    
    void main()
    {
        color = vec4(1.0, 1.0, 0.0, 1.0);
    }
"#;

pub struct Shader {
    m_shader_program: u32
}

impl Shader {
    pub fn new(vertex_source: &str, fragment_source: &str) -> Self {
        let shader_program = unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let vert_source = std::ffi::CString::new(vertex_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &vert_source.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex_shader);

            let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let frag_source = std::ffi::CString::new(fragment_source.as_bytes()).unwrap();
            gl::ShaderSource(frag_shader, 1, &frag_source.as_ptr(), std::ptr::null());
            gl::CompileShader(frag_shader);

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, frag_shader);
            gl::LinkProgram(shader_program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(frag_shader);
            
            shader_program
        };

        Self { m_shader_program: shader_program }
    }

    pub fn default() -> Shader {
        return Shader::new(VERTSOURCE, FRAGSOURCE);
    }

    pub fn bind(&self) -> () {
        unsafe { gl::UseProgram(self.m_shader_program); }
    }
}