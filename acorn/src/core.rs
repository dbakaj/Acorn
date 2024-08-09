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