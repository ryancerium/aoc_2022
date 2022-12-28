use std::{
    cmp::{max, min},
    collections::BTreeSet,
};

use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Position {
    col: i32,
    row: i32,
}

#[derive(Copy, Clone, Debug)]
struct DevicePair {
    sensor: Position,
    beacon: Position,
}

impl DevicePair {
    fn manhattan_distance(&self) -> i32 {
        (self.sensor.row - self.beacon.row).abs() + (self.sensor.col - self.beacon.col).abs()
    }

    fn range_at_row(&self, row: i32) -> Option<Range> {
        let distance_to_row = (self.sensor.row - row).abs();
        let leftover = self.manhattan_distance() - distance_to_row;

        if leftover < 0 {
            return None;
        }
        // println!("Sensor position: {:?}", device.sensor);
        Some(Range {
            start: self.sensor.col - leftover,
            end: self.sensor.col + leftover + 1,
        })
    }
}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    fn merge(&self, other: &Self) -> Range {
        Range {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }

    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

impl core::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.start, self.end))
    }
}

fn parse_positions(s: &String) -> Vec<DevicePair> {
    let re =
        Regex::new(r"Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)")
            .unwrap();

    let mut devices = Vec::new();
    for line in s.lines() {
        let captures = re.captures(line).unwrap();
        let scol = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let srow = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let bcol = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let brow = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
        devices.push(DevicePair {
            sensor: Position {
                col: scol,
                row: srow,
            },
            beacon: Position {
                col: bcol,
                row: brow,
            },
        });
    }
    devices
}

type Rect = (Position, Position);

fn boundaries(devices: &Vec<DevicePair>) -> Rect {
    let tl = Position {
        col: i32::MAX,
        row: i32::MAX,
    };
    let br = Position {
        col: i32::MIN,
        row: i32::MIN,
    };

    devices
        .iter()
        .fold((tl, br), |(mut tl, mut br), device_pair| {
            tl.row = min(tl.row, device_pair.sensor.row);
            tl.row = min(tl.row, device_pair.beacon.row);
            tl.col = min(tl.col, device_pair.sensor.col);
            tl.col = min(tl.col, device_pair.beacon.col);
            // Increment these by one so the boundary is exclusive
            br.row = max(br.row, device_pair.sensor.row + 1);
            br.row = max(br.row, device_pair.beacon.row + 1);
            br.col = max(br.col, device_pair.sensor.col + 1);
            br.col = max(br.col, device_pair.beacon.col + 1);

            (tl, br)
        })
}

fn count_openings(row: i32, devices: &Vec<DevicePair>) -> usize {
    let mut not_beacons = BTreeSet::new();
    for device in devices.iter() {
        let distance_to_row = (device.sensor.row - row).abs();
        let leftover = device.manhattan_distance() - distance_to_row;

        // println!("Sensor position: {:?}", device.sensor);
        for i in device.sensor.col - leftover..device.sensor.col + leftover + 1 {
            // print!("{} ", i);
            not_beacons.insert(i);
        }
        // println!("");
    }

    for device in devices.iter() {
        if device.beacon.row == row {
            not_beacons.remove(&device.beacon.col);
        }
    }
    not_beacons.len()
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let devices = parse_positions(&s);
    for device in devices.iter() {
        println!("{:?} distance: {}", device, device.manhattan_distance());
    }
    let boundary = boundaries(&devices);
    println!("Boundary: {:?}", boundary);

    // println!(
    //     "{} spots in row 10 can't be a beacon\n",
    //     count_openings(10, &devices)
    // );
    // println!(
    //     "{} spots in row 2000000 can't be a beacon\n",
    //     count_openings(2000000, &devices)
    // );

    for row in boundary.0.row..boundary.1.row {
        let mut ranges: Vec<Range> = devices
            .iter()
            .map(|device| device.range_at_row(row))
            .filter_map(|range| range)
            .collect();
        ranges.sort();

        // println!("{:3} {:?}", row, ranges);

        let mut known = ranges[0].clone();
        for range in ranges.iter().skip(1) {
            if known.overlaps(range) {
                known = known.merge(range);
            } else {
                println!(
                    "row {} doesn't overlap known range: {:?} at range {:?}",
                    row, known, range
                );

                println!(
                    "{} * 4000000 + {} = {}",
                    known.end,
                    row,
                    known.end as i64 * 4000000 + row as i64
                );
                return Ok(());
                // known = *range;
            }
        }
        // println!("Mega Range: {:?}\n", known)
    }

    Ok(())
}
