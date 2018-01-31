use std::env;
use std::io;
use std::thread;
use std::process::Command;

#[warn(deprecated)] 
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut guess = String::new();

    if args.len() == 2 {
        guess = args[1].clone();
    } else {
        println!("Please loop Commands:");    
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");   
        println!("OK Loop {} \n ##### start #####",&guess);
    }
    
    loop{
        Command::new("sh").arg("-c").arg("date +'%Y/%m/%d %I:%M:%S'").spawn().expect("failed to execute process");
        Command::new("sh").arg("-c").arg(&guess).spawn().expect("failed to execute process");
        thread::sleep_ms(6000);
    }
}
