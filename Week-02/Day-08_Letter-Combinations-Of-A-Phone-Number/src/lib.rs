use std::collections::HashMap;

// generates a hashmap with the rules from the picture
pub fn generate_key_pad() -> HashMap<u8, Vec<char>> {
    let mut phone_keypad: HashMap<u8, Vec<char>> = HashMap::new();

    phone_keypad.insert(2, vec!['a', 'b', 'c']);
    phone_keypad.insert(3, vec!['d', 'e', 'f']);
    phone_keypad.insert(4, vec!['g', 'h', 'i']);
    phone_keypad.insert(5, vec!['j', 'k', 'l']);
    phone_keypad.insert(6, vec!['m', 'n', 'o']);
    phone_keypad.insert(7, vec!['p', 'q', 'r', 's']);
    phone_keypad.insert(8, vec!['t', 'u', 'v']);
    phone_keypad.insert(9, vec!['w', 'x', 'y', 'z']);

    return phone_keypad;
}

// ie "123" -> [1, 2, 3]
pub fn str_to_digits(str: &str) -> Vec<u8> {
    let digits: Vec<u8> = str
        .chars() // Get all the individual chars in the string
        .filter_map(|c| {
            return c.to_digit(10).map(|d| d as u8);
        }) // cast to u8
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
pub fn generate_permutations(vec_of_letters: Vec<&Vec<char>>) -> Vec<String> {
    let mut result = vec![String::from("")]; // Start with a vector containing an empty string

    for group in vec_of_letters {
        let mut new_result: Vec<String> = Vec::new();
        for prefix in &result {
            for letter in group {
                let new_word = format!("{}{}", prefix, letter);
                new_result.push(new_word);
            }
        }
        result = new_result;
    }

    return result;
}

pub fn vec_to_letters(digits_vec: Vec<u8>) -> Vec<String> {
    let keypad = generate_key_pad();

    let mut found_letters: Vec<&Vec<char>> = [].to_vec();

    for digit in digits_vec {
        if keypad.contains_key(&digit) {
            let unwrap: &Vec<char> = keypad.get(&digit).unwrap();
            found_letters.push(unwrap);
        }
    }

    let rst = generate_permutations(found_letters);

    return rst;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number_seven() {
        let digits_str = "7";
        let expected_vec = vec![7];
        let digits_vec = str_to_digits(digits_str);
        assert_eq!(digits_vec, expected_vec);
        let permutations = vec_to_letters(digits_vec);
        let expected_permutations: Vec<String> = vec!["p".to_string(), "q".to_string(), "r".to_string(), "s".to_string()];
        assert_eq!(permutations, expected_permutations);
    }

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
