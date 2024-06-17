/**
 * Finds the next index in the vec that has the same or higher value
 */
pub fn find_next_same_or_higher(arr: &Vec<i16>, current_index: usize) -> Option<usize> {
    if current_index >= arr.len() {
        return None;
    }
    let current_value = arr[current_index];
    for (index, &value) in arr.iter().enumerate().skip(current_index + 1) {
        if value >= current_value {
            return Some(index);
        }
    }
    return None;
}
/**
 * Safely Returns a portion of a vec with ranges
 */
pub fn get_slice(arr: &Vec<i16>, start: usize, end: usize) -> Option<&[i16]> {
    if start > end || end >= arr.len() {
        return None;
    }
    return Some(&arr[start..=end]);
}

pub fn calculate_water(arr: &Vec<i16>, start: usize, end: usize) -> i16 {
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
pub fn trap_water_v1(height_map: Vec<i16>) -> i16 {
    let mut current_max_elevation: i16 = 0;
    let mut accumulated_water: i16 = 0;
    let mut pos = 0;

    // loop the height_map
    while pos <= height_map.len() - 1 {
        let mut next_pos = pos + 1;
        if height_map[pos] > current_max_elevation {
            current_max_elevation = height_map[pos]; // Log current max elevation
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_1() {
        let height_map: Vec<i16> = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let output = trap_water_v1(height_map);
        assert_eq!(output, 6);
    }

    #[test]
    fn water_2() {
        let height_map: Vec<i16> = vec![4, 2, 0, 3, 2, 5];
        let output = trap_water_v1(height_map);
        assert_eq!(output, 9);
    }

    #[test]
    fn water_3() {
        let height_map: Vec<i16> = vec![0, 0, 4, 0, 0, 5, 0, 5];
        let output = trap_water_v1(height_map);
        assert_eq!(output, 13);
    }

    #[test]
    fn water_4() {
        let height_map: Vec<i16> = vec![0, 0, 4, 0, 0, 5, 0, 1];
        let output = trap_water_v1(height_map);
        assert_eq!(output, 9);
    }
}
