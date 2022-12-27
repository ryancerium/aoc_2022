use std::collections::{BTreeMap, BTreeSet};

fn parse_positions(s: &String) -> Option<BTreeSet<(i64, i64)>> {
    let mut positions = BTreeSet::new();

    for (y, line) in s.lines().enumerate() {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(x, _)| {
                let y = -(y as i64);
                println!("Found elf at {x}, {y}");
                positions.insert((x as i64, y as i64));
            });
    }

    Some(normalize(positions))
}

trait Cardinal {
    fn n(&self) -> Self;
    fn s(&self) -> Self;
    fn w(&self) -> Self;
    fn e(&self) -> Self;
    fn neast(&self) -> Self;
    fn nw(&self) -> Self;
    fn se(&self) -> Self;
    fn sw(&self) -> Self;
}

impl Cardinal for (i64, i64) {
    fn n(&self) -> Self {
        (self.0, self.1 + 1)
    }

    fn s(&self) -> Self {
        (self.0, self.1 - 1)
    }

    fn w(&self) -> Self {
        (self.0 - 1, self.1)
    }

    fn e(&self) -> Self {
        (self.0 + 1, self.1)
    }

    fn neast(&self) -> Self {
        (self.0 + 1, self.1 + 1)
    }

    fn nw(&self) -> Self {
        (self.0 - 1, self.1 + 1)
    }

    fn se(&self) -> Self {
        (self.0 + 1, self.1 - 1)
    }

    fn sw(&self) -> Self {
        (self.0 - 1, self.1 - 1)
    }
}

fn propose_nothing(
    elf_position: &(i64, i64),
    positions: &BTreeSet<(i64, i64)>,
    proposals: &mut BTreeMap<(i64, i64), Vec<(i64, i64)>>,
) -> bool {
    if true
        && positions.contains(&elf_position.n()) == false
        && positions.contains(&elf_position.nw()) == false
        && positions.contains(&elf_position.neast()) == false
        && positions.contains(&elf_position.s()) == false
        && positions.contains(&elf_position.sw()) == false
        && positions.contains(&elf_position.se()) == false
        && positions.contains(&elf_position.w()) == false
        && positions.contains(&elf_position.e()) == false
    {
        add_proposal(proposals, *elf_position, &elf_position);
        true
    } else {
        false
    }
}

fn propose_n(
    elf_position: &(i64, i64),
    positions: &BTreeSet<(i64, i64)>,
    proposals: &mut BTreeMap<(i64, i64), Vec<(i64, i64)>>,
) -> bool {
    if true
        && positions.contains(&elf_position.n()) == false
        && positions.contains(&elf_position.nw()) == false
        && positions.contains(&elf_position.neast()) == false
    {
        add_proposal(proposals, elf_position.n(), elf_position);
        true
    } else {
        false
    }
}

fn propose_s(
    elf_position: &(i64, i64),
    positions: &BTreeSet<(i64, i64)>,
    proposals: &mut BTreeMap<(i64, i64), Vec<(i64, i64)>>,
) -> bool {
    if true
        && positions.contains(&elf_position.s()) == false
        && positions.contains(&elf_position.sw()) == false
        && positions.contains(&elf_position.se()) == false
    {
        add_proposal(proposals, elf_position.s(), elf_position);
        true
    } else {
        false
    }
}

fn propose_w(
    elf_position: &(i64, i64),
    positions: &BTreeSet<(i64, i64)>,
    proposals: &mut BTreeMap<(i64, i64), Vec<(i64, i64)>>,
) -> bool {
    if true
        && positions.contains(&elf_position.w()) == false
        && positions.contains(&elf_position.nw()) == false
        && positions.contains(&elf_position.sw()) == false
    {
        add_proposal(proposals, elf_position.w(), elf_position);
        true
    } else {
        false
    }
}

fn propose_e(
    elf_position: &(i64, i64),
    positions: &BTreeSet<(i64, i64)>,
    proposals: &mut BTreeMap<(i64, i64), Vec<(i64, i64)>>,
) -> bool {
    if true
        && positions.contains(&elf_position.e()) == false
        && positions.contains(&elf_position.neast()) == false
        && positions.contains(&elf_position.se()) == false
    {
        add_proposal(proposals, elf_position.e(), elf_position);
        true
    } else {
        false
    }
}

fn propose_remain(
    elf_position: &(i64, i64),
    proposals: &mut BTreeMap<(i64, i64), Vec<(i64, i64)>>,
) -> bool {
    add_proposal(proposals, *elf_position, elf_position);
    true
}

fn add_proposal(
    proposals: &mut BTreeMap<(i64, i64), Vec<(i64, i64)>>,
    proposal: (i64, i64),
    elf_position: &(i64, i64),
) {
    if let Some(current_positions) = proposals.get_mut(&proposal) {
        current_positions.push(*elf_position);
    } else {
        proposals.insert(proposal, vec![*elf_position]);
    }
}

