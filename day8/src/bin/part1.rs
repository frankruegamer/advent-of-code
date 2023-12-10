use day8::Direction;

const INPUT: &str = include_str!("../../input");

fn main() {
    let (directions, forks) = day8::parse_input(INPUT);

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
