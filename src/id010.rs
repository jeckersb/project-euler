use crate::prime::PrimeIter;

pub fn solve(limit: u64) -> u64 {
    PrimeIter::new().take_while(|n| *n < limit).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(10), 17);
    }

    #[test]
    fn solution() {
        assert_eq!(solve(2_000_000), 142913828922);
    }
}
