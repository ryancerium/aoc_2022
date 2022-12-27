use std::io::BufRead;

#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
struct Elf(i32);
#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
struct Calories(i32);

fn main() -> color_eyre::eyre::Result<()> {
    let input = std::fs::File::open("input.txt")?;
    let input = std::io::BufReader::new(input);

    let mut elves = Vec::new();
    let mut e = Elf(0);
    let mut c = Calories(0);
    for line in input.lines().into_iter() {
        if let Ok(line) = line {
            if line.is_empty() {
                elves.push((e, c));
                e.0 += 1;
                c.0 = 0;
            } else {
                c.0 += line.parse::<i32>()?;
            }
        }
    }

    elves.sort_by(|lhs, rhs| { rhs.1.0.cmp(&lhs.1.0) });

    let mut total_calories = 0;
    for (elf, calories) in elves.iter().take(3) {
        println!("Elf {} had {} calories", elf.0, calories.0);
        total_calories += calories.0;
    }
    println!("Top 3 elves had {} calories", total_calories);

    Ok(())
}
