
#![allow(dead_code)]
pub mod core {
    extern crate glfw;

    use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};


    pub struct Mglfw {
        pub glfw: Glfw,
        pub window: PWindow,
        pub events: GlfwReceiver<(f64, WindowEvent)>,
        pub i_events: Vec<glfw::WindowEvent>
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
                i_events: Vec::new()
            }
    
        }

        pub fn is_running(&mut self) -> bool {
            !self.window.should_close()
        }

        pub fn quit(&mut self) {
            self.window.set_should_close(true);
        }
    
    
        pub fn input_update(&mut self) {
            self.glfw.poll_events();
            let mut p_events = Vec::new();
            for (_, event) in glfw::flush_messages(&self.events) {
                p_events.push(event);
            }
            self.i_events = p_events;
        }
    
    
    }
    
}


pub mod input {

    extern crate glfw;





    use super::core::Mglfw;
    
    #[derive(Clone, Copy)]
    pub enum Activation {
        Release = 0,
        Press = 1,
        Repeat = 2
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
        pub glfw_keys: glfw::Key,
        pub activation: Activation,
        pub glfw_activation: glfw::Action,
        active: bool
    }
    impl Keybind {

    }
    pub struct Input {
        
        pub keybinds: Vec<Keybind>
        // pub events: GlfwReceiver<(f64, WindowEvent)>
    }
    impl Input {
        pub fn init() -> Input {
            // let events: GlfwReceiver<(f64, WindowEvent)> = mglfw.events;
            Input {
                keybinds: Vec::new()
                // events: events
            }
        }
        pub fn new(&mut self, name: &str, k: KeyCode, a: Activation) -> Keybind {
            let new_keybind = Keybind {
                name: name.to_string(),
                keys: k,
                glfw_keys: unsafe { std::mem::transmute(k as i32) },
                activation: a,
                glfw_activation: unsafe { std::mem::transmute(a as i32) },
                active: false
            };
            //self.keybinds.push(new_keybind);
            new_keybind
        }

        pub fn is_bind_active(&self, mglfw: &Mglfw, keybind: &Keybind) -> bool { //keybind_name: &str

            for event in mglfw.i_events.iter() {
                match event {
                    glfw::WindowEvent::Key(key, _, activation, _) if *key == keybind.glfw_keys && *activation == keybind.glfw_activation => {

                        return true;
                    }
                    _ => {}
                }
            }
            false
        }
    }

}
