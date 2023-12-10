use std::collections::HashMap;
use std::iter::Cycle;
use std::slice::Iter;
use std::str::FromStr;

#[derive(Debug)]
pub struct Directions {
    directions: Vec<Direction>,
}

#[derive(Debug)]
pub struct ParseMovesErr;

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
    pub fn moves_iter(&self) -> Cycle<Iter<'_, Direction>> {
        self.directions.iter().cycle()
    }
}

#[derive(Debug)]
pub enum Direction {
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
pub struct Fork {
    pub name: String,
    pub left: String,
    pub right: String,
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

pub fn parse_input(input: &str) -> (Directions, HashMap<String, Fork>) {
    let option = input.split_once("\n\n");
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
    (directions, forks)
}
