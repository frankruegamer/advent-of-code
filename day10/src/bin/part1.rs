use std::ops::Add;

const INPUT: &str = include_str!("../../input2");
const SIZE: usize = 5;
type Grid = [[char; SIZE]; SIZE];

#[derive(Debug)]
struct Position {
    pos: (usize, usize),
    symbol: Symbol,
}

impl Add<Direction> for &Position {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        let (current_x, current_y) = self.pos;
        let (direction_x, direction_y) = rhs.vector();
        (
            current_x.checked_add_signed(direction_x).unwrap(),
            current_y.checked_add_signed(direction_y).unwrap(),
        )
    }
}

#[derive(Debug, PartialEq)]
enum Symbol {
    Nothing,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    StartingPosition,
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '.' => Symbol::Nothing,
            '|' => Symbol::NorthSouth,
            '-' => Symbol::EastWest,
            'L' => Symbol::NorthEast,
            'J' => Symbol::NorthWest,
            '7' => Symbol::SouthWest,
            'F' => Symbol::SouthEast,
            'S' => Symbol::StartingPosition,
            _ => panic!("unknown char {}", value),
        }
    }
}

impl Symbol {
    fn string_repr(&self) -> char {
        match self {
            Symbol::Nothing => '.',
            Symbol::NorthSouth => '|',
            Symbol::EastWest => '-',
            Symbol::NorthEast => 'L',
            Symbol::NorthWest => 'J',
            Symbol::SouthWest => '7',
            Symbol::SouthEast => 'F',
            Symbol::StartingPosition => 'S',
        }
    }

    fn connections(&self) -> Option<(Direction, Direction)> {
        match self {
            Symbol::Nothing => None,
            Symbol::NorthSouth => Some((Direction::North, Direction::South)),
            Symbol::EastWest => Some((Direction::East, Direction::West)),
            Symbol::NorthEast => Some((Direction::North, Direction::East)),
            Symbol::NorthWest => Some((Direction::North, Direction::West)),
            Symbol::SouthWest => Some((Direction::South, Direction::West)),
            Symbol::SouthEast => Some((Direction::South, Direction::East)),
            Symbol::StartingPosition => None,
        }
    }

    fn has_connection_to(&self, direction: Direction) -> bool {
        match self.connections() {
            None => false,
            Some((first, second)) => [first, second].contains(&direction),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn vector(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

fn main() {
    // grid is transposed, so that grid[y][x]
    let grid: Grid = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>().try_into().unwrap())
        .collect::<Vec<[char; SIZE]>>()
        .try_into()
        .unwrap();
    let starting_pos = find_starting_pos(&grid);
    println!("{:?}", starting_pos);
    let places = connecting_places(&grid, &starting_pos);
    println!("{:?}", places);
}

fn find_starting_pos(grid: &Grid) -> Position {
    for (x, row) in grid.iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            if Symbol::from(*char) == Symbol::StartingPosition {
                return Position {
                    pos: (x, y),
                    symbol: Symbol::StartingPosition,
                };
            }
        }
    }
    panic!("no starting position found");
}

fn connecting_places(grid: &Grid, pos: &Position) -> (Position, Position) {
    if pos.symbol == Symbol::StartingPosition {
        return starting_connections(grid, pos);
    }

    let (first_connection, second_connection) = pos
        .symbol
        .connections()
        .unwrap_or_else(|| panic!("unexpected symbol at this point {:?}", pos));

    let first = position_in_direction(grid, pos, first_connection);
    let second = position_in_direction(grid, pos, second_connection);
    (first, second)
}

fn starting_connections(grid: &Grid, pos: &Position) -> (Position, Position) {
    let mut positions = Vec::new();

    let mut add_if_possible = |to, opposite| {
        let position = position_in_direction(grid, pos, to);
        if position.symbol.has_connection_to(opposite) {
            positions.push(position);
        }
    };
    add_if_possible(Direction::North, Direction::South);
    add_if_possible(Direction::East, Direction::West);
    add_if_possible(Direction::South, Direction::North);
    add_if_possible(Direction::West, Direction::East);
    if positions.len() != 2 {
        panic!(
            "more than one direction from the starting position {:?}",
            positions
        );
    }
    (positions.swap_remove(0), positions.swap_remove(0))
}

fn position_in_direction(grid: &Grid, from_position: &Position, direction: Direction) -> Position {
    let (x, y) = from_position + direction.clone();
    let symbol = grid[y][x].into();
    Position {
        pos: (x, y),
        symbol,
    }
}
