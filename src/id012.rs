use crate::factor::num_factors;
use crate::triangle_numbers::TriangleIter;

pub fn solve(over: u64) -> u64 {
    TriangleIter::new()
        .find(|n| num_factors(*n) > over)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(5), 28);
    }

    #[test]
    fn solution() {
        assert_eq!(solve(500), 76576500);
    }
}
