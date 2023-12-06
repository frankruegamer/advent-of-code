use std::str::FromStr;

const INPUT: &str = include_str!("../../input");

struct Races(Vec<Race>);

#[derive(Debug)]
pub struct ParseRacesError;

impl FromStr for Races {
    type Err = ParseRacesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let times = lines.next().ok_or(ParseRacesError)?;
        let distances = lines.next().ok_or(ParseRacesError)?;

        let times = Self::parse_line(times)?;
        let distances = Self::parse_line(distances)?;
        let races = times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect();

        Ok(Races(races))
    }
}

impl Races {
    fn multiply(&self) -> usize {
        self.0.iter().map(Race::solve).product()
    }

    fn parse_line(times: &str) -> Result<Vec<usize>, <Races as FromStr>::Err> {
        let whitespace = times
            .split_once(':')
            .map(|(_, line)| line.split_whitespace())
            .ok_or(ParseRacesError)?;
        let vec: Vec<usize> = whitespace
            .map(|number| number.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| ParseRacesError)?;
        Ok(vec)
    }
}

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn solve(&self) -> usize {
        (1..self.time)
            .map(|charging_time| charging_time * (self.time - charging_time))
            .filter(|distance| *distance > self.distance)
            .count()
    }
}

fn main() {
    let result = INPUT.parse::<Races>().expect("error parsing").multiply();
    println!("Result: {}", result);
}
