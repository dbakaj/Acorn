extern crate acorn;

fn main() {

    let mut window = acorn::core::Window::new(640, 480, "Test");

    while !window.should_close() {
        window.update();

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
