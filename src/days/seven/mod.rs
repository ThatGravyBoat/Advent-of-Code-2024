use std::fs;

fn collect_numbers() -> Vec<(u128, Vec<u128>)> {
    let contents = fs::read_to_string("./inputs/day_7.txt").unwrap();
    let mut output: Vec<(u128, Vec<u128>)> = vec![];

    for line in contents.lines() {
        let (result, numbers) = line.split_once(": ").unwrap();
        output.push((
            result.parse::<u128>().unwrap(),
            numbers.split(" ").map(|it| it.parse::<u128>().unwrap()).collect::<Vec<u128>>(),
        ));
    }

    output
}

pub fn day_seven_part_1() {
    fn is_true(target: u128, numbers: &Vec<u128>, index: usize, current: u128) -> bool {
        let new_index = index + 1;
        if new_index >= numbers.len() {
            current == target
        } else if current > target {
            false
        } else {
            let num = numbers[new_index];
            is_true(target, numbers, new_index, current + num) ||
            is_true(target, numbers, new_index, current * num)
        }
    }

    println!(
        "Part 1: {}",
        collect_numbers().iter()
            .filter(|(result, numbers)| is_true(*result, &numbers, 0, numbers[0]))
            .map(|(result, _)| result)
            .sum::<u128>()
    );
}

pub fn day_seven_part_2() {
    fn is_true(target: u128, numbers: &Vec<u128>, index: usize, current: u128) -> bool {
        let new_index = index + 1;
        if new_index >= numbers.len() {
            current == target
        } else if current > target {
            false
        } else {
            let num = numbers[new_index];
            return if is_true(target, numbers, new_index, current + num) || is_true(target, numbers, new_index, current * num) {
                true
            } else {
                let mut power = 10;
                loop {
                    if num < power { break }
                    power *= 10;
                }
                is_true(target, numbers, new_index, (current * power) + num)
            }
        }
    }

    println!(
        "Part 2: {}",
        collect_numbers().iter()
            .filter(|(result, numbers)| is_true(*result, &numbers, 0, numbers[0]))
            .map(|(result, _)| result)
            .sum::<u128>()
    );
}