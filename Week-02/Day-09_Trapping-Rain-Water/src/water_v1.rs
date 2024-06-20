pub fn trap_water_v1(height_map: Vec<i16>) -> i16 {
    let mut left_max: i16 = 0;
    let mut right_max: i16 = 0;

    let left_map = height_map
        .iter()
        .rev()
        .map(|&x| {
            left_max = left_max.max(x);
            return left_max;
        })
        .collect::<Vec<i16>>();

    println!("left_map: {:?}", left_map);

    let right_map = height_map
        .iter()
        .map(|&x| {
            right_max = right_max.max(x);
            return right_max;
        })
        .collect::<Vec<i16>>();

    println!("right_map: {:?}", right_map);

    let accumulated_water = height_map
        .iter()
        .zip(left_map.iter().zip(right_map.iter()))
        .map(|(&current, (&left_max, &right_max))| return left_max.min(right_max) - current);

    return accumulated_water.sum();
}
