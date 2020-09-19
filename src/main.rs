use std::time::{Duration};
use std::thread;
mod tick;

pub use tick::{tickPhysics, tickEngine};

fn main()
{

    let mut phys = tickPhysics::new();
    let mut eng = tickEngine::new();
    phys.start();
    eng.start();
    thread::sleep(Duration::from_secs(2));
    phys.stop();
    eng.stop();

    println!("Exiting!");

    return;

}
