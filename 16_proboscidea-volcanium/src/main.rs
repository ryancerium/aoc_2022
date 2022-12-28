use std::collections::{BTreeMap, BTreeSet};

use derive_more::{Add, AddAssign, Mul};
use regex::Regex;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: usize,
    neighbors: Vec<String>,
}

fn parse_valves(s: &String) -> BTreeMap<String, Valve> {
    let re =
        Regex::new(r"Valve (..) has flow rate=(\d*); tunnels? leads? to valves? (.*)").unwrap();

    let mut valves = BTreeMap::new();
    for line in s.lines() {
        let captures = re.captures(line).unwrap();
        let name = captures.get(1).unwrap().as_str().to_owned();
        let flow_rate = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let neighbors: Vec<String> = captures
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(str::to_owned)
            .collect();
        valves.insert(
            name.clone(),
            Valve {
                name,
                flow_rate,
                neighbors,
            },
        );
    }
    valves
}

#[derive(Add, AddAssign, Mul, Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct PressureReleased(usize);

#[derive(Add, Mul, Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct ValveState(usize);

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct StateKey {
    m: String,
    e: String,
    valve_state: ValveState,
}

impl StateKey {
    fn new(m: String, e: String, valve_state: ValveState) -> StateKey {
        let (m, e) = if m < e { (m, e) } else { (e, m) };
        StateKey {
            m: m,
            e: e,
            valve_state: valve_state,
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Step {
    state_key: StateKey,
    pressure_released: PressureReleased,
}

fn making_enough_progress(i: usize, pressure_released: PressureReleased) -> bool {
    if i < 10 {
        true
    } else if i < 15 {
        pressure_released > PressureReleased(1000)
    } else if i < 20 {
        pressure_released > PressureReleased(1500)
    } else {
        pressure_released > PressureReleased(2000)
    }
}

fn find_best_elephant_path(
    valves: &BTreeMap<String, Valve>,
    pressurized_valve_bits: &BTreeMap<String, usize>,
) -> usize {
    let mut current_steps = BTreeSet::new();
    current_steps.insert(Step {
        state_key: StateKey::new("AA".to_owned(), "AA".to_owned(), ValveState(0)),
        pressure_released: PressureReleased(0),
    });

    let mut position_to_best_flow: BTreeMap<StateKey, PressureReleased> = BTreeMap::new();
    for i in 0..26 {
        let mut next_steps = BTreeSet::new();
        for step in current_steps.iter() {
            let current_pressure_released = &step.pressure_released;
            if let Some(most_pressure_released) = position_to_best_flow.get(&step.state_key) {
                if most_pressure_released >= &current_pressure_released {
                    continue;
                }
            }
            position_to_best_flow.insert(step.state_key.clone(), step.pressure_released);

            let m_valve = valves.get(&step.state_key.m).unwrap();
            let e_valve = valves.get(&step.state_key.e).unwrap();

            // m opens, e moves
            if m_valve.flow_rate > 0 {
                let current_valve_bit = pressurized_valve_bits.get(&step.state_key.m).unwrap();
                if step.state_key.valve_state.0 & current_valve_bit == 0 {
                    let mut new_valve_state = step.state_key.valve_state;
                    new_valve_state.0 |= current_valve_bit;

                    let new_pressure_released =
                        step.pressure_released + PressureReleased(m_valve.flow_rate * (25 - i));

                    for e_neighbor in e_valve.neighbors.iter() {
                        if making_enough_progress(i, new_pressure_released) {
                            next_steps.insert(Step {
                                state_key: StateKey::new(
                                    step.state_key.m.clone(),
                                    e_neighbor.clone(),
                                    new_valve_state,
                                ),
                                pressure_released: new_pressure_released,
                            });
                        }
                    }
                }
            }

            // If they're on the same valve, let m have precedence for opening it in the above loop
            if step.state_key.m != step.state_key.e {
                // m moves, e opens
                if e_valve.flow_rate > 0 {
                    let current_valve_bit = pressurized_valve_bits.get(&step.state_key.e).unwrap();
                    if step.state_key.valve_state.0 & current_valve_bit == 0 {
                        let mut new_valve_state = step.state_key.valve_state;
                        new_valve_state.0 |= current_valve_bit;

                        let new_pressure_released =
                            step.pressure_released + PressureReleased(e_valve.flow_rate * (25 - i));

                        for m_neighbor in m_valve.neighbors.iter() {
                            if making_enough_progress(i, new_pressure_released) {
                                next_steps.insert(Step {
                                    state_key: StateKey::new(
                                        m_neighbor.clone(),
                                        step.state_key.e.clone(),
                                        new_valve_state,
                                    ),
                                    pressure_released: new_pressure_released,
                                });
                            }
                        }
                    }
                }

                // m opens, e opens (only on different valves)
                if m_valve.flow_rate > 0 && e_valve.flow_rate > 0 {
                    let m_current_valve_bit =
                        pressurized_valve_bits.get(&step.state_key.m).unwrap();
                    let e_current_valve_bit =
                        pressurized_valve_bits.get(&step.state_key.e).unwrap();

                    if step.state_key.valve_state.0 & m_current_valve_bit == 0
                        && step.state_key.valve_state.0 & e_current_valve_bit == 0
                    {
                        let mut new_valve_state = step.state_key.valve_state;
                        new_valve_state.0 |= m_current_valve_bit;
                        new_valve_state.0 |= e_current_valve_bit;

                        let mut new_pressure_released = step.pressure_released;
                        new_pressure_released += PressureReleased(m_valve.flow_rate * (25 - i));
                        new_pressure_released += PressureReleased(e_valve.flow_rate * (25 - i));

                        if making_enough_progress(i, new_pressure_released) {
                            next_steps.insert(Step {
                                state_key: StateKey::new(
                                    step.state_key.m.clone(),
                                    step.state_key.e.clone(),
                                    new_valve_state,
                                ),
                                pressure_released: new_pressure_released,
                            });
                        }
                    }
                }
            }

            // both move
            for m_neighbor in m_valve.neighbors.iter() {
                for e_neighbor in e_valve.neighbors.iter() {
                    if making_enough_progress(i, step.pressure_released) {
                        next_steps.insert(Step {
                            state_key: StateKey::new(
                                m_neighbor.clone(),
                                e_neighbor.clone(),
                                step.state_key.valve_state,
                            ),
                            pressure_released: step.pressure_released,
                        });
                    }
                }
            } // m moves, e moves
        }
        let (min, sum, max) = position_to_best_flow.values().fold(
            (usize::MAX, 0, usize::MIN),
            |(min, sum, max), pressure_released| {
                (
                    std::cmp::min(min, pressure_released.0),
                    sum + pressure_released.0,
                    std::cmp::max(max, pressure_released.0),
                )
            },
        );
        println!(
            "step: {:2} pressures (min, avg, max): ({:5} {:5} {:5}) next_steps.len(): {}",
            i,
            min,
            sum / position_to_best_flow.len(),
            max,
            next_steps.len()
        );
        current_steps = next_steps;
        position_to_best_flow
            .retain(|_state_key, pressure_released| making_enough_progress(i, *pressure_released));
    }

    position_to_best_flow.values().max().unwrap().0
}

fn find_best_path(
    valves: &BTreeMap<String, Valve>,
    pressurized_valve_bits: &BTreeMap<String, usize>,
) -> usize {
    let mut current_steps = vec![(("AA".to_owned(), ValveState(0)), PressureReleased(0))];
    let mut position_to_best_flow: BTreeMap<(String, ValveState), PressureReleased> =
        BTreeMap::new();
    for i in 0..30 {
        let mut next_steps = Vec::new();
        for step in current_steps.iter() {
            let current_pressure_released = &step.1;
            if let Some(most_pressure_released) = position_to_best_flow.get(&step.0) {
                if most_pressure_released >= &current_pressure_released {
                    continue;
                }
            }
            position_to_best_flow.insert(step.0.clone(), step.1);

            let current_valve = valves.get(&step.0 .0).unwrap();
            if current_valve.flow_rate > 0 {
                let current_valve_bit = pressurized_valve_bits.get(&current_valve.name).unwrap();

                if step.0 .1 .0 & current_valve_bit == 0 {
                    let mut new_valve_state = step.0 .1;
                    new_valve_state.0 |= current_valve_bit;

                    let new_pressure_released = *current_pressure_released
                        + PressureReleased(current_valve.flow_rate * (29 - i));
                    next_steps.push((
                        (current_valve.name.clone(), new_valve_state),
                        new_pressure_released,
                    ));
                }
            }

            for neighbor in current_valve.neighbors.iter() {
                next_steps.push(((neighbor.clone(), step.0 .1), step.1));
            }
        }
        current_steps = next_steps;
    }

    position_to_best_flow.values().max().unwrap().0
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let valves = parse_valves(&s);
    // for valve in valves.iter() {
    //     println!("{:?}", valve);
    // }
    // println!("");

    let pressurized_valve_bits: BTreeMap<String, usize> = valves
        .iter()
        .filter(|(_name, valve)| valve.flow_rate > 0)
        .enumerate()
        .map(|(index, (_name, valve))| (valve.name.to_owned(), (1 << index)))
        .collect();

    // for pvb in pressurized_valve_bits.iter() {
    //     println!("{:?}", pvb);
    // }

    let best_path = find_best_path(&valves, &pressurized_valve_bits);
    println!("Step 1) best pressure release is {}\n", best_path);
    let best_elephant_path = find_best_elephant_path(&valves, &pressurized_valve_bits);
    println!("Best elephant path is {}", best_elephant_path);
    Ok(())
}
