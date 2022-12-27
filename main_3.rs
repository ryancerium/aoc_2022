use std::{io::BufRead, str::FromStr};

use color_eyre::eyre::eyre;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn play_value(&self) -> i32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
    fn round_value(&self, other: &Self) -> i32 {
        if self == other {
            return 3;
        }

        match (self, other) {
            (Play::Rock, Play::Paper) => 0,
            (Play::Paper, Play::Scissors) => 0,
            (Play::Scissors, Play::Rock) => 0,
            _ => 6,
        }
    }

    pub fn value(&self, other: &Self) -> i32 {
        self.play_value() + self.round_value(other)
    }

    pub fn from_cheat(s: &str, other: &Play) -> Result<Self, color_eyre::eyre::Report> {
        match (s, other) {
            // X lose
            ("X", Play::Rock) => Ok(Play::Scissors),
            ("X", Play::Paper) => Ok(Play::Rock),
            ("X", Play::Scissors) => Ok(Play::Paper),

            // Z win
            ("Z", Play::Rock) => Ok(Play::Paper),
            ("Z", Play::Paper) => Ok(Play::Scissors),
            ("Z", Play::Scissors) => Ok(Play::Rock),

            // Y draw
            ("Y", other) => Ok(*other),
            (_, _) => Err(eyre!(format!("Unknown value for the cheat: '{}'", s))),
        }
    }
}

impl FromStr for Play {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            _ => Err(eyre!(format!("Found unknown value for Play: '{}'", s))),
        }
    }
}

fn round(me: &Play, them: &Play) {
    println!("me: {:?} vs {:?} = {}", me, them, me.value(them));
}

fn main() -> color_eyre::eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"code.txt".to_owned()).clone();
    println!("Opening '{}", input);
    let input = std::fs::File::open(input)?;
    let input = std::io::BufReader::new(input);

    let mut sum = 0;
    for line in input.lines() {
        if let Ok(line) = line {
            let mut words = line.split_ascii_whitespace();
            let them = Play::from_str(words.next().unwrap())?;
            //let me = Play::from_str(words.next().unwrap())?;
            let me = Play::from_cheat(words.next().unwrap(), &them)?;
            round(&me, &them);
            sum += me.value(&them);
        }
    }

    println!("Total of games was: {}", sum);

    Ok(())
}
