use itertools::Itertools;
use regex::{Match, Regex};

#[derive(PartialEq)]
enum NextBuy {
    Unknown,
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct RobotCost {
    ore: isize,
    clay: isize,
    obsidian: isize,
}

struct BluePrint {
    id: isize,
    ore_robot_cost: RobotCost,
    clay_robot_cost: RobotCost,
    obsidian_robot_cost: RobotCost,
    geode_robot_cost: RobotCost,
}

fn parse_input(file: &str) -> Vec<BluePrint> {
    fn parse_re_capture_int(capture: Option<Match>) -> isize {
        capture.unwrap().as_str().parse::<isize>().unwrap()
    }
    let re = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
    std::fs::read_to_string(file).unwrap()
        .lines().map(|line| {
        let caps = re.captures(line).expect("regex matches line");
        BluePrint {
            id: parse_re_capture_int(caps.get(1)),
            ore_robot_cost: RobotCost {
                ore: parse_re_capture_int(caps.get(2)),
                clay: 0,
                obsidian: 0,
            },
            clay_robot_cost: RobotCost {
                ore: parse_re_capture_int(caps.get(3)),
                clay: 0,
                obsidian: 0,
            },
            obsidian_robot_cost: RobotCost {
                ore: parse_re_capture_int(caps.get(4)),
                clay: parse_re_capture_int(caps.get(5)),
                obsidian: 0,
            },
            geode_robot_cost: RobotCost {
                ore: parse_re_capture_int(caps.get(6)),
                clay: 0,
                obsidian: parse_re_capture_int(caps.get(7)),
            },
        }
    }).collect_vec()
}

fn calc_quality(blueprint: &BluePrint, best: &mut isize, next_buy: NextBuy, minute: isize, ore: isize, clay: isize, obsidian: isize, geode: isize, ore_robots: isize, clay_robots: isize, obsidian_robots: isize, geode_robots: isize) -> isize {
    *best = (*best).max(geode);
    if minute == 0 {
        geode
    } else if ore_robots > blueprint.ore_robot_cost.ore && ore_robots > blueprint.clay_robot_cost.ore && ore_robots > blueprint.obsidian_robot_cost.ore && ore_robots > blueprint.geode_robot_cost.ore {
        0
    } else if clay_robots > blueprint.ore_robot_cost.clay && clay_robots > blueprint.clay_robot_cost.clay && clay_robots > blueprint.obsidian_robot_cost.clay && clay_robots > blueprint.geode_robot_cost.clay {
        0
    } else if obsidian_robots > blueprint.ore_robot_cost.obsidian && obsidian_robots > blueprint.clay_robot_cost.obsidian && obsidian_robots > blueprint.obsidian_robot_cost.obsidian && obsidian_robots > blueprint.geode_robot_cost.obsidian {
        0
    } else if geode + minute * geode_robots + minute * (minute - 1) / 2 <= *best {
        0
    } else if next_buy == NextBuy::Ore && blueprint.ore_robot_cost.ore <= ore && blueprint.ore_robot_cost.clay <= clay && blueprint.ore_robot_cost.obsidian <= obsidian {
        calc_quality(blueprint, best, NextBuy::Unknown, minute - 1, ore + ore_robots - blueprint.ore_robot_cost.ore, clay + clay_robots - blueprint.ore_robot_cost.clay, obsidian + obsidian_robots - blueprint.ore_robot_cost.obsidian, geode + geode_robots, ore_robots + 1, clay_robots, obsidian_robots, geode_robots)
    } else if next_buy == NextBuy::Clay && blueprint.clay_robot_cost.ore <= ore && blueprint.clay_robot_cost.clay <= clay && blueprint.clay_robot_cost.obsidian <= obsidian {
        calc_quality(blueprint, best, NextBuy::Unknown, minute - 1, ore + ore_robots - blueprint.clay_robot_cost.ore, clay + clay_robots - blueprint.clay_robot_cost.clay, obsidian + obsidian_robots - blueprint.clay_robot_cost.obsidian, geode + geode_robots, ore_robots, clay_robots + 1, obsidian_robots, geode_robots)
    } else if next_buy == NextBuy::Obsidian && blueprint.obsidian_robot_cost.ore <= ore && blueprint.obsidian_robot_cost.clay <= clay && blueprint.obsidian_robot_cost.obsidian <= obsidian {
        calc_quality(blueprint, best, NextBuy::Unknown, minute - 1, ore + ore_robots - blueprint.obsidian_robot_cost.ore, clay + clay_robots - blueprint.obsidian_robot_cost.clay, obsidian + obsidian_robots - blueprint.obsidian_robot_cost.obsidian, geode + geode_robots, ore_robots, clay_robots, obsidian_robots + 1, geode_robots)
    } else if next_buy == NextBuy::Geode && blueprint.geode_robot_cost.ore <= ore && blueprint.geode_robot_cost.clay <= clay && blueprint.geode_robot_cost.obsidian <= obsidian {
        calc_quality(blueprint, best, NextBuy::Unknown, minute - 1, ore + ore_robots - blueprint.geode_robot_cost.ore, clay + clay_robots - blueprint.geode_robot_cost.clay, obsidian + obsidian_robots - blueprint.geode_robot_cost.obsidian, geode + geode_robots, ore_robots, clay_robots, obsidian_robots, geode_robots + 1)
    } else if next_buy != NextBuy::Unknown {
        calc_quality(blueprint, best, next_buy, minute - 1, ore + ore_robots, clay + clay_robots, obsidian + obsidian_robots, geode + geode_robots, ore_robots, clay_robots, obsidian_robots, geode_robots)
    } else {
        let mut max_geode = geode;
        max_geode = max_geode.max(calc_quality(blueprint, best, NextBuy::Ore, minute, ore, clay, obsidian, geode, ore_robots, clay_robots, obsidian_robots, geode_robots));
        max_geode = max_geode.max(calc_quality(blueprint, best, NextBuy::Clay, minute, ore, clay, obsidian, geode, ore_robots, clay_robots, obsidian_robots, geode_robots));
        max_geode = max_geode.max(calc_quality(blueprint, best, NextBuy::Obsidian, minute, ore, clay, obsidian, geode, ore_robots, clay_robots, obsidian_robots, geode_robots));
        max_geode = max_geode.max(calc_quality(blueprint, best, NextBuy::Geode, minute, ore, clay, obsidian, geode, ore_robots, clay_robots, obsidian_robots, geode_robots));
        max_geode
    }
}

fn star_1(blueprints: &[BluePrint]) -> isize {
    let mut val = 0;
    for blueprint in blueprints {
        val += blueprint.id * calc_quality(blueprint, &mut 0, NextBuy::Unknown, 24, 0, 0, 0, 0, 1, 0, 0, 0);
    }
    val
}

fn star_2(blueprints: &[BluePrint]) -> isize {
    let mut val = 1;
    for idx in 0..3 {
        val *= calc_quality(&blueprints[idx], &mut 0, NextBuy::Unknown, 32, 0, 0, 0, 0, 1, 0, 0, 0);
    }
    val
}

fn main() {
    let input_file = "src/bin/day_19/input.txt";
    let blueprints = parse_input(input_file);
    let out_1 = star_1(&blueprints);
    println!("star 1: {out_1}");
    let out_2 = star_2(&blueprints);
    println!("star 2: {out_2}");
}
