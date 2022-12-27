use std::io::{BufRead, Read};

use itertools::Itertools;

fn get_priority(c: char) -> i32 {
    let priority = if c.is_ascii_lowercase() {
        (c as i32) - ('a' as i32) + 1
    } else if c.is_ascii_uppercase() {
        (c as i32) - ('A' as i32) + 27
    } else {
        0
    };
    //println!("priority of {} is {}", c, priority);
    priority
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let mut sum = 0;
    for line in s.lines() {
        let chars: Vec<char> = line.chars().collect();
        let (r1, r2) = chars.split_at(chars.len() / 2);

        println!("Ruck1:  {:?}", r1);
        println!("Ruck2:  {:?}", r2);
        let mut r1_items: [bool; 26 * 2 + 1] = [false; 26 * 2 + 1];
        let mut r2_items: [bool; 26 * 2 + 1] = [false; 26 * 2 + 1];

        for item in r1.iter() {
            r1_items[get_priority(*item) as usize] = true;
        }

        for item in r2.iter() {
            let i = get_priority(*item) as usize;
            if r2_items[i] == false && r1_items[i] == true {
                println!("Duplicate item is '{}'", item);
                sum += i;
            }
            r2_items[i] = true;
        }
        println!("");
    }
    println!("Sum of priorities is {}", sum);

    let mut sum = 0;

    for triplet in &s.lines().into_iter().chunks(3) {
        let rucksacks: Vec<[bool; 53]> = triplet
            .map(|s| {
                let mut items: [bool; 26 * 2 + 1] = [false; 26 * 2 + 1];
                for item in s.chars() {
                    let i = get_priority(item) as usize;
                    items[i] = true;
                }

                items
            })
            .collect();
        let mut badge: [bool; 53] = [true; 53];

        for rucksack in rucksacks {
            for (i, present) in rucksack.iter().enumerate() {
                badge[i] &= present;
            }
        }

        for (i, present) in badge.iter().enumerate() {
            if *present {
                //println!("Badge priority is {}", i);
                sum += i;
            }
        }
    }

    println!("Sum of priorities is {}", sum);

    Ok(())
}
