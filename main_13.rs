use std::collections::VecDeque;

fn are_unique(mut chars: VecDeque<char>) -> bool {
    let sorted = chars.make_contiguous();    
    sorted.sort();
    
    for i in 1..sorted.len() {
        if sorted[i - 1] == sorted[i] {
            return false;
        }
    }
    return true;
}

fn find_start(s: &str, n_unique_chars_required: usize) -> usize {
    let mut chars = VecDeque::new();
    
    for (i, c) in s.chars().into_iter().enumerate() {
        chars.push_back(c);
        while chars.len() > n_unique_chars_required {
            chars.pop_front();
        }

        if chars.len() == n_unique_chars_required && are_unique(chars.clone()) {
            return i + 1;
        }
    }

    return 0;
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    for line in s.lines() {
        let start = find_start(line, 4);
        println!("{}\nStart 4 after {} characters", line, start);
        let start = find_start(line, 14);
        println!("{}\nStart 14 after {} characters", line, start);
    }

    Ok(())
}
