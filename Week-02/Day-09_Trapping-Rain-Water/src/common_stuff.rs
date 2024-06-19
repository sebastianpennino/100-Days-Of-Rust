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