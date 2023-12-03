use day2::{Game, Set};

const INPUT: &str = include_str!("../../input");

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
