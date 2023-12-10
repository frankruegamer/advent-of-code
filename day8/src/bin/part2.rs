use day8::Map;

const INPUT: &str = include_str!("../../input");

fn main() {
    let map: Map = INPUT.parse().unwrap();

    let start_keys: Vec<_> = map.forks.keys().filter(|key| is_start_key(key)).collect();
    let results: Vec<_> = start_keys
        .iter()
        .map(|key| map.get_earliest_goal(key, |key| is_end_key(key)))
        .collect();
    let result = least_common_multiple(&results);
    println!("Result {}", result);
}

fn is_start_key(key: impl AsRef<str>) -> bool {
    key.as_ref().ends_with('A')
}

fn is_end_key(key: impl AsRef<str>) -> bool {
    key.as_ref().ends_with('Z')
}

fn least_common_multiple(numbers: &[usize]) -> usize {
    let all_divisors: Vec<_> = numbers.iter().map(get_divisors).collect();
    let mut common_factors = Vec::new();
    for mut divisors in all_divisors {
        for factor in &common_factors {
            if let Some(index) = divisors.iter().position(|num| num == factor) {
                divisors.remove(index);
            }
        }
        common_factors.append(&mut divisors);
    }
    common_factors.iter().product()
}

fn get_divisors(number: &usize) -> Vec<usize> {
    let divisors = divisors::get_divisors(*number);
    // number should not be part of divisors, except 2 which seems to be a bug
    if divisors.is_empty() {
        vec![*number]
    } else {
        divisors
    }
}
