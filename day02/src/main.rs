use std::cmp::*;
use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("bad input"),
        }
    }
}

impl RPS {
    fn value(&self) -> i32 {
        match *self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
    fn from_str_tuple(s: &str) -> Result<Vec<RPS>, &'static str> {
        let tuple: Vec<&str> = s.split_terminator(" ").collect();
        let opponent = tuple[0].parse::<RPS>()?;
        let outcome = tuple[1].parse::<Outcome>()?;

        Ok(vec![
            opponent,
            match (opponent, outcome) {
                (RPS::Rock, Outcome::Lose) => RPS::Scissors,
                (RPS::Paper, Outcome::Lose) => RPS::Rock,
                (RPS::Scissors, Outcome::Lose) => RPS::Paper,
                (same, Outcome::Draw) => same,
                (RPS::Rock, Outcome::Win) => RPS::Paper,
                (RPS::Paper, Outcome::Win) => RPS::Scissors,
                (RPS::Scissors, Outcome::Win) => RPS::Rock,
            },
        ])
    }
}

impl FromStr for RPS {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err("bad input"),
        }
    }
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => {
                Some(Ordering::Less)
            }
            (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) | (RPS::Rock, RPS::Scissors) => {
                Some(Ordering::Greater)
            }
            (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => {
                Some(Ordering::Equal)
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("./data/in").expect("Something went wrong with this file");

    let split = contents
        .split_terminator("\n")
        // First
        // .map(
        //     x.split_terminator(" ")
        //     .map(|x| x.parse::<RPS>().unwrap())
        //     .collect()
        // )
        // -----------------------------------------------------------------
        // Second
        .map(|x| RPS::from_str_tuple(x).unwrap())
        .map(|x: Vec<RPS>| {
            x[1].value()
                + match x[0].partial_cmp(&x[1]).unwrap() {
                    Ordering::Less => 6,
                    Ordering::Equal => 3,
                    Ordering::Greater => 0,
                }
        })
        .sum::<i32>();
    println!("{:#?}", split);
}
