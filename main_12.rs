use itertools::Itertools;

fn print_stacks(stacks: &Vec<Vec<char>>) {
    println!("Stacks: ");
    for stack in stacks.iter() {
        if !stack.is_empty() {
            println!("{:?}", stack);
        }
    }
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for line in s.lines() {
        if line.is_empty() {
            break;
        }
        for (mut i, mut chars) in line.chars().into_iter().chunks(4).into_iter().enumerate() {
            i += 1; // Increment by 1 to match the ordering of stacks in the problem?
            while stacks.len() <= i {
                stacks.push(Vec::new());
            }

            let lead = chars.next().unwrap();
            let value = chars.next().unwrap();
            if lead == '[' && value != ' ' {
                match stacks.get_mut(i) {
                    Some(stack) => stack.push(value),
                    _ => (),
                };
            }
        }
    }

    stacks.iter_mut().for_each(|stack| stack.reverse());
    let mut stacks2 = stacks.clone();
    print_stacks(&stacks);

    for line in s.lines().skip_while(|line| !line.starts_with("m")) {
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        let count = words[1].parse::<i32>()?;
        let src = words[3].parse::<usize>()?;
        let dst = words[5].parse::<usize>()?;
        println!("> {}", line);
        println!("> {} crates from column {} to column {}\n", count, src, dst);

        let mut crane = Vec::new();
        for _ in 0..count {
            let c = stacks[src].pop().unwrap();
            stacks[dst].push(c);

            crane.push(stacks2[src].pop().unwrap());
        }

        for _ in 0..count {
            let c = crane.pop().unwrap();
            stacks2[dst].push(c);
        }

        //print_stacks(&stacks);
        print_stacks(&stacks2);
    }

    stacks
        .iter()
        .filter(|stack| !stack.is_empty())
        .for_each(|stack| print!("{}", stack.last().unwrap()));
    println!("");
    stacks2
        .iter()
        .filter(|stack| !stack.is_empty())
        .for_each(|stack| print!("{}", stack.last().unwrap()));
    Ok(())
}
