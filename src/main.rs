use std::env;
use std::io;
use std::thread;
use std::process::Command;

#[warn(deprecated)] 
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("My path is {}. \n Please loop Commands:", args[0]);    
    let mut guess = String::new();

    if args.len() == 2 {
        guess = args[1].clone();
    } else {
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");
    }
    
    loop{
        println!("OK Loop {}",&guess);
        Command::new("sh").arg("-c").arg("date").spawn().expect("failed to execute process");
        Command::new("sh").arg("-c").arg(&guess).spawn().expect("failed to execute process");
        thread::sleep_ms(6000);
    }
}
