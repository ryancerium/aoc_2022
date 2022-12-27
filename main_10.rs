use std::hash::Hash;

use derive_more::{Add, AddAssign, Mul, Sub, SubAssign};
use regex::Regex;

#[derive(Add, Sub, Mul, SubAssign, Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Ore(i32);
#[derive(Add, Sub, Mul, SubAssign, Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Clay(i32);
#[derive(Add, Sub, Mul, SubAssign, Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Obsidian(i32);
#[derive(Add, Sub, Mul, SubAssign, Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Geode(i32);
#[derive(Add, AddAssign, Sub, Mul, Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct OreRobot(i32);
#[derive(Add, AddAssign, Sub, Mul, Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct ClayRobot(i32);
#[derive(Add, AddAssign, Sub, Mul, Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct ObsidianRobot(i32);
#[derive(Add, AddAssign, Sub, Mul, Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
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
    fn generate_possible_states(&self, state: &State) -> Vec<State> {
        let mut states = Vec::new();

        // base case:  Do nothing and let it ride!
        states.push(*state);
        self.generate_geode_robot_states(*state, &mut states);
        self.generate_obsidian_robot_states(*state, &mut states);
        self.generate_clay_robot_states(*state, &mut states);
        self.generate_ore_robot_states(*state, &mut states);
        self.generate_resources(state, &mut states);
        states
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
        let ore_cost = &[
            self.ore_robot_cost,
            self.clay_robot_cost,
            self.obsidian_robot_cost.0,
            self.geode_robot_cost.0,
        ]
        .into_iter()
        .max()
        .unwrap();
        OreRobot(ore_cost.0)
    }

    fn generate_geode_robot_states(&self, mut state: State, states: &mut Vec<State>) {
        if state.ore >= self.geode_robot_cost.0 && state.obsidian >= self.geode_robot_cost.1 {
            state.ore -= self.geode_robot_cost.0;
            state.obsidian -= self.geode_robot_cost.1;
            state.geode_robots += GeodeRobot(1);
            states.push(state);
        }
    }

    fn generate_obsidian_robot_states(&self, mut state: State, states: &mut Vec<State>) {
        if state.obsidian_robots < self.max_required_obsidian_robots()
            && state.ore >= self.obsidian_robot_cost.0
            && state.clay >= self.obsidian_robot_cost.1
        {
            state.ore -= self.obsidian_robot_cost.0;
            state.clay -= self.obsidian_robot_cost.1;
            state.obsidian_robots += ObsidianRobot(1);
            states.push(state);
        }
    }

    fn generate_clay_robot_states(&self, mut state: State, states: &mut Vec<State>) {
        if state.clay_robots < self.max_required_clay_robots() && state.ore >= self.clay_robot_cost
        {
            state.ore -= self.clay_robot_cost;
            state.clay_robots += ClayRobot(1);
            states.push(state);
        }
    }

    fn generate_ore_robot_states(&self, mut state: State, states: &mut Vec<State>) {
        if state.ore_robots < self.max_required_ore_robots() && state.ore >= self.ore_robot_cost {
            state.ore -= self.ore_robot_cost;
            state.ore_robots += OreRobot(1);
            states.push(state);
        }
    }

    fn generate_resources(&self, initial_state: &State, states: &mut Vec<State>) {
        states.iter_mut().for_each(|state| {
            state.ore.0 += initial_state.ore_robots.0;
       BMж         |   ,   ,                               џ  џ  џ      џ niW                                                                џ    џ    џ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ MIEРMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEРџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ MIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџMIEџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ {tџzsџysџ~xrџ}wqџ}wqџ|vpџysmџupiџojeџhc^џ`\Wџ[WRџVRMџTOKџOKHџOKHџSNJџUQNџYUPџ^ZUџd_Zџic_џlgaџmhbџmhcџmgbџlgaџkfaџje`џjd_џid_џџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ |uџ{uџztџzsџyrџ~xrџ|vpџxrlџqlfџid_џa\WџVRMџGD?џmjhџџџџџljgџEB?џUQLџ^YUџd`Zџie_џmhcџoidџnicџmhbџlgbџlfaџkfaџje`џџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ }vџ|vџ|uџ{tџztџ~xrџ|voџupiџlgaџb^YџQMIџywuџмммџђђђџђђђџђђђџђђђџђђђџђђђџммлџxvtџQNJџ^ZUџfa\џlfaџnicџojdџoidџnhcџmhbџlgbџkfaџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ ~xџ~wџ}vџ|uџ{uџyrџytmџqkfџfa]џRNIџІЅЃџђђђџђђђџђђђџяпЦџэЭџэЮџямЕџђђђџђђђџђђђџЅЃЂџSOKџb]Yџje_џnicџqkfџpjeџojdџnidџnhcџmhbџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ yџxџ~wџ}wџ}vџyrџwrlџnicџ]ZTџџђђђџђђђџэЭАџшЄ]џшЂMџшЄGџщІ@џщЉ9џъЏ>џюдџђђђџђђђџџ\XSџgb]џnhdџqlfџqlfџpkeџpjeџoidџnicџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ zџyџyџxџ}vџ~ysџwpkџkf`џca\џђђђџђђђџыОЈџцgџч`џчYџч RџшЂLџшЅEџщЇ>џщЉ8џэЪџђђђџђђђџc`\џe`[џmhcџqlfџsmgџrlgџqlfџpkeџojdџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ {џzџzџyџ~wџysџvqjџfa\џГВБџђђђџяйжџх{џхtџцmџцfџч^џчWџш QџшЃJџшЅCџщЈ<џ№рУџђђђџВАЏџc_Zџmhcџsmgџtnhџsmhџrmgџrlfџqkfџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ |џ|џ{џzџyџztџvqjџ_[UџнннџђђђџчЄЇџфџхџхyџцrџцkџцdџч]џчVџшЁOџшЄHџъЙmџђђђџнммџb^Yџnhcџsmhџupjџtoiџtnhџsmhџrmgџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ ~џ}џ|џ{џzџ{uџxrlџ^ZVџђђђџђђђџуџфџфџфџх~џхwџцpџцiџчbџч[џчTџшЂMџђђђџђђђџa\Xџojdџtoiџvqkџvpjџuojџtoiџsnhџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џ~џ}џ}џ{џ}vџzunџa]WџђђђџђђђџтЉџуЁџуџфџфџхџх|џхuџцnџцgџч`џчYџђђђџђђђџc^Zџqlfџwqkџxrlџwqkџvqkџvpjџuoiџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџ~џ~џ|џxџ}xqџfa\џђђђџђђђџтЎџтЎџтЇџуџуџфџфџхџхzџцsџцlџцeџђђђџђђђџif`џuoiџxrlџysmџxsmџxrlџwqkџvpjџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџџ~џ|џ|uџsmgџпооџђђђџфЖџтЎџтЎџтЌџуЅџуџуџфџфџхџхyџч џђђђџоонџpkeџxrlџztnџzunџztnџysmџxrlџwrlџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџџџ~џyџ}xrџКИЖџђђђџъИаџтЎџтЎџтЎџтЎџтЊџуЃџуџфџфџфџыСИџђђђџКЙЗџvpkџ{uoџ}wpџ|vpџ{uoџztnџztnџysmџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџџџџ|џ~wџ}xџђђђџђђђџхЛџтЎџтЎџтЎџтЎџтЎџтЈџуЁџуџчЄџђђђџђђђџ|xџ{uoџ~xqџ~xqџ}wqџ|wpџ|voџ{uoџztnџџџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџ џџџџџџџ|џ}vџЅЃ џђђђџёыюџцПџтЎџтЎџтЎџтЎџтЎџтЎџцЙџёыэџђђђџІЃ џ|voџyrџzsџџУЙЏџ~џ}wqџ|vpџ{