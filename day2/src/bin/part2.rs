use day2::Game;

const INPUT: &str = include_str!("../../input");

fn main() {
    let sum_of_possible_powers: usize = INPUT
        .lines()
        .map(|line| line.parse::<Game>().expect("could not parse"))
        .map(|game| game.minimum_set())
        .map(|set| set.power())
        .sum();
    println!("Result: {}", sum_of_possible_powers);
}
