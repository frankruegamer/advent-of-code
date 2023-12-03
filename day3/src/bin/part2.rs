use day3::{get_surrounding_kinds, look_for_appearances, Appearance, Symbol};

const INPUT: &str = include_str!("../../input");

fn main() {
    let lines: Vec<Vec<Appearance>> = INPUT.lines().map(look_for_appearances).collect();

    let mut sum: usize = 0;
    for (n, line) in lines.iter().enumerate() {
        let numbers = get_surrounding_kinds(&lines, n, Appearance::as_number);

        for appearance in line {
            if let Appearance::SymbolKind(Symbol {
                is_gear: true,
                index,
            }) = appearance
            {
                let numbers: Vec<_> = numbers
                    .iter()
                    .filter(|number| number.is_next_to_this_symbol(*index))
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
