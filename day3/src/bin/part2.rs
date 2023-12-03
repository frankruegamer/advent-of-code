use day3::{Appearance, Symbol};

const INPUT: &str = include_str!("../../input");

fn main() {
    let lines: Vec<Vec<Appearance>> = INPUT.lines().map(day3::look_for).collect();

    let mut sum: usize = 0;
    for (n, line) in lines.iter().enumerate() {
        let mut numbers: Vec<_> = line.iter().filter_map(Appearance::as_number).collect();
        if n > 0 {
            if let Some(prev_line) = lines.get(n - 1) {
                numbers.extend(prev_line.iter().filter_map(Appearance::as_number));
            }
        }
        if let Some(next_line) = lines.get(n + 1) {
            numbers.extend(next_line.iter().filter_map(Appearance::as_number));
        }
        for appearance in line {
            if let Appearance::SymbolKind(Symbol {
                is_gear: true,
                index,
            }) = appearance
            {
                let numbers: Vec<_> = numbers
                    .iter()
                    .filter(|number| number.is_next_to_this_symbol(*index))
                    .copied()
                    .collect();

                if numbers.len() == 2 {
                    sum += numbers
                        .iter()
                        .map(|number| number.number as usize)
                        .product::<usize>();
                }
            }
        }
    }
    println!("Result: {}", sum);
}
