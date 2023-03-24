use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main(){
    let args : Vec<String>= env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error: {}",e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;
    println!("This is the file contents:\n {} ", contents);
    Ok(())
}

struct Config {
    pattern: String,
    path: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len()<3 {
            return Err("Not enough args");
        }
        let pattern = args[1].clone();
        let path = args[2].clone();
        Ok(Config{
            pattern,
            path
        })
    }
}