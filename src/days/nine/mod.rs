use crate::days::nine::FileSystemEntry::{File, Space};
use crate::utils::extensions::IndexOf;
use std::fs;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum FileSystemEntry {
    Space(usize), // SIZE
    File(u32, usize), // ID/SIZE
}

impl FileSystemEntry {

    fn is_file(&self) -> bool {
        matches!(*self, File(_, _))
    }

    fn is_specific_file(&self, id: u32) -> bool {
        if let File(other_id, _) = self {
            *other_id == id
        } else {
            false
        }
    }

    fn get_size(&self) -> usize {
        match self {
            Space(size) => *size,
            File(_, size) => *size,
        }
    }
}

fn read_file_system<T>(
    space_creator: fn(&u32, &mut Vec<T>),
    file_creator: fn(&u32, &u32, &mut Vec<T>),
) -> (Vec<T>, u32, u32) {
    let contents = fs::read_to_string("./inputs/day_9.txt").unwrap();

    let mut filesystem: Vec<T> = vec![];
    let mut is_free_space = false;

    let mut file_id: u32 = 0;
    let mut file_count: u32 = 0;

    for char in contents.chars() {
        let amount = char.to_digit(10).unwrap();
        if is_free_space {
            space_creator(&amount, &mut filesystem);
        } else {
            file_creator(&amount, &file_id, &mut filesystem);
            file_count += amount;
            file_id += 1;
        }
        is_free_space = !is_free_space;
    }

    (filesystem, file_count, file_id)
}

pub fn day_nine_part_1() {
    let (mut filesystem, file_count, _) = read_file_system(
        |amount, filesystem| {
            for _ in 0..*amount {
                filesystem.push(None);
            }
        },
        |amount, id, filesystem| {
            for _ in 0..*amount {
                filesystem.push(Some(*id));
            }
        }
    );

    let mut first_free_space = filesystem.index_of(|it| it.is_none());
    let mut last_file = filesystem.last_index_of(|it| it.is_some());

    while first_free_space < file_count as i32 {
        filesystem.swap(first_free_space as usize, last_file as usize);
        first_free_space = filesystem.index_of_at(first_free_space as usize, |it| it.is_none());
        last_file = filesystem.last_index_of_at(last_file as usize, |it| it.is_some());
    }

    let mut total = 0;

    for (index, value) in filesystem.iter().enumerate() {
        if let Some(value) = value {
            total += *value as u128 * index as u128;
        } else {
            break
        }
    }

    println!("Part 1: {}", total);
}

fn find_space_for(filesystem: &Vec<FileSystemEntry>, file: FileSystemEntry) -> Option<usize> {
    for (index, entry) in filesystem.iter().enumerate() {
        if *entry == file {
            break
        }

        if let Space(size) = entry {
            if *size >= file.get_size() {
                return Some(index)
            }
        }
    }
    None
}

pub fn day_nine_part_2() {
    let (mut filesystem, _, next_file_id) = read_file_system(
        |amount, filesystem| filesystem.push(Space(*amount as usize)),
        |amount, id, filesystem| filesystem.push(File(*id, *amount as usize))
    );

    for i in (0..next_file_id).rev() {
        let mut file = filesystem.last_index_of(|it| it.is_specific_file(i)) as usize;

        if let Some(space) = find_space_for(&filesystem, filesystem[file]) {
            let diff = filesystem[space].get_size() - filesystem[file].get_size();

            if diff == 0 {
                filesystem.swap(space, file);
            } else {
                let file_entry = filesystem[file];
                filesystem[space] = Space(diff);
                filesystem[file] = Space(file_entry.get_size());
                filesystem.insert(space, file_entry);
            }
        }
    }

    let mut total = 0;
    let mut i = 0;

    for value in filesystem {
        if let File(id, size) = value {
            for _ in 0..size {
                total += id as u128 * i as u128;
                i += 1;
            }
        } else {
            i += value.get_size()
        }
    }

    println!("Part 2: {}", total)
}