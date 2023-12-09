use std::collections::HashMap;
use std::iter::{Chain, Cycle};
use std::slice::Iter;
use std::str::FromStr;

const INPUT: &str = include_str!("../../input");

#[derive(Debug)]
struct Directions {
    directions: Vec<Direction>,
}

#[derive(Debug)]
struct ParseMovesErr;

impl FromStr for Directions {
    type Err = ParseMovesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let directions = s
            .chars()
            .map(|s| s.to_string().parse::<Direction>())
            .collect::<Result<_, _>>()
            .map_err(|_| ParseMovesErr)?;
        Ok(Self { directions })
    }
}

impl Directions {
    fn moves_iter(&self) -> Cycle<Iter<'_, Direction>> {
        self.directions.iter().cycle()
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ParseMovesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => Err(ParseMovesErr)?,
        };
        Ok(direction)
    }
}

#[derive(Debug)]
struct Fork {
    name: String,
    left: String,
    right: String,
}

impl FromStr for Fork {
    type Err = ParseMovesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut string = s.to_owned();
        string.retain(|s| !['=', '(', ',', ')'].contains(&s));
        let mut parts = string.split_whitespace();
        let name = parts.next().ok_or(ParseMovesErr)?.to_owned();
        let left = parts.next().ok_or(ParseMovesErr)?.to_owned();
        let right = parts.next().ok_or(ParseMovesErr)?.to_owned();
        Ok(Fork { name, left, right })
    }
}

fn main() {
    let option = INPUT.split_once("\n\n");
    let (first_line, lines) = option.unwrap();
    let directions: Directions = first_line.parse().unwrap();
    let lines = lines.lines();
    let forks: Vec<_> = lines
        .map(|line| line.parse::<Fork>())
        .collect::<Result<_, _>>()
        .unwrap();

    let forks = forks.into_iter().fold(HashMap::new(), |mut map, fork| {
        map.insert(fork.name.clone(), fork);
        map
    });

    let mut current_node = "AAA";
    let mut count = 0;
    for direction in directions.moves_iter().clone() {
        let current_fork = forks.get(current_node).unwrap();
        current_node = match direction {
            Direction::Left => &current_fork.left,
            Direction::Right => &current_fork.right,
        };
        count += 1;
        if current_node == "ZZZ" {
            break;
        }
    }
    println!("Result {}", count);
}
