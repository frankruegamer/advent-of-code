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

        let times = times
            .strip_prefix("Time:")
            .map(|line| line.split_whitespace())
            .ok_or(ParseRacesError)?;
        let distances = distances
            .strip_prefix("Distance:")
            .map(|line| line.split_whitespace())
            .ok_or(ParseRacesError)?;
        let races = times
            .zip(distances)
            .map(|(time, distance)| Race {
                time: time.parse().unwrap(),
                distance: distance.parse().unwrap(),
            })
            .collect();

        Ok(Races(races))
    }
}

impl Races {
    fn multiply(&self) -> usize {
        self.0.iter().map(Race::solve).product()
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
