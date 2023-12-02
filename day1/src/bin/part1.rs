static INPUT: &str = include_str!("../../input");

fn main() {
    let sum_of_all_lines: u32 = INPUT.lines().map(get_number).sum();
    println!("Result: {}", sum_of_all_lines)
}

fn get_number(line: &str) -> u32 {
    let mut first_number = None;
    let mut last_number = None;
    for c in line.chars() {
        if c.is_numeric() {
            if first_number.is_none() {
                first_number = Some(c);
            }
            last_number = Some(c);
        }
    }
    let first_number = first_number.expect("could not parse number from line");
    let last_number = last_number.unwrap();
    let combined_number = format!("{}{}", first_number, last_number);
    combined_number.parse().unwrap_or_else(|_| panic!("could not parse {} into a number", combined_number))
}
