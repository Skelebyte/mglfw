mod mglfw;

use mglfw::{core, input};

fn main() {
    let mut mglfw = core::Mglfw::new("Hello this is window", 300, 300);
    let mut input = input::Input::init();

    let esc = input.new("esc", input::KeyCode::Escape, input::Activation::Press);

    while mglfw.is_running() {
        mglfw.input_update();

        if input.is_bind_active(&mglfw, &esc) {
            mglfw.quit();
        }
    }
}