// Given an integer, create a function that returns the next prime. If the number is prime, return the number itself.
fn next_prime_under_1k_hardcoded(num: u32) -> u32 {
    let known_primes_table: [u32; 169] = [
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

    if known_primes_table.contains(&num) {
        return num;
    } else {
        let found = known_primes_table.iter().find(|&&x| x > num);
        match found {
            Some(&prime) => prime,
            None => 1009, // return 1009 if no larger prime is found
        }
    }
}

// 6 to 4294967295
fn is_prime(n: u32) -> bool {
    let mut is_prime = true;
    let mut count = n / 2;

    if n % 6 == 0 || n % 5 == 0 || n % 4 == 0 || n % 3 == 0 || n % 2 == 0 {
        return false; // fast exits to avoid the while
    }
    println!("input: {}", n);

    while count > 6 {
        if n % count == 0 {
            println!("divisble: {} % {}", n, count);
            is_prime = false;
            break;
        }
        count -= 1;
    }

    println!("output: {} is a prime", n);
    return is_prime;
}

// 0 to 2147483646
fn next_prime(n: u32) -> u32 {
    match n {
        _ if n <= 6 => {
            let rst_table: [u32; 7] = [2, 2, 2, 3, 5, 5, 7];
            return rst_table[n as usize];
        },
        _ if n >= 2147483646 => {
            return 0; // TODO: I think should return an error
        },
        _ => {
            // Using Bertrand's postulate
            // https://en.wikipedia.org/wiki/Bertrand%27s_postulate
            // for any integer greater than 3, always exists at least one prime number p with n < p < 2n − 2
            let max = 2 * n - 2;
            let mut count = n;
            while count <= max {
                let is_prime = is_prime(count);
                if is_prime {
                    break;
                }
                count += 1;
            }
            return count;
        }
    };
}

fn main() {
    let some_primes: [u32; 166] = [
        7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101,
        103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
        197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293,
        307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409,
        419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521,
        523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641,
        643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757,
        761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881,
        883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009,
    ];

    let r1 = some_primes.iter().all(|p| return is_prime(*p));
    let r2 = some_primes
        .iter()
        .all(|p| return next_prime_under_1k_hardcoded(*p) == *p);
    let r3 = some_primes
        .iter()
        .all(|p| return next_prime(*p) == *p);

    println!("result: {}, result: {}, result: {}", r1, r2, r3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_1() {
        let input = 223;
        assert_eq!(next_prime_under_1k_hardcoded(input), 223);
        assert_eq!(next_prime(input), 223);
    }

    #[test]
    fn prime_2() {
        let input = 786;
        assert_eq!(next_prime_under_1k_hardcoded(input), 787);
        assert_eq!(next_prime(input), 787);
    }

    #[test]
    fn prime_3() {
        let input = 684;
        assert_eq!(next_prime_under_1k_hardcoded(input), 691);
        assert_eq!(next_prime(input), 691);
    }

    #[test]
    fn prime_4() {
        let input = 1;
        assert_eq!(next_prime_under_1k_hardcoded(input), 2);
        assert_eq!(next_prime(input), 2);
    }

    #[test]
    fn prime_5() {
        let input = 2147483646;
        assert_eq!(next_prime(input), 2147483647);
    }
}
