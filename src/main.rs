extern crate glfw;

use std::time::{Duration};
use std::thread;
mod tick;
mod input;

use glfw::{Action, Context, Key};
use tick::{tickPhysics, tickEngine};
use input::{handle_window_event, InputState};

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
    window.make_current();

    // while !window.should_close() {

    // }

    let mut inpState = InputState::new();
    let mut quit = false;

    'game: loop
    {

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            quit = handle_window_event(&mut window, event, &mut inpState);
        }

        if quit
        {
            break 'game;
        }
    }

    phys.stop();
    eng.stop();
    
    println!("Exiting!");

    return;

}