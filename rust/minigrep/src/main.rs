use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match   Config::new(&args) {
        Ok(f)=>f,
        Err(e)=> e,
    };

    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something wen wrong reading the file");

    println!("With text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();

        let filename = args[2].clone();
        Ok(Self { query, filename })
    }
}
