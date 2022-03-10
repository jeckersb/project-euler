pub fn solve(n: usize) -> usize {
    (1..=n).fold(1, |acc, i| acc * (n + i) / i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(2), 6);
    }

    #[test]
    fn solution() {
        assert_eq!(solve(20), 137846528820);
    }
}
