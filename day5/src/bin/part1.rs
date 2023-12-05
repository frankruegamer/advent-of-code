use day5::MappingSet;

const INPUT: &str = include_str!("../../input");

fn main() {
    let mut blocks = INPUT.split("\n\n");
    let seeds: Vec<usize> = blocks
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .expect("no prefix")
        .split_whitespace()
        .map(|seed| seed.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    let mapping_sets: Vec<MappingSet> = blocks
        .map(|block| block.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    let min_location = seeds
        .iter()
        .map(|seed| {
            let location = mapping_sets
                .iter()
                .fold(*seed, |acc, set| set.get_destination_value(acc));
            location
        })
        .min()
        .unwrap();

    println!("Result: {}", min_location);
}
