use std::fs;

fn read_numbers() -> Vec<Vec<i32>> {
    let contents = fs::read_to_string("./inputs/day_2.txt").unwrap();
    let mut numbers: Vec<Vec<i32>> = vec![];

    for line in contents.lines() {
        numbers.push(
            line.split(" ")
             .map(|it| it.parse::<i32>().unwrap())
             .collect::<Vec<i32>>()
        );
    }

    numbers
}

// can probably be cleaned up but who cares :3
fn is_safe(numbers: &Vec<i32>) -> bool {
    if !numbers.is_sorted_by(|a, b| a <= b) && !numbers.is_sorted_by(|a, b| a >= b) {
        false
    } else {
        let mut last: Option<i32> = None;

        for i in 0..numbers.len() {
            if let Some(last) = last {
                let diff = (numbers[i] - last).abs();
                if diff == 0 || diff >= 4 {
                    return false;
                }
            }
            last = Some(numbers[i]);
        }

        true
    }
}

pub fn day_two_part_1() {
    println!("{}", read_numbers()
        .iter()
        .filter(|it| is_safe(it))
        .count()
    );
}

pub fn day_two_part_2() {
    println!("{}", read_numbers()
        .iter()
        .filter(|&numbers| {
            if is_safe(numbers) {
                true
            } else {
                for i in 0..numbers.len() {
                    let mut values = numbers.clone();
                    values.remove(i);
                    if is_safe(&values) {
                        return true;
                    }
                }
                false
            }
        })
        .count()
    );
}