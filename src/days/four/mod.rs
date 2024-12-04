use std::fs;

const DIRS: [(i32, i32); 8] = [(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];

fn read_contents() -> Vec<Vec<char>> {
    let contents = fs::read_to_string("./inputs/day_4.txt").unwrap();
    contents.lines().map(|it| it.chars().collect()).collect::<Vec<Vec<char>>>()
}

pub fn day_four_part_1() {
    let contents = read_contents();
    let mut found = 0;

    for y in 0..contents.len() {
        let row = &contents[y];

        for x in 0..row.len() {
            if row[x] == 'X' {

                let check = |x_in: usize, y_in: usize, mult_x: i32, mult_y: i32| -> bool {
                    let x = x_in as i32;
                    let y = y_in as i32;

                    (mult_x != 1 || x_in < row.len() - 3) && (mult_x != -1 || x_in >= 3) && (mult_y != 1 || y_in < contents.len() - 3) && (mult_y != -1 || y_in >= 3) &&
                        (&contents[(y + 1 * mult_y) as usize])[(x + 1 * mult_x) as usize] == 'M' &&
                        (&contents[(y + 2 * mult_y) as usize])[(x + 2 * mult_x) as usize] == 'A' &&
                        (&contents[(y + 3 * mult_y) as usize])[(x + 3 * mult_x) as usize] == 'S'
                };

                for (x_mult, y_mult) in DIRS {
                    if check(x, y, x_mult, y_mult) {
                        found += 1;
                    }
                }
            }
        }
    }

    println!("{}", found)
}

pub fn day_four_part_2() {
    let contents = read_contents();
    let mut found = 0;

    for y in 1..contents.len() - 1 {
        let row = &contents[y];

        for x in 1..row.len() - 1 {
            if row[x] == 'A' {
                let above = &contents[y - 1];
                let below = &contents[y + 1];

                let top_left_bottom_right = (above[x - 1] == 'M' && below[x + 1] == 'S') || (above[x - 1] == 'S' && below[x + 1] == 'M');
                let top_right_bottom_left = (above[x + 1] == 'M' && below[x - 1] == 'S') || (above[x + 1] == 'S' && below[x - 1] == 'M');

                if top_left_bottom_right && top_right_bottom_left {
                    found += 1;
                }
            }
        }
    }

    println!("{}", found)
}