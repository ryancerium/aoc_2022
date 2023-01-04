use eyre::eyre;
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

fn parse_cube(spots: &Spots) -> Option<(Vec<CubeFace>, usize)> {
    let mut cubefaces: Vec<CubeFace> = Vec::new();

    let stride = if spots.len() > 50 { 50 } else { 4 };
    let h = spots.len();
    let w = spots[0].len();
    {
        let mut top_face = CubeFace::new(stride, Face::Top, Edge::T0, Edge::T1, Edge::T2, Edge::T3);
        top_face.row = 0;
        top_face.col = spots[0].iter().position(|spot| *spot != ' ')?;
        cubefaces.push(top_face);
    }

    println!("Found {}", cubefaces[0]);

    let mut remaining_faces = vec![
        CubeFace::new(stride, Face::Bottom, Edge::B0, Edge::B1, Edge::B2, Edge::B3),
        CubeFace::new(stride, Face::Front, Edge::T2, Edge::V1, Edge::B0, Edge::V2),
        CubeFace::new(stride, Face::Back, Edge::T0, Edge::V3, Edge::B2, Edge::V0),
        CubeFace::new(stride, Face::Left, Edge::T3, Edge::V2, Edge::B3, Edge::V3),
        CubeFace::new(stride, Face::Right, Edge::T1, Edge::V0, Edge::B1, Edge::V1),
    ];

    while !remaining_faces.is_empty() {
        let mut new_cubefaces = Vec::new();

        for cur_face in cubefaces.iter() {
            // Check the square to the right
            if (cur_face.col + stride) < w && spots[cur_face.row][cur_face.col + stride] != ' ' {
                if let Some(i) = remaining_faces
                    .iter()
                    .position(|cubeface| cubeface.edges.contains(&cur_face.right()))
                {
                    let mut cubeface = remaining_faces.remove(i);
                    cubeface.row = cur_face.row;
                    cubeface.col = cur_face.col + stride;

                    while cubeface.left() != cur_face.right() {
                        cubeface.edges.rotate_right(1);
                    }

                    println!("Found {}", cubeface);
                    new_cubefaces.push(cubeface);
                }
            }

            // Check the square to the left
            if cur_face.col > 0 && spots[cur_face.row][cur_face.col - stride] != ' ' {
                if let Some(i) = remaining_faces
                    .iter()
                    .position(|cubeface| cubeface.edges.contains(&cur_face.left()))
                {
                    let mut cubeface = remaining_faces.remove(i);
                    cubeface.row = cur_face.row;
                    cubeface.col = cur_face.col - stride;
                    while cubeface.right() != cur_face.left() {
                        cubeface.edges.rotate_right(1);
                    }

                    println!("Found {}", cubeface);
                    new_cubefaces.push(cubeface);
                }
            }

            // Check the square below
            if cur_face.row + stride < h && spots[cur_face.row + stride][cur_face.col] != ' ' {
                if let Some(i) = remaining_faces
                    .iter()
                    .position(|cubeface| cubeface.edges.contains(&cur_face.bottom()))
                {
                    let mut cubeface = remaining_faces.remove(i);
                    cubeface.row = cur_face.row + stride;
                    cubeface.col = cur_face.col;
                    while cubeface.top() != cur_face.bottom() {
                        cubeface.edges.rotate_right(1);
                    }

                    println!("Found {}", cubeface);
                    new_cubefaces.push(cubeface);
                }
            }
        }

        cubefaces.append(&mut new_cubefaces);
    }

    Some((cubefaces, stride))
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

#[derive(Copy, Clone)]
struct CubeFace {
    col: usize,
    row: usize,
    stride: usize,
    face: Face,
    edges: [Edge; 4],
}

impl CubeFace {
    fn new(
        stride: usize,
        face: Face,
        top: Edge,
        right: Edge,
        bottom: Edge,
        left: Edge,
    ) -> CubeFace {
        CubeFace {
            col: 0,
            row: 0,
            stride,
            face,
            edges: [top, right, bottom, left],
        }
    }

    fn top(&self) -> Edge {
        self.edges[0]
    }
    fn right(&self) -> Edge {
        self.edges[1]
    }
    fn bottom(&self) -> Edge {
        self.edges[2]
    }
    fn left(&self) -> Edge {
        self.edges[3]
    }

    fn coming_from(&self, edge: Edge, from_left_edge: usize) -> (usize, usize, Face, Facing) {
        // println!("{from_left_edge} on {edge:?}");
        let from_right_edge = self.stride - from_left_edge - 1;
        if edge == self.top() {
            (
                self.col + from_right_edge,
                self.row,
                self.face,
                Facing::Down,
            )
        } else if edge == self.right() {
            (
                self.col + self.stride - 1,
                self.row + from_right_edge,
                self.face,
                Facing::Left,
            )
        } else if edge == self.bottom() {
            (
                self.col + from_left_edge,  //
                self.row + self.stride - 1, //
                self.face,                  //
                Facing::Up,                 //
            )
        } else if edge == self.left() {
            (
                self.col,
                self.row + from_left_edge,
                self.face,
                Facing::Right,
            )
        } else {
            panic!(
                "Asked to come from {edge:?} that face {:?} doesn't have!",
                self.face
            );
        }
    }
}

impl Display for CubeFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "col={}, row={} [top: {:?}, right: {:?}, bottom: {:?}, left: {:?}]  {:?}",
            self.col,
            self.row,
            self.top(),
            self.right(),
            self.bottom(),
            self.left(),
            self.face,
        ))
    }
}

