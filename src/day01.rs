use std::vec::Vec;

pub use crate::expenses::{example_input, input};
pub use crate::expenses::sums_to;

pub fn pt1(input: Vec<i32>) -> i32 {
    sums_to(2020, 2, &input).unwrap().iter().fold(1, |a, b| a * b)
}

pub fn pt2(input: Vec<i32>) -> i32 {
    sums_to(2020, 3, &input).unwrap().iter().fold(1, |a, b| a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_example() {
        assert_eq!(pt1(example_input()), 1721 * 299);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(pt1(input()), 1014171);
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(pt2(example_input()), 241861950);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(pt2(input()), 46584630);
    }
}
