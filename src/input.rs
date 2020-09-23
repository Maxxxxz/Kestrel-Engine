use glfw::{Action, Context, Key};

// Input States go here
pub struct InputState
{
    // Perhaps instead of keeping bools for each control
    // I could simply modify and access an array for
    // pressed buttons and held buttons, along with modifiers
    pub w_Active: bool,
    pub a_Active: bool,
    pub s_Active: bool,
    pub d_Active: bool,
}

impl InputState
{
    pub fn new() -> InputState
    {
        InputState
        {
            w_Active: false,
            a_Active: false,
            s_Active: false,
            d_Active: false,
        }
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