use std::{
    fmt::{Display, Write},
    ops::Range,
};

///      ____T0____
///    /|         /|
///  T3 |       T1 |
///  /  V3      /  V0
/// /____T2____/   |
/// |   |     |    |
/// |   |___B2|____|
/// V2  /     V1  /
/// | B3      | B1
/// |/___B0___|/

enum Edge {
    T0,
    T1,
    T2,
    T3,
    V0,
    V1,
    V2,
    V3,
    B0,
    B1,
    B2,
    B3,
}

enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

struct MapFace {
    col: usize,
    row: usize,
    face: Face,
    edges: [Face; 4],
}

enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn to_char(&self) -> char {
        match self {
            Facing::Right => '>',
            Facing::Down => 'v',
            Facing::Left => '<',
            Facing::Up => '^',
        }
    }

    fn to_usize(&self) -> usize {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }

    fn turn_right(&self) -> Facing {
        match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }

    fn turn_left(&self) -> Facing {
        match self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Advance(usize),
    TurnLeft,
    TurnRight,
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let mut chars = s.chars().peekable();
    loop {
        match chars.next() {
            Some('L') => instructions.push(Instruction::TurnLeft),
            Some('R') => instructions.push(Instruction::TurnRight),
            Some(c) => {
                let mut advance = c.to_string();
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_ascii_digit() {
                        advance.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                instructions.push(Instruction::Advance(advance.parse::<usize>().unwrap()));
            }
            None => {
                return instructions;
            }
        }
    }
}

type Spots = Vec<Vec<char>>;

#[derive(Clone)]
struct Map {
    spots: Spots,
    // For a given column, the first and last+1 row indexes that are valid
    col_ranges: Vec<Range<usize>>,
    // For a given row, the first and last+1 column indexes that are valid
    row_ranges: Vec<Range<usize>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.spots.iter() {
            for c in row.iter() {
                f.write_char(*c)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

struct Character {
    row: usize,
    col: usize,
    facing: Facing,
    face: Face,
}

impl Character {
    fn instruct1(&mut self, instruction: &Instruction, map: &mut Map) {
        match instruction {
            Instruction::Advance(count) => {
                for _ in 0..*count {
                    self.advance_flat_map(map)
                }
            }
            Instruction::TurnLeft => self.facing = self.facing.turn_left(),
            Instruction::TurnRight => self.facing = self.facing.turn_right(),
        }
    }

    fn advance_flat_map(&mut self, map: &mut Map) {
        // first valid column in this row
        let col_start = map.row_ranges[self.row].start;
        // last+1 valid column in this row
        let col_end = map.row_ranges[self.row].end;
        // first valid row in this column
        let row_start = map.col_ranges[self.col].start;
        // last+1 valid row in this column
        let row_end = map.col_ranges[self.col].end;

        let (next_row, next_col) = match self.facing {
            Facing::Right => {
                if self.col + 1 == col_end {
                    (self.row, col_start)
                } else {
                    (self.row, self.col + 1)
                }
            }
            Facing::Down => {
                if self.row + 1 == row_end {
                    (row_start, self.col)
                } else {
                    (self.row + 1, self.col)
                }
            }
            Facing::Left => {
                if self.col == col_start {
                    (self.row, col_end - 1)
                } else {
                    (self.row, self.col - 1)
                }
            }
            Facing::Up => {
                if self.row == row_start {
                    (row_end - 1, self.col)
                } else {
                    (self.row - 1, self.col)
                }
            }
        };

        if map.spots[next_row][next_col] != '#' {
            map.spots[self.row][self.col] = self.facing.to_char();
            (self.row, self.col) = (next_row, next_col);
        }
    }
}

fn parse_map(s: &String) -> (Map, Vec<Instruction>) {
    let mut spots: Spots = s
        .lines()
        .into_iter()
        .take_while(|&line| line.is_empty() == false)
        .map(|line| line.chars().collect())
        .collect();

    let h = spots.len();
    let w = spots
        .iter()
        .fold(0, |max, row| std::cmp::max(max, row.len()));

    spots.iter_mut().for_each(|row| row.resize(w, ' '));

    let row_ranges = spots
        .iter()
        .map(|row| {
            let start = row.iter().position(|spot| *spot != ' ').unwrap();
            let end = row.iter().rev().position(|spot| *spot != ' ').unwrap();
            start..(w - end)
        })
        .collect();

    let mut col_ranges = Vec::new();
    for col in 0..w {
        let mut row = 0;
        while spots[row][col] == ' ' {
            row += 1;
        }
        let start = row;
        while row < h && spots[row][col] != ' ' {
            row += 1;
        }
        let end = row;
        col_ranges.push(start..end);
    }

    let instructions = parse_instructions(
        s.lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .next()
            .unwrap(),
    );

    (
        Map {
            spots,
            col_ranges: col_ranges,
            row_ranges: row_ranges,
        },
        instructions,
    )
}

fn part1(instructions: &Vec<Instruction>, mut map: Map) {
    let mut character = Character {
        col: map.spots[0].iter().position(|spot| *spot == '.').unwrap(),
        row: 0,
        face: Face::Top,
        facing: Facing::Right,
    };

    for instruction in instructions.iter() {
        character.instruct1(instruction, &mut map);
    }
    map.spots[character.row][character.col] = character.facing.to_char();
    println!("{}", map);
    dbg!(1000 * (character.row + 1) + 4 * (character.col + 1) + character.facing.to_usize());
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let (map, instructions) = parse_map(&s);
    println!("{}", map);
    instructions.iter().for_each(|i| println!("{i:?}"));

    part1(&instructions, map.clone());

    Ok(())
}
