// Given an integer, create a function that returns the next prime. If the number is prime, return the number itself.
fn next_prime_under_1k_hardcoded(num: u16) -> u16 {
    let known_primes: [u16; 169] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
        631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
        751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
        877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
        1009,
    ];

    if known_primes.contains(&num) {
        return num;
    } else {
        let found = known_primes.iter().find(|&&x| x > num);
        match found {
            Some(&prime) => prime,
            None => 1009, // return 1009 if no larger prime is found
        }
    }
}

fn next_primer_under_1k(num: u16) -> u16 {
    // returning something until I work on the code
    return 1
}

fn main() {
    println!("{}", next_prime_under_1k_hardcoded(33));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_1() {
        let input = 223;
        assert_eq!(next_prime_under_1k_hardcoded(input), 223);
        assert_eq!(next_primer_under_1k(input), 223);
    }

    #[test]
    fn prime_2() {
        let input = 786;
        assert_eq!(next_prime_under_1k_hardcoded(input), 787);
        assert_eq!(next_primer_under_1k(input), 223);
    }

    #[test]
    fn prime_3() {
        let input = 200;
        assert_eq!(next_prime_under_1k_hardcoded(input), 211);
        assert_eq!(next_primer_under_1k(input), 211);
    }
}
