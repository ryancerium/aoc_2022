use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct Assignment {
    pub start: i32,
    pub end: i32,
}

impl Assignment {
    pub fn contains(self: &Self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(self: &Self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

impl FromStr for Assignment {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split("-");

        Ok(Assignment {
            start: pieces.next().unwrap().parse::<i32>()?,
            end: pieces.next().unwrap().parse::<i32>()?,
        })
    }
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let mut contains = 0;
    for line in s.lines() {
        let mut assignments = line.split(",");
        let a1 = Assignment::from_str(assignments.next().unwrap())?;
        let a2 = Assignment::from_str(assignments.next().unwrap())?;
        if a1.contains(&a2) {
            println!("{:?} contains {:?}", a1, a2);
            contains += 1;
        } else if a2.contains(&a1) {
            println!("{:?} contains {:?}", a2, a1);
            contains += 1;
        }
    }
    println!("Found {} contains", contains);

    let mut overlaps = 0;
    for line in s.lines() {
        let mut assignments = line.split(",");
        let a1 = Assignment::from_str(assignments.next().unwrap())?;
        let a2 = Assignment::from_str(assignments.next().unwrap())?;
        if a1.overlaps(&a2) {
            println!("{:?} overlaps {:?}", a1, a2);
            overlaps += 1;
        }
    }
    println!("Found {} overlaps", overlaps);

    Ok(())
}
