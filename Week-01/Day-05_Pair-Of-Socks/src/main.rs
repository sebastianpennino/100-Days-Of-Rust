use std::collections::HashMap;

fn sock_pairs_hash(all_socks: &str) -> i32 {
    let socks_chars = all_socks.chars();
    let mut sorted_socks: HashMap<char, i32> = HashMap::new();

    for sock in socks_chars {
        if sorted_socks.contains_key(&sock) {
            let val = sorted_socks.get(&sock).unwrap();
            sorted_socks.insert(sock, val + 1);
        } else {
            sorted_socks.insert(sock, 1);
        }
    }

    let total_values: Vec<i32> = sorted_socks.values().cloned().collect();
    let total_pairs: i32 = total_values.iter().map(|x| x/2).sum();

    return total_pairs;
}

struct MyAccumulator {
    pairs: i32,
    streak: i32,
    previous: Option<char>,
}

fn sock_pairs_fold(all_socks: &str) -> i32 {
    let mut socks_letters: Vec<char> = all_socks.trim().chars().collect();
    // Sort mutates the variable
    socks_letters.sort();
    
    let total_pairs = socks_letters.iter().fold(
        MyAccumulator {
            pairs: 0,
            streak: 0,
            previous: None,
        },
        |mut acc: MyAccumulator, &current: &char| {
            match acc.previous {
                Some(prev) => {
                    if prev == current {
                        acc.streak += 1;
                    } else {
                        acc.pairs += acc.streak / 2;
                        acc.streak = 1;
                    }
                },
                None => {
                    acc.streak = 1;
                },
            }
            acc.previous = Some(current);
            return acc
        },
    );

    // Add remaining pairs from the last streak
    return total_pairs.pairs + total_pairs.streak / 2
}

fn sock_pairs_for_each(all_socks: &str) -> i32 {
    let mut socks_letters: Vec<char> = all_socks.trim().chars().collect();
    // Sort mutates the variable
    socks_letters.sort();
    let sorted = socks_letters;

    let mut streak = 0;
    let mut pairs = 0;
    let mut previous = ' ';

    sorted.iter().for_each(|&c| {
        if c == previous {
            streak += 1;
        } else {
            pairs += streak/2;
            streak = 1;
        }
        previous = c;
    });

    return pairs + streak / 2
}

fn sock_pairs_while(socks: &str) -> usize {
    let mut socks_vec: Vec<char> = socks.chars().collect();
    socks_vec.sort();

    let mut pairs = 0;
    let mut i = 0;

    while i < socks_vec.len() - 1 {
        // Comparing current and next sock
        if socks_vec[i] == socks_vec[i + 1] {
            pairs += 1;
            i += 2; // Skip the next sock as it forms a pair with the current sock
        } else {
            i += 1; // Move to the next entry
        }
    }

    pairs
}

// Notes:
// Helpful video about iterators https://www.youtube.com/watch?v=Zcg6wmqdbzc
// Started debug using CodeLLDB for VSCode (https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
// I had trouble debugging until I found this on stack overflow
// Source: https://stackoverflow.com/questions/77218022/why-is-my-debugger-in-vscode-not-working-with-rust-after-mac-update-to-sonoma-14
// Basically renaming ~/.vscode/extensions/vadimcn.vscode-lldb-1.10.0/lldb/bin/debugserver and restarted VS-Code worked for me.
fn main() {
    println!("{}", sock_pairs_hash("AAABCADBBC"));
    println!("{}", sock_pairs_fold("AAABCADBBC"));
    println!("{}", sock_pairs_for_each("AAABCADBBC"));
    println!("{}", sock_pairs_while("AAABCADBBC"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socks_1() {
        let input = "AA";
        assert_eq!(sock_pairs_hash(input), 1);
        assert_eq!(sock_pairs_fold(input), 1);
        assert_eq!(sock_pairs_for_each(input), 1);
        assert_eq!(sock_pairs_while(input), 1);
    }

    #[test]
    fn socks_2() {
        let input = "ABABC";
        assert_eq!(sock_pairs_hash(input), 2);
        assert_eq!(sock_pairs_fold(input), 2);
        assert_eq!(sock_pairs_for_each(input), 2);
        assert_eq!(sock_pairs_while(input), 2);
    }

    #[test]
    fn socks_3() {
        let input = "CABBACCC";
        assert_eq!(sock_pairs_hash(input), 4);
        assert_eq!(sock_pairs_fold(input), 4);
        assert_eq!(sock_pairs_for_each(input), 4);
        assert_eq!(sock_pairs_while(input), 4);
    }

    #[test]
    fn socks_4() {
        let input = "ABC";
        assert_eq!(sock_pairs_hash(input), 0);
        assert_eq!(sock_pairs_fold(input), 0);
        assert_eq!(sock_pairs_for_each(input), 0);
        assert_eq!(sock_pairs_while(input), 0);
    }
}
