use std::{collections::HashSet, hash::Hash};

fn only_distinct<T: Eq + Hash>(s: &[T]) -> bool {
    HashSet::<&T>::from_iter(s.iter()).len() == s.len()
}

fn main() {
    let input = include_str!("../data/in");
    // let slice = input.chars().collect::<Vec<char>>();
    const D: usize = 14;
    let pos = input.as_bytes()
        .windows(D)
        .position(only_distinct)
        .map(|p|p+D);
    println!("{:?}", pos);
}
