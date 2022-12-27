use std::{iter::Peekable, str::Chars};

#[derive(Clone, Debug)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

impl Eq for Packet {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Integer(l0), Self::List(r0)) => {
                let l1 = vec![Packet::Integer(*l0)];
                &l1 == r0
            }
            (Self::List(l0), Self::Integer(r0)) => {
                let r1 = vec![Packet::Integer(*r0)];
                l0 == &r1
            }
            (Self::List(l0), Self::List(r0)) => l0 == r0,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0.partial_cmp(r0),
            (Self::Integer(l0), Self::List(r0)) => {
                let l1 = vec![Packet::Integer(*l0)];
                l1.partial_cmp(r0)
            }
            (Self::List(l0), Self::Integer(r0)) => {
                let r1 = vec![Packet::Integer(*r0)];
                l0.partial_cmp(&r1)
            }
            (Self::List(l0), Self::List(r0)) => l0.partial_cmp(r0),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_int(chars: &mut Peekable<Chars<'_>>) -> Packet {
    let mut int_chars = String::new();
    loop {
        match chars.peek() {
            Some(next) => {
                if next.is_ascii_digit() {
                    int_chars.push(chars.next().unwrap())
                } else {
                    let value = int_chars.parse::<i32>().unwrap();
                    return Packet::Integer(value);
                }
            }
            None => {
                panic!("Ran out of input to parse_int()!");
            }
        }
    }
}

fn parse_list(chars: &mut Peekable<Chars<'_>>) -> Packet {
    let mut packets = Vec::new();

    loop {
        match chars.peek() {
            Some(next) => {
                if next.is_ascii_digit() {
                    packets.push(parse_int(chars));
                } else if *next == ',' {
                    let _ = chars.next();
                } else if *next == ']' {
                    let _ = chars.next();
                    return Packet::List(packets);
                } else if *next == '[' {
                    let _ = chars.next();
                    packets.push(parse_list(chars));
                }
            }
            None => {
                panic!("Ran out of input to parse_list()!");
            }
        }
    }
}

fn parse_packets(s: &String) -> Vec<Packet> {
    let mut packets: Vec<Packet> = Vec::new();

    for line in s.lines() {
        if !line.is_empty() {
            let mut chars = line.chars().peekable();
            let _ = chars.next(); // Skip the leading '[' character
            packets.push(parse_list(&mut chars));
        }
    }
    packets
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let mut ordered_indices_sum = 0;
    let mut packets = parse_packets(&s);

    let mut packet_pairs = Vec::new();
    for i in (1..packets.len()).step_by(2) {
        packet_pairs.push((packets[i - 1].clone(), packets[i].clone()));
    }

    for (index, packet_pair) in packet_pairs.iter().enumerate() {
        // println!("== Pair {} ==", index + 1);
        // println!("{:?}", packet_pair.0);
        // println!("{:?}", packet_pair.1);
        if packet_pair.0 < packet_pair.1 {
            ordered_indices_sum += index + 1;
        }
        // println!("");
    }
    println!("Ordered indices sum to {}\n", ordered_indices_sum);

    let dividers = parse_packets(&"[[2]]\n[[6]]".to_owned());
    packets.append(&mut dividers.clone());

    let mut index2 = 0;
    let mut index6 = 0;

    packets.sort();

    for (index, packet) in packets.iter().enumerate() {
        if *packet == dividers[0] {
            index2 = index + 1;
        } else if *packet == dividers[1] {
            index6 = index + 1;
        }
    }

    println!(
        "Decoder key is {} * {} = {}\n",
        index2,
        index6,
        index2 * index6
    );

    Ok(())
}
