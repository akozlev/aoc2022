use std::fs;

fn main() {
    let contents = fs::read_to_string("./data/in").expect("Something went wrong with this file");

    let mut split: Vec<i32> = contents
        .split_terminator("\n\n")
        .map(|x| {
            x.split_terminator("\n")
                .map(|y| y.parse::<i32>().unwrap())
                .sum()
        })
        .collect();

    split.sort();
    split.reverse();

    let ans1 = split.first().unwrap();
    let ans2: i32 = split.windows(3).next().unwrap().iter().sum();

    println!("Top Elf: {:?}", ans1);
    println!("Top 3 Elves: {:?}", ans2);
}
