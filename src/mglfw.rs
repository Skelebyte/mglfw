
#![allow(dead_code)]
pub mod core {
    extern crate glfw;

    use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};

    pub struct Mglfw {
        pub glfw: Glfw,
        pub window: PWindow,
        pub events: GlfwReceiver<(f64, WindowEvent)>
    }
    impl Mglfw {
        pub fn new(title: &str, w: u32, h: u32) -> Mglfw {
            let mut g = glfw::init(glfw::fail_on_errors).unwrap();
            let (mut w, e) = g.create_window(w, h, title, glfw::WindowMode::Windowed).expect("Failed to create GLFW window!");
            w.set_key_polling(true);
            w.make_current();
            Mglfw {
                glfw: g,
                window: w,
                events: e,
            }
    
        }

        pub fn is_running(mut self) -> bool {
            if self.window.should_close() == false {
                true
            } else {
                self.window.set_should_close(true);
                false
            }
        }

        pub fn quit(&mut self) {
            self.window.set_should_close(true);
        }
    
        pub fn update(&mut self, target: fn()) {
            while !self.window.should_close() {
                self.glfw.poll_events();
                let events: Vec<_> = glfw::flush_messages(&self.events).collect();
                for (_, event) in events {
                    self.input_update(event);
                }
                target();
            }
            
        }
    
        fn input_update(&mut self, we: WindowEvent) {
            match we {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => {}
            }
        }
    
    
    }
    
}


pub mod input {

    extern crate glfw;

    use std::ffi::c_int;

    use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};

    use super::core::Mglfw;
    
    #[derive(Clone, Copy)]
    pub enum Activation {
        RELEASE = 0,
        PRESS = 1,
        REPEAT = 2
    }
    
    
    #[derive(Clone, Copy)]
    #[repr(i32)]
    pub enum KeyCode {
        Space = 32,
        Apostrophe = 39,
        Comma = 44,
        Minus = 45,
        Period = 46,
        Slash = 47,
        Num0 = 48,
        Num1 = 49,
        Num2 = 50,
        Num3 = 51,
        Num4 = 52,
        Num5 = 53,
        Num6 = 54,
        Num7 = 55,
        Num8 = 56,
        Num9 = 57,
        Semicolon = 59,
        Equal = 61,
        A = 65,
        B = 66,
        C = 67,
        D = 68,
        E = 69,
        F = 70,
        G = 71,
        H = 72,
        I = 73,
        J = 74,
        K = 75,
        L = 76,
        M = 77,
        N = 78,
        O = 79,
        P = 80,
        Q = 81,
        R = 82,
        S = 83,
        T = 84,
        U = 85,
        V = 86,
        W = 87,
        X = 88,
        Y = 89,
        Z = 90,
        LeftBracket = 91,
        Backslash = 92,
        RightBracket = 93,
        GraveAccent = 96,
        World1 = 161,
        World2 = 162,
        Escape = 256,
        Enter = 257,
        Tab = 258,
        Backspace = 259,
        Insert = 260,
        Delete = 261,
        Right = 262,
        Left = 263,
        Down = 264,
        Up = 265,
        PageUp = 266,
        PageDown = 267,
        Home = 268,
        End = 269,
        CapsLock = 280,
        ScrollLock = 281,
        NumLock = 282,
        PrintScreen = 283,
        Pause = 284,
        F1 = 290,
        F2 = 291,
        F3 = 292,
        F4 = 293,
        F5 = 294,
        F6 = 295,
        F7 = 296,
        F8 = 297,
        F9 = 298,
        F10 = 299,
        F11 = 300,
        F12 = 301,
        F13 = 302,
        F14 = 303,
        F15 = 304,
        F16 = 305,
        F17 = 306,
        F18 = 307,
        F19 = 308,
        F20 = 309,
        F21 = 310,
        F22 = 311,
        F23 = 312,
        F24 = 313,
        F25 = 314,
        Kp0 = 320,
        Kp1 = 321,
        Kp2 = 322,
        Kp3 = 323,
        Kp4 = 324,
        Kp5 = 325,
        Kp6 = 326,
        Kp7 = 327,
        Kp8 = 328,
        Kp9 = 329,
        KpDecimal = 330,
        KpDivide = 331,
        KpMultiply = 332,
        KpSubtract = 333,
        KpAdd = 334,
        KpEnter = 335,
        KpEqual = 336,
        LeftShift = 340,
        LeftControl = 341,
        LeftAlt = 342,
        LeftSuper = 343,
        RightShift = 344,
        RightControl = 345,
        RightAlt = 346,
        RightSuper = 347,
        Menu = 348,
        Unknown = -1,
    }
    
    pub struct Keybind {
        pub name: String,
        pub keys: KeyCode,
        glfw_keys: glfw::Key,
        pub activation: Activation,
        active: bool
    }
    impl Keybind {

    }
    pub struct Input<'a> {
        pub mglfw: &'a Mglfw,
        pub keybinds: Vec<Keybind>
        // pub events: GlfwReceiver<(f64, WindowEvent)>
    }
    impl<'a> Input<'a> {
        pub fn init(mglfw: &Mglfw) -> Input {
            // let events: GlfwReceiver<(f64, WindowEvent)> = mglfw.events;
            Input {
                mglfw: mglfw,
                keybinds: Vec::new()
                // events: events
            }
        }
        pub fn new(&mut self, name: &str, k: KeyCode, a: Activation) {
            let new_keybind = Keybind {
                name: name.to_string(),
                keys: k,
                glfw_keys: unsafe { std::mem::transmute(k as i32) },
                activation: a,
                active: false
            };
            self.keybinds.push(new_keybind);
        }

        pub fn is_bind_active(&self, keybind_name: &str) -> bool {


            for kb in &self.keybinds {
                if kb.name.to_lowercase() != keybind_name.to_lowercase() {
                    return false;
                } else {
                    let _activ: Activation = kb.activation;
                    let key = kb.keys;
        
                    let events: Vec<_> = glfw::flush_messages(&self.mglfw.events).collect();
                    
                    for (_, event) in events {
                        match event {
                            glfw::WindowEvent::Key(key, _, _activ, _) => {
                                // kb.active = true;
                                return true;
                            }
                            _ => {}
                        }
                    }
                }

            }



            false
        }
    }

}
