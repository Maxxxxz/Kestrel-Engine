extern crate glfw;

use std::time::{Duration};
use std::thread;
mod tick;
mod input;

use glfw::{Action, Context, Key};
use tick::{tickPhysics, tickEngine};
use input::{handleWindowEvent, InputState};

fn main()
{

    let mut phys = tickPhysics::new();
    let mut eng = tickEngine::new();
    phys.start();
    eng.start();
    
    // Temporary code: https://github.com/PistonDevelopers/glfw-rs#using-glfw-rs

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = 
    glfw.create_window(750, 750, "Kestrel Engine", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.make_current();

    

    // while !window.should_close() {

    // }

    let mut inpState = InputState::new();
    let mut quit = false;

    'game: loop
    {

        // Get cursor position
        // let (xpos, ypos) = window.get_cursor_pos();
        // println!("xpos: {} | ypos: {}", xpos, ypos);
        
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            quit = handleWindowEvent(&mut window, event, &mut inpState);
        }

        // This line lets me quit for reasons other than
        // the window needing to close
        quit = quit || window.should_close();
        
        if quit
        {
            break 'game;
        }
    }

    window.close();
    phys.stop();
    eng.stop();
    
    println!("Exiting!");

    return;

}