use crate::common_stuff::find_next_same_or_higher;
use crate::common_stuff::get_slice;

fn calculate_water_arr(arr: &Vec<i16>, start: usize, end: usize) -> Vec<i16> {
    let left_side = arr[start];
    let right_side = arr[end];
    let water_level = std::cmp::min(left_side, right_side);
    let adjusted_end = end - 1;

    match get_slice(arr, start, adjusted_end) {
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

    // crawl the height_map
    while pos <= height_map.len() - 1 {
        // println!("[ pos: {}, val: {} ]", pos, height_map[pos]);
        let mut next_pos = pos + 1;

        if height_map[pos] > current_max_elevation {
            // println!("Yes > current_max_elevation");
            current_max_elevation = height_map[pos]; // save current max elevation

            match find_next_same_or_higher(&height_map, pos) {
                Some(move_to_pos) => {
                    // There's a gap!
                    if (move_to_pos - pos) > 1 {
                        next_pos = move_to_pos;
                        let found_water = calculate_water_arr(&height_map, pos, move_to_pos);
                        // println!("Found_water: {:?}", found_water);

                        let mut new_vec = accumulated_water.clone();

                        new_vec.extend(found_water);
                        // add the two arr

                        accumulated_water = new_vec.clone();
                    } else {
                        // println!("No gap -> adding a zero [0]");
                        accumulated_water.push(0);
                    }
                }
                None => {
                    // println!("Warn: No same or higher value found after index: {}", pos);
                    // println!("No higher value found -> adding a zero [0]");
                    current_max_elevation = -1; // TODO: Detect a peak!?
                    accumulated_water.push(0);
                }
            }
        } else {
            accumulated_water.push(0);
            // println!("Not higher that current_max_elevation");
            // println!("Not entered -> adding a zero [0]");
        }
        pos = next_pos;
    }

    return accumulated_water;
}

/**
* Going to the right and then to the left,
* saving the "found water" in a vec, then adding the two
*/
pub fn trap_water_v1(height_map: Vec<i16>) -> i16 {
    // let mut trimmed_map = trim_zeros(&height_map);
    let mut trimmed_map = height_map.clone();

    let water_ltr = find_water(&trimmed_map);
    trimmed_map.reverse();

    let mut water_rtl = find_water(&trimmed_map);
    // println!("water_rtl {:?} len {}", water_rtl, water_rtl.len());
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
