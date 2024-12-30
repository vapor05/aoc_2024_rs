use aoc_2024_rs::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let res = aoc_2024_rs::run(config);

    let solution = match res {
        Ok(solution) => solution,
        Err(err) => {
            print!("Application error: {:#?}", err);
            process::exit(1);
        }
    };

    println!("solution: {solution}");
}
