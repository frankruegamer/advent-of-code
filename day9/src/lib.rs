use std::str::FromStr;

#[derive(Debug)]
pub struct Sequence(Vec<isize>);

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

    fn next_element_of_sequence(sequence: &[isize]) -> isize {
        let differences: Vec<_> = sequence
            .windows(2)
            .map(|pairs| {
                let [left, right]: [_; 2] = pairs.try_into().unwrap();
                right - left
            })
            .collect();

        if differences.iter().all(|num| *num == 0) {
            *sequence.last().unwrap()
        } else {
            *sequence.last().unwrap() + Self::next_element_of_sequence(&differences)
        }
    }
}
