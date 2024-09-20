use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("error parsing arguments:  {err}");
        process::exit(1);
    });

    let query = config.query.clone();

    let contents = match minigrep::run(&config){
        Ok(content) => content,
        Err(e) => {
            eprintln!("Application Error: {} ", e);
            process::exit(1);
        },
    };

    let results: Vec<&str> = if config.ignore_case { 
        minigrep::search_case_insensitive(&query, &contents)
    }else {
        minigrep::search(&query, &contents)
    };
    
    for line in results{
        println!("{line}");
    }

}