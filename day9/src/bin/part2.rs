use day9::{parse_input, Sequence};

const INPUT: &str = include_str!("../../input");

fn main() {
    let sequences: Vec<_> = parse_input(INPUT).unwrap();
    let result: isize = sequences.iter().map(Sequence::previous_element).sum();
    println!("Result {}", result);
}
