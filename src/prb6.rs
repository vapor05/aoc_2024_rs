use std::error::Error;
use std::collections::HashSet;


pub fn part1(data: &String) -> Result<String, Box<dyn Error>> {
    let (rows, mut guard) = match load_grid(data) {
        Ok(t) => t,
        Err(err) => return Err(err),
    };
    let mut guard_facing = Facing::UP;
    let mut steps = 0;
    let max_col = rows.len() as i32;
    let max_row = rows[0].len() as i32;
    let mut visited: HashSet<Point> = HashSet::from([guard]);

    // while guard is still on grid
    while guard.row >=0 && guard.row < max_row && guard.col >= 0 && guard.col < max_col {
        steps+=1;
        let check_point = match guard_facing {
            Facing::UP => Point{row: guard.row - 1, col: guard.col},
            Facing::RIGHT => Point{row: guard.row, col: guard.col+1},
            Facing::DOWN => Point{row: guard.row+1, col: guard.col},
            Facing::LEFT => Point{row: guard.row, col: guard.col -1}
        };

        // check if check point is off grid
        if check_point.row < 0 || check_point.row >= max_row || check_point.col < 0 || check_point.col >= max_col {
            break;
        }
        let row = check_point.row as usize;
        let col = check_point.col as usize;
        let check = rows[row][col];

        if check == '#' {
            guard_facing = turn_right(guard_facing);
        } else {
            guard = check_point;
            visited.insert(guard);
        }

        if steps > 10000 {
            return Err("too many steps".into());
        }
    }
    return Ok(visited.len().to_string())
}


fn load_grid(data: &String) -> Result<(Vec<Vec<char>>, Point), Box<dyn Error>> {
    let mut rows: Vec<Vec<char>> = Vec::new();
    let mut guard = Point{row: 0, col: 0};

    for (row, line) in data.lines().enumerate() {
        let mut cols: Vec<char> = Vec::new();

        for (col, c) in line.chars().enumerate() {
            if c == '^' {
                guard = Point{row: row as i32, col: col as i32};
            }
            cols.push(c);
        }

        rows.push(cols)
    }

    if guard.row == 0 && guard.col == 0 {
        return Err("guard starting position not found".into())
    }

    return Ok((rows, guard));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Facing {
    UP,
    RIGHT,
    LEFT,
    DOWN,
}

fn turn_right(facing: Facing) -> Facing {
    match facing {
        Facing::UP => Facing::RIGHT,
        Facing::RIGHT => Facing::DOWN,
        Facing::DOWN => Facing::LEFT,
        Facing::LEFT => Facing::UP,
    }
}

pub fn part2(data: &String) -> Result<String, Box<dyn Error>> {
    let (mut rows, guard) = match load_grid(data) {
        Ok(t) => t,
        Err(err) => return Err(err),
    };
    let (visited, _) = match run_guard_path(&rows, guard) {
        Ok(r) => r,
        Err(err) => return Err(err),
    };
    let mut check_points: HashSet<Point> = HashSet::new();
    
    for point in visited {
        check_points.insert(Point{row: point.row, col: point.col});
    }

    let mut loops = 0;

    for point in check_points {
        let row = point.row as usize;
        let col = point.col as usize;
        let replace = rows[row][col];

        if replace == '#' || replace == '^' {
            continue;
        }

        rows[row][col] = '#';
        let (_, has_loop) = match run_guard_path(&rows, guard) {
            Ok(r) => r,
            Err(err) => return Err(err),
        };

        if has_loop {
            loops += 1;
        }

        rows[row][col] = replace;
    }

    return Ok(loops.to_string());
}

fn run_guard_path(rows: &Vec<Vec<char>>, guard_pos: Point) -> Result<(HashSet<PointFacing>, bool), Box<dyn Error>> {
    let mut guard_facing = Facing::UP;
    let mut guard = PointFacing{
        row: guard_pos.row, 
        col: guard_pos.col, 
        facing: guard_facing
    };
    let mut steps = 0;
    let max_col = rows.len() as i32;
    let max_row = rows[0].len() as i32;
    let mut visited: HashSet<PointFacing> = HashSet::from([guard]);

    // while guard is still on grid
    while guard.row >=0 && guard.row < max_row && guard.col >= 0 && guard.col < max_col {
        steps+=1;
        let check_point = match guard_facing {
            Facing::UP => Point{row: guard.row - 1, col: guard.col},
            Facing::RIGHT => Point{row: guard.row, col: guard.col+1},
            Facing::DOWN => Point{row: guard.row+1, col: guard.col},
            Facing::LEFT => Point{row: guard.row, col: guard.col -1}
        };

        // check if check point is off grid
        if check_point.row < 0 || check_point.row >= max_row || check_point.col < 0 || check_point.col >= max_col {
            break;
        }
        let row = check_point.row as usize;
        let col = check_point.col as usize;
        let check = rows[row][col];

        if check == '#' {
            guard_facing = turn_right(guard_facing);
        } else {
            guard = PointFacing{row: check_point.row, col: check_point.col, facing: guard_facing};

            if visited.contains(&guard) {
                return Ok((visited, true))
            }

            visited.insert(guard);
        }

        if steps > 10000 {
            return Err("too many steps".into());
        }
    }   

    return Ok((visited, false))
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct PointFacing {
    row: i32,
    col: i32,
    facing: Facing,
}