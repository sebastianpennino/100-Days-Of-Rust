use crate::common_stuff::get_slice;
use crate::common_stuff::find_next_same_or_higher;

fn calculate_water(arr: &Vec<i16>, start: usize, end: usize) -> i16 {
    let left_side = arr[start];
    let right_side = arr[end];
    let water_level = std::cmp::min(left_side, right_side);
    let mut water: i16 = 0;

    match get_slice(arr, start, end) {
        Some(sliced) => {
            water = sliced
                .iter()
                .map(|x| {
                    // Using the lower side as water level, return the diff
                    return std::cmp::max(water_level - x, 0);
                })
                .sum();
        }
        None => {
            return water;
        }
    }

    return water;
}

/**
* Once I found a "pillar" I try to find another with same or higher height
* to the right then I count the water, and "move there" for the next loop.
*/
pub fn trap_water_v1(height_map: Vec<i16>) -> i16 {
    let mut current_max_elevation: i16 = 0;
    let mut accumulated_water: i16 = 0;
    let mut pos = 0;

    // loop the height_map
    while pos <= height_map.len() - 1 {
        let mut next_pos = pos + 1;
        if height_map[pos] > current_max_elevation {
            current_max_elevation = height_map[pos]; // Save current max elevation
            match find_next_same_or_higher(&height_map, pos) {
                Some(move_to_pos) => {
                    // There's a gap!
                    if (move_to_pos - pos) > 1 {
                        next_pos = move_to_pos;
                        accumulated_water += calculate_water(&height_map, pos, move_to_pos);
                    }
                }
                None => {
                    println!("Warn: No same or higher value found after index: {}", pos);
                    current_max_elevation = -1; // TODO: Detect a peak!?
                }
            }
        }
        pos = next_pos;
    }

    return accumulated_water;
}


// MODIFY THIS TO GET A MAP OF WATER.
// ON THE TEST WE CHECK IT AND THEN SUM, AND CHECK THE SUM!