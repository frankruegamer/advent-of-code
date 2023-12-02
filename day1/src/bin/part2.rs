static INPUT: &str = include_str!("../../input");

const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let sum_of_all_lines: u32 = INPUT.lines().map(get_number).sum();
    println!("Result: {}", sum_of_all_lines)
}

#[derive(Debug, Clone)]
struct Entry {
    index: usize,
    digit: usize,
}

fn get_number(line: &str) -> u32 {
    let mut entries = get_digit_numbers(line);
    for entry in get_string_numbers(line) {
        entries.push(entry);
    }
    let first_digit = get_first_digit(&entries);
    let last_digit = get_last_digit(&entries);
    let combined_number = format!("{}{}", first_digit, last_digit);
    combined_number.parse().unwrap_or_else(|_| panic!("could not parse {} into a number", combined_number))
}

fn get_digit_numbers(line: &str) -> Vec<Entry> {
    let mut first_number = None;
    let mut last_number = None;
    for (index, c) in line.chars().enumerate() {
        if c.is_numeric() {
            let option = Some(Entry { index, digit: format!("{}", c).parse().unwrap() });
            if first_number.is_none() {
                first_number = option.clone();
            }
            last_number = option;
        }
    }
    let first_number = first_number.expect("could not parse number from line");
    let last_number = last_number.unwrap();
    vec![first_number, last_number]
}

fn get_string_numbers(line: &str) -> Vec<Entry> {
    let mut entries = vec![];
    for (digit, string) in DIGITS.iter().enumerate() {
        if let Some(index) = line.find(string) {
            entries.push(Entry{index, digit: digit + 1});
        }
        if let Some(index) = line.rfind(string) {
            entries.push(Entry{index, digit: digit + 1});
        }
    }
    entries
}

fn get_first_digit(entries: &[Entry]) -> usize {
    entries.iter().min_by_key(|e| e.index).expect("no digits for this line").digit
}

fn get_last_digit(entries: &[Entry]) -> usize {
    entries.iter().max_by_key(|e| e.index).expect("no digits for this line").digit
}
