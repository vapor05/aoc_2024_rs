use std::collections::HashMap;
use std::error::Error;

pub fn part1(data: &String) -> Result<String, Box<dyn Error>> {
    let (mut left, mut right) = match create_lists(data) {
        Ok(t) => t,
        Err(err) => return Err(err)
    };
    left.sort();
    right.sort();
    let mut sum: i32 = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        sum += (l-r).abs();
    }
    return Ok(sum.to_string())
}

pub fn part2(data: &String) -> Result<String, Box<dyn Error>> {
    let (mut left, mut right) = match create_lists(data) {
        Ok(t) => t,
        Err(err) => return Err(err),
    };
    left.sort();
    right.sort();
    let mut freq = HashMap::new();

    for n in right {
        freq.entry(n).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut sum = 0;

    for n in left {
        sum += n * freq.get(&n).unwrap_or(&0);
    }

    Ok(sum.to_string())
}

fn create_lists(data: &String) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in data.lines() {
        let mut parts = line.split_whitespace();
        let le = match parts.next() {
            Some(elem) => elem.parse::<i32>(),
            None => return Err("error reading input data, no left number".into()),
        }?;
        let re = match parts.next() {
            Some(elem) => elem.parse::<i32>(),
            None => return Err("error reading input data, no right number".into()),
        }?;
        left.push(le);
        right.push(re);
    }
    Ok((left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_lists() {
        let data = "\
1   4
2   5
3   6    
"
        .to_string();
        let want: (Vec<i32>, Vec<i32>) = (vec![1, 2, 3], vec![4, 5, 6]);
        let res = create_lists(&data);
        assert!(!res.is_err());
        let actual = res.unwrap();
        assert_eq!(want, actual);
    }
}
