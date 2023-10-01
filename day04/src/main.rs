use std::{fs, ops::RangeInclusive};

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;
    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }
    fn overlaps_range(&self, other: &Self) -> bool;
    fn overlaps_or_is_overlapped(&self, other: &Self) -> bool {
        self.overlaps_range(other) || other.overlaps_range(self)
    }
}

impl<T> InclusiveRangeExt for RangeInclusive<T> where T: PartialOrd {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps_range(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn parse_range (s: &str) -> RangeInclusive<u32> {
    s.split_once('-').map(|x| (x.0.parse::<u32>().unwrap()..=x.1.parse().unwrap())).unwrap()
}

fn main() {
    let input = fs::read_to_string("./data/in").expect("Something went wrong with this file");

    let contained: usize = input
        .lines()
        .into_iter()
        .map(|line| line.split_once(',').map(|elves| (parse_range(elves.0),  parse_range(elves.1))).unwrap())
        .map(|elves| elves.0.contains_or_is_contained(&elves.1))
        .filter(|&x| x)
        .count();

    println!("{:?}", contained);

    let overlapped: usize  = input
        .lines()
        .into_iter()
        .map(|line| line.split_once(',').map(|elves| (parse_range(elves.0),  parse_range(elves.1))).unwrap())
        .map(|elves| elves.0.overlaps_or_is_overlapped(&elves.1))
        .filter(|&x| x)
        .count();

    println!("{:?}", overlapped);
}
