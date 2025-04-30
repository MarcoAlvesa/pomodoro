use std::time::{Instant, Duration};
use std::thread;
use std::io;

    pub fn timer(num: u64){

    let duration = Duration::from_secs(num * 60);
    let start = Instant::now();

    while start.elapsed() < duration{
        let remaining = duration - start.elapsed();
        let secs = remaining.as_secs();
        let mins = secs / 60;
        let remaining_secs = secs % 60;

        println!("Time remaining: {:02}:{:02}", mins, remaining_secs);
        thread::sleep(Duration::from_secs(1));
    }
    }

    pub fn user_choice() -> u64 {
        loop{
            println!("Set the time in minutes");
    
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
    
            match input.trim().parse::<u64>(){
                Ok(time) => return time,
                Err(_) =>{ 
                    eprintln!("Error: Invalid Input!");
                    continue;
                }
            }
        }
    }