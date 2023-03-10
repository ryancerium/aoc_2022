use std::{collections::HashMap, hash::Hash};

use derive_more::{AddAssign, SubAssign};
use regex::Regex;

#[derive(SubAssign, Debug, Default, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Ore(i32);
#[derive(SubAssign, Debug, Default, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Clay(i32);
#[derive(SubAssign, Debug, Default, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Obsidian(i32);
#[derive(SubAssign, Debug, Default, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Geode(i32);
#[derive(
    AddAssign, SubAssign, Debug, Default, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord,
)]
struct OreRobot(i32);
#[derive(AddAssign, Debug, Default, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct ClayRobot(i32);
#[derive(AddAssign, Debug, Default, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct ObsidianRobot(i32);
#[derive(
    AddAssign, SubAssign, Debug, Default, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord,
)]
struct GeodeRobot(i32);

#[derive(Debug)]
struct Blueprint {
    index: i32,
    ore_robot_cost: Ore,
    clay_robot_cost: Ore,
    obsidian_robot_cost: (Ore, Clay),
    geode_robot_cost: (Ore, Obsidian),
}

impl Blueprint {
    fn generate_possible_states(
        &self,
        robot_state: &RobotState,
        resource_state: &ResourceState,
        next_states: &mut HashMap<RobotState, Vec<ResourceState>>,
    ) {
        if false
            || robot_state.geode_robots > GeodeRobot(1)
            || resource_state.obsidian.0 < self.max_required_obsidian_robots().0
            || resource_state.clay.0 < self.max_required_clay_robots().0
            || resource_state.ore.0 < self.max_required_ore_robots().0
        {
            consolidate_next_states(
                next_states,
                *robot_state,
                robot_state.generate_resources(resource_state.clone()),
            );
        }
        self.generate_geode_robot_states(robot_state, resource_state, next_states);
        self.generate_obsidian_robot_states(robot_state, resource_state, next_states);
        self.generate_clay_robot_states(robot_state, resource_state, next_states);
        self.generate_ore_robot_states(robot_state, resource_state, next_states);
    }

    fn max_required_obsidian_robots(&self) -> ObsidianRobot {
        let obsidian_cost = self.geode_robot_cost.1;
        ObsidianRobot(obsidian_cost.0)
    }

    fn max_required_clay_robots(&self) -> ClayRobot {
        let clay_cost = self.obsidian_robot_cost.1;
        ClayRobot(clay_cost.0)
    }

    fn max_required_ore_robots(&self) -> OreRobot {
        OreRobot(
            std::cmp::max(
                self.ore_robot_cost,
                std::cmp::max(
                    self.clay_robot_cost,
                    std::cmp::max(self.obsidian_robot_cost.0, self.geode_robot_cost.0),
                ),
            )
            .0,
        )
    }

    fn generate_geode_robot_states(
        &self,
        robot_state: &RobotState,
        resource_state: &ResourceState,
        next_states: &mut HashMap<RobotState, Vec<ResourceState>>,
    ) {
        if resource_state.ore >= self.geode_robot_cost.0
            && resource_state.obsidian >= self.geode_robot_cost.1
        {
            let mut next_robot = robot_state.clone();
            let mut next_resource = resource_state.clone();
            next_resource.ore -= self.geode_robot_cost.0;
            next_resource.obsidian -= self.geode_robot_cost.1;
            next_robot.geode_robots += GeodeRobot(1);
            consolidate_next_states(
                next_states,
                next_robot,
                robot_state.generate_resources(next_resource),
            );
        }
    }

    fn generate_obsidian_robot_states(
        &self,
        robot_state: &RobotState,
        resource_state: &ResourceState,
        next_states: &mut HashMap<RobotState, Vec<ResourceState>>,
    ) {
        if robot_state.obsidian_robots < self.max_required_obsidian_robots()
            && resource_state.ore >= self.obsidian_robot_cost.0
            && resource_state.clay >= self.obsidian_robot_cost.1
        {
            let mut next_robot = robot_state.clone();
            let mut next_resource = resource_state.clone();
            next_resource.ore -= self.obsidian_robot_cost.0;
            next_resource.clay -= self.obsidian_robot_cost.1;
            next_robot.obsidian_robots += ObsidianRobot(1);
            consolidate_next_states(
                next_states,
                next_robot,
                robot_state.generate_resources(next_resource),
            );
        }
    }

