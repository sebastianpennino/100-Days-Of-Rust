pub fn merge_v2<'a>(main_array: &'a mut [i32], m: usize, aux_array: &[i32], n: usize) -> &'a mut [i32] {
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
