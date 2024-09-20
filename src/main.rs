use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("error parsing arguments:  {err}");
        process::exit(1);
    });

    println!("{}", &config.query);

    if let Err(e) = minigrep::run(config){
        println!("Application Error: {}", e);
        process::exit(1);
    }
}