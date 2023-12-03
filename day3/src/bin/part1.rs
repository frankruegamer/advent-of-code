use crate::Appearance::{NumberKind, SymbolKind};
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../input");

#[derive(Debug)]
enum Appearance {
    NumberKind(Number),
    SymbolKind(Symbol),
}

impl Appearance {
    fn as_symbol(&self) -> Option<&Symbol> {
        match self {
            SymbolKind(symbol) => Some(symbol),
            NumberKind(_) => None,
        }
    }
}

#[derive(Debug)]
struct Number {
    number: u16,
    index_range: RangeInclusive<usize>,
}

impl Number {
    fn is_next_to_this_symbol(&self, index: usize) -> bool {
        self.index_range.contains(&index)
    }
}

struct NumberBuilder {
    start_index: usize,
    tmp_number: String,
}

impl NumberBuilder {
    fn new(start_index: usize) -> NumberBuilder {
        NumberBuilder {
            start_index,
            tmp_number: String::new(),
        }
    }

    fn append(&mut self, char: char) {
        self.tmp_number.push(char);
    }

    fn build(self, end_index: usize) -> Number {
        Number {
            number: self.tmp_number.parse().unwrap(),
            index_range: RangeInclusive::new(self.start_index.saturating_sub(1), end_index),
        }
    }
}

#[derive(Debug)]
struct Symbol {
    index: usize,
}

fn main() {
    let lines: Vec<Vec<Appearance>> = INPUT.lines().map(look_for).collect();

    let mut sum: usize = 0;
    for (n, line) in lines.iter().enumerate() {
        let mut symbols: Vec<_> = line.iter().filter_map(Appearance::as_symbol).collect();
        // TODO better check for previous line
        if let Some(prev_line) = lines.get(n.overflowing_sub(1).0) {
            symbols.extend(prev_line.iter().filter_map(Appearance::as_symbol));
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

fn look_for(line: &str) -> Vec<Appearance> {
    let mut appearances = Vec::new();
    let mut number_builder: Option<NumberBuilder> = None;
    for (index, char) in line.chars().enumerate() {
        if char.is_numeric() {
            if number_builder.is_none() {
                number_builder = Some(NumberBuilder::new(index));
            }
            number_builder.as_mut().unwrap().append(char);
        } else {
            if let Some(builder) = number_builder {
                let number = builder.build(index);
                number_builder = None;
                appearances.push(NumberKind(number));
            }
            if char != '.' {
                appearances.push(SymbolKind(Symbol { index }));
            }
        }
    }
    appearances
}
