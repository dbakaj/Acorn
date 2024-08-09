extern crate acorn;

struct AcornApplication {
    m_window: acorn::core::Window
}

impl AcornApplication {

    pub fn new(width: u32, height: u32, title: &str) -> Self {

        let window = acorn::core::Window::new(width, height, title);
        Self { m_window: window }
    }

    pub fn run(&mut self) {
        
        while !self.m_window.should_close() {
            self.m_window.update();
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.5, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
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
