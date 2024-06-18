pub fn merge_v1(arr1: &mut [i32], arr2: &[i32]) -> Vec<i32> {
    let mut temp: Vec<i32> = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;
    let mut loops = 0;

    while loops < arr1.len() {
        if i <= arr1.len() && j <= arr2.len() {
            // num1 is special because 0 are actually "empty" spaces to fill
            if arr1[i] > arr2[j] || arr1[i] == 0 {
                temp.push(arr2[j]);
                j += 1;
            } else {
                temp.push(arr1[i]);
                i += 1;
            }
        } else if i <= arr1.len() {
            temp.push(arr1[i]);
            i += 1;
        } else if  j <= arr2.len() {
            temp.push(arr2[j]);
            j += 1;
        }
        loops += 1;
    }

    return temp;
}
