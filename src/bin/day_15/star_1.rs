use itertools::Itertools;
use crate::{Beacon, get_ranges_on_y};

fn get_num_beacons_on_y(beacons: &[Beacon], y: i64) -> usize {
    beacons.iter().filter_map(|beacon| if beacon.by == y {
        Some(beacon.bx)
    } else {
        None
    }).unique().count()
}

pub(crate) fn run(beacons: &[Beacon], y: i64) -> u64 {
    get_ranges_on_y(beacons, y).iter()
        .map(|(start, end)| end.abs_diff(*start) + 1)
        .sum::<u64>() - get_num_beacons_on_y(beacons, y) as u64
}
