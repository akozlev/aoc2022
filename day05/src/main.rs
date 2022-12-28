use std::fs;

//move 5 from 7 to 6
#[derive(Debug)]
struct Move {
    amount: u8,
    from: u8,
    to: u8,
}

fn parse_move(s: &str) -> Move {
    let mut it = s.split_whitespace().skip(1).step_by(2);

    Move {
        amount: it.next().unwrap().parse().unwrap(),
        from: it.next().unwrap().parse::<u8>().unwrap() - 1u8,
        to: it.next().unwrap().parse::<u8>().unwrap() - 1u8
    }
}

fn parse_stack(s: &str) -> Vec<Vec<char>> {
    let mut layers = s.lines().rev();

    let stack_count = layers.next().unwrap().split_whitespace().collect::<Vec<_>>().len();

    let layers = layers
        // .map(|x| x.chars().collect::<Vec<_>>().chunks(4).map(|y| y.iter().collect::<String>()).to_owned()).collect::<Vec<_>>();
        .map(|x| x
             .chars()
             .collect::<Vec<_>>()
             .chunks(4)
             .map(|y| y
                  .iter()
                  .filter(|c|c.is_alphabetic())
                  .collect::<String>())
             .map(|y| y.chars().next())
             .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut stacks: Vec<Vec<char>> = (0..stack_count).map(|_| Vec::new()).collect();
    // println!("{:#?}", stack_count);
    for layer in &layers {
        for (i, el) in layer.iter().enumerate() {
            if let Some(c) = el {
                stacks[i].push(*c);
            }
        }
    }

    stacks
}
fn main() {
    let input = fs::read_to_string("./data/in").expect("Something went wrong with this file");

    let (stack, moves) = input.split_once("\n\n").unwrap();

    let mut stack  = parse_stack(stack);
    let moves: Vec<_>  = moves.lines().map(|m| parse_move(m)).collect();
    for current_move in moves {
        let range = stack[current_move.from as usize].len()-current_move.amount as usize..;
        let mut boxes = stack[current_move.from as usize].drain(range).collect::<Vec<_>>();
        stack[current_move.to as usize].append(&mut boxes);
        // for _ in 0..current_move.amount
        // {
        //     if let Some(b) = stack[current_move.from as usize].pop()
        //     {
        //         stack[current_move.to as usize].push(b);
        //     }
        // }
    }
    let ans = stack.iter().flat_map(|x| x.last()).collect::<String>();
    println!("Stack: {:?}", ans);
}
