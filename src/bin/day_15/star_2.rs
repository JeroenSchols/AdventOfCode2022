use crate::{Beacon, get_ranges_on_y};

pub(crate) fn run(beacons: &[Beacon], max_coord: i64) -> i64 {
    for y in 0..max_coord + 1 {
        for (_start, end) in get_ranges_on_y(beacons, y) {
            if -1 <= end && end < max_coord {
                return (end + 1) * 4000000 + y;
            }
        }
    }
    unreachable!()
}
