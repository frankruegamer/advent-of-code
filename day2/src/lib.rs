use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    pub number: u8,
    pub sets: Vec<Set>,
}

#[derive(Debug)]
pub struct ParseLineError;

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
    pub fn is_possible(&self, set: &Set) -> bool {
        self.sets.iter().all(|s| s.is_possible(set))
    }
}

#[derive(Debug)]
pub struct Set {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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
