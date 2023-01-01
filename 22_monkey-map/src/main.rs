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
///
///
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
    Up,
    Down,
    Left,
    Right,
}

enum Instruction {
    Advance(usize),
    TurnLeft,
    TurnRight,
}

type Map = Vec<Vec<char>>;

fn parse_map(s: &String) -> Map {
    let mut map: Map = s
        .lines()
        .into_iter()
        .take_while(|&line| line.is_empty() == false)
        .map(|line| line.chars().collect())
        .collect();

    let width = map.iter().fold(0, |max, row| std::cmp::max(max, row.len()));

    map.iter_mut().for_each(|row| row.resize(width, ' '));

    map
}

fn main() {
    println!("Hello, world!");
}
