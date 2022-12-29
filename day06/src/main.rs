use std::{collections::HashSet, hash::Hash};

fn only_distinct<T: Eq + Hash>(s: &[T]) -> bool {
    HashSet::<&T>::from_iter(s.iter()).len() == s.len()
}
fn main() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let slice = input.chars().collect::<Vec<char>>();
    const D: usize = 14;
    let pos = slice
        .windows(D)
        .position(|w| only_distinct(w))
        .map(|p|p+D);
    println!("{:?}", pos);
}
