use std::collections::HashSet;
use crate::utils::utils::Grid;
use std::fs;

fn read<'a>() -> (Grid<char>, HashSet<char>) {
    let contents = fs::read_to_string("./inputs/day_8.txt").unwrap();
    let antennas: Grid<char> = Grid::from(contents);

    let mut letters: HashSet<char> = HashSet::new();
    for char in antennas.get_storage().iter().flatten() {
        if char != &'.' { letters.insert(*char); }
    }

    (antennas, letters)
}

pub fn day_eight_part_1() {
    let (antennas, letters) = read();
    let mut antinodes: Grid<bool> = Grid::new(antennas.width(), antennas.height(), false);

    for letter in letters {
        let occurrences = antennas.get_occurrences(|it| *it == letter);
        for (x, y) in &occurrences {
            let x = *x as i32;
            let y = *y as i32;
            for (other_x, other_y) in &occurrences {
                let other_x = *other_x as i32;
                let other_y = *other_y as i32;
                if other_x != x || other_y != y {
                    let x_diff = x - other_x;
                    let y_diff = y - other_y;

                    antinodes.try_set(x + x_diff, y + y_diff, true);
                    antinodes.try_set(other_x - x_diff, other_y - y_diff, true);
                }
            }
        }
    }

    println!("Part 1: {}", antinodes.count(|it| *it))
}

pub fn day_eight_part_2() {
    let (antennas, letters) = read();
    let mut antinodes: Grid<bool> = Grid::new(antennas.width(), antennas.height(), false);

    for letter in letters {
        let occurrences = antennas.get_occurrences(|it| *it == letter);
        for (x, y) in &occurrences {
            let x = *x as i32;
            let y = *y as i32;
            for (other_x, other_y) in &occurrences {
                let other_x = *other_x as i32;
                let other_y = *other_y as i32;
                if other_x != x || other_y != y {
                    let x_diff = x - other_x;
                    let y_diff = y - other_y;

                    let mut i = 0;
                    while antinodes.try_set(x + x_diff * i, y + y_diff * i, true) || antinodes.try_set(x - x_diff * i, y - y_diff * i, true) {
                        i += 1;
                    }
                }
            }
        }
    }

    println!("Part 2: {}", antinodes.count(|it| *it))
}