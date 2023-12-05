use day5::{get_min_location, MappingSet, Seeds};

const INPUT: &str = include_str!("../../input");

fn main() {
    let mut blocks = INPUT.split("\n\n");
    let seeds = blocks.next().unwrap().parse::<Seeds>().unwrap().seeds;

    let mapping_sets: Vec<MappingSet> = blocks
        .map(|block| block.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    let min_location = get_min_location(&seeds, &mapping_sets);

    println!("Result: {}", min_location);
}
