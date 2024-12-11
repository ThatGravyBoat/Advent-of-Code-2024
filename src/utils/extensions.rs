use std::collections::HashMap;
use std::hash::Hash;

pub trait IndexOf<A> {

    fn index_of(&self, predicate: impl Fn(&A) -> bool) -> i32 {
        self.index_of_at(0, predicate)
    }
    fn index_of_at(&self, starting_from: usize, predicate: impl Fn(&A) -> bool) -> i32;

    fn last_index_of(&self, predicate: impl Fn(&A) -> bool) -> i32;
    fn last_index_of_at(&self, starting_from: usize, predicate: impl Fn(&A) -> bool) -> i32;
}

impl <A : Eq> IndexOf<A> for Vec<A> {
    fn index_of_at(&self, starting_from: usize, predicate: impl Fn(&A) -> bool) -> i32 {
        for index in starting_from..self.len() {
            if predicate(&self[index]) {
                return index as i32
            }
        }
        -1
    }

    fn last_index_of(&self, predicate: impl Fn(&A) -> bool) -> i32 {
        self.last_index_of_at(self.len() - 1, predicate)
    }

    fn last_index_of_at(&self, starting_from: usize, predicate: impl Fn(&A) -> bool) -> i32 {
        for index in (0..starting_from + 1).rev() {
            if predicate(&self[index]) {
                return index as i32
            }
        }
        -1
    }
}

pub trait MapExt<K, V> {

    fn insert_plus(&mut self, key: K, amount: V);
}

impl <K> MapExt<K, u128> for HashMap<K, u128> where K: Eq, K: Hash {

    fn insert_plus(&mut self, key: K, amount: u128) {
        let current = self.get(&key).unwrap_or(&0);
        self.insert(key, current + amount);
    }
}