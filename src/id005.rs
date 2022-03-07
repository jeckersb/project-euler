pub fn solve(start: u64, end: u64) -> u64 {
    (1..).find(|n| (start..=end).all(|d| n % d == 0)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(1, 10), 2520);
    }

    #[test]
    fn solution() {
        assert_eq!(solve(1, 20), 232792560);
    }
}
