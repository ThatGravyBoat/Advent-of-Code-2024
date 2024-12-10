use crate::utils::utils::Grid;
use std::collections::HashSet;
use std::fs;

// UP, RIGHT, DOWN, LEFT
const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn travel(grid: &Grid<u32>, start: (i32, i32), cache: &mut HashSet<(i32, i32)>, should_cache: bool) -> u32 {
    let current = *grid.try_get(start.0, start.1).unwrap_or(&0);

    let mut amount = 0;
    for (x, y) in DIRS {
        let new_pos = (start.0 + x, start.1 + y);
        if let Some(value) = grid.try_get(new_pos.0, new_pos.1) {
            if *value == 9 && current == 8 && (!should_cache || !cache.contains(&new_pos)) {
                cache.insert(new_pos);
                amount += 1;
            } else if *value == current + 1 {
                amount += travel(grid, new_pos, cache, should_cache);
            }
        }
    }
    amount
}

fn calculate(should_cache: bool) -> u32 {
    let grid = Grid::from(fs::read_to_string("./inputs/day_10.txt").unwrap()).map(|it| it.to_digit(10).unwrap());
    let zeros = grid.find_all(|it| *it == 0);

    let mut total = 0;

    for (x, y) in zeros {
        total += travel(&grid, (x as i32, y as i32), &mut HashSet::new(), should_cache)
    }

    total
}

pub fn day_ten_part_1() {
    println!("Part 1: {}", calculate(true));
}

pub fn day_ten_part_2() {
    println!("Part 2: {}", calculate(false));
}