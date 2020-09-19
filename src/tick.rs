use std::thread;
use std::time::{Instant, Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};



////////////////////////////////////////////////
// Important consts

// Updates per second
const PHYS_TICK: u64 = 20;
const TICK_TIME: u64 = 40;

const ONE_SECOND_IN_MILLISECONDS: u64 = 1000;
////////////////////////////////////////////////

////////////////////////////////////////////////
// ticking structs
// These structs contain shared atomic bools that
// can be changed by the main thread in order to stop them
// at any time.
pub struct tickPhysics
{
    pub do_stop: Arc<AtomicBool>
}

pub struct tickEngine
{
    pub do_stop: Arc<AtomicBool>
}

////////////////////////////////////////////////

////////////////////////////////////////////////
// Implementations

impl tickPhysics
{
    pub fn new() -> tickPhysics
    {
        tickPhysics
        {
            do_stop: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn stop(&mut self)
    {
        self.do_stop.store(true, Ordering::Relaxed);
    }

    pub fn start(&mut self)
    {
        let l_stop = self.do_stop.clone();
        
        let phys = thread::spawn(move ||
        {
            let mut oldNow = Instant::now();
            loop
            {
                if l_stop.load(Ordering::Relaxed)
                {
                    println!("Shutting Down Phsyics");
                    break;
                }
                thread::sleep(Duration::from_millis(ONE_SECOND_IN_MILLISECONDS/PHYS_TICK));
                let t = Instant::now().duration_since(oldNow);
                println!("Physics tick: {:?}", t);
                oldNow = Instant::now();
            }
        });
    }
}

impl tickEngine
{
    pub fn new() -> tickEngine
    {
        tickEngine
        {
            do_stop: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn stop(&mut self)
    {
        self.do_stop.store(true, Ordering::Relaxed);
    }

    pub fn start(&mut self)
    {
        let l_stop = self.do_stop.clone();
        
        let eng = thread::spawn(move ||
        {
            let mut oldNow = Instant::now();
            loop
            {
                if l_stop.load(Ordering::Relaxed)
                {
                    println!("Shutting Down Engine");
                    break;
                }
                thread::sleep(Duration::from_millis(ONE_SECOND_IN_MILLISECONDS/TICK_TIME));
                let t = Instant::now().duration_since(oldNow);
                println!("Engine tick: {:?}", t);
                oldNow = Instant::now();
            }
        });
    }
}

////////////////////////////////////////////////