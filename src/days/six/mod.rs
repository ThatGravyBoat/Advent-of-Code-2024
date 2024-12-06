use std::collections::{HashMap, HashSet};
use std::fs;
use crate::utils::utils::Grid;

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {

    fn next(&self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    fn dir(&self) -> (i32, i32) {
        match self {
            Direction::UP => (0, -1),
            Direction::RIGHT => (1, 0),
            Direction::DOWN => (0, 1),
            Direction::LEFT => (-1, 0),
        }
    }
}

fn read_board() -> ((i32, i32), Grid<char>) {
    let contents = fs::read_to_string("./inputs/day_6.txt").unwrap();
    let grid: Vec<Vec<char>> = contents.lines().map(|it| it.chars().collect()).collect();

    let mut pos: Option<(i32, i32)> = None;

    'guard_finder:
    for y in 0..grid.len() {
        let row = &grid[y];
        for x in 0..row.len() {
            if row[x] == '^' {
                pos = Some((x as i32, y as i32));
                break 'guard_finder
            }
        }
    }

    (pos.unwrap(), Grid::from(grid))
}

pub fn day_six() {
    let (mut guard, mut board) = read_board();

    let original_guard = (guard.0, guard.1);

    let mut direction = Direction::UP;
    let mut path: HashSet<(i32, i32)> = HashSet::new();

    while board.contains(guard.0, guard.1) {
        path.insert((guard.0, guard.1));
        board.set(guard.0 as usize, guard.1 as usize, 'X');
        
        let new_pos = (guard.0 + direction.dir().0, guard.1 + direction.dir().1);
        if board.contains(new_pos.0, new_pos.1) {
            if board.get(new_pos.0 as usize, new_pos.1 as usize) == &'#' {
                direction = direction.next();
            } else {
                guard = new_pos;
            }
        } else {
            guard = new_pos;
        }
    }

    println!("Part 1: {}", board.count(|it| it == &'X'));

    let mut repeating_locations = 0;
    let mut last_point: Option<(i32, i32)> = None;

    // Brute forced but idc :3

    for point in path {

        if let Some(last_point) = last_point {
            board.set(last_point.0 as usize, last_point.1 as usize, 'X');
        }

        board.set(point.0 as usize, point.1 as usize, '#');

        last_point = Some(point);
        guard = original_guard;
        direction = Direction::UP;

        let mut new_path: HashMap<(i32, i32), i32> = HashMap::new();

        while board.contains(guard.0, guard.1) {
            let path_count = *new_path.get(&(guard.0, guard.1)).unwrap_or(&0);
            if path_count > 5 {
                repeating_locations += 1;
                break
            }
            new_path.insert((guard.0, guard.1), path_count + 1);

            let new_pos = (guard.0 + direction.dir().0, guard.1 + direction.dir().1);
            if board.contains(new_pos.0, new_pos.1) {
                if board.get(new_pos.0 as usize, new_pos.1 as usize) == &'#' {
                    direction = direction.next();
                } else {
                    guard = new_pos;
                }
            } else {
                guard = new_pos;
            }
        }
    }

    println!("Part 2: {}", repeating_locations);
}