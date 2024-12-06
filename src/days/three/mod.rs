use std::fs;
use regex::Regex;

pub fn day_three_part_1() {
    let contents = fs::read_to_string("./inputs/day_3.txt").unwrap();
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    println!("{}", regex.captures_iter(contents.as_str())
        .map(|captures| {
            let (_, [first, second]) = captures.extract();
            first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap()
        }).sum::<i32>()
    );
}

pub fn day_three_part_2() {
    let contents = fs::read_to_string("./inputs/day_3.txt").unwrap();
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();

    let mut active = true;

    println!("{}", regex.find_iter(contents.as_str())
        .map(|it| {
            match it.as_str() {
                "don't()" => active = false,
                "do()" => active = true,
                _ => {
                    if active {
                        let (first, second) = it.as_str().get(4..it.len() - 1).unwrap().split_once(",").unwrap();
                        return first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
                    }
                },
            }
            0
        })
        .reduce(|a, b| a + b)
        .unwrap());
}