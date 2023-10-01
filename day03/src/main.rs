use std::fs;

fn to_priority(c: char) -> Option<u32> {
    if !c.is_alphabetic() {
        return None;
    }

    if c.is_uppercase() {
        Some(c as u32 - 64 + 26)
    } else {
        Some(c as u32 - 96)
    }
}

fn main() {
    let input = fs::read_to_string("./data/in").expect("Something went wrong with this file");

    let rucksacks: Vec<u32> = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|compartments| {
            (compartments.0.to_string() + compartments.1)
                .chars()
                .filter(|c| compartments.0.contains(*c))
                .filter(|c| compartments.1.contains(*c))
                .collect::<String>()
        })
        .map(|extra_items| {
            let c = extra_items.chars().collect::<Vec<_>>()[0];
            to_priority(c).unwrap()
        })
        .collect();

    println!("{:#?}", rucksacks.iter().sum::<u32>());
    // println!("{:#?}", rucksacks);
    let groups = input.split('\n').collect::<Vec<&str>>();
    let chunked = groups
        .chunks_exact(3)
        .map(|chunk| {
            chunk[0]
                .chars()
                .filter(|c| chunk[1].contains(*c))
                .filter(|c| chunk[2].contains(*c))
                .collect::<Vec<char>>()[0]
        })
        .map(|c| to_priority(c).unwrap())
        .sum::<u32>();
    println!("{:#?}", chunked);
}
