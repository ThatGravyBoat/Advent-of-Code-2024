use crate::utils::utils::Grid;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

// UP, RIGHT, DOWN, LEFT
const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn travel(grid: &Grid<char>, pos: (usize, usize), output: &mut HashSet<(usize, usize)>) -> (i32, Vec<((usize, usize), [bool; 4])>) {
    output.insert(pos);
    let value = grid.get(pos.0, pos.1);
    let mut area = 1;
    let mut perimeter: Vec<((usize, usize), [bool; 4])> = vec![];
    let mut current_perimeter = [false, false, false, false];

    for (x, y) in DIRS {
        let new_pos = (pos.0 as i32 + x, pos.1 as i32 + y);
        if let Some(next) = grid.try_get(new_pos.0, new_pos.1) {
            if next == value {
                let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                if !output.contains(&new_pos) {
                    let (dir_area, mut dir_perimeter) = travel(grid, new_pos, output);
                    area += dir_area;
                    perimeter.append(&mut dir_perimeter);
                }
                continue;
            }
        }
        match (x, y) {
            (0, -1) => current_perimeter[0] = true,
            (1, 0) => current_perimeter[1] = true,
            (0, 1) => current_perimeter[2] = true,
            (-1, 0) => current_perimeter[3] = true,
            _ => panic!(),
        }
    }

    perimeter.push((pos, current_perimeter));

    (area, perimeter)
}

pub fn day_twelve_part_1() {
    let mut grid = Grid::from(fs::read_to_string("./inputs/day_12.txt").unwrap());
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut total = 0;

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if !visited.contains(&(x, y)) {
                let (area, perimeter) = travel(&grid, (x, y), &mut visited);
                total += area * perimeter.iter().map(|(_, perimeter)|perimeter.iter().filter(|it| **it).count() as i32).sum::<i32>();
            }
        }
    }

    println!("Part 1: {:?}", total);
}

// This is awful
pub fn day_twelve_part_2() {
    let mut grid = Grid::from(fs::read_to_string("./inputs/day_12.txt").unwrap());
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut total = 0;

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if !visited.contains(&(x, y)) {
                let mut up = Grid::new(grid.width(), grid.height(), false);
                let mut right = Grid::new(grid.width(), grid.height(), false);
                let mut down = Grid::new(grid.width(), grid.height(), false);
                let mut left = Grid::new(grid.width(), grid.height(), false);

                let mut sides = 0;

                let (area, perimeter) = travel(&grid, (x, y), &mut visited);
                for (pos, values) in perimeter {
                    if values[0] { up.set(pos.0, pos.1, true); }
                    if values[1] { right.set(pos.0, pos.1, true); }
                    if values[2] { down.set(pos.0, pos.1, true); }
                    if values[3] { left.set(pos.0, pos.1, true); }
                }

                for y in 0..grid.height() {
                    let mut found_up = false;
                    let mut found_down = false;
                    for x in 0..grid.width() {
                        if *up.get(x, y) {
                            if !found_up { sides += 1; }
                            found_up = true;
                        } else {
                            found_up = false;
                        }

                        if *down.get(x, y) {
                            if !found_down { sides += 1; }
                            found_down = true;
                        } else {
                            found_down = false;
                        }
                    }
                }

                for x in 0..grid.width() {
                    let mut found_left = false;
                    let mut found_right = false;
                    for y in 0..grid.height() {
                        if *left.get(x, y) {
                            if !found_left { sides += 1; }
                            found_left = true;
                        } else {
                            found_left = false;
                        }

                        if *right.get(x, y) {
                            if !found_right { sides += 1; }
                            found_right = true;
                        } else {
                            found_right = false;
                        }
                    }
                }

                total += area * sides;
            }
        }
    }

    println!("Part 2: {:?}", total)
}