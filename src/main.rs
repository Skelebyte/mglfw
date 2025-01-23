mod mglfw;
mod input;

use mglfw::core;




// fn tick() {
//     let mut my_bind = Keybind::new(KeyCode::W, Activation::PRESS);

//     if Input::is_bind_active(, kb)
    
// }

// fn handle_window_event(mglfw: &mut core::Mglfw, event: glfw::WindowEvent) {
//     match event {
//         glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
//             mglfw.window.set_should_close(true)
//         }
//         _ => {}
//     }
// }

fn main() {
    let mut mglfw: core::Mglfw = core::Mglfw::new("hi mum!", 300, 300);
    let mut input =  mglfw::input::Input::init(&mut mglfw);
    // core::Mglfw::update(&mut mglfw, tick);

    input.new("quit", mglfw::input::KeyCode::Escape, mglfw::input::Activation::PRESS);

    while mglfw.is_running() {
        //mglfw.glfw.poll_events();
        if input.is_bind_active("quit") {
            
        }
    }
    println!("quit");
}

