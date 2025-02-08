# MGLFW
A minimal wrapper for GLFW, written in rust.
## What is this?
MGLFW (or Minimal GLFW) is a easy to use wrapper for [glfw-rs](https://crates.io/crates/glfw). It aims to make using the library at minimum a little easier.
## Why?
Because I wanted to make using GLFW easier (for my self mostly). I find its just too difficult to set a simple window up with basic input.
## Examples
Example of a simple window with just the glfw-rs crate:
```rs
extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
```

Now here is the same example, but written using mglfw:
```rs
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
```

## Before you use
MGLFW is not ready for use in a proper project. If you need input and windows just use the glfw crate, or some other crate will also do the job. MGLFW is just a little side project I'll try to update every now and again.
MGLFW also may not be as performant as the standard crate, which might be something I'll try to solve later.

This project is pretty much just for fun. Use it if you want, but just be aware there's a good chance it'll just break.