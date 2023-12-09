use day8::{Direction, Directions, Fork};
use std::collections::HashMap;

const INPUT: &str = include_str!("../../input");

fn main() {
    let option = INPUT.split_once("\n\n");
    let (first_line, lines) = option.unwrap();
    let directions: Directions = first_line.parse().unwrap();
    let lines = lines.lines();
    let forks: Vec<_> = lines
        .map(|line| line.parse::<Fork>())
        .collect::<Result<_, _>>()
        .unwrap();

    let forks = forks.into_iter().fold(HashMap::new(), |mut map, fork| {
        map.insert(fork.name.clone(), fork);
        map
    });

    let mut current_node = "AAA";
    let mut count = 0;
    for direction in directions.moves_iter().clone() {
        let current_fork = forks.get(current_node).unwrap();
        current_node = match direction {
            Direction::Left => &current_fork.left,
            Direction::Right => &current_fork.right,
        };
        count += 1;
        if current_node == "ZZZ" {
            break;
        }
    }
    println!("Result {}", count);
}
