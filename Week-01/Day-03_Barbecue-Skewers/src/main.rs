fn is_vegan(s: &str) -> bool {
    s.chars().all(|c| c == 'o' || c == '-')
}

fn check_grill(strings: &[&str]) -> [i32; 2] {
    let result = strings.iter().fold([0, 0], |mut acc, &row| {
        if is_vegan(row) {
            acc[0] += 1;
        } else {
            acc[1] += 1;
        }
        acc
    });

    return result;
}

fn main() {
    check_grill(&["--o-o-o--", "--o-o-o--"]);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn barbacue_1() {
        assert_eq!(
            check_grill(&[
                "--oooo-ooo--",
                "--xx--x--xx--",
                "--o---o--oo--",
                "--xx--x--ox--",
                "--xx--x--ox--"
            ]),
            [2, 3]
        );
    }

    #[test]
    fn barbacue_2() {
        assert_eq!(
            check_grill(&[
                "--oooo-ooo--",
                "--xxxxxxxx--",
                "--o---",
                "-o-----o---x--",
                "--o---o-----"
            ]),
            [3, 2]
        )
    }
    #[test]
    fn barbacue_3() {
        assert_eq!(
            check_grill(&[
                "--xo--x--ox--",
                "--xx--x--xx--",
                "--oo--o--oo--",
                "--xx--x--ox--",
                "--xx--x--ox--",
            ]),
            [1, 4]
        )
    }
}
