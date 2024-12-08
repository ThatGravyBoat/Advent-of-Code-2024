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
    let grid = Grid::from(contents);
    let pos: (i32, i32) = grid.find_first(|it| it == &'^').map(|it| (it.0 as i32, it.1 as i32)).unwrap();
    (pos, grid)
}

pub fn day_six() {
    let (mut guard, mut board) = read_board();

    let original_guard = (guard.0, guard.1);

    let mut direction = Direction::UP;
    let mut path: HashSet<(i32, i32)> = HashSet::new();

    while board.contains(guard.0, guard.1) {
        path.insert((guard.0, guard.1));
        board.try_set(guard.0, guard.1, 'X');
        
        let new_pos = (guard.0 + direction.dir().0, guard.1 + direction.dir().1);
        if let Some(char) = board.try_get(new_pos.0, new_pos.1) {
            if char == &'#' {
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
            board.try_set(last_point.0, last_point.1, 'X');
        }

        board.try_set(point.0, point.1, '#');

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
            if let Some(char) = board.try_get(new_pos.0, new_pos.1) {
                if char == &'#' {
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