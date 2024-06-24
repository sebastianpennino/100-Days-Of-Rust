
pub fn calc_left_map(height_map: &Vec<i16>) -> Vec<i16> {
    let mut left_map: Vec<i16> = height_map.iter().rev().enumerate().map(|(idx, &val)| {
        let map_slice = &height_map[idx..];
        let found_val = map_slice.iter().find(|&&v| v > val);
        match found_val {
            Some(&v) => v,
            None => val,
        }
    }).collect();
    // reverse before return
    left_map.reverse();

    return left_map;
}

pub fn calc_right_map(height_map: &Vec<i16>) -> Vec<i16> {
    let right_map: Vec<i16> = height_map.iter().enumerate().map(|(idx, &val)| {
        let map_slice = &height_map[idx..];
        let found_val = map_slice.iter().find(|&&v| v > val);
        match found_val {
            Some(&v) => v,
            None => val,
        }
    }).collect();

    return right_map;
}

pub fn trap_water_v1(height_map: Vec<i16>) -> i16 {

    let left_map: Vec<i16> = calc_left_map(&height_map);

    println!("left_map: {:?}", left_map);

    let right_map = calc_right_map(&height_map);

    println!("right_map: {:?}", right_map);

    let accumulated_water = height_map
        .iter()
        .zip(left_map.iter().zip(right_map.iter()))
        .map(|(&current, (&left_max, &right_max))| return left_max.min(right_max) - current);

    return accumulated_water.sum();
}
