use std::collections::HashSet;

use derive_more::{Add, AddAssign, Rem, Sub};

#[derive(Add, AddAssign, Sub, Rem, Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Minute(usize);

#[derive(Add, Sub, Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Location {
    col: usize,
    row: usize,
}

type Valley = Vec<Vec<char>>;

enum Traveling {
    North,
    South,
    East,
    West,
}

struct Blizzard {
    origin_row: usize,
    origin_col: usize,
    traveling: Traveling,
}

impl Blizzard {
    fn new(col: usize, row: usize, traveling: Traveling) -> Blizzard {
        Blizzard {
            origin_col: col - 1,
            origin_row: row - 1,
            traveling: traveling,
        }
    }

    fn location(&self, minute: Minute, valley: &Valley) -> Location {
        let innerh = valley.len() - 2;
        let innerw = valley[0].len() - 2;
        let minuteh = minute.0 % innerh;
        let minutew = minute.0 % innerw;

        match self.traveling {
            Traveling::North => Location {
                col: self.origin_col + 1,
                row: (innerh + self.origin_row - minuteh) % innerh + 1,
            },
            Traveling::South => Location {
                col: self.origin_col + 1,
                row: (innerh + self.origin_row + minuteh) % innerh + 1,
            },
            Traveling::East => Location {
                col: (innerw + self.origin_col + minutew) % innerw + 1,
                row: self.origin_row + 1,
            },
            Traveling::West => Location {
                col: (innerw + self.origin_col - minutew) % innerw + 1,
                row: self.origin_row + 1,
            },
        }
    }

    fn as_char(&self) -> char {
        match self.traveling {
            Traveling::North => '^',
            Traveling::South => 'v',
            Traveling::East => '>',
            Traveling::West => '<',
        }
    }
}

fn parse_map(s: &String) -> (Vec<Blizzard>, Vec<Vec<char>>) {
    let mut valley = Valley::new();
    let mut blizzards = Vec::new();

    for (row, line) in s.lines().enumerate() {
        let mut valley_row = Vec::new();
        for (col, char) in line.chars().enumerate() {
            match char {
                '#' => valley_row.push('#'),
                '.' => valley_row.push('.'),
                '^' => {
                    valley_row.push('.');
                    blizzards.push(Blizzard::new(col, row, Traveling::North))
                }
                '>' => {
                    valley_row.push('.');
                    blizzards.push(Blizzard::new(col, row, Traveling::East))
                }
                'v' => {
                    valley_row.push('.');
                    blizzards.push(Blizzard::new(col, row, Traveling::South))
                }
                '<' => {
                    valley_row.push('.');
                    blizzards.push(Blizzard::new(col, row, Traveling::West))
                }
                c => panic!("Unknown character in map: '{}'", c),
            }
        }
        valley.push(valley_row);
    }

    (blizzards, valley)
}

fn print_map(valley: &Valley, blizzards: &Vec<Blizzard>, minute: Minute) {
    let h = valley.len();
    let w = valley[0].len();

    let mut blizzard_locations: Vec<Vec<Vec<char>>> = vec![vec![Vec::new(); w]; h];

    for blizzard in blizzards.iter() {
        let location = blizzard.location(minute, valley);
        blizzard_locations[location.row][location.col].push(blizzard.as_char());
    }

    println!("Minute: {}", minute.0);
    for (row, valley_row) in valley.iter().enumerate() {
        for (col, valley_spot) in valley_row.iter().enumerate() {
            if valley_spot == &'#' {
                print!("#");
            } else if blizzard_locations[row][col].len() > 1 {
                print!("{}", blizzard_locations[row][col].len());
            } else if blizzard_locations[row][col].len() == 1 {
                print!("{}", blizzard_locations[row][col][0]);
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn part1(
    mut minute: Minute,
    start: Location,
    end: Location,
    blizzards: &Vec<Blizzard>,
    valley: &Valley,
) -> Minute {
    let n_blizzard_states = Minute((valley.len() - 2) * (valley[0].len() - 2));

    let mut locations: HashSet<Location> = HashSet::new();
    locations.insert(start);

    let mut seen_states: HashSet<(Minute, Location)> = HashSet::new();
    seen_states.insert((minute, start));

    loop {
        let mut next_locations: HashSet<Location> = HashSet::new();

        let next_minute = Minute((minute + Minute(1)).0 % n_blizzard_states.0);

        let next_blizzard_locations: Vec<Location> = blizzards
            .iter()
            .map(|blizzard| blizzard.location(minute + Minute(1), &valley))
            .collect();

        // println!("{minute:?}");
        for location in locations.iter() {
            if location == &end {
                println!("Found the end at {:?}!", minute);
                return minute;
            }

            // println!("  {location:?}");

            // Up
            if location.row != 0 {
                let next_location = *location - Location { col: 0, row: 1 };
                if true
                    && !next_blizzard_locations.contains(&next_location)
                    && !seen_states.contains(&(next_minute, next_location))
                    && valley[next_location.row][next_location.col] != '#'
                {
                    next_locations.insert(next_location);
                    seen_states.insert((next_minute, next_location));
                }
            }

            // Down
            if location.row != valley.len() - 1 {
                let next_location = *location + Location { col: 0, row: 1 };
                if true
                    && !next_blizzard_locations.contains(&next_location)
                    && !seen_states.contains(&(next_minute, next_location))
                    && valley[next_location.row][next_location.col] != '#'
                {
                    next_locations.insert(next_location);
                    seen_states.insert((next_minute, next_location));
                }
            }

            // Left
            let next_location = *location - Location { col: 1, row: 0 };
            if true
                && !next_blizzard_locations.contains(&next_location)
                && !seen_states.contains(&(next_minute, next_location))
                && valley[next_location.row][next_location.col] != '#'
            {
                next_locations.insert(next_location);
                seen_states.insert((next_minute, next_location));
            }

            // Right
            let next_location = *location + Location { col: 1, row: 0 };
            if true
                && !next_blizzard_locations.contains(&next_location)
                && !seen_states.contains(&(next_minute, next_location))
                && valley[next_location.row][next_location.col] != '#'
            {
                next_locations.insert(next_location);
                seen_states.insert((next_minute, next_location));
            }

            // Stay
            let next_location = *location;
            if true
                && !next_blizzard_locations.contains(&next_location)
                && !seen_states.contains(&(next_minute, next_location))
                && valley[next_location.row][next_location.col] != '#'
            {
                next_locations.insert(next_location);
                seen_states.insert((next_minute, next_location));
            }
        }

        minute += Minute(1);
        locations = next_locations;
    }

    print_map(&valley, &blizzards, Minute(0));
    print_map(&valley, &blizzards, Minute(24));
    println!("");
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let (blizzards, valley) = parse_map(&s);
    let start = Location {
        row: 0,
        col: valley[0].iter().position(|p| p == &'.').unwrap(),
    };
    let end = Location {
        row: valley.len() - 1,
        col: valley
            .last()
            .unwrap()
            .iter()
            .position(|p| p == &'.')
            .unwrap(),
    };

    let first_trip = part1(Minute(0), start, end, &blizzards, &valley);
    let backtrack = part1(first_trip, end, start, &blizzards, &valley);
    let return_trip = part1(backtrack, start, end, &blizzards, &valley);


    Ok(())
}
