use crate::common_stuff::get_slice;
use crate::common_stuff::find_next_same_or_higher;

fn _trim_zeros(arr: &Vec<i16>) -> Vec<i16> {
    // Find the first non-zero index
    let start = arr.iter().position(|&x| x != 0).unwrap_or(arr.len());
    // Find the last non-zero index
    let end = arr.iter().rposition(|&x| x != 0).unwrap_or(0);
    // If start is greater than end, return an empty Vec
    if start > end {
        return Vec::new();
    }
    // Return the slice from start to end inclusive
    arr[start..=end].to_vec()
}

fn calculate_water_arr(arr: &Vec<i16>, start: usize, end: usize) -> Vec<i16> {
    let left_side = arr[start];
    let right_side = arr[end];
    let water_level = std::cmp::min(left_side, right_side);

    match get_slice(arr, start, end) {
        Some(sliced) => {
            let water = sliced
                .iter()
                .map(|x| {
                    // Using the lower side as water level, return the diff
                    std::cmp::max(water_level - x, 0)
                })
                .collect();

            return water;
        }
        None => {
            return vec![]; // Return an empty vector if slicing fails
        }
    }
}

fn find_water(height_map: &Vec<i16>) -> Vec<i16> {
    let mut pos = 0;
    let mut current_max_elevation: i16 = 0;
    let mut accumulated_water: Vec<i16> = vec![];
    // loop the height_map
    while pos <= height_map.len() - 1 {
        let mut next_pos = pos + 1;
        if height_map[pos] > current_max_elevation {
            current_max_elevation = height_map[pos]; // save current max elevation
            match find_next_same_or_higher(&height_map, pos) {
                Some(move_to_pos) => {
                    // There's a gap!
                    if (move_to_pos - pos) > 1 {
                        next_pos = move_to_pos;
                        let found_water = calculate_water_arr(&height_map, pos, move_to_pos);

                        let mut new_vec = accumulated_water.clone();

                        new_vec.extend(found_water);
                        // add the two arr

                        accumulated_water = new_vec.clone();
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

/**
* Same as v1, but going to the right and then to the left,
* saving the "found water" in a vec, then adding the two
*/
pub fn trap_water_v2(height_map: Vec<i16>) -> i16 {
    // let mut trimmed_map = trim_zeros(&height_map);
    let mut trimmed_map = height_map.clone();

    let water_ltr = find_water(&trimmed_map);

    trimmed_map.reverse();

    let mut water_rtl = find_water(&trimmed_map);

    water_rtl.reverse();

    let water_map_sum: i16 = water_ltr
        .into_iter()
        .zip(water_rtl.into_iter())
        .map(|(a, b)| {
            // return the bigger
            std::cmp::max(a, b)
        })
        .sum();

    return water_map_sum;
}
