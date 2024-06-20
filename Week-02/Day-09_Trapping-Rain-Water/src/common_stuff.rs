/**
* Safely Returns a portion of a vec with ranges
*/
pub fn get_slice(arr: &Vec<i16>, start: usize, end: usize) -> Option<&[i16]> {
    if start > end || end >= arr.len() {
        return None;
    }
    return Some(&arr[start..=end]);
}

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
 * Removes zeroes at the beginning and the end of a vec
 */
pub fn trim_zeros(arr: &Vec<i16>) -> Vec<i16> {
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