use std::env;
use std::fs::File;
use std::fs;
use std::process;
use minigrep::search;
struct Config{
    query: String,
    file_path: String
}

impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("Not enough arguments");
        }
        let query = &args[1].clone();
        let file_path = &args[2].clone();
    
        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string()
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });   
    
    run(config);
}

fn run(config: Config){
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    for line in search(&config.query, &contents){
        println!("{}", line);
    }

}

