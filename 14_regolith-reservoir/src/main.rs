use std::cmp::max;

type Cave = Vec<Vec<char>>;

const EMPTY: char = '.';
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    col: usize,
    row: usize,
}

fn parse_cave(s: &String) -> Cave {
    let mut rocks = Vec::new();

    let mut high_row = 0;
    let mut high_col = 0;

    for line in s.lines() {
        let line = line.replace("->", "");
        let points: Vec<Point> = line
            .split_ascii_whitespace()
            .map(|p| {
                let mut point = p.split(",");
                let col = point.next().unwrap().parse::<usize>().unwrap();
                let row = point.next().unwrap().parse::<usize>().unwrap();
                high_col = max(col, high_col);
                high_row = max(row, high_row);
                Point { row, col }
            })
            .collect();

        rocks.push(points);
    }

    high_col += 502;
    high_row += 2;
    println!("{} columns, {} rows", high_col, high_row);

    let mut cave = {
        let mut row = Vec::new();
        row.resize(high_col, EMPTY);

        let mut cave = Vec::new();
        cave.resize(high_row, row);
        cave
    };

    for points in rocks {
        let mut p = points.iter();
        let mut start = *p.next().unwrap();

        //println!("{:?}", points);
        for end in p {
            loop {
                cave[start.row][start.col] = '#';
                let row_diff = start.row.cmp(&end.row);
                let col_diff = start.col.cmp(&end.col);
                match row_diff {
                    std::cmp::Ordering::Less => start.row += 1,
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => start.row -= 1,
                };
                match col_diff {
                    std::cmp::Ordering::Less => start.col += 1,
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => start.col -= 1,
                };

                if start == *end {
                    cave[start.row][start.col] = '#';
                    break;
                }
            }
        }
    }
    cave
}

enum NextPoint {
    Abyss,
    Spot(Point),
    Stuck,
}

fn can_move(cave: &Cave, grain: Point) -> NextPoint {
    let cave_height = cave.len() - 1;
    let cave_width = cave[0].len() - 1;

    if grain.row == cave_height {
        return NextPoint::Abyss;
    }

    let next_row = grain.row + 1;

    if cave[next_row][grain.col] == EMPTY {
        return NextPoint::Spot(Point {
            row: next_row,
            col: grain.col,
        });
    }

    if grain.col > 0 && cave[next_row][grain.col - 1] == EMPTY {
        return NextPoint::Spot(Point {
            row: next_row,
            col: grain.col - 1,
        });
    }

    if grain.col < cave_width && cave[next_row][grain.col + 1] == EMPTY {
        return NextPoint::Spot(Point {
            row: next_row,
            col: grain.col + 1,
        });
    }

    return NextPoint::Stuck;
}

fn drop_sand(cave: &mut Cave, source: Point, trace: bool) {
    let mut grain_count = 0;
    loop {
        let mut grain = source.clone();
        grain_count += 1;
        loop {
            match can_move(cave, grain) {
                NextPoint::Abyss => {
                    cave[grain.row][grain.col] = if !trace { EMPTY } else { '~' };
                    print_cave(&cave, 0);
                    println!("{} grains of sand\n", grain_count - 1);
                    return;
                }
                NextPoint::Spot(point) => {
                    cave[grain.row][grain.col] = if !trace { EMPTY } else { '~' };
                    cave[point.row][point.col] = 'o';
                    cave[source.row][source.col] = '+';
                    grain = point;
                }
                NextPoint::Stuck => {
                    if grain == source {
                        print_cave(&cave, 1);
                        println!("Filled up at {} grains of sand\n", grain_count);
                        return;
                    }
                    break;
                }
            }
            //print_cave(&cave);
        }
    }
}

fn print_cave(cave: &Cave, skip_last: usize) {
    let mut low_col = usize::MAX;
    for row in cave.iter().take(cave.len() - skip_last) {
        low_col = std::cmp::min(
            low_col,
            row.iter()
                .enumerate()
                .find(|(_index, c)| **c != EMPTY)
                .map(|(index, _c)| index)
                .unwrap_or(usize::MAX),
        );
    }
    if low_col > 0 {
        low_col -= 1;
    }

    for row in cave.iter().take(cave.len() - skip_last) {
        for point in row.iter().skip(low_col) {
            print!("{}", point);
        }
        println!("");
    }
    println!("");

    println!("{} rows, low col is {}", cave.len(), low_col);
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let mut cave = parse_cave(&s);
    let source = Point { row: 0, col: 500 };
    cave[source.row][source.col] = '+';
    print_cave(&cave, 0);
    drop_sand(&mut cave.clone(), source, false);
    // drop_sand(&mut cave, source, true);

    let mut floor = Vec::new();
    floor.resize(cave[0].len(), '#');
    cave.push(floor);
    drop_sand(&mut cave.clone(), source, false);

    Ok(())
}
