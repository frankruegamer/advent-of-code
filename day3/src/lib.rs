use std::ops::RangeInclusive;

#[derive(Debug)]
pub enum Appearance {
    NumberKind(Number),
    SymbolKind(Symbol),
}

impl Appearance {
    pub fn as_symbol(&self) -> Option<&Symbol> {
        match self {
            Appearance::SymbolKind(symbol) => Some(symbol),
            Appearance::NumberKind(_) => None,
        }
    }
}

#[derive(Debug)]
pub struct Number {
    pub number: u16,
    index_range: RangeInclusive<usize>,
}

impl Number {
    pub fn is_next_to_this_symbol(&self, index: usize) -> bool {
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
pub struct Symbol {
    pub index: usize,
}

pub fn look_for(line: &str) -> Vec<Appearance> {
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
                appearances.push(Appearance::NumberKind(number));
            }
            if char != '.' {
                appearances.push(Appearance::SymbolKind(Symbol { index }));
            }
        }
    }
    appearances
}
