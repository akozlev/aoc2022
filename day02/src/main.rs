use std::cmp::*;
use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Move {
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

impl Move {
    fn value(&self) -> i32 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn from_str_tuple(s: &str) -> Result<Vec<Move>, &'static str> {
        let tuple: Vec<&str> = s.split_terminator(' ').collect();
        let opponent = tuple[0].parse::<Move>()?;
        let outcome = tuple[1].parse::<Outcome>()?;

        Ok(vec![
            opponent,
            match (opponent, outcome) {
                (Move::Rock, Outcome::Lose) => Move::Scissors,
                (Move::Paper, Outcome::Lose) => Move::Rock,
                (Move::Scissors, Outcome::Lose) => Move::Paper,
                (same, Outcome::Draw) => same,
                (Move::Rock, Outcome::Win) => Move::Paper,
                (Move::Paper, Outcome::Win) => Move::Scissors,
                (Move::Scissors, Outcome::Win) => Move::Rock,
            },
        ])
    }
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("bad input"),
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Move::Rock, Move::Paper) | (Move::Paper, Move::Scissors) | (Move::Scissors, Move::Rock) => {
                Some(Ordering::Less)
            }
            (Move::Paper, Move::Rock) | (Move::Scissors, Move::Paper) | (Move::Rock, Move::Scissors) => {
                Some(Ordering::Greater)
            }
            (Move::Rock, Move::Rock) | (Move::Paper, Move::Paper) | (Move::Scissors, Move::Scissors) => {
                Some(Ordering::Equal)
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("./data/in").expect("Something went wrong with this file");

    let split = contents
        .split_terminator('\n')
        // First
        // .map(
        //     x.split_terminator(" ")
        //     .map(|x| x.parse::<Move>().unwrap())
        //     .collect()
        // )
        // -----------------------------------------------------------------
        // Second
        .map(|x| Move::from_str_tuple(x).unwrap())
        .map(|x: Vec<Move>| {
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
