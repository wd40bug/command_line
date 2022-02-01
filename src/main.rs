use std::{env, process};
use command_line::Config;
fn main(){
    let args:Vec<String> = env::args().collect();

    let arguments = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing file: {}",err);
        process::exit(1);
    });

    if let Err(e) = command_line::run(arguments){
        println!("Application error: {}",e);
        process::exit(1);
    }
    
}


