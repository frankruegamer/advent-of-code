use day8::Map;

const INPUT: &str = include_str!("../../input");

fn main() {
    let map: Map = INPUT.parse().unwrap();
    let count = map.get_earliest_goal("AAA", |key| key == "ZZZ");
    println!("Result {}", count);
}
