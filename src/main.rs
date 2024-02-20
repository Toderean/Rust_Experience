use std::{env, fs};
use std::error::Error;



struct Config{
    query: String,
    file_path: String,
}

fn main(){
    let args: Vec<String> = env::args().collect();

    // println!("command line = {:?}", args); 
    // dbg!(args);
    
    // let first_command = &args[1];
    // let second_command = &args[2];

    // let config = parse_config(&args);
    let config = Config::new(&args).unwrap();

    println!("first command = {}", config.query);
    println!("second command = {}", config.file_path);

    run(config);
}

fn run(Config: Config)-> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(Config.file_path)?;
    println!("with text {}", contents);
    Ok(())
}

impl Config{
    fn new(args: &[String]) -> Result<Config,&'static str> {
        if(args.len() < 3){
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config {query, file_path})    
    }
}
fn parse_config(args: &[String]) -> Config{
    // let first_command = &args[1];
    // let second_command = &args[2];
    // (first_command, second_command)
    
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config {query, file_path}
}