use std::{cmp::min, error::Error};

use regex::Regex;

use super::PuzzleResult;

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost_ore: usize,
    clay_robot_cost_ore: usize,
    obsidian_robot_cost_ore: usize,
    obsidian_robot_cost_clay: usize,
    geode_robot_cost_ore: usize,
    geode_robot_cost_obsidian: usize,
}

fn parse_input(input: &str) -> Result<Vec<Blueprint>, Box<dyn Error>> {
    lazy_static! {
        static ref BLUEPRINT_RE: Regex = Regex::new(
            r"(?m)^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$"
        ).unwrap();
    }
    let mut blueprints = vec![];
    for (i, captures) in BLUEPRINT_RE.captures_iter(input).enumerate() {
        if i + 1 != captures[1].parse()? {
            return Err(format!(
                "Mismatched blueprint order: blueprint {} is in place {}",
                &captures[1],
                i + 1
            )
            .into());
        }
        blueprints.push(Blueprint {
            ore_robot_cost_ore: captures[2].parse()?,
            clay_robot_cost_ore: captures[3].parse()?,
            obsidian_robot_cost_ore: captures[4].parse()?,
            obsidian_robot_cost_clay: captures[5].parse()?,
            geode_robot_cost_ore: captures[6].parse()?,
            geode_robot_cost_obsidian: captures[7].parse()?,
        })
    }
    Ok(blueprints)
}

/*
    Optimization:
        - do not build more robots than the factory can consume
        - keep track of the best geode production, and prune the tree if it cannot keep up
        - try to build the best robots first
        - if mining resources are sufficient to build a geode bot at each turn, calculate the result of doing that and return it
      //  - calculate the result of waiting for each bot, and sen
*/

fn max_geodes(
    minutes: usize,
    (ore_bots, clay_bots, obsidian_bots, geode_bots): (usize, usize, usize, usize),
    (ore, clay, obsidian): (usize, usize, usize),
    blueprint: &Blueprint,
    max_bots: &(usize, usize, usize),
    best_until_now: usize,
) -> usize {
    if minutes == 0 {
        return 0; // time's up!
    }
    if geode_bots * minutes + minutes * (minutes - 1) / 2 <= best_until_now {
        // this branch cannot possibily beat the best, aborting
        return best_until_now;
    }
    if ore_bots >= blueprint.geode_robot_cost_ore
        && obsidian_bots >= blueprint.geode_robot_cost_obsidian
    {
        // simply produce only geode bots
        return if ore >= blueprint.geode_robot_cost_ore
            && obsidian >= blueprint.geode_robot_cost_obsidian
        {
            geode_bots * minutes + minutes * (minutes - 1) / 2
        } else {
            // we have to wait for resources
            geode_bots * minutes + (minutes - 1) * (minutes - 2) / 2
        };
    }
    // setting the bar to beat
    let mut best = if best_until_now > geode_bots {
        best_until_now - geode_bots
    } else {
        0
    };
    // i try to build robots, starting from the best possible ones
    if ore >= blueprint.geode_robot_cost_ore && obsidian >= blueprint.geode_robot_cost_obsidian {
        best = best.max(max_geodes(
            minutes - 1,
            (ore_bots, clay_bots, obsidian_bots, geode_bots + 1),
            (
                ore + ore_bots - blueprint.geode_robot_cost_ore,
                clay + clay_bots,
                obsidian + obsidian_bots - blueprint.geode_robot_cost_obsidian,
            ),
            blueprint,
            max_bots,
            best,
        ))
    }
    if ore >= blueprint.obsidian_robot_cost_ore
        && clay >= blueprint.obsidian_robot_cost_clay
        && obsidian_bots < max_bots.2
    {
        best = best.max(max_geodes(
            minutes - 1,
            (ore_bots, clay_bots, obsidian_bots + 1, geode_bots),
            (
                ore + ore_bots - blueprint.obsidian_robot_cost_ore,
                clay + clay_bots - blueprint.obsidian_robot_cost_clay,
                obsidian + obsidian_bots,
            ),
            blueprint,
            max_bots,
            best,
        ))
    }
    if ore >= blueprint.clay_robot_cost_ore && clay_bots < max_bots.1 {
        best = best.max(max_geodes(
            minutes - 1,
            (ore_bots, clay_bots + 1, obsidian_bots, geode_bots),
            (
                ore + ore_bots - blueprint.clay_robot_cost_ore,
                clay + clay_bots,
                obsidian + obsidian_bots,
            ),
            blueprint,
            max_bots,
            best,
        ))
    }
    if ore >= blueprint.ore_robot_cost_ore && ore_bots < max_bots.0 {
        best = best.max(max_geodes(
            minutes - 1,
            (ore_bots + 1, clay_bots, obsidian_bots, geode_bots),
            (
                ore + ore_bots - blueprint.ore_robot_cost_ore,
                clay + clay_bots,
                obsidian + obsidian_bots,
            ),
            blueprint,
            max_bots,
            best,
        ))
    }
    // last guess is i do nothing
    best = best.max(max_geodes(
        minutes - 1,
        (ore_bots, clay_bots, obsidian_bots, geode_bots),
        (ore + ore_bots, clay + clay_bots, obsidian + obsidian_bots),
        blueprint,
        max_bots,
        best,
    ));

    best + geode_bots
}

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let blueprints = parse_input(input)?;
    let blueprints_geodes = blueprints.iter().enumerate().map(|(i, blueprint)| {
        let geodes = max_geodes(
            24,
            (1, 0, 0, 0),
            (0, 0, 0),
            blueprint,
            // do not build more bots that the factory could consume
            &(
                [
                    blueprint.ore_robot_cost_ore,
                    blueprint.clay_robot_cost_ore,
                    blueprint.obsidian_robot_cost_ore,
                    blueprint.geode_robot_cost_ore,
                ]
                .into_iter()
                .max()
                .unwrap(),
                blueprint.obsidian_robot_cost_clay,
                blueprint.geode_robot_cost_obsidian,
            ),
            0,
        );
        // println!("Blueprint {} => {} geodes", i + 1, geodes);
        geodes
    });
    let quality_levels = blueprints_geodes
        .enumerate()
        .map(|(i, geodes)| (i + 1) * geodes);
    Ok(PuzzleResult::Numeric((quality_levels.sum::<usize>()) as _))
}

pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let all_blueprints = parse_input(input)?;
    let blueprints = all_blueprints[..3].iter();
    let blueprints_geodes = blueprints.enumerate().map(|(i, blueprint)| {
        let geodes = max_geodes(
            32,
            (1, 0, 0, 0),
            (0, 0, 0),
            blueprint,
            // do not build more bots that the factory could consume
            &(
                [
                    blueprint.ore_robot_cost_ore,
                    blueprint.clay_robot_cost_ore,
                    blueprint.obsidian_robot_cost_ore,
                    blueprint.geode_robot_cost_ore,
                ]
                .into_iter()
                .max()
                .unwrap(),
                blueprint.obsidian_robot_cost_clay,
                blueprint.geode_robot_cost_obsidian,
            ),
            0,
        );
        // println!("Blueprint {} => {} geodes", i + 1, geodes);
        geodes
    });
    Ok(PuzzleResult::Numeric(
        (blueprints_geodes.fold(1, |acc, v| acc * v)) as _,
    ))
}
