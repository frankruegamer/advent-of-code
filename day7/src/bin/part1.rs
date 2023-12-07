use day7::Hand;

const INPUT: &str = include_str!("../../input");

fn main() {
    let mut hands: Vec<Hand> = INPUT
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .expect("error while parsing");
    hands.sort();
    let sum_of_all_bids: usize = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();
    println!("Result {}", sum_of_all_bids);
}
