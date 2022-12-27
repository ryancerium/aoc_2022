use std::collections::BTreeSet;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Height(i32);
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StepCount(i32);
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]

struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Spot {
    height: Height,
    step_count: StepCount,
    position: Position,
}

impl Spot {
    fn can_reach(self: &Self, other: &Self) -> bool {
        other.height.0 <= self.height.0 + 1 && other.step_count.0 >= self.step_count.0 + 1
    }
}

type Map = Vec<Vec<Spot>>;

fn list_reachable(map: &Map, current_pos: Position) -> BTreeSet<Position> {
    let mut reachable = BTreeSet::new();
    let r = current_pos.row;
    let c = current_pos.col;

    if r > 0 && map[r][c].can_reach(&map[r - 1][c]) {
        reachable.insert(map[r - 1][c].position);
    }
    if r < (map.len() - 1) && map[r][c].can_reach(&map[r + 1][c]) {
        reachable.insert(map[r + 1][c].position);
    }
    if c > 0 && map[r][c].can_reach(&map[r][c - 1]) {
        reachable.insert(map[r][c - 1].position);
    }
    if c < (map[r].len() - 1) && map[r][c].can_reach(&map[r][c + 1]) {
        reachable.insert(map[r][c + 1].position);
    }

    reachable
}

fn parse_map(s: &String) -> (Map, Spot, Spot) {
    let mut map = Vec::new();
    let mut start = None;
    let mut end = None;
    for (row_i, line) in s.lines().enumerate() {
        let mut row = Vec::new();
        for (col_i, c) in line.chars().enumerate() {
            let height = Height(match c {
                'S' => 0,
                'E' => 25,
                c => c as i32 - 'a' as i32,
            });
            let step_count = StepCount(i32::MAX);
            let position = Position::new(row_i, col_i);
            let mut spot = Spot {
                height,
                step_count,
                position,
            };
            if c == 'S' {
                start = Some(spot);
            } else if c == 'E' {
                end = Some(spot);
            }
            row.push(spot);
        }
        map.push(row);
    }

    (map, start.unwrap(), end.unwrap())
}

fn print_map(map: &Map) {
    for row in map {
        for spot in row {
            print!("{:2}", spot.height.0);
        }
        println!("");
    }
    println!("");
}

fn print_map_steps(map: &Map) {
    for row in map {
        for spot in row {
            if spot.step_count.0 != i32::MAX {
                print!("{:3} ", spot.step_count.0);
            } else {
                print!(" XX ");
            }
        }
        println!("");
    }
}

fn dijkstra(mut map: Map, s: Spot, e: Spot) -> Option<StepCount> {
    let mut step_count = StepCount(0);
    let mut reachable = BTreeSet::new();
    reachable.insert(s.position);

    loop {
        if reachable.is_empty() {
            return None;
        }
        let mut next_reachable = BTreeSet::new();
        for current_pos in reachable.iter() {
            map[current_pos.row][current_pos.col].step_count = step_count;

            //println!("Current spot: {:?}", spot);
            if *current_pos == e.position {
                return Some(step_count);
            }
            next_reachable.append(&mut list_reachable(&map, *current_pos));
        }
        // println!(
        //     "{} next spots after {} steps",
        //     reachable.len(),
        //     step_count.0
        // );
        step_count.0 += 1;
        reachable = next_reachable;

        //print_map_steps(&map);
    }
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let (map, start, end) = parse_map(&s);
    print_map(&map);
    println!(
        "From the start it takes {} steps",
        dijkstra(map.clone(), start, end).unwrap().0
    );

    let mut shortest_step_count = StepCount(i32::MAX);
    let mut shortest_position = start.position;
    for row in map.iter() {
        for spot in row {
            if spot.height.0 == 0 {
                let step_count = dijkstra(map.clone(), *spot, end);
                //println!("Takes {} steps from {:?}", step_count.0, *spot);
                if let Some(step_count) = step_count {
                    if step_count < shortest_step_count {
                        shortest_step_count = step_count;
                        shortest_position = spot.position;
                    }
                }
            }
        }
    }

    println!(
        "Shortest path takes {} steps from {:?}",
        shortest_step_count.0, shortest_position
    );
    Ok(())
}
