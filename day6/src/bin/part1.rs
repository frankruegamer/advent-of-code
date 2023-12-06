use day6::Races;

const INPUT: &str = include_str!("../../input");

fn main() {
    let result = INPUT.parse::<Races>().expect("error parsing").multiply();
    println!("Result: {}", result);
}
