use eyre::Result;

#[derive(Debug, Clone)]
struct Item(i64);

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: String,
    divisible_by: i64,
    lcd: i64,
    dst_true: usize,
    dst_false: usize,
    inspections: i32,
}

impl Monkey {
    fn inspect(self: &mut Self, item: Item) -> (usize, Item) {
        let mut worry_level = item.0;

        let operation = self
            .operation
            .replace("old", format!("{}", worry_level).as_str());
        let mut tokens = operation.split_ascii_whitespace();

        let lhs = tokens.next().unwrap().parse::<i64>().unwrap();
        let op = tokens.next().unwrap().trim();
        let rhs = tokens.next().unwrap().parse::<i64>().unwrap();

        // Do operation
        worry_level = match op {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            op => panic!("Unknown operation type: '{}'", op),
        };

        // Divide by 3, rounding towards zero
        worry_level /= 3;

        self.inspections += 1;

        if worry_level % self.divisible_by == 0 {
            (self.dst_true, Item(worry_level))
        } else {
            (self.dst_false, Item(worry_level))
        }
    }

    fn inspect2(self: &mut Self, item: Item) -> (usize, Item) {
        let mut worry_level = item.0;

        let operation = self
            .operation
            .replace("old", format!("{}", worry_level).as_str());
        let mut tokens = operation.split_ascii_whitespace();

        let lhs = tokens.next().unwrap().parse::<i64>().unwrap();
        let op = tokens.next().unwrap().trim();
        let rhs = tokens.next().unwrap().parse::<i64>().unwrap();

        // Do operation
        worry_level = match op {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            op => panic!("Unknown operation type: '{}'", op),
        };

        worry_level %= self.lcd;

        self.inspections += 1;

        if worry_level % self.divisible_by == 0 {
            (self.dst_true, Item(worry_level))
        } else {
            (self.dst_false, Item(worry_level))
        }
    }
}

fn parse_monkeys(s: &String) -> Result<Vec<Monkey>> {
    let mut monkeys = Vec::new();

    let mut lines = s.lines().into_iter();
    loop {
        let line = match lines.next() {
            None => return Ok(monkeys),
            Some(line) => line,
        };

        if !line.starts_with("Monkey ") {
            continue;
        }

        let line = lines.next().unwrap();
        let item_list = line.split(":").skip(1).next().unwrap();
        let mut items = Vec::new();
        for item in item_list.split(",") {
            items.push(Item(item.trim().parse::<i64>()?));
        }

        let operation = lines
            .next()
            .unwrap()
            .split("=")
            .last()
            .unwrap()
            .trim()
            .to_owned();

        let divisible_by = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<i64>()?;

        let dst_true = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()?;

        let dst_false = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()?;

        let monkey = Monkey {
            items,
            operation,
            divisible_by,
            lcd: 0,
            dst_true,
            dst_false,
            inspections: 0,
        };

        monkeys.push(monkey);
    }
}

fn part_one(mut monkeys: Vec<Monkey>) {
    for round in 1..21 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items.clear();
            for item in items {
                let (dst, item) = monkeys[i].inspect(item);
                monkeys[dst].items.push(item);
            }
        }

        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            round
        );
        for (i, monkey) in monkeys.iter().enumerate() {
            print!("Monkey {}: ", i);
            for item in monkey.items.iter() {
                print!("{}, ", item.0);
            }
            println!("");
        }
        println!("");
    }

    println!("");
    let mut inspections = Vec::new();
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {} inspected items {} times.", i, monkey.inspections);
        inspections.push(monkey.inspections);
    }

    println!("Inspections: {:?}", inspections);
    inspections.sort();
    inspections.reverse();
    println!("Monkey business: {}", inspections[0] * inspections[1]);
}

fn part_two(mut monkeys: Vec<Monkey>) {
    for round in 1..10001 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items.clear();
            for item in items {
                let (dst, item) = monkeys[i].inspect2(item);
                monkeys[dst].items.push(item);
            }
        }

        if round == 1 || round == 20 || round % 50 == 0 {
            println!("\n== After round {} ==", round);
            for (i, monkey) in monkeys.iter().enumerate() {
                println!("Monkey {} inspected items {} times.", i, monkey.inspections);
            }
            println!("");
        }
    }

    println!("");
    let mut inspections = Vec::new();
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {} inspected items {} times.", i, monkey.inspections);
        inspections.push(monkey.inspections);
    }

    println!("Inspections: {:?}", inspections);
    inspections.sort();
    inspections.reverse();
    let i0 = inspections[0] as i64;
    let i1 = inspections[1] as i64;
    println!("Monkey business: {}", i0 * i1);
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let mut monkeys = parse_monkeys(&s).unwrap();
    let lcd = monkeys.iter().fold(1, |lcd, monkey| lcd * monkey.divisible_by);
    for monkey in monkeys.iter_mut() {
        monkey.lcd = lcd;
        println!("{:?}", monkey);
    }
    println!("");

    part_one(monkeys.clone());
    println!("\n\n PART TWO \n");
    part_two(monkeys.clone());

    Ok(())
}
