use std::collections::{BTreeMap, BTreeSet};

#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn wiggle(&mut self, direction: &str) {
        match direction {
            "R" => self.x += 1,
            "L" => self.x -= 1,
            "U" => self.y += 1,
            "D" => self.y -= 1,
            _ => println!("Unknown direction: '{}'", direction),
        }
    }

    fn chase(&mut self, head: Self) {
        let x_d = head.x - self.x;
        let y_d = head.y - self.y;

        if x_d.abs() < 2 && y_d.abs() < 2 {
            return;
        }

        if self.x != head.x && self.y != head.y {
            if x_d.is_positive() {
                self.x += 1;
            } else {
                self.x -= 1;
            }

            if y_d.is_positive() {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        } else if self.x == head.x {
            if y_d.is_positive() {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        } else if self.y == head.y {
            if x_d.is_positive() {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        }
    }
}

fn show_rope(head: Position, tail: Position, x0: i32, x1: i32, y0: i32, y1: i32) {
    let h = (head.x, head.y);
    let t = (tail.x, tail.y);
    let s = (0, 0);
    for y in (y0..y1).rev() {
        print!("  ");
        for x in x0..x1 {
            if (x, y) == h {
                print!("H");
            } else if (x, y) == t {
                print!("T");
            } else if (x, y) == s {
                print!("s")
            } else {
                print!(".")
            }
        }
        println!("");
    }
    println!("");
}

fn wiggle(s: &String) -> eyre::Result<()> {
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    let mut positions: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();

    for movement in s.lines() {
        let mut words = movement.split_ascii_whitespace();
        let direction = words.next().unwrap();
        let steps = words.next().unwrap().parse::<i32>()?;
        println!("\n== {} {} ==\n", direction, steps);

        for _ in 0..steps {
            head.wiggle(direction);
            tail.chase(head);
            positions
                .entry(tail.x)
                .and_modify(|y_positions| {
                    y_positions.insert(tail.y);
                })
                .or_insert_with(|| {
                    let mut y_positions = BTreeSet::new();
                    y_positions.insert(tail.y);
                    y_positions
                });
            //show_rope(head, tail, 0, 6, 0, 5);
        }
    }

    let unique_positions = positions
        .iter()
        .fold(0, |count, (_x_position, y_positions)| {
            count + y_positions.len()
        });
    println!("{} unique tail positions\n", unique_positions);

    Ok(())
}

fn wrangle(s: &String) -> eyre::Result<()> {
    let mut rope: [Position; 10] = [Position { x: 0, y: 0 }; 10];

    let mut positions: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();

    for movement in s.lines() {
        let mut words = movement.split_ascii_whitespace();
        let direction = words.next().unwrap();
        let steps = words.next().unwrap().parse::<i32>()?;
        println!("\n== {} {} ==\n", direction, steps);

        for _ in 0..steps {
            rope[0].wiggle(direction);
            for i in 1..rope.len() {
                rope[i].chase(rope[i-1]);
            }

            positions
                .entry(rope.last().unwrap().x)
                .and_modify(|y_positions| {
                    y_positions.insert(rope.last().unwrap().y);
                })
                .or_insert_with(|| {
                    let mut y_positions = BTreeSet::new();
                    y_positions.insert(rope.last().unwrap().y);
                    y_positions
                });
            }
            //show_rope(rope[0], *rope.last().unwrap(), -11, 15, -5, 16);
    }

    let unique_positions = positions
        .iter()
        .fold(0, |count, (_x_position, y_positions)| {
            count + y_positions.len()
        });
    println!("{} unique tail positions\n\n", unique_positions);

    Ok(())
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let _ = wiggle(&s);
    let _ = wrangle(&s);

    Ok(())
}
