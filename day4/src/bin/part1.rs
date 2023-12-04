use std::str::FromStr;

const INPUT: &str = include_str!("../../input");

#[derive(Debug)]
struct ScratchCard {
    winning_numbers: Vec<u8>,
    own_numbers: Vec<u8>,
}

#[derive(Debug)]
struct ParseScratchCardError;

impl FromStr for ScratchCard {
    type Err = ParseScratchCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (winning_numbers, own_numbers) = s
            .split_once(':')
            .map(|tuple| tuple.1)
            .and_then(|card| card.split_once('|'))
            .ok_or(ParseScratchCardError)?;

        let winning_numbers = Self::get_numbers(winning_numbers)?;
        let own_numbers = Self::get_numbers(own_numbers)?;
        Ok(ScratchCard {
            winning_numbers,
            own_numbers,
        })
    }
}

impl ScratchCard {
    fn get_numbers(winning_numbers: &str) -> Result<Vec<u8>, <ScratchCard as FromStr>::Err> {
        winning_numbers
            .split_whitespace()
            .map(|number| number.parse().map_err(|_| ParseScratchCardError))
            .collect::<Result<_, _>>()
    }

    fn get_value(&self) -> usize {
        self.own_numbers.iter().fold(0, |acc, e| {
            if self.winning_numbers.contains(e) {
                if acc == 0 {
                    acc + 1
                } else {
                    acc * 2
                }
            } else {
                acc
            }
        })
    }
}

fn main() {
    let sum_of_results: usize = INPUT
        .lines()
        .map(|line| line.parse::<ScratchCard>().expect("could not parse line"))
        .map(|card| card.get_value())
        .sum();
    println!("Result: {}", sum_of_results);
}
