use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    println!("Test app, designed to test the build process.");
    println!("Please input whether or not you want the program to run. (yes/no)");
    loop{
    let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read input");
    if input.trim().to_lowercase() == "yes"{
        //new function goes here
        println!("understood, running program");
        delay();
        broadcaster();
    }
    else if input.trim().to_lowercase() == "no" {
     break;   
    }
    
    
    }
   
}
fn broadcaster(){
    println!("broadcasting!");
    
}
fn delay(){
    let interval = Duration::from_secs(1);
    let mut next_time = Instant::now() + interval;
    loop{
        sleep(next_time - Instant::now());
        next_time += interval;
        break;
    }

}