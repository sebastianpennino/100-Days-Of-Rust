fn progress_days(miles_record: &[i32]) -> i32 {
    let mut days: i32 = 0;
    let mut i: usize = 0;
    while i < miles_record.len() {
        if i + 1 == miles_record.len() {
            break;
        }
        if miles_record[i] < miles_record[i + 1] {
            days += 1
        }
        i += 1;
    }
    return days;
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
