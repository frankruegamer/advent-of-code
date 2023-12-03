use day3::Appearance::NumberKind;
use day3::{get_surrounding_kinds, look_for_appearances, Appearance};

const INPUT: &str = include_str!("../../input");

fn main() {
    let lines: Vec<Vec<Appearance>> = INPUT.lines().map(look_for_appearances).collect();

    let mut sum: usize = 0;
    for (n, line) in lines.iter().enumerate() {
        let symbols = get_surrounding_kinds(&lines, n, Appearance::as_symbol);

        for appearance in line {
            if let NumberKind(number) = appearance {
                if symbols
                    .iter()
                    .any(|symbol| number.is_next_to_this_symbol(symbol.index))
                {
                    sum += number.number as usize;
                }
            }
        }
    }

    println!("Result: {}", sum);
}