#[derive(Copy, Clone)]
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
    cube: Vec<CubeFace>,
    stride: usize,
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

    fn instruct2(&mut self, instruction: &Instruction, map: &mut Map) {
        match instruction {
            Instruction::Advance(count) => {
                for _ in 0..*count {
                    let _ = self.advance_cube(map);
                }
                // println!("{}", map);
            }
            Instruction::TurnLeft => self.facing = self.facing.turn_left(),
            Instruction::TurnRight => self.facing = self.facing.turn_right(),
        }
    }

    fn advance_cube(&mut self, map: &mut Map) -> Option<()> {
        let current_face = map
            .cube
            .iter()
            .find(|cubeface| cubeface.face == self.face)?;

        let (next_col, next_row, next_face, next_facing) = match self.facing {
            Facing::Right => {
                if self.col + 1 == current_face.col + map.stride {
                    let adjacent_face = map.cube.iter().find(|&cubeface| {
                        cubeface.face != self.face && cubeface.edges.contains(&current_face.right())
                    })?;
                    let from_left_edge = self.row - current_face.row;
                    // println!(
                    //     "{from_left_edge} facing right at {:?} of {:?}",
                    //     current_face.right(),
                    //     current_face.face
                    // );
                    adjacent_face.coming_from(current_face.right(), from_left_edge)
                } else {
                    (self.col + 1, self.row, self.face, self.facing)
                }
            }
            Facing::Down => {
                if self.row + 1 == current_face.row + map.stride {
                    let adjacent_face = map.cube.iter().find(|&cubeface| {
                        cubeface.face != self.face
                            && cubeface.edges.contains(&current_face.bottom())
                    })?;
                    let from_left_edge = current_face.col + map.stride - 1 - self.col;
                    // println!(
                    //     "{from_left_edge} facing down at {:?} of {:?}",
                    //     current_face.bottom(),
                    //     current_face.face
                    // );
                    adjacent_face.coming_from(current_face.bottom(), from_left_edge)
                } else {
                    (self.col, self.row + 1, self.face, self.facing)
                }
            }
            Facing::Left => {
                if self.col == current_face.col {
                    let adjacent_face = map.cube.iter().find(|&cubeface| {
                        cubeface.face != self.face && cubeface.edges.contains(&current_face.left())
                    })?;
                    let from_left_edge = current_face.row + map.stride - 1 - self.row;
                    // println!(
                    //     "{from_left_edge} facing left at {:?} of {:?}",
                    //     current_face.left(),
                    //     current_face.face
                    // );
                    adjacent_face.coming_from(current_face.left(), from_left_edge)
                } else {
                    (self.col - 1, self.row, self.face, self.facing)
                }
            }
            Facing::Up => {
                if self.row == current_face.row {
                    let adjacent_face = map.cube.iter().find(|&cubeface| {
                        cubeface.face != self.face && cubeface.edges.contains(&current_face.top())
                    })?;
                    let from_left_edge = self.col - current_face.col;
                    // println!(
                    //     "{from_left_edge} facing up at {:?} of {:?}",
                    //     current_face.top(),
                    //     current_face.face
                    // );
                    adjacent_face.coming_from(current_face.top(), from_left_edge)
                } else {
                    (self.col, self.row - 1, self.face, self.facing)
                }
            }
        };

        if map.spots[next_row][next_col] != '#' {
            map.spots[self.row][self.col] = self.facing.to_char();
            (self.col, self.row, self.face, self.facing) =
                (next_col, next_row, next_face, next_facing);
            map.spots[self.row][self.col] = self.facing.to_char();
        }

        Some(())
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

fn parse_map(s: &String) -> Option<(Map, Vec<Instruction>)> {
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
            .next()?,
    );

    let (cube, stride) = parse_cube(&spots)?;

    Some((
        Map {
            spots,
            cube,
            stride,
            col_ranges,
            row_ranges,
        },
        instructions,
    ))
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

fn part2(instructions: &Vec<Instruction>, mut map: Map) {
    let mut character = Character {
        col: map.spots[0].iter().position(|spot| *spot == '.').unwrap(),
        row: 0,
        face: Face::Top,
        facing: Facing::Right,
    };

    for instruction in instructions.iter() {
        character.instruct2(instruction, &mut map);
    }
    map.spots[character.row][character.col] = character.facing.to_char();
    println!("{}", map);
    dbg!(1000 * (character.row + 1) + 4 * (character.col + 1) + character.facing.to_usize());
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let (map, instructions) = parse_map(&s).ok_or(eyre!("Could not parse map"))?;
    println!("{}", map);
    // instructions.iter().for_each(|i| println!("{i:?}"));

    part1(&instructions, map.clone());
    part2(&instructions, map.clone());

    Ok(())
}
