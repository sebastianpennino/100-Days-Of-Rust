fn merge_v1(arr1: &mut [i32], arr2: &[i32]) -> Vec<i32> {
    // TODO: actually return the num1? fn merge_into_one<'a>(arr1: &'a mut [i32], arr2: &[i32]) -> &'a mut [i32] {
    // TODO: safeguard if arr1 len less or equal than arr2 len return error!

    let mut temp: Vec<i32> = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;
    let mut loops = 0;

    while loops < arr1.len() {
        // num1 is special because 0 are actually "empty" spaces to fill
        if arr1[i] > arr2[j] || arr1[i] == 0 {
            temp.push(arr2[j]);
            j += 1;
        } else {
            temp.push(arr1[i]);
            i += 1;
        }
        loops += 1;
    }

    println!("{:?}", temp);

    return temp;
}

fn merge_v2<'a>(main_array: &'a mut [i32], m: usize, aux_array: &[i32], n: usize) -> &'a mut [i32] {
    let mut i = m - 1;
    let mut j = n - 1;
    let mut pos = m + n;

    while pos > 0 {
        println!("loop {}", pos);
        let mut bigger: &i32 = &main_array[i];
        // println!("comparing main {} vs aux {} ", main_array[i], aux_array[j]);
        println!("comparing {} <= {}", main_array[i], aux_array[j]);
        if main_array[i] <= aux_array[j] {
            bigger = &aux_array[j];
            println!("bigger is aux > {}", bigger);
            if j > 0 {
                j -= 1;
            }
        } else if i > 0 {
            println!("bigger is main < {}", bigger);
            i -= 1;
        } else {
            println!("else, bigger is main {}", bigger);
        }

        main_array[pos - 1] = *bigger;
        pos -= 1;
    }

    println!("{:?}", main_array);

    return main_array;
}

fn merge_v3<'a>(main_array: &'a mut [i32], m: usize, aux_array: &[i32], n: usize) -> &'a mut [i32] {
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

fn merge_v3b<'a>(
    main_array: &'a mut [i32],
    m: usize,
    aux_array: &[i32],
    n: usize,
) -> &'a mut [i32] {
    let mut i = m as isize - 1;
    let mut j = n as isize - 1;
    let mut pos = (m + n) as isize - 1;
    println!("init, i={} j={} pos={}", i, j, pos);

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

// still broken
fn merge_v3c<'a>(main_arr: &'a mut [i32], m: i32, aux_arr: &[i32], n: i32) -> &'a mut [i32] {
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

fn merge_v4<'a>(m_arr: &'a mut [i32], m: usize, n_arr: &mut [i32], n: usize) -> &'a mut [i32] {
    let mut m_idx = 0;
    let mut n_idx = 0;
    let mut count = 0;

    m_arr.reverse();
    n_arr.reverse();

    while count < m + n {
        let max: &i32;
        let compare = [m_arr[m_idx], n_arr[n_idx]];

        match compare[0].cmp(&compare[1]) {
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => {
                max = &compare[0];
                m_idx = m_idx + 1;
            }
            std::cmp::Ordering::Less => {
                max = &compare[1];
                n_idx = n_idx + 1;
            }
        }
        m_arr[count] = *max;
        count += 1;
    }

    m_arr.reverse();

    println!("{:?}", m_arr);

    return m_arr;
}


fn main() {
    let mut num1 = [1, 2, 3, 0, 0, 0];
    let num2 = [2, 5, 6];
    let mut mut_num2 = [2, 5, 6];
    merge_v1(&mut num1, &num2);
    merge_v2(&mut num1, 3, &num2, 3);
    merge_v3(&mut num1, 3, &num2, 3);
    merge_v3b(&mut num1, 3, &num2, 3);
    merge_v3c(&mut num1, 3, &num2, 3);
    merge_v4(&mut num1, 3, &mut mut_num2, 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_1() {
        let mut num1 = [1, 2, 3, 0, 0, 0];
        let num2 = [2, 5, 6];
        assert_eq!(merge_v3b(&mut num1, 3, &num2, 3), [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn merge_2() {
        let mut num1 = [-10, -6, 5, 0, 0, 0, 0];
        let num2 = [-22, 12, 55, 66];
        assert_eq!(
            merge_v3b(&mut num1, 3, &num2, 4),
            [-22, -10, -6, 5, 12, 55, 66]
        );
    }

    #[test]
    fn merge_3() {
        let mut num1 = [-1, 4, 19, 0, 0, 0];
        let num2 = [-221, 0, 636];
        assert_eq!(merge_v3b(&mut num1, 3, &num2, 3), [-221, -1, 0, 4, 19, 636]);
    }

    #[test]
    fn merge_4() {
        let mut num1 = [0, 0];
        let mut num2 = [-666];
        assert_eq!(merge_v3b(&mut num1, 1, &mut num2, 1), [-666, 0]);
    }
}
