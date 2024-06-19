// Declare the modules
pub mod merge_1;
pub mod merge_2;
pub mod merge_3;
pub mod merge_4;

pub use merge_1::merge_v1;
pub use merge_2::merge_v2;
pub use merge_3::merge_v3a;
pub use merge_3::merge_v3b;
pub use merge_3::merge_v3c;
pub use merge_4::merge_v4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn merge_1_v1() {
        let mut num1 = [1, 2, 3, 0, 0, 0];
        let num2 = [2, 5, 6];
        assert_eq!(merge_v1(&mut num1, &num2), [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    pub fn merge_1_v2() {
        let mut num1 = [1, 2, 3, 0, 0, 0];
        let num2 = [2, 5, 6];
        assert_eq!(merge_v2(&mut num1, 3, &num2, 3), [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    pub fn merge_1_v3a() {
        let mut num1 = [1, 2, 3, 0, 0, 0];
        let num2 = [2, 5, 6];
        assert_eq!(merge_v3a(&mut num1, 3, &num2, 3), [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    pub fn merge_1_v3b() {
        let mut num1 = [1, 2, 3, 0, 0, 0];
        let num2 = [2, 5, 6];
        assert_eq!(merge_v3b(&mut num1, 3, &num2, 3), [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    pub fn merge_1_v3c() {
        let mut num1 = [1, 2, 3, 0, 0, 0];
        let num2 = [2, 5, 6];
        assert_eq!(merge_v3c(&mut num1, 3, &num2, 3), [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    pub fn merge_1_v4() {
        let mut num1 = [1, 2, 3, 0, 0, 0];
        let mut num2 = [2, 5, 6];
        assert_eq!(merge_v4(&mut num1, 3, &mut num2, 3), [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    pub fn merge_2_v1() {
        let mut num1 = [-10, -6, 5, 0, 0, 0, 0];
        let num2 = [-22, 12, 55, 66];
        assert_eq!(
            merge_v1(&mut num1, &num2),
            [-22, -10, -6, 5, 12, 55, 66]
        );
    }

    #[test]
    pub fn merge_2_v2() {
        let mut num1 = [-10, -6, 5, 0, 0, 0, 0];
        let num2 = [-22, 12, 55, 66];
        assert_eq!(
            merge_v2(&mut num1, 3, &num2, 4),
            [-22, -10, -6, 5, 12, 55, 66]
        );
    }

    #[test]
    pub fn merge_2_v3a() {
        let mut num1 = [-10, -6, 5, 0, 0, 0, 0];
        let num2 = [-22, 12, 55, 66];
        assert_eq!(
            merge_v3a(&mut num1, 3, &num2, 4),
            [-22, -10, -6, 5, 12, 55, 66]
        );
    }

    #[test]
    pub fn merge_2_v3b() {
        let mut num1 = [-10, -6, 5, 0, 0, 0, 0];
        let num2 = [-22, 12, 55, 66];
        assert_eq!(
            merge_v3b(&mut num1, 3, &num2, 4),
            [-22, -10, -6, 5, 12, 55, 66]
        );
    }

    #[test]
    pub fn merge_2_v3c() {
        let mut num1 = [-10, -6, 5, 0, 0, 0, 0];
        let num2 = [-22, 12, 55, 66];
        assert_eq!(
            merge_v3c(&mut num1, 3, &num2, 4),
            [-22, -10, -6, 5, 12, 55, 66]
        );
    }

    #[test]
    pub fn merge_2_v4() {
        let mut num1 = [-10, -6, 5, 0, 0, 0, 0];
        let mut num2 = [-22, 12, 55, 66];
        assert_eq!(
            merge_v4(&mut num1, 3, &mut num2, 4),
            [-22, -10, -6, 5, 12, 55, 66]
        );
    }

    #[test]
    pub fn merge_3_v1() {
        let mut num1 = [-1, 4, 19, 0, 0, 0];
        let num2 = [-221, 0, 636];
        assert_eq!(merge_v1(&mut num1, &num2), [-221, -1, 0, 4, 19, 636]);
    }

    #[test]
    pub fn merge_3_v2() {
        let mut num1 = [-1, 4, 19, 0, 0, 0];
        let num2 = [-221, 0, 636];
        assert_eq!(merge_v2(&mut num1, 3, &num2, 3), [-221, -1, 0, 4, 19, 636]);
    }

    #[test]
    pub fn merge_3_v3a() {
        let mut num1 = [-1, 4, 19, 0, 0, 0];
        let num2 = [-221, 0, 636];
        assert_eq!(merge_v3a(&mut num1, 3, &num2, 3), [-221, -1, 0, 4, 19, 636]);
    }

    #[test]
    pub fn merge_3_v3b() {
        let mut num1 = [-1, 4, 19, 0, 0, 0];
        let num2 = [-221, 0, 636];
        assert_eq!(merge_v3b(&mut num1, 3, &num2, 3), [-221, -1, 0, 4, 19, 636]);
    }


    #[test]
    pub fn merge_3_v3c() {
        let mut num1 = [-1, 4, 19, 0, 0, 0];
        let num2 = [-221, 0, 636];
        assert_eq!(merge_v3c(&mut num1, 3, &num2, 3), [-221, -1, 0, 4, 19, 636]);
    }

    #[test]
    pub fn merge_3_v4() {
        let mut num1 = [-1, 4, 19, 0, 0, 0];
        let mut num2 = [-221, 0, 636];
        assert_eq!(merge_v4(&mut num1, 3, &mut num2, 3), [-221, -1, 0, 4, 19, 636]);
    }

    #[test]
    pub fn merge_4_v1() {
        let mut num1 = [0, 0];
        let num2 = [-666];
        assert_eq!(merge_v1(&mut num1, &num2), [-666, 0]);
    }

    #[test]
    pub fn merge_4_v2() {
        let mut num1 = [0, 0];
        let num2 = [-666];
        assert_eq!(merge_v2(&mut num1, 1, &num2, 1), [-666, 0]);
    }

    #[test]
    pub fn merge_4_v3a() {
        let mut num1 = [0, 0];
        let num2 = [-666];
        assert_eq!(merge_v3a(&mut num1, 1, &num2, 1), [-666, 0]);
    }

    #[test]
    pub fn merge_4_v3b() {
        let mut num1 = [0, 0];
        let num2 = [-666];
        assert_eq!(merge_v3b(&mut num1, 1, &num2, 1), [-666, 0]);
    }

    #[test]
    pub fn merge_4_v3c() {
        let mut num1 = [0, 0];
        let num2 = [-666];
        assert_eq!(merge_v3c(&mut num1, 1, &num2, 1), [-666, 0]);
    }

    #[test]
    pub fn merge_4_v4() {
        let mut num1 = [0, 0];
        let mut num2 = [-666];
        assert_eq!(merge_v4(&mut num1, 1, &mut num2, 1), [-666, 0]);
    }
}
