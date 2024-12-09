use crate::utils::extensions::IndexOf;
use std::collections::HashMap;
use std::fs;

fn is_abiding_by_rules(pages: &Vec<&str>, rules: &HashMap<&str, Vec<&str>>) -> Result<(), (usize, usize)> {
    for (i, page) in pages.iter().enumerate() {
        if let Some(need_before) = rules.get(page) {
            for page_before in need_before {
                let index = pages.index_of(|it| *&it == *&page_before);
                if index != -1 && index >= i as i32 {
                    return Err((i, index as usize));
                }
            }
        }
    }
    Ok(())
}

pub fn day_five() {
    let contents = fs::read_to_string("./inputs/day_5.txt").unwrap();

    let mut before: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut found_rules = false;

    let mut ordered_pages: Vec<Vec<&str>> = vec![];
    let mut unordered_pages: Vec<Vec<&str>> = vec![];

    for line in contents.lines() {
        if found_rules {
            let pages: Vec<&str> = line.split(",").collect();
            if is_abiding_by_rules(&pages, &before).is_err() {
                unordered_pages.push(pages);
                continue;
            }
            ordered_pages.push(pages);
        } else if line.is_empty() {
            found_rules = true;
        } else {
            let (first, second) = line.split_once("|").unwrap();
            if let Some(list) = before.get_mut(second) {
                list.push(first)
            } else {
                before.insert(second, vec![first]);
            }
        }
    }

    println!(
        "part 1: {:?}",
        ordered_pages.iter()
            .map(|it| it[it.len() / 2].parse::<i32>().unwrap())
            .sum::<i32>()
    );

    let mut ordered_pages: Vec<Vec<&str>> = vec![];

    for mut pages in unordered_pages {
        loop {
            if let Err((one, two)) = is_abiding_by_rules(&pages, &before) {
                pages.swap(one, two)
            } else {
                ordered_pages.push(pages);
                break
            }
        }
    }

    println!(
        "part 2: {:?}",
        ordered_pages.iter()
            .map(|it| it[it.len() / 2].parse::<i32>().unwrap())
            .sum::<i32>()
    );

}