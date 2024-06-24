
/**
 * Instead of searching like v1, I will just save
 * in two vec the highest values "to the right" and
 * "to the left" and then just use the min of the two
 * minus the height map to calculate the water in
 * a given column
 */
pub fn trap_water_v2(height_map: Vec<i16>) -> i16 {
    let n = height_map.len();
    if n == 0 {
        return 0;
    }

    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let mut acc_water = 0;

    // Fill left array
    left[0] = height_map[0]; // the leftmost item is always the same
    // completing from the index 1
    for i in 1..n {
        // this will loop from 1 to n-1 (0..x is exclusive range; it does not include the endpoint x)
        left[i] = left[i - 1].max(height_map[i]);
    }

    // Fill right array (reverse order)
    right[n - 1] = height_map[n - 1]; // the furthest to the right item is the same
    // note: The .rev() method reverses the order of the items produced by the iterator
    for i in (0..n - 1).rev() {
        // this will go from n-2 to 0 (0..x is exclusive range; it does not include the endpoint x)
        right[i] = right[i + 1].max(height_map[i]);
    }

    // Calculate the accumulated water
    for i in 0..n {
        // using the lowest height as water level
        acc_water += left[i].min(right[i]) - height_map[i];
    }

    return acc_water;
}
