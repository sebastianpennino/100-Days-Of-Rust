fn progress_days(miles_record: &[i32]) -> i32 {
    // The minus one is a hacky way for me to skip the calculation on the first iteration
    let days: [i32; 2] = miles_record.iter().fold([0, -1], |acc, &current_value| {
        let current_total = acc[0];
        let previous_value = acc[1];

        if previous_value != -1 && previous_value < current_value {
            return [current_total + 1, current_value];
        }
        return [current_total, current_value];
    });

    return days[0];
}

fn main() {
    progress_days(&[3, 4, 1, 2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_1() {
        assert_eq!(progress_days(&[3, 4, 1, 2]), 2);
    }

    #[test]
    fn progress_2() {
        assert_eq!(progress_days(&[10, 11, 12, 9, 10]), 3);
    }

    #[test]
    fn progress_3() {
        assert_eq!(progress_days(&[6, 5, 4, 3, 2, 9]), 1);
    }

    #[test]
    fn progress_4() {
        assert_eq!(progress_days(&[9, 9]), 0);
    }

    #[test]
    fn progress_5() {
        assert_eq!(progress_days(&[9, 4, 4, 3, 2, 1]), 0);
    }
}
