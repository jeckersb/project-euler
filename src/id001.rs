pub fn solve(limit: usize) -> usize {
    (3..limit).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(10), 23);
    }

    #[test]
    fn solution() {
        assert_eq!(solve(1000), 233168);
    }
}
