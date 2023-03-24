use std::env;
use std::fs;

fn main(){
    let args : Vec<String>= env::args().collect();
    let path = &args[2];

    let contents = fs::read_to_string(path)
        .expect("No such file exists");

    println!("This is the file contents:\n {} ", contents);


}