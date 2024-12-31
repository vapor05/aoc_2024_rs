use std::env;
use std::error::Error;
use std::fs;
use std::os::unix::process;

pub mod prb1;

#[derive(Debug)]
pub struct RunError(String);

pub fn run(config: Config) -> Result<String, RunError> {
    println!("running problem {} part {}", config.problem, config.part);
    let data = load_data(&config.problem, config.test)
        .map_err(|err| RunError(format!("failed to read problem input data: {}", err)))?;
    let solution: Result<String, RunError> = match config.problem.as_str() {
        "1" => {
            if config.part == "1" {
                prb1::part1(&data).map_err(|err| RunError(format!("failed to run problem: {}", err)))
            } else if config.part == "2" {
                prb1::part2(&data).map_err(|err| RunError(format!("failed to run problem: {}", err)))
            } else {
                return Err(RunError(format!("no part {} for problem {}", config.part, config.problem)));
            }
        }
        _ => {
            return Err(RunError(format!(
                "no solution defined for problem {}",
                config.problem
            )))
        }
    };
    solution
}

pub fn load_data(prb_nmb: &str, test: bool) -> Result<String, Box<dyn Error>> {
    let root_dir = env::current_dir()?;
    let file_name = if test {
        format!("problem_data/problem{prb_nmb}_test.txt")
    } else {
        format!("problem_data/problem{prb_nmb}.txt")
    };
    let prb_file = root_dir.join(file_name);
    let data = fs::read_to_string(prb_file)?;
    Ok(data)
}

pub struct Config {
    pub problem: String,
    pub part: String,
    pub test: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let problem = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a problem number to solve for"),
        };
        let part = match args.next() {
            Some(arg) => arg,
            None => String::from("1"),
        };
        let test = env::var("TEST").is_ok();
        Ok(Config {
            problem,
            part,
            test,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        let res = load_data("1", true);
        assert!(!res.is_err());
        let actual = res.unwrap();
        let want = "\
3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(want, actual);
    }
}
