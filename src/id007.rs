pub use crate::prime::PrimeIter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(PrimeIter::new().take(6).last().unwrap(), 13);
    }

    #[test]
    fn solution() {
        assert_eq!(PrimeIter::new().take(10001).last().unwrap(), 104743);
    }
}
