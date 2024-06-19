/**
 * I don't think I'm following the exercise instructions with this method.
 * I'm creating a vector with the two arrays/slices, and then sorting them.
 *
 * I think the intention of the excersise is to manuall merge/sort the two
 * already sorted arrays.
 */
pub fn merge_v2<'a>(main_array: &'a mut [i32], aux_array: &[i32], n: usize) -> &'a [i32] {
    let len = main_array.len();
    let all_but_last_n: &mut [i32] = &mut main_array[..len - n];
    let mut merged = [all_but_last_n, aux_array].concat();
    merged.sort();
    let mut i: usize = 0;
    while i < main_array.len() {
        main_array[i] = merged[i];
        i += 1;
    }
    return main_array;
}
