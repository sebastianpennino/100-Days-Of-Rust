pub fn merge_v3a<'a>(main_array: &'a mut [i32], m: usize, aux_array: &[i32], n: usize) -> &'a mut [i32] {
    let mut i = m - 1;
    let mut j = n - 1;
    let mut pos = m + n;
    println!("init, i={} j={} pos={}", i, j, pos);

    while pos > 0 {
        let max: &i32;
        let compare = [main_array[i], aux_array[j]];

        match compare[0].cmp(&compare[1]) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                max = &compare[1];
                if j > 0 {
                    j -= 1
                }
            }
            std::cmp::Ordering::Greater => {
                max = &compare[0];
                if i > 0 {
                    i -= 1
                }
            }
        };
        main_array[pos - 1] = *max;
        pos -= 1;
    }

    println!("{:?}", main_array);

    return main_array;
}

// THIS IS THE ONE THAT WORKS
pub fn merge_v3b<'a>(main_array: &'a mut [i32], m: usize, aux_array: &[i32], n: usize) -> &'a mut [i32] {
    let mut i = m as isize - 1;
    let mut j = n as isize - 1;
    let mut pos = (m + n) as isize - 1;
    println!("init, i={} j={} pos={}", i, j, pos);

    // let mut num1 = [1, 2, 3, 0, 0, 0];
    // let num2 = [2, 5, 6];

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

    println!("{:?}", main_array);

    return main_array;
}

pub fn merge_v3c<'a>(main_arr: &'a mut [i32], m: i32, aux_arr: &[i32], n: i32) -> &'a mut [i32] {
    let mut mm = m - 1;
    let mut nn = n - 1;
    let mut loops = m + n - 1;

    while loops >= 0 {
        let mm_idx = mm as usize;
        let nn_idx = nn as usize;
        let idx = loops as usize;
        let mut bigger: &i32 = &main_arr[mm_idx];

        if mm >= 0 && nn >= 0 {
            if main_arr[mm_idx] > aux_arr[nn_idx] {
                mm -= 1;
            } else {
                bigger = &aux_arr[nn_idx];
                nn -= 1;
            }
        } else if nn >= 0 {
            bigger = &aux_arr[nn_idx];
            nn -= 1;
        } else if mm >= 0 {
            mm -= 1;
        }
        main_arr[idx] = *bigger;
        loops -= 1;
    }

    println!("{:?}", main_arr);

    main_arr
}
