use std::time::{Instant, Duration};
use std::thread;

    pub fn sprint(){

    let duration = Duration::from_secs(25);
    let start = Instant::now();

    while start.elapsed() < duration{
        let remaining = duration - start.elapsed();
        println!("Time remaining: {:?}m", remaining);
        thread::sleep(Duration::from_secs(1));
    }
    }

    pub fn break_time(){

    let duration = Duration::from_secs(5);
    let start = Instant::now();

    while start.elapsed() < duration{
        let remaining = duration - start.elapsed();
        println!("Time remaining: {:?}m", remaining);
        thread::sleep(Duration::from_secs(1));
    }
    }