use glfw::{Action, Context, Key};

// create consts for each key to properly modify the inputstate unsigned integers

// Input States go here
pub struct InputState
{
    // Perhaps instead of keeping bools for each control
    // I could simply modify and access an array for
    // pressed buttons and held buttons, along with modifiers

    // What other key categories should I keep track of?

    pub standard_keys_press: u64,
    pub standard_keys_held: u64,
    pub modifier_keys_press: u32,
    pub modifier_keys_held: u32,
    pub mouse_button_press: u16,
    pub mouse_button_held: u16,

    pub w_Active: bool,
    pub a_Active: bool,
    pub s_Active: bool,
    pub d_Active: bool,
    pub space_Active: bool,
}

impl InputState
{
    pub fn new() -> InputState
    {
        InputState
        {
            standard_keys_press: 0,
            standard_keys_held: 0,
            modifier_keys_press: 0,
            modifier_keys_held: 0,
            mouse_button_press: 0,
            mouse_button_held: 0,
            w_Active: false,
            a_Active: false,
            s_Active: false,
            d_Active: false,
            space_Active: false,
        }
    }

    pub fn isStandardPressed(&mut self, key: u64) -> bool
    {
        return self.standard_keys_press & (1 << key) != 0;
    }

    pub fn isStandardHeld(&mut self, key: u64) -> bool
    {
        return self.standard_keys_held & (1 << key) != 0;
    }

    pub fn isModifierPressed(&mut self, key: u32) -> bool
    {
        return self.modifier_keys_press & (1 << key) != 0;
    }

    pub fn isModifierHeld(&mut self, key: u32) -> bool
    {
        return self.modifier_keys_held & (1 << key) != 0;
    }

    pub fn isMouseButtonPressed(&mut self, key: u16) -> bool
    {
        return self.mouse_button_press & (1 << key) != 0;
    }
    
    pub fn isMouseButtonHeld(&mut self, key: u16) -> bool
    {
        return self.mouse_button_held & (1 << key) != 0;
    }
}

pub fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, inState: &mut InputState) -> bool
{
    match event
    {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) =>
        {
            window.set_should_close(true);
            return window.should_close();
        }
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) =>
        {
            println!("W has been pressed!")
        }
        glfw::WindowEvent::Key(Key::W, _, Action::Repeat, _) =>
        {
            println!("W is being held!")
        }
        glfw::WindowEvent::Key(Key::W, _, Action::Release, _) =>
        {
            println!("W has been released!")
        }
        glfw::WindowEvent::Key(Key::A, _, Action::Press, _) =>
        {
            println!("A has been pressed!")
        }
        glfw::WindowEvent::Key(Key::A, _, Action::Repeat, _) =>
        {
            println!("A is being held!")
        }
        glfw::WindowEvent::Key(Key::A, _, Action::Release, _) =>
        {
            println!("A has been released!")
        }
        glfw::WindowEvent::Key(Key::S, _, Action::Press, _) =>
        {
            println!("S has been pressed!")
        }
        glfw::WindowEvent::Key(Key::S, _, Action::Repeat, _) =>
        {
            println!("S is being held!")
        }
        glfw::WindowEvent::Key(Key::S, _, Action::Release, _) =>
        {
            println!("S has been released!")
        }
        glfw::WindowEvent::Key(Key::D, _, Action::Press, _) =>
        {
            println!("D has been pressed!")
        }
        glfw::WindowEvent::Key(Key::D, _, Action::Repeat, _) =>
        {
            println!("D is being held!")
        }
        glfw::WindowEvent::Key(Key::D, _, Action::Release, _) =>
        {
            println!("D has been released!")
        }
        _ => {}
    }
    return false;
}