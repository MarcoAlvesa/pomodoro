mod app;
use std::io;

use app::timer::{timer, user_choice};
fn main() {
    while true {
        println!("1. Sprint");
        println!("2. Break");
        println!("3. Exit");
        println!("Choose one option!");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => sprint_choice(),
            "2" => break_choice(),
            "3" => break,
            _ => println!("Please choose a valid option!"),
        }
    }
}

fn sprint_choice() {
    let input = user_choice();
    timer(input);
}
fn break_choice() {
    let input = user_choice();
    timer(input);
}
