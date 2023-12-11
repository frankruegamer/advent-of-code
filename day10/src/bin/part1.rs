use std::ops::Add;

const INPUT: &str = include_str!("../../input");
const SIZE: usize = 140;
type Grid = [[char; SIZE]; SIZE];

#[derive(Debug, Clone)]
struct Position {
    pos: (usize, usize),
    symbol: Symbol,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Add<Direction> for &Position {
    type Output = Option<(usize, usize)>;

    fn add(self, rhs: Direction) -> Self::Output {
        let (current_x, current_y) = self.pos;
        let (direction_x, direction_y) = rhs.vector();
        Some((
            current_x.checked_add_signed(direction_x)?,
            current_y.checked_add_signed(direction_y)?,
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    let grid = {
        let mut grid: Grid = [['.'; SIZE]; SIZE];
        for (y, line) in INPUT.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                grid[x][y] = char;
            }
        }
        grid
    };

    let mut counter: usize = 0;
    let starting_pos = find_starting_pos(&grid);
    let mut previous_pos = starting_pos.clone();
    let (mut current_pos, _) = connecting_places(&grid, &starting_pos);

    while current_pos != starting_pos {
        let (connection1, connection2) = connecting_places(&grid, &current_pos);
        let next_position = vec![connection1, connection2]
            .into_iter()
            .find(|con| *con != previous_pos)
            .unwrap();
        previous_pos = current_pos;
        current_pos = next_position;
        counter += 1;
    }

    let result = (counter + 1).div_ceil(2);
    println!("Result {}", result);
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

    let first = position_in_direction(grid, pos, first_connection).unwrap();
    let second = position_in_direction(grid, pos, second_connection).unwrap();
    (first, second)
}

fn starting_connections(grid: &Grid, pos: &Position) -> (Position, Position) {
    // TODO fix if S is at border
    let mut positions = Vec::new();

    let mut add_if_possible = |to, opposite| {
        if let Some(position) = position_in_direction(grid, pos, to) {
            if position.symbol.has_connection_to(opposite) {
                positions.push(position);
            }
        }
    };
    add_if_possible(Direction::North, Direction::South);
    add_if_possible(Direction::East, Direction::West);
    add_if_possible(Direction::South, Direction::North);
    add_if_possible(Direction::West, Direction::East);
    if positions.len() != 2 {
        panic!(
            "more than one direction from the starting {:?} position {:?}",
            pos, positions
        );
    }
    (positions.swap_remove(0), positions.swap_remove(0))
}

fn position_in_direction(
    grid: &Grid,
    from_position: &Position,
    direction: Direction,
) -> Option<Position> {
    let (x, y) = (from_position + direction.clone())?;
    let symbol = grid[x][y].into();
    Some(Position {
        pos: (x, y),
        symbol,
    })
}

#[cfg(test)]
mod test {
    use crate::Symbol::Nothing;
    use crate::{Direction, Position};

    #[test]
    fn test_add_direction_to_position() {
        let position = Position {
            pos: (0, 0),
            symbol: Nothing,
        };
        assert_eq!(&position + Direction::North, None);
        assert_eq!(&position + Direction::East, Some((1, 0)));
        assert_eq!(&position + Direction::South, Some((0, 1)));
        assert_eq!(&position + Direction::West, None);

        let position = Position {
            pos: (1, 1),
            symbol: Nothing,
        };
        assert_eq!(&position + Direction::North, Some((1, 0)));
        assert_eq!(&position + Direction::East, Some((2, 1)));
        assert_eq!(&position + Direction::South, Some((1, 2)));
        assert_eq!(&position + Direction::West, Some((0, 1)));
    }
}
