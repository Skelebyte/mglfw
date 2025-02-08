mod mglfw;

use mglfw::{core, input};




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
    let mut input =  input::Input::init();

    let quit = input.new( "quit", input::KeyCode::Escape, input::Activation::Press);
    let w = input.new( "w", input::KeyCode::W, input::Activation::Press);
    
    while mglfw.is_running() {
        mglfw.input_update();

        if input.is_bind_active(&mglfw,&w) {
            println!("100");
        }
        if input.is_bind_active(&mglfw, &quit) {
            mglfw.quit();
        }
    }
    println!("quit");
}

