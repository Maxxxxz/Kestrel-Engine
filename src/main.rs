use std::time::{Duration, Instant};
use std::thread;

const ONE_SECOND_IN_MILLISECONDS: u64 = 1000;

// Updates per second
const PHYS_TICK: i8 = 20;
const TICK_TIME: i8 = 20;


fn main()
{
    // move ticks to separate threads so I can use sleep(ONE_SECOND_IN_MICROSECONDS/PHYS_TICK)
    let mut oldNow = Instant::now();
    loop
    {
        thread::sleep(Duration::from_millis(ONE_SECOND_IN_MILLISECONDS/TICK_TIME as u64));

        let t = Instant::now().duration_since(oldNow);
        println!("ticked: {:?}", t);



        oldNow = Instant::now();
    }

}
