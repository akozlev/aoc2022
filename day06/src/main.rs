use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("./data/in").expect("Something went wrong with this file");
    let slice = input.chars().collect::<Vec<char>>();
    let mut windows = slice.windows(14);

    while let Some(s) = windows.next() {
        if s.iter().collect::<HashSet<_>>().len() == 14 {
            break;
        }
    }
    println!("{:?}", input.len() - windows.count());
}
