use std::collections::HashMap;
use std::fs;

fn collect_lists() -> (Vec<i64>, Vec<i64>) {
    let contents = fs::read_to_string("./inputs/day_1.txt").unwrap();
    let mut list_1: Vec<i64> = vec![];
    let mut list_2: Vec<i64> = vec![];

    for line in contents.lines() {
        let split: Vec<&str> = line.split("   ").collect::<Vec<&str>>();
        list_1.push(split[0].parse::<i64>().unwrap());
        list_2.push(split[1].parse::<i64>().unwrap());
    }

    list_1.sort();
    list_2.sort();

    (list_1, list_2)
}

pub fn day_one_part_1() {
    let (list_1, list_2) = collect_lists();

    let mut total = 0;

    for i in 0..list_1.len() {
        total += (list_1[i] - list_2[i]).abs();
    }

    println!("{}", total)
}

pub fn day_one_part_2() {
    let (list_1, list_2) = collect_lists();

    let mut occurances: HashMap<i64, i64> = HashMap::new();

    for x in list_2 {
        let amount = occurances.get(&x).unwrap_or(&0);
        occurances.insert(x, amount + 1);
    }

    let mut total = 0;

    for x in list_1 {
        total += x * occurances.get(&x).unwrap_or(&0);
    }

    println!("{}", total)
}