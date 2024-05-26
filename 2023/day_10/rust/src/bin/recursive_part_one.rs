// didn't end with this solution cause it stack overflowed in debug..

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn apply_to_coords(&self, (row, col): (usize, usize)) -> (usize, usize) {
        match self {
            Self::North => (row - 1, col),
            Self::South => (row + 1, col),
            Self::West => (row, col - 1),
            Self::East => (row, col + 1),
        }
    }
}

impl std::ops::Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }
}

fn main() {
    let points: Vec<Vec<char>> = include_str!("../../../input.txt")
        .lines()
        .enumerate()
        .map(|line| line.1.char_indices().map(|c| (c.1)).collect())
        .collect();

    let starting_coords = points
        .iter()
        .enumerate()
        .find_map(|(row, chars)| chars.iter().position(|c| *c == 'S').map(|col| (row, col)))
        .unwrap();

    let dir = starting_dir(&points, &starting_coords);
    let mut path = Vec::from([starting_coords]);

    traverse(
        &points,
        &dir.apply_to_coords(starting_coords),
        dir,
        &mut path,
    );

    println!("steps to max point: {}", path.len() / 2);
}

fn tile_to_possible_dirs(tile: char) -> Option<(Direction, Direction)> {
    match tile {
        '|' => Some((Direction::North, Direction::South)),
        '-' => Some((Direction::East, Direction::West)),
        'L' => Some((Direction::North, Direction::East)),
        'J' => Some((Direction::North, Direction::West)),
        '7' => Some((Direction::South, Direction::West)),
        'F' => Some((Direction::South, Direction::East)),
        _ => None,
    }
}

fn exclude(dirs: &(Direction, Direction), dir: Direction) -> Option<Direction> {
    if dirs.0 == dir {
        Some(dirs.1)
    } else if dirs.1 == dir {
        Some(dirs.0)
    } else {
        None
    }
}

fn starting_dir(points: &Vec<Vec<char>>, (row, col): &(usize, usize)) -> Direction {
    let get = |row: usize, col: usize, dir: Direction| {
        points
            .get(row)
            .and_then(|row| row.get(col))
            .and_then(|tile| tile_to_possible_dirs(*tile))
            .and_then(|paths| exclude(&paths, dir))
    };

    let north = row
        .checked_sub(1)
        .and_then(|row| get(row, *col, !Direction::North));
    let south = get(row + 1, *col, !Direction::South);
    let west = col
        .checked_sub(1)
        .and_then(|col| get(*row, col, !Direction::West));
    let east = get(*row, col + 1, !Direction::East);

    north.or(south).or(west).or(east).unwrap()
}

fn traverse(
    points: &Vec<Vec<char>>,
    (row, col): &(usize, usize),
    prev_dir: Direction,
    path: &mut Vec<(usize, usize)>,
) {
    path.push((*row, *col));

    let tile = points[*row][*col];

    if tile == 'S' {
        return;
    }

    let possible_dirs = tile_to_possible_dirs(tile).unwrap();
    let next_dir = exclude(&possible_dirs, !prev_dir).unwrap();

    traverse(
        points,
        &next_dir.apply_to_coords((*row, *col)),
        next_dir,
        path,
    )
}
