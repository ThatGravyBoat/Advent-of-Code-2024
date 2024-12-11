use crate::utils::extensions::MapExt;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::string::ToString;

fn trim_keeping_one(input: &str, char: char) -> String {
    let trimmed = input.trim_start_matches(char);
    if trimmed.is_empty() && !input.is_empty() {
        char.to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn day_eleven_part_1() {
    let mut arrangement = fs::read_to_string("./inputs/day_11.txt")
        .unwrap()
        .split(" ")
        .map(|it| it.to_string())
        .collect::<Vec<String>>();

    for _ in 0..25 {
        let mut new: Vec<String> = vec![];
        for number in arrangement {
            if number == "0" {
                new.push("1".to_string());
            } else if number.len() % 2 == 0 {
                let (first, second) = number.split_at(number.len() / 2);
                new.push(trim_keeping_one(first, '0'));
                new.push(trim_keeping_one(second, '0'));
            } else {
                new.push((number.parse::<u128>().unwrap() * 2024).to_string());
            }
        }

        arrangement = new;
    }

    println!("Part 1: {:?}", arrangement.len());
}

pub fn day_eleven_part_2() {
    let mut arrangement = fs::read_to_string("./inputs/day_11.txt")
        .unwrap()
        .split(" ")
        .map(|it| (it.to_string(), 1u128))
        .collect::<HashMap<String, u128>>();

    for _ in 0..75 {
        let mut new: HashMap<String, u128> = HashMap::new();

        for (number, count) in arrangement {
            if number == "0" {
                new.insert_plus("1".to_string(), count);
            } else if number.len() % 2 == 0 {
                let (first, second) = number.split_at(number.len() / 2);
                new.insert_plus(trim_keeping_one(first, '0'), count);
                new.insert_plus(trim_keeping_one(second, '0'), count);
            } else {
                new.insert_plus((number.parse::<u128>().unwrap() * 2024).to_string(), count);
            }
        }

        arrangement = new;
    }

    println!("Part 2: {:?}", arrangement.values().sum::<u128>());
}