mod app;
use std::io;

use app::timer::{break_time, sprint};
fn main() {
    while true {

        println!("1. Sprint (45 minutes)");
        println!("2. Break (5 minutes)");
        println!("3. Exit");
        println!("Choose one option!");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => sprint(),
            "2" => break_time(),
            "3" => break,
            _ => println!("Please choose a valid option!"),
        }
    }
}
