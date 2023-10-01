fn main() {
    let input = include_str!("../data/in");

    let mut split: Vec<i32> = input
        .split_terminator("\n\n")
        .map(|x| {
            x.split_terminator('\n')
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
