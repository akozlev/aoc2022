use std::{fs, ops::Range};

fn parse_range (s: &str) -> Range<u32> {
    s.split_once('-').map(|x| (x.0.parse::<u32>().unwrap()..x.1.parse().unwrap())).unwrap()
}

fn contained<T: PartialOrd>(a: Range<T>, b: Range<T>) -> bool {
    a.start >= b.start && a.end <= b.end || 
        b.start >= a.start && b.end <= a.end
}

fn overlapped<T: PartialOrd>(a: Range<T>, b: Range<T>) -> bool {
    a.start >= b.start && a.start <= b.end || 
        a.end >= b.start && a.end <= b.end || 
        b.start >= a.start && b.start <= a.end ||
        b.end >= a.start && b.end <= a.end
}

fn main() {
    let input = fs::read_to_string("./data/in").expect("Something went wrong with this file");

    let contained: Vec<_> = input
        .lines()
        .into_iter()
        .map(|line| line.split_once(',').map(|elves| (parse_range(elves.0),  parse_range(elves.1))).unwrap())
        .map(|elves| contained(elves.0, elves.1))
        .filter(|&x| x)
        .collect();

    println!("{:?}", contained.len());

    let overlapped: Vec<_>  = input
        .lines()
        .into_iter()
        .map(|line| line.split_once(',').map(|elves| (parse_range(elves.0),  parse_range(elves.1))).unwrap())
        .map(|elves| overlapped(elves.0, elves.1))
        .filter(|&x| x)
        .collect();

    println!("{:?}", overlapped.len());
}
