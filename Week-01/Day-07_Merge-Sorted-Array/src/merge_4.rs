pub fn merge_v4<'a>(m_arr: &'a mut [i32], m: usize, n_arr: &mut [i32], n: usize) -> &'a mut [i32] {
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
