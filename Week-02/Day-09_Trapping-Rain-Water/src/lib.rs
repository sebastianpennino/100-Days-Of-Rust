// Declare the modules
pub mod water_v1;
pub mod water_v2;
pub mod common_stuff;

// Re-export functions for easier access
pub use water_v1::trap_water_v1;
pub use water_v2::trap_water_v2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_1() {
        let height_map_v1: Vec<i16> = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1, 0];
        let height_map_v2: Vec<i16> = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1, 0];
        let output_v1 = trap_water_v1(height_map_v1);
        let output_v2 = trap_water_v2(height_map_v2);
        assert_eq!(output_v1, 6);
        assert_eq!(output_v2, 6);
    }

    #[test]
    fn water_2() {
        let height_map_v1: Vec<i16> = vec![4, 2, 0, 3, 2, 5];
        let height_map_v2: Vec<i16> = vec![4, 2, 0, 3, 2, 5];
        let output_v1 = trap_water_v1(height_map_v1);
        let output_v2 = trap_water_v2(height_map_v2);
        assert_eq!(output_v1, 9);
        assert_eq!(output_v2, 9);
    }

    #[test]
    fn water_3() {
        let height_map_v1: Vec<i16> = vec![0, 0, 4, 0, 0, 5, 0, 5];
        let height_map_v2: Vec<i16> = vec![0, 0, 4, 0, 0, 5, 0, 5];
        let output_v1 = trap_water_v1(height_map_v1);
        let output_v2 = trap_water_v2(height_map_v2);
        assert_eq!(output_v1, 13);
        assert_eq!(output_v2, 13);
    }

    #[test]
    fn water_4() {
        let height_map_v1: Vec<i16> = vec![0, 0, 4, 0, 0, 5, 0, 1, 0];
        let height_map_v2: Vec<i16> = vec![0, 0, 4, 0, 0, 5, 0, 1, 0];
        let output_v1 = trap_water_v1(height_map_v1);
        let output_v2 = trap_water_v2(height_map_v2);
        assert_eq!(output_v1, 9);
        assert_eq!(output_v2, 9);
    }

    #[test]
    fn water_5() {
        let height_map_v1: Vec<i16> = vec![0, 4, 3, 0, 1, 0, 3, 0, 1, 0];
        let height_map_v2: Vec<i16> = vec![0, 4, 3, 0, 1, 0, 3, 0, 1, 0];
        let output_v1 = trap_water_v1(height_map_v1);
        let output_v2 = trap_water_v2(height_map_v2);
        assert_eq!(output_v1, 9);
        assert_eq!(output_v2, 9);
    }

    #[test]
    fn water_6() {
        let height_map_v1: Vec<i16> = vec![0, 4, 3, 0, 1, 0, 2, 0, 1, 0];
        let height_map_v2: Vec<i16> = vec![0, 4, 3, 0, 1, 0, 2, 0, 1, 0];
        let output_v1 = trap_water_v1(height_map_v1);
        let output_v2 = trap_water_v2(height_map_v2);
        assert_eq!(output_v1, 6);
        assert_eq!(output_v2, 6);
    }
}
