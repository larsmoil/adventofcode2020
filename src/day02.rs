pub use crate::passwords::{Policy, PolicyType, input};

pub fn pt1(input: std::vec::Vec<String>) -> i32 {
    input
        .iter()
        .map(|policy_and_password| Policy::new(policy_and_password, PolicyType::MinMaxOccurrences).is_valid())
        .map(|valid| if valid { 1 } else { 0 })
        .fold(0, |a, b| a + b)
}

pub fn pt2(input: std::vec::Vec<String>) -> i32 {
    input
        .iter()
        .map(|policy_and_password| Policy::new(policy_and_password, PolicyType::ExactlyOnePositionMatches).is_valid())
        .map(|valid| if valid { 1 } else { 0 })
        .fold(0, |a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(pt1(input()), 515);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(pt2(input()), 711);
    }
}
