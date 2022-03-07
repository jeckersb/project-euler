pub use crate::prime::prime_factors;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(prime_factors(13195), vec![5, 7, 13, 29]);
    }

    #[test]
    fn solution() {
        assert_eq!(
            prime_factors(600851475143).into_iter().last().unwrap(),
            6857
        );
    }
}
