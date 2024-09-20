use std::{fs, error::Error};



pub struct Config{
    pub query: String,
    pub file_path: String,
}

// impl Config {
//     pub fn new(args: &[String]) -> Config{
//         Config{query: args[1].clone(), file_path: args[2].clone()}
//     }
// }


impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
        Ok(Config{query: args[1].clone(), file_path: args[2].clone()})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    println!("With Text\n{content}");
    Ok(())
}
