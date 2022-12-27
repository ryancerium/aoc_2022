use std::{
    cmp::{max, min},
    collections::HashMap,
};

use regex::Regex;

fn part1(s: &String) -> eyre::Result<()> {
    let mut points: HashMap<(i32, i32, i32), i32> = HashMap::new();
    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    for s in s.lines() {
        let captures = re.captures(s).unwrap();
        let (x, y, z) = (
            captures.get(1).unwrap().as_str().parse::<i32>()?,
            captures.get(2).unwrap().as_str().parse::<i32>()?,
            captures.get(3).unwrap().as_str().parse::<i32>()?,
        );

        let mut point = 6;
        if let Some(other) = points.get_mut(&(x - 1, y, z)) {
            *other -= 1;
            point -= 1;
        }
        if let Some(other) = points.get_mut(&(x + 1, y, z)) {
            *other -= 1;
            point -= 1;
        }
        if let Some(other) = points.get_mut(&(x, y - 1, z)) {
            *other -= 1;
            point -= 1;
        }
        if let Some(other) = points.get_mut(&(x, y + 1, z)) {
            *other -= 1;
            point -= 1;
        }
        if let Some(other) = points.get_mut(&(x, y, z - 1)) {
            *other -= 1;
            point -= 1;
        }
        if let Some(other) = points.get_mut(&(x, y, z + 1)) {
            *other -= 1;
            point -= 1;
        }
        points.insert((x, y, z), point);
        println!("({x}, {y}, {z}) = {point}");
    }
    let total: i32 = points.values().sum();
    println!("total: {total}\n");
    Ok(())
}

enum Point {
    Air,
    Steam,
    Lava(i32),
}

fn part2(s: &String) -> eyre::Result<()> {
    let mut points: HashMap<(i32, i32, i32), Point> = HashMap::new();
    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();

    let mut minx = i32::MAX;
    let mut maxx = i32::MIN;
    let mut miny = i32::MAX;
    let mut maxy = i32::MIN;
    let mut minz = i32::MAX;
    let mut maxz = i32::MIN;

    for s in s.lines() {
        let captures = re.captures(s).unwrap();
        let (x, y, z) = (
            captures.get(1).unwrap().as_str().parse::<i32>()?,
            captures.get(2).unwrap().as_str().parse::<i32>()?,
            captures.get(3).unwrap().as_str().parse::<i32>()?,
        );

        let mut point = 6;
        if let Some(Point::Lava(other)) = points.get_mut(&(x - 1, y, z)) {
            *other -= 1;
            point -= 1;
        }
        if let Some(Point::Lava(other)) = points.get_mut(&(x + 1, y, z)) {
            *other -= 1;
            point -= 1;
        }
        if let Some(Point::Lava(other)) = points.get_mut(&(x, y - 1, z)) {
            *other -= 1;
            point -= 1;
        }
        if let Some(Point::Lava(other)) = points.get_mut(&(x, y + 1, z)) {
            *other -= 1;
            point -= 1;
        }
        if let Some(Point::Lava(other)) = points.get_mut(&(x, y, z - 1)) {
            *other -= 1;
            point -= 1;
        }
        if let Some(Point::Lava(other)) = points.get_mut(&(x, y, z + 1)) {
            *other -= 1;
            point -= 1;
        }

        points.insert((x, y, z), Point::Lava(point));

        minx = min(minx, x);
        miny = min(miny, y);
        minz = min(minz, z);
        maxx = max(maxx, x);
        maxy = max(maxy, y);
        maxz = max(maxz, z);
    }

    maxx += 1;
    maxy += 1;
    maxz += 1;
    println!("({minx}, {miny}, {minz}), ({maxx}, {maxy}, {maxz})");

    for y in miny..maxy {
        for z in minz..maxz {
            points.insert((minx - 1, y, z), Point::Steam);
            points.insert((maxx, y, z), Point::Steam);
        }
    }
    for x in minx..maxz {
        for z in minz..maxz {
            points.insert((x, miny - 1, z), Point::Steam);
            points.insert((x, maxy, z), Point::Steam);
        }
    }
    for x in minx..maxx {
        for y in miny..maxy {
            points.insert((x, y, minz - 1), Point::Steam);
            points.insert((x, y, maxz), Point::Steam);
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for x in minx..maxx {
            for y in miny..maxy {
                for z in minz..maxz {
                    if !points.contains_key(&(x, y, z)) {
                        if adjacent_to_steam(x, y, z, &points) {
                            points.insert((x, y, z), Point::Steam);
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    for x in minx..maxx {
        for y in miny..maxy {
            for z in minz..maxz {
                if !points.contains_key(&(x, y, z)) {
                    // println!("Bubble ({x}, {y}, {z})");

                    if let Some(Point::Lava(other)) = points.get_mut(&(x - 1, y, z)) {
                        *other -= 1;
                    }
                    if let Some(Point::Lava(other)) = points.get_mut(&(x + 1, y, z)) {
                        *other -= 1;
                    }
                    if let Some(Point::Lava(other)) = points.get_mut(&(x, y - 1, z)) {
                        *other -= 1;
                    }
                    if let Some(Point::Lava(other)) = points.get_mut(&(x, y + 1, z)) {
                        *other -= 1;
                    }
                    if let Some(Point::Lava(other)) = points.get_mut(&(x, y, z - 1)) {
                        *other -= 1;
                    }
                    if let Some(Point::Lava(other)) = points.get_mut(&(x, y, z + 1)) {
                        *other -= 1;
                    }
                }
            }
        }
    }

    let mut total = 0;
    for point in points.values() {
        if let Point::Lava(sides) = point {
            total += sides;
        }
    }
    println!("total: {total}\n");

    Ok(())
}

fn adjacent_to_steam(x: i32, y: i32, z: i32, points: &HashMap<(i32, i32, i32), Point>) -> bool {
    if let Some(Point::Steam) = points.get(&(x - 1, y, z)) {
        return true;
    }
    if let Some(Point::Steam) = points.get(&(x + 1, y, z)) {
        return true;
    }
    if let Some(Point::Steam) = points.get(&(x, y - 1, z)) {
        return true;
    }
    if let Some(Point::Steam) = points.get(&(x, y + 1, z)) {
        return true;
    }
    if let Some(Point::Steam) = points.get(&(x, y, z - 1)) {
        return true;
    }
    if let Some(Point::Steam) = points.get(&(x, y, z + 1)) {
        return true;
    }
    false
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let _ = part1(&s)?;
    part2(&s)
}
