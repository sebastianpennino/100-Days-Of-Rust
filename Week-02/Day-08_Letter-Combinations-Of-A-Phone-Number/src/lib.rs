use std::collections::HashMap;

// generates a hashmap with the rules from the picture
pub fn generate_key_pad() -> HashMap<i32, Vec<String>> {
    let mut phone_keypad: HashMap<i32, Vec<String>> = HashMap::new();

    // phone_keypad.insert(1, vec![]);
    phone_keypad.insert(2, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    phone_keypad.insert(3, vec!["d".to_string(), "e".to_string(), "f".to_string()]);
    phone_keypad.insert(4, vec!["g".to_string(), "h".to_string(), "i".to_string()]);
    phone_keypad.insert(5, vec!["j".to_string(), "k".to_string(), "l".to_string()]);
    phone_keypad.insert(6, vec!["m".to_string(), "n".to_string(), "o".to_string()]);
    phone_keypad.insert(7, vec!["p".to_string(), "q".to_string(), "r".to_string(), "s".to_string()]);
    phone_keypad.insert(8, vec!["t".to_string(), "u".to_string(), "v".to_string()]);
    phone_keypad.insert(9, vec!["w".to_string(), "x".to_string(), "y".to_string(), "z".to_string()]);

    return phone_keypad;
}

// ie "123" -> [1, 2, 3]
pub fn str_to_digits(str: &str) -> Vec<i32> {
    let digits: Vec<i32> = str
        .chars() // Get all the individual chars in the string
        .filter_map(|c| {
            return c.to_digit(10).map(|d| d as i32);
        }) // Cast to i32 (unwrap Option and convert from u32)
        .collect();
    return digits;
}

// TODO: I got this function working by banging my head against a wall.
// I need to deepen my understanding of borrowing and lifetimes, as this was a pain in the a**
//
// I started with a js function in mind, then tried to convert it to Rust
//
// function generatePermutations(selectedKeypadGroups) {
//     let result = [""]; // need a string there so the first for gets a loop
//
//     for (const groupOfLetters of selectedKeypadGroups) {
//         const newResult = [];
//         for (const prefix of result) {
//             for (const character of groupOfLetters) {
//                 /* in the first loop prefix will be "" */
//                 const newWord = prefix + character;
//                 newResult.push(newWord);
//             }
//         }
//         result = newResult;
//     }
//
//     return result;
// }
pub fn generate_permutations(vec_of_letters: Vec<Vec<&str>>) -> Vec<String> {
    let mut result = vec![String::from("")]; // Start with a vector containing an empty string

    for group in vec_of_letters {
        let mut new_result: Vec<String> = Vec::new();
        for prefix in &result {
            for letter in &group {
                let new_word = format!("{}{}", prefix, letter);
                new_result.push(new_word);
            }
        }
        result = new_result;
    }

    return result;
}

pub fn vec_to_letters(digits_vec: Vec<i32>) -> Vec<String> {
    let keypad = generate_key_pad();

    let mut found_letters: Vec<Vec<&str>> = [].to_vec();

    for digit in digits_vec {
        if keypad.contains_key(&digit) {
            let unwrap = keypad.get(&digit).unwrap();
            // TODO: see if we can find a better method (converting Vec<String> to Vec<str>)
            let str: Vec<&str> = unwrap.iter().map(AsRef::as_ref).collect();
            found_letters.push(str);
        }
    }

    let rst = generate_permutations(found_letters);

    return rst;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypad_generation() {
        let keypad = generate_key_pad();

        // Check the number of keys in the keypad
        assert_eq!(keypad.len(), 8);

        // Check specific mappings
        // assert_eq!(keypad.get(&1), Some(&vec![]));
        assert_eq!(keypad.get(&2), Some(&vec!["a".to_string(), "b".to_string(), "c".to_string()]));
        assert_eq!(keypad.get(&3), Some(&vec!["d".to_string(), "e".to_string(), "f".to_string()]));
        assert_eq!(keypad.get(&4), Some(&vec!["g".to_string(), "h".to_string(), "i".to_string()]));
        assert_eq!(keypad.get(&5), Some(&vec!["j".to_string(), "k".to_string(), "l".to_string()]));
        assert_eq!(keypad.get(&6), Some(&vec!["m".to_string(), "n".to_string(), "o".to_string()]));
        assert_eq!(
            keypad.get(&7),
            Some(&vec!["p".to_string(), "q".to_string(), "r".to_string(), "s".to_string()])
        );
        assert_eq!(keypad.get(&8), Some(&vec!["t".to_string(), "u".to_string(), "v".to_string()]));
        assert_eq!(
            keypad.get(&9),
            Some(&vec!["w".to_string(), "x".to_string(), "y".to_string(), "z".to_string()])
        );
    }

    #[test]
    fn single_number_seven() {
        let digits_str = "7";
        let expected_vec = vec![7];
        let digits_vec = str_to_digits(digits_str);
        assert_eq!(digits_vec, expected_vec);
        let permutations = vec_to_letters(digits_vec);
        let expected_permutations = vec!["p", "q", "r", "s"];
        assert_eq!(permutations, expected_permutations);
    }

    // #[test]
    // fn single_number_weird() {
    //     let digits_str = "1";
    //     let expected_vec = vec![1];
    //     let digits_vec = str_to_digits(digits_str);
    //     assert_eq!(digits_vec, expected_vec);
    //     let permutations = vec_to_letters(digits_vec);
    //     let expected_permutations: Vec<&str> = vec![];
    //     assert_eq!(permutations, expected_permutations);
    // }

    #[test]
    fn two_numbers() {
        let digits_str = "23";
        let expected_vec = vec![2, 3];
        let digits_vec = str_to_digits(digits_str);
        assert_eq!(digits_vec, expected_vec);
        let permutations = vec_to_letters(digits_vec);
        let expected_permutations = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        assert_eq!(permutations, expected_permutations);
    }

    #[test]
    fn three_numbers() {
        let digits_str = "476";
        let expected_vec = vec![4, 7, 6];
        let digits_vec = str_to_digits(digits_str);
        assert_eq!(digits_vec, expected_vec);
        let permutations = vec_to_letters(digits_vec);
        let expected_permutations = vec![
            "gpm", "gpn", "gpo", "gqm", "gqn", "gqo", "grm", "grn", "gro", "gsm", "gsn", "gso", // starts with G
            "hpm", "hpn", "hpo", "hqm", "hqn", "hqo", "hrm", "hrn", "hro", "hsm", "hsn", "hso", // starts with H
            "ipm", "ipn", "ipo", "iqm", "iqn", "iqo", "irm", "irn", "iro", "ism", "isn", "iso", // starts with I
        ];
        assert_eq!(permutations, expected_permutations);
    }

    #[test]
    fn empty_case() {
        let digits_str = "";
        let expected_vec = vec![];
        let digits_vec = str_to_digits(digits_str);
        assert_eq!(digits_vec, expected_vec);
        let permutations = vec_to_letters(digits_vec);
        let expected_permutations = vec![""];
        assert_eq!(permutations, expected_permutations);
    }
}
