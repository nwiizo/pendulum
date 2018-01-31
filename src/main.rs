use std::env;
use std::io;
use std::thread;
use std::process::Command;

#[warn(deprecated)] 
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("My path is {}.", args[0]);
    let mut guess = String::new();
    println!("Please loop Command.");
    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");
    println!("OK Loop {}",guess);

    loop{
        thread::sleep_ms(6000);
        Command::new("sh").arg("-c").arg("date").spawn().expect("failed to execute process");
        Command::new("sh").arg("-c").arg(&guess).spawn().expect("failed to execute process");
    }
}
