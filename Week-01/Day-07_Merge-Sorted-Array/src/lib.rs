// Declare the modules
pub mod merge_1;
pub mod merge_2;
pub mod merge_3;
pub mod merge_4;

pub use merge_1::merge_v1;
pub use merge_2::merge_v2;
pub use merge_3::merge_v3;
pub use merge_4::merge_v4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn merge_1() {
        let mut num1_a = [1, 2, 3, 0, 0, 0];
        let mut num1_b = [1, 2, 3, 0, 0, 0];
        let mut num1_c = [1, 2, 3, 0, 0, 0];
        let mut num1_d = vec![1, 2, 3, 0, 0, 0];
        let num2_a = [2, 5, 6];
        let num2_b = vec![2, 5, 6];
        assert_eq!(merge_v1(&mut num1_a, &num2_a), [1, 2, 2, 3, 5, 6]);
        assert_eq!(merge_v2(&mut num1_b, &num2_a, 3), [1, 2, 2, 3, 5, 6]);
        assert_eq!(merge_v3(&mut num1_c, 3, &num2_a, 3), [1, 2, 2, 3, 5, 6]);

        let merged_result = merge_v4(&mut num1_d, 3, &num2_b, 3);
        assert_eq!(*merged_result, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    pub fn merge_2() {
        let mut num1_a = [-10, -6, 5, 0, 0, 0, 0];
        let mut num1_b = [-10, -6, 5, 0, 0, 0, 0];
        let mut num1_c = [-10, -6, 5, 0, 0, 0, 0];
        // let mut num1_d = vec![-10, -6, 5, 0, 0, 0, 0];
        let num2_a = [-22, 12, 55, 66];
        // let num2_b = vec![-22, 12, 55, 66];
        assert_eq!(merge_v1(&mut num1_a, &num2_a), [-22, -10, -6, 5, 12, 55, 66]);
        assert_eq!(merge_v2(&mut num1_b, &num2_a, 4), [-22, -10, -6, 5, 12, 55, 66]);
        assert_eq!(merge_v3(&mut num1_c, 3, &num2_a, 4), [-22, -10, -6, 5, 12, 55, 66]);

        // let merged_result = merge_v4(&mut num1_d, 3, &num2_b, 4);
        // assert_eq!(*merged_result, [-22, -10, -6, 5, 12, 55, 66]);
    }

    #[test]
    pub fn merge_3() {
        let mut num1_a = [-1, 4, 19, 0, 0, 0];
        let mut num1_b = [-1, 4, 19, 0, 0, 0];
        let mut num1_c = [-1, 4, 19, 0, 0, 0];
        // let mut num1_d = vec![-1, 4, 19, 0, 0, 0];
        let num2_a = [-221, 0, 636];
        // let num2_b = vec![-221, 0, 636];
        assert_eq!(merge_v1(&mut num1_a, &num2_a), [-221, -1, 0, 4, 19, 636]);
        assert_eq!(merge_v2(&mut num1_b, &num2_a, 3), [-221, -1, 0, 4, 19, 636]);
        assert_eq!(merge_v3(&mut num1_c, 3, &num2_a, 3), [-221, -1, 0, 4, 19, 636]);

        // let merged_result = merge_v4(&mut num1_d, 3, &num2_b, 3);
        // assert_eq!(*merged_result, [-221, -1, 0, 4, 19, 636]);
    }

    #[test]
    pub fn merge_4() {
        let mut num1_a = [0, 0];
        let mut num1_b = [0, 0];
        let mut num1_c = [0, 0];
        // let mut num1_d = vec![0, 0];
        let num2_a = [-666];
        // let num2_b = vec![-666];
        assert_eq!(merge_v1(&mut num1_a, &num2_a), [-666, 0]);
        assert_eq!(merge_v2(&mut num1_b, &num2_a, 1), [-666, 0]);
        assert_eq!(merge_v3(&mut num1_c, 1, &num2_a, 1), [-666, 0]);

        // let merged_result = merge_v4(&mut num1_d, 1, &num2_b, 1);
        // assert_eq!(*merged_result, [-666, 0]);
    }

    #[test]
    pub fn merge_5() {
        let mut num1_a = [1, 2, 4, 4, 0, 0, 0, 0, 0, 0];
        let mut num1_b = [1, 2, 4, 4, 0, 0, 0, 0, 0, 0];
        let mut num1_c = [1, 2, 4, 4, 0, 0, 0, 0, 0, 0];
        let mut num1_d = vec![1, 2, 4, 4, 0, 0, 0, 0, 0, 0];
        let num2_a = [1, 2, 2, 3, 3, 4];
        let num2_b = vec![1, 2, 2, 3, 3, 4];
        assert_eq!(merge_v1(&mut num1_a, &num2_a), [1, 1, 2, 2, 2, 3, 3, 4, 4, 4]);
        assert_eq!(merge_v2(&mut num1_b, &num2_a, 6), [1, 1, 2, 2, 2, 3, 3, 4, 4, 4]);
        assert_eq!(merge_v3(&mut num1_c, 4, &num2_a, 6), [1, 1, 2, 2, 2, 3, 3, 4, 4, 4]);

        let merged_result = merge_v4(&mut num1_d, 4, &num2_b, 6);
        assert_eq!(*merged_result, [1, 1, 2, 2, 2, 3, 3, 4, 4, 4]);
    }
}
