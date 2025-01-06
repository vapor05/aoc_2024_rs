use std::error::Error;

pub fn part1(data: &String) -> Result<String, Box<dyn Error>> {
    let reports = create_lists(data)?;
    let mut safe = 0;
    
    for report in reports {
        let mut is_safe = true;
        let asc = report[0] < report[1];

        for i in 0..report.len()-1 {
            if asc {
                if report[i] >= report[i+1] {
                    is_safe = false;
                    break;
                }
            } else {
                if report[i] <= report[i+1] {
                    is_safe = false;
                    break;
                }
            }

            let diff = (report[i] - report[i+1]).abs();

            if diff < 1 || diff > 3 {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            safe += 1;
        }
    }

    Ok(safe.to_string())
}


fn create_lists(data: &String) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in data.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect();
        reports.push(levels);
    }
    Ok(reports)
}
