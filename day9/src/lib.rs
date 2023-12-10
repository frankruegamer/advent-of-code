use std::str::FromStr;

#[derive(Debug)]
pub struct Sequence(pub Vec<isize>);

#[derive(Debug)]
pub struct ParseSequenceError;

impl FromStr for Sequence {
    type Err = ParseSequenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sequence = s
            .split_whitespace()
            .map(|num| num.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| ParseSequenceError)?;
        Ok(Self(sequence))
    }
}

impl Sequence {
    pub fn next_element(&self) -> isize {
        Self::next_element_of_sequence(&self.0)
    }

    pub fn previous_element(&self) -> isize {
        Self::previous_element_of_sequence(&self.0)
    }

    fn next_element_of_sequence(sequence: &[isize]) -> isize {
        let differences: Vec<_> = Self::get_differences(sequence);
        if differences.iter().all(|num| *num == 0) {
            *sequence.last().unwrap()
        } else {
            *sequence.last().unwrap() + Self::next_element_of_sequence(&differences)
        }
    }

    fn previous_element_of_sequence(sequence: &[isize]) -> isize {
        let differences: Vec<_> = Self::get_differences(sequence);
        if differences.iter().all(|num| *num == 0) {
            *sequence.first().unwrap()
        } else {
            *sequence.first().unwrap() - Self::previous_element_of_sequence(&differences)
        }
    }

    fn get_differences(sequence: &[isize]) -> Vec<isize> {
        sequence
            .windows(2)
            .map(|pairs| {
                let [left, right]: [_; 2] = pairs.try_into().unwrap();
                right - left
            })
            .collect()
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Sequence>, ParseSequenceError> {
    input
        .lines()
        .map(|line| line.parse::<Sequence>())
        .collect::<Result<_, _>>()
}
