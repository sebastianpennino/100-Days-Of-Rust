// Currently fails. Will find another way.
pub fn merge_v4<'a>(main_array: &'a mut Vec<i32>, m: usize, aux_array: &Vec<i32>, n: usize) -> &'a mut Vec<i32> {
    let mut last = m + n - 1;
    let mut i = m as isize - 1;
    let mut j = n as isize - 1;

    while j >= 0 {
        if i >= 0 && main_array[i as usize] > aux_array[j as usize] {
            main_array[last] = main_array[i as usize];
            i -= 1;
        } else {
            main_array[last] = aux_array[j as usize];
            j -= 1;
        }
        last -= 1;
    }

    return main_array;
}
