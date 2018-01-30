use std::env;
use std::thread;
use std::process::Command;

#[warn(deprecated)] 

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("My path is {}.", args[0]);
    if args.len() == 3 {
        loop{
            thread::sleep_ms(600);
            Command::new("sh").arg("-c").arg("ls").spawn().expect("failed to execute process");
            }
    }else{
        println!("Error Args")
    }
}
