use std::collections::HashMap;
use day4::ScratchCard;

const INPUT: &str = include_str!("../../input");

fn main() {
    let cards: Vec<_> = INPUT
        .lines()
        .map(|line| line.parse::<ScratchCard>().expect("could not parse line"))
        .collect();

    let mut total_cards = 0;
    let mut copies: HashMap<u8, usize> = HashMap::new();
    for card in cards {
        let winning_numbers = card.count_winning_numbers();
        let times = copies.get(&card.number).unwrap_or(&0) + 1;
        total_cards += times;
        let new_numbers = (card.number + 1)..=(card.number + winning_numbers);
        for _ in 0..times {
            for new_copy in new_numbers.clone() {
                *copies.entry(new_copy).or_default() += 1
            }
        }
    }

    println!("Result {}", total_cards);
}