fn generate_proposals(
    positions: &BTreeSet<(i64, i64)>,
    iteration: usize,
) -> BTreeMap<(i64, i64), Vec<(i64, i64)>> {
    let mut proposals: BTreeMap<(i64, i64), Vec<(i64, i64)>> = BTreeMap::new();

    let iteration = iteration % 4;
    for elf_position in positions.iter() {
        if iteration == 0 {
            let _ = false
                || propose_nothing(elf_position, positions, &mut proposals)
                || propose_n(elf_position, positions, &mut proposals)
                || propose_s(elf_position, positions, &mut proposals)
                || propose_w(elf_position, positions, &mut proposals)
                || propose_e(elf_position, positions, &mut proposals)
                || propose_remain(elf_position, &mut proposals);
        } else if iteration == 1 {
            let _ = false
                || propose_nothing(elf_position, positions, &mut proposals)
                || propose_s(elf_position, positions, &mut proposals)
                || propose_w(elf_position, positions, &mut proposals)
                || propose_e(elf_position, positions, &mut proposals)
                || propose_n(elf_position, positions, &mut proposals)
                || propose_remain(elf_position, &mut proposals);
        } else if iteration == 2 {
            let _ = false
                || propose_nothing(elf_position, positions, &mut proposals)
                || propose_w(elf_position, positions, &mut proposals)
                || propose_e(elf_position, positions, &mut proposals)
                || propose_n(elf_position, positions, &mut proposals)
                || propose_s(elf_position, positions, &mut proposals)
                || propose_remain(elf_position, &mut proposals);
        } else {
            // iteration == 3
            let _ = false
                || propose_nothing(elf_position, positions, &mut proposals)
                || propose_e(elf_position, positions, &mut proposals)
                || propose_n(elf_position, positions, &mut proposals)
                || propose_s(elf_position, positions, &mut proposals)
                || propose_w(elf_position, positions, &mut proposals)
                || propose_remain(elf_position, &mut proposals);
        }
    }

    proposals
}

fn execute_proposals(proposals: &BTreeMap<(i64, i64), Vec<(i64, i64)>>) -> BTreeSet<(i64, i64)> {
    let mut positions = BTreeSet::new();

    proposals.iter().for_each(|(proposal, proposers)| {
        if proposers.len() == 1 {
            positions.insert(*proposal);
        } else {
            for proposer in proposers.iter() {
                positions.insert(*proposer);
            }
        }
    });

    normalize(positions)
}

fn get_min_max(positions: &BTreeSet<(i64, i64)>) -> (i64, i64, i64, i64) {
    let mut minmax = positions.iter().fold(
        (i64::MAX, i64::MAX, i64::MIN, i64::MIN),
        |(minx, miny, maxx, maxy), (x, y)| {
            (
                std::cmp::min(minx, *x),
                std::cmp::min(miny, *y),
                std::cmp::max(maxx, *x),
                std::cmp::max(maxy, *y),
            )
        },
    );
    minmax.2 += 1;
    minmax.3 += 1;
    minmax
}

fn print_map(positions: &BTreeSet<(i64, i64)>) {
    let (minx, miny, maxx, maxy) = get_min_max(&positions);
    //println!("X: {minx} to {maxx} Y: {miny} to {maxy}");
    for y in (miny..maxy).rev() {
        for x in minx..maxx {
            if positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn part1(s: &String) -> Option<()> {
    let mut positions = parse_positions(s)?;

    println!("== Initial State ==");
    print_map(&positions);
    println!("");

    let mut iteration = 0;
    loop {
        if iteration > 9 {
            break;
        }

        let proposals = generate_proposals(&positions, iteration);
        let new_positions = execute_proposals(&proposals);

        if &positions == &new_positions {
            break;
        }
        positions = new_positions;
        iteration += 1;

        println!("== End of Round {iteration} ==");
        let (minx, miny, maxx, maxy) = get_min_max(&positions);
        let w = maxx - minx;
        let h = maxy - miny;
        let area = w * h;
        let empty = area - positions.len() as i64;
        print_map(&positions);
        println!("{area} area, {empty} empty spots");
        println!("");
    }

    println!("");

    Some(())
}

fn part2(s: &String) -> Option<()> {
    let mut positions = parse_positions(s)?;

    println!("== Initial State ==");
    print_map(&positions);
    println!("");

    let mut round = 1;
    loop {
        let proposals = generate_proposals(&positions, round - 1);
        let new_positions = execute_proposals(&proposals);

        if &positions == &new_positions {
            println!("Nobody moved in round {round}");
            break;
        }
        positions = new_positions;
        println!("== End of Round {round} ==");
        round += 1;
    }

    println!("");

    Some(())
}

fn normalize(positions: BTreeSet<(i64, i64)>) -> BTreeSet<(i64, i64)> {
    let (minx, miny, _, _) = get_min_max(&positions);

    positions
        .into_iter()
        .map(|(x, y)| (x - minx, y - miny))
        .collect()
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    part1(&s);
    part2(&s);
    Ok(())
}
