use itertools::Itertools;
use regex::{Match, Regex};

mod star_1;
mod star_2;

struct Beacon {
    x: i64,
    y: i64,
    bx: i64,
    by: i64,
}

fn parse_re_capture_int(capture: Option<Match>) -> i64 {
    capture.unwrap().as_str().parse::<i64>().unwrap()
}

fn parse_input(input_file: &str) -> Vec<Beacon> {
    let re = Regex::new(r"^Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)$").unwrap();
    std::fs::read_to_string(input_file).unwrap()
        .lines().map(|line| {
        let caps = re.captures(line).expect("regex matches line");
        Beacon {
            x: parse_re_capture_int(caps.get(1)),
            y: parse_re_capture_int(caps.get(2)),
            bx: parse_re_capture_int(caps.get(3)),
            by: parse_re_capture_int(caps.get(4)),
        }
    }).collect()
}

fn get_covered_range(beacon: &Beacon, y: i64) -> Option<(i64, i64)> {
    let manhattan_dist = beacon.x.abs_diff(beacon.bx) + beacon.y.abs_diff(beacon.by);
    let dx = manhattan_dist as i64 - beacon.y.abs_diff(y) as i64;
    if dx >= 0 {
        Some((beacon.x - dx, beacon.x + dx))
    } else {
        None
    }
}

fn join_ranges(ranges: Vec<Option<(i64, i64)>>) -> Vec<(i64, i64)> {
    let mut starts = ranges.iter()
        .filter_map(|range|
            range.as_ref().map(|(start, _end)| *start)
        ).sorted();
    let mut ends = ranges.iter()
        .filter_map(|range|
            range.as_ref().map(|(_start, end)| *end)
        ).sorted();
    let mut start = starts.next();
    let mut end = ends.next();
    let mut active = 0;
    let mut range_start = 0;
    let mut joined_ranges = Vec::new();
    while end.is_some() {
        if start.is_some() && start.unwrap() <= end.unwrap() {
            if active == 0 {
                range_start = start.unwrap();
            }
            active += 1;
            start = starts.next();
        } else {
            if active == 1 {
                joined_ranges.push((range_start, end.unwrap()));
            }
            active -= 1;
            end = ends.next();
        }
    }
    joined_ranges
}

fn get_ranges_on_y(beacons: &[Beacon], y: i64) -> Vec<(i64, i64)> {
    join_ranges(beacons.iter().map(|beacon| get_covered_range(beacon, y)).collect())
}

fn main() {
    let file = "src/bin/day_15/input.txt";
    let beacons = parse_input(file);
    let out_1 = star_1::run(&beacons, 2000000);
    println!("star 1: {out_1:?}");
    let out_2 = star_2::run(&beacons, 4000000);
    println!("start 2: {out_2:?}")
}
