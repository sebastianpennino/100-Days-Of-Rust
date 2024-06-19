/**
 * I don't think I'm following the exercise instructions with this method.
 * I'm creating a vector with the two arrays/slices, and then sorting them.
 * 
 * I think the intention of the excersise is to manuall merge/sort the two
 * already sorted arrays and also use the first array/slice to write the results.
 */
pub fn merge_v1(arr1: &mut [i32], arr2: &[i32]) -> Vec<i32> {
    let len = arr1.len();
    let mut result:Vec<i32>  = arr1[..len - arr2.len()].to_vec(); 

    result.extend_from_slice(&arr2);
    result.sort();
    return result
}
