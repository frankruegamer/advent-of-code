use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
pub struct Seeds {
    pub seeds: Vec<usize>,
}

#[derive(Debug)]
pub struct ParseMappingError;

impl FromStr for Seeds {
    type Err = ParseMappingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seeds = s.strip_prefix("seeds: ").ok_or(ParseMappingError)?;
        let seeds = seeds
            .split_whitespace()
            .map(|seed| seed.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| ParseMappingError)?;

        Ok(Self { seeds })
    }
}

#[derive(Debug)]
pub struct MappingSet {
    _name: String,
    mappings: Vec<Mapping>,
}

impl FromStr for MappingSet {
    type Err = ParseMappingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let header = lines.next().ok_or(ParseMappingError)?;

        let mappings: Vec<_> = lines.map(|line| line.parse()).collect::<Result<_, _>>()?;

        Ok(Self {
            _name: header.to_owned(),
            mappings,
        })
    }
}

impl MappingSet {
    pub fn get_destination_value(&self, source: usize) -> usize {
        self.mappings
            .iter()
            .filter_map(|mapping| mapping.get_destination_value(source))
            .next()
            .unwrap_or(source)
    }
}

#[derive(Debug)]
struct Mapping {
    source_range: Range<usize>,
    target: usize,
}

impl FromStr for Mapping {
    type Err = ParseMappingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let target = iter
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or(ParseMappingError)?;
        let source_start = iter
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or(ParseMappingError)?;
        let range_length = iter
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or(ParseMappingError)?;
        Ok(Self::new(target, source_start, range_length))
    }
}

impl Mapping {
    fn new(target: usize, source_start: usize, range_length: usize) -> Self {
        Mapping {
            source_range: source_start..(source_start + range_length),
            target,
        }
    }

    fn get_destination_value(&self, source: usize) -> Option<usize> {
        if self.source_range.contains(&source) {
            Some((source - self.source_range.start) + self.target)
        } else {
            None
        }
    }
}

pub fn get_min_location(seeds: &[usize], mapping_sets: &[MappingSet]) -> usize {
    seeds
        .iter()
        .map(|seed| {
            let location = mapping_sets
                .iter()
                .fold(*seed, |acc, set| set.get_destination_value(acc));
            location
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{Mapping, MappingSet};

    #[test]
    fn test() {
        let source_to_destinations: HashMap<usize, usize> = [
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
            (8, 8),
            (9, 9),
            (10, 10),
            (11, 11),
            (12, 12),
            (13, 13),
            (14, 14),
            (15, 15),
            (16, 16),
            (17, 17),
            (18, 18),
            (19, 19),
            (20, 20),
            (21, 21),
            (22, 22),
            (23, 23),
            (24, 24),
            (25, 25),
            (26, 26),
            (27, 27),
            (28, 28),
            (29, 29),
            (30, 30),
            (31, 31),
            (32, 32),
            (33, 33),
            (34, 34),
            (35, 35),
            (36, 36),
            (37, 37),
            (38, 38),
            (39, 39),
            (40, 40),
            (41, 41),
            (42, 42),
            (43, 43),
            (44, 44),
            (45, 45),
            (46, 46),
            (47, 47),
            (48, 48),
            (49, 49),
            (50, 52),
            (51, 53),
            (52, 54),
            (53, 55),
            (54, 56),
            (55, 57),
            (56, 58),
            (57, 59),
            (58, 60),
            (59, 61),
            (60, 62),
            (61, 63),
            (62, 64),
            (63, 65),
            (64, 66),
            (65, 67),
            (66, 68),
            (67, 69),
            (68, 70),
            (69, 71),
            (70, 72),
            (71, 73),
            (72, 74),
            (73, 75),
            (74, 76),
            (75, 77),
            (76, 78),
            (77, 79),
            (78, 80),
            (79, 81),
            (80, 82),
            (81, 83),
            (82, 84),
            (83, 85),
            (84, 86),
            (85, 87),
            (86, 88),
            (87, 89),
            (88, 90),
            (89, 91),
            (90, 92),
            (91, 93),
            (92, 94),
            (93, 95),
            (94, 96),
            (95, 97),
            (96, 98),
            (97, 99),
            (98, 50),
            (99, 51),
            (100, 100),
        ]
        .into();

        let mapping = Mapping::new(50, 98, 2);
        let mapping2 = Mapping::new(52, 50, 48);
        let set = MappingSet {
            _name: "water-to-soil".to_string(),
            mappings: vec![mapping, mapping2],
        };

        for (source, destination) in source_to_destinations {
            assert_eq!(set.get_destination_value(source), destination);
        }
    }
}
