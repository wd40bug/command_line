use std::{env, fs};

fn main(){
    let args:Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("Looking for {}",query);
    println!("In file {}",filename);

    let contents = fs::read_to_string(filename).expect("couldn't read file");

    println!("With text:\n{}",contents);
    
}
