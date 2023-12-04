use day4::ScratchCard;

const INPUT: &str = include_str!("../../input");

fn main() {
    let sum_of_results: usize = INPUT
        .lines()
        .map(|line| line.parse::<ScratchCard>().expect("could not parse line"))
        .map(|card| card.get_value())
        .sum();
    println!("Result: {}", sum_of_results);
}
