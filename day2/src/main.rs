use std::str::FromStr;

const INPUT: &str = include_str!("../input");

#[derive(Debug)]
struct Game {
    number: u8,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct ParseLineError;

impl FromStr for Game {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (number, sets) = s
            .strip_prefix("Game ")
            .and_then(|s| s.split_once(": "))
            .ok_or(ParseLineError)?;

        let number = number.parse().map_err(|_| ParseLineError)?;
        let sets: Vec<Set> = sets
            .split("; ")
            .map(|set| set.parse::<Set>().map_err(|_| ParseLineError))
            .collect::<Result<_, _>>()?;

        Ok(Game { number, sets })
    }
}

impl Game {
    fn is_possible(&self, set: &Set) -> bool {
        self.sets.iter().all(|s| s.is_possible(set))
    }
}

#[derive(Debug)]
struct Set {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for Set {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for pair in s.split(", ") {
            let (n, color) = pair.split_once(' ').ok_or(ParseLineError)?;
            let n: u8 = n.parse().map_err(|_| ParseLineError)?;
            match color {
                "red" => red += n,
                "green" => green += n,
                "blue" => blue += n,
                _ => Err(ParseLineError)?,
            }
        }

        Ok(Set { red, green, blue })
    }
}

impl Set {
    fn is_possible(&self, set: &Set) -> bool {
        self.red <= set.red && self.green <= set.green && self.blue <= set.blue
    }
}

fn main() {
    let set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum_of_possible_games: usize = INPUT
        .lines()
        .map(|line| line.parse::<Game>().expect("could not parse"))
        .filter(|game| game.is_possible(&set))
        .map(|g| g.number as usize)
        .sum();
    println!("Result: {}", sum_of_possible_games);
}