    fn generate_clay_robot_states(
        &self,
        robot_state: &RobotState,
        resource_state: &ResourceState,
        next_states: &mut HashMap<RobotState, Vec<ResourceState>>,
    ) {
        if robot_state.clay_robots < self.max_required_clay_robots()
            && resource_state.ore >= self.clay_robot_cost
        {
            let mut next_robot = robot_state.clone();
            let mut next_resource = resource_state.clone();
            next_resource.ore -= self.clay_robot_cost;
            next_robot.clay_robots += ClayRobot(1);
            consolidate_next_states(
                next_states,
                next_robot,
                robot_state.generate_resources(next_resource),
            );
        }
    }

    fn generate_ore_robot_states(
        &self,
        robot_state: &RobotState,
        resource_state: &ResourceState,
        next_states: &mut HashMap<RobotState, Vec<ResourceState>>,
    ) {
        if robot_state.ore_robots < self.max_required_ore_robots()
            && resource_state.ore >= self.ore_robot_cost
        {
            let mut next_robot = robot_state.clone();
            let mut next_resource = resource_state.clone();
            next_resource.ore -= self.ore_robot_cost;
            next_robot.ore_robots += OreRobot(1);
            consolidate_next_states(
                next_states,
                next_robot,
                robot_state.generate_resources(next_resource),
            );
        }
    }
}

