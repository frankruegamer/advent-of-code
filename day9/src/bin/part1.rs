use day9::Sequence;

const INPUT: &str = include_str!("../../input");

fn main() {
    let sequences: Vec<_> = INPUT
        .lines()
        .map(|line| line.parse::<Sequence>())
        .collect::<Result<_, _>>()
        .unwrap();
    let result: isize = sequences.iter().map(Sequence::next_element).sum();
    println!("Result {}", result);
}
