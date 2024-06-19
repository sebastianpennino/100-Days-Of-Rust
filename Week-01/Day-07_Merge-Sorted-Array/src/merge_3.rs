// Working backwards from the last element to the first
// Things I would like to improve some day: 
// - The casting to isize everywhere
// - Find a way to make this more human readable
pub fn merge_v3<'a>(main_array: &'a mut [i32], m: usize, aux_array: &[i32], n: usize) -> &'a mut [i32] {
    let mut i = m as isize - 1;
    let mut j = n as isize - 1;
    let mut pos = (m + n) as isize - 1;

    while pos >= 0 {
        if i >= 0 && j >= 0 {
            if main_array[i as usize] > aux_array[j as usize] {
                main_array[pos as usize] = main_array[i as usize];
                i -= 1;
            } else {
                main_array[pos as usize] = aux_array[j as usize];
                j -= 1;
            }
        } else if j >= 0 {
            main_array[pos as usize] = aux_array[j as usize];
            j -= 1;
        } else if i >= 0 {
            main_array[pos as usize] = main_array[i as usize];
            i -= 1;
        }
        pos -= 1;
    }

    return main_array;
}