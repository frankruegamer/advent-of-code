use day6::Race;

const INPUT: &str = include_str!("../../input");

fn main() {
    let mut lines = INPUT.lines();
    let time = parse_line(lines.next().unwrap());
    let distance = parse_line(lines.next().unwrap());
    let result = Race { time, distance }.solve();
    println!("Result: {}", result);
}

fn parse_line(line: &str) -> usize {
    line.split_once(':')
        .map(|(_, line)| line.split_whitespace().collect::<String>())
        .and_then(|line| line.parse().ok())
        .unwrap()
}