fn consolidate_next_states(
    next_states: &mut HashMap<RobotState, Vec<ResourceState>>,
    next_robot: RobotState,
    next_resource: ResourceState,
) {
    next_states
        .entry(next_robot)
        .and_modify(|resource_states| {
            let mut inserted = false;
            for resource_state in resource_states.iter_mut() {
                if next_resource.strictly_superior(resource_state) {
                    *resource_state = next_resource;
                    inserted = true;
                } else if resource_state.strictly_superior(&next_resource) {
                    return;
                }
            }
            if !inserted {
                resource_states.push(next_resource);
            }
        })
        .or_insert(vec![next_resource]);
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct RobotState {
    geode_robots: GeodeRobot,
    obsidian_robots: ObsidianRobot,
    clay_robots: ClayRobot,
    ore_robots: OreRobot,
}

impl RobotState {
    fn new() -> RobotState {
        RobotState {
            geode_robots: GeodeRobot(0),
            obsidian_robots: ObsidianRobot(0),
            clay_robots: ClayRobot(0),
            ore_robots: OreRobot(1),
        }
    }

    fn generate_resources(&self, mut next_resource: ResourceState) -> ResourceState {
        next_resource.ore.0 += self.ore_robots.0;
        next_resource.clay.0 += self.clay_robots.0;
        next_resource.obsidian.0 += self.obsidian_robots.0;
        next_resource.geode.0 += self.geode_robots.0;
        next_resource
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct ResourceState {
    geode: Geode,
    obsidian: Obsidian,
    clay: Clay,
    ore: Ore,
}

impl ResourceState {
    fn strictly_superior(&self, other: &Self) -> bool {
        self.geode >= other.geode
            && self.obsidian >= other.obsidian
            && self.clay >= other.clay
            && self.ore >= other.ore
            && (self.geode > other.geode
                || self.obsidian > other.obsidian
                || self.clay > other.clay
                || self.ore > other.ore)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    geode: Geode,
    geode_robots: GeodeRobot,
    obsidian: Obsidian,
    obsidian_robots: ObsidianRobot,
    clay: Clay,
    clay_robots: ClayRobot,
    ore: Ore,
    ore_robots: OreRobot,
}

impl Default for State {
    fn default() -> Self {
        Self {
            geode: Default::default(),
            geode_robots: Default::default(),
            obsidian: Default::default(),
            obsidian_robots: Default::default(),
            clay: Default::default(),
            clay_robots: Default::default(),
            ore: Default::default(),
            ore_robots: OreRobot(1),
        }
    }
}

fn find_most_geodes(blueprint: &Blueprint, minutes: i32) -> Geode {
    let mut states: HashMap<RobotState, Vec<ResourceState>> = HashMap::new();

    states.insert(RobotState::new(), vec![ResourceState::default()]);

    for _i in 0..minutes {
        let mut next_states = HashMap::new();

        for (robot_state, ore_states) in states.iter() {
            for ore_state in ore_states.iter() {
                blueprint.generate_possible_states(robot_state, ore_state, &mut next_states);
            }
        }

        states = next_states;

        for (_, resource_states) in states.iter_mut() {
            resource_states.sort();
            resource_states.dedup();
        }

        // Prune the shitty geode states out
        let (mut most_geode_robots, mut most_geodes) = states.iter().fold(
            (GeodeRobot(0), Geode(0)),
            |(max_geode_robots, max_geodes), (robot_state, resource_states)| {
                (
                    std::cmp::max(max_geode_robots, robot_state.geode_robots),
                    resource_states
                        .iter()
                        .fold(max_geodes, |max_geodes, resource_state| {
                            std::cmp::max(resource_state.geode, max_geodes)
                        }),
                )
            },
        );

        if most_geode_robots > GeodeRobot(3) {
            most_geode_robots -= GeodeRobot(2);
            states.retain(|robot_state, _| robot_state.geode_robots > most_geode_robots);
        }

        if most_geodes > Geode(3) {
            most_geodes -= Geode(3);
            states.iter_mut().for_each(|(_, resource_states)| {
                resource_states.retain(|resource| resource.geode > most_geodes)
            });
        }

        // let n_states = states
        //     .iter()
        //     .fold(0, |sum, (_, resource_states)| sum + resource_states.len());
        // println!("{_i}: {} robot states {n_states} res states", states.len());
    }

    let mut most_geodes = Geode(0);
    for (_, resource_states) in states.iter() {
        for resource_state in resource_states.iter() {
            most_geodes = std::cmp::max(most_geodes, resource_state.geode);
        }
    }
    most_geodes
}

fn part1(blueprints: &Vec<Blueprint>) {
    let mut sum = 0;
    for blueprint in blueprints.iter() {
        let geodes = find_most_geodes(blueprint, 24);
        println!("Blueprint {} can make {:?}", blueprint.index, geodes);
        sum += geodes.0 * blueprint.index;
    }
    println!("Total quality is: {sum}");
}

fn part2(blueprints: &Vec<Blueprint>) {
    let mut product = 1;
    for blueprint in blueprints.iter().take(3) {
        let geodes = find_most_geodes(blueprint, 32);
        println!("Blueprint {} can make {:?}", blueprint.index, geodes);
        product *= geodes.0 as usize;
    }
    println!("Total quality is: {product}");
}

fn parse_blueprints(s: &String) -> Vec<Blueprint> {
    let r = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    s.lines()
        .map(|line| {
            let captures = r.captures(line).unwrap();

            Some(Blueprint {
                index: captures.get(1)?.as_str().parse::<i32>().ok()?,
                ore_robot_cost: Ore(captures.get(2)?.as_str().parse::<i32>().ok()?),
                clay_robot_cost: Ore(captures.get(3)?.as_str().parse::<i32>().ok()?),
                obsidian_robot_cost: (
                    Ore(captures.get(4)?.as_str().parse::<i32>().ok()?),
                    Clay(captures.get(5)?.as_str().parse::<i32>().ok()?),
                ),
                geode_robot_cost: (
                    Ore(captures.get(6)?.as_str().parse::<i32>().ok()?),
                    Obsidian(captures.get(7)?.as_str().parse::<i32>().ok()?),
                ),
            })
        })
        .filter_map(|blueprint| blueprint)
        .collect()
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let blueprints = parse_blueprints(&s);

    // for blueprint in blueprints.iter() {
    //     println!("{blueprint:?}");
    // }

    part1(&blueprints);
    part2(&blueprints);

    Ok(())
}
