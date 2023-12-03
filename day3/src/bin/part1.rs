use day3::Appearance;
use day3::Appearance::NumberKind;

const INPUT: &str = include_str!("../../input");

fn main() {
    let lines: Vec<Vec<Appearance>> = INPUT.lines().map(day3::look_for).collect();

    let mut sum: usize = 0;
    for (n, line) in lines.iter().enumerate() {
        let mut symbols: Vec<_> = line.iter().filter_map(Appearance::as_symbol).collect();
        if n > 0 {
            if let Some(prev_line) = lines.get(n - 1) {
                symbols.extend(prev_line.iter().filter_map(Appearance::as_symbol));
            }
        }
        if let Some(next_line) = lines.get(n + 1) {
            symbols.extend(next_line.iter().filter_map(Appearance::as_symbol));
        }

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
