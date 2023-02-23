use regex::Regex;
use std::{collections::VecDeque, io::BufRead};

#[derive(Copy, Clone)]
struct State {
    num_ore_robot: i32,
    num_clay_robot: i32,
    num_obsidian_robot: i32,
    num_geode_robot: i32,
    ore: i32,
    clay: i32,
    obisidan: i32,
    geode: i32,
    minutes_left: i32,
}

impl State {
    fn generate_resources(&mut self) {
        self.ore += self.num_ore_robot;
        self.clay += self.num_clay_robot;
        self.obisidan += self.num_obsidian_robot;
        self.geode += self.num_geode_robot;
        self.minutes_left -= 1;
    }
}

fn calculate_blueprint(
    ore_cost: i32,
    clay_cost: i32,
    obsidian_cost: (i32, i32),
    geode_cost: (i32, i32),
    minute_limits: (i32, i32, i32, i32),
) -> i32 {
    let initial_state = State {
        num_ore_robot: 1,
        num_clay_robot: 0,
        num_obsidian_robot: 0,
        num_geode_robot: 0,
        ore: 0,
        clay: 0,
        obisidan: 0,
        geode: 0,
        minutes_left: 32,
    };

    let mut stack = VecDeque::new();
    stack.push_front(initial_state);
    let mut max_geode = 0;
    while let Some(mut state) = stack.pop_front() {
        if state.minutes_left <= 0 {
            continue;
        }

        if state.ore >= ore_cost
            && (state.num_ore_robot < ore_cost
                || state.num_ore_robot < clay_cost
                || state.num_ore_robot < obsidian_cost.0
                || state.num_ore_robot < geode_cost.0)
            && state.minutes_left >= minute_limits.0
        {
            let mut new_state = state;
            new_state.ore -= ore_cost;
            new_state.generate_resources();
            new_state.num_ore_robot += 1;
            stack.push_front(new_state);
        }

        let no_more_ore_bots = state.num_ore_robot
            >= ore_cost
                .max(clay_cost)
                .max(obsidian_cost.0)
                .max(geode_cost.0);
        if state.ore >= clay_cost
            && state.num_clay_robot < obsidian_cost.1
            && state.minutes_left >= minute_limits.1
            && no_more_ore_bots
        {
            let mut new_state = state;
            new_state.generate_resources();
            new_state.ore -= clay_cost;
            new_state.num_clay_robot += 1;
            stack.push_front(new_state);
        }

        if state.clay >= obsidian_cost.1
            && state.ore >= obsidian_cost.0
            && state.num_obsidian_robot < geode_cost.1
            && state.minutes_left >= minute_limits.2
            && no_more_ore_bots
        {
            let mut new_state = state;
            new_state.generate_resources();
            new_state.ore -= obsidian_cost.0;
            new_state.clay -= obsidian_cost.1;
            new_state.num_obsidian_robot += 1;
            stack.push_front(new_state);
        }

        if state.ore >= geode_cost.0
            && state.obisidan >= geode_cost.1
            && state.minutes_left >= minute_limits.3
            && no_more_ore_bots
        {
            let mut new_state = state;
            new_state.generate_resources();
            new_state.ore -= geode_cost.0;
            new_state.obisidan -= geode_cost.1;
            new_state.num_geode_robot += 1;
            stack.push_front(new_state);
        }

        if !(state.ore > ore_cost
            && state.ore > clay_cost
            && state.ore > obsidian_cost.0
            && state.clay > obsidian_cost.1
            && state.ore > geode_cost.0
            && state.obisidan > geode_cost.1)
        {
            let mut new_state = state;
            new_state.generate_resources();
            stack.push_front(new_state);
        }

        state.generate_resources();
        max_geode = max_geode.max(state.geode);
    }
    return max_geode;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day19.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();
    let re = Regex::new(
        r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.",
    ).unwrap();
    let mut quality_sum = 1;
    for line_result in lines {
        let line = line_result.ok().unwrap();
        let cap = re.captures_iter(&line).next().unwrap();
        let blueprint_id = cap[1].parse::<i32>().ok().unwrap();
        let ore_robot_cost_ore = cap[2].parse::<i32>().ok().unwrap();
        let clay_robot_cost_ore = cap[3].parse::<i32>().ok().unwrap();
        let obsidian_robot_cost_ore = cap[4].parse::<i32>().ok().unwrap();
        let obsidian_robot_cost_clay = cap[5].parse::<i32>().ok().unwrap();
        let geode_robot_cost_ore = cap[6].parse::<i32>().ok().unwrap();
        let geode_robot_cost_obsidian = cap[7].parse::<i32>().ok().unwrap();

        let num_geode = calculate_blueprint(
            ore_robot_cost_ore,
            clay_robot_cost_ore,
            (obsidian_robot_cost_ore, obsidian_robot_cost_clay),
            (geode_robot_cost_ore, geode_robot_cost_obsidian),
            (20, 10, 3, 0),
        );
        quality_sum *= num_geode;
        println!(
            "Blueprint {} score {}, curr quality sum {}",
            blueprint_id, num_geode, quality_sum
        );
        if blueprint_id == 3 {
            break;
        }
    }
    println!("Product of max geodes: {}", quality_sum);
    Ok(())
}
