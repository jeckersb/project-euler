#[derive(Default)]
pub struct PrimeIter(u64);

impl PrimeIter {
    pub fn new() -> Self {
        Self(0)
    }
}

impl Iterator for PrimeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let item = ((self.0 + 1)..).find(|n| is_prime(*n)).unwrap();
        self.0 = item;
        Some(item)
    }
}

pub fn prime_factors(n: u64) -> Vec<u64> {
    (2..=largest_candidate(n))
        .filter(|i| (n % *i == 0) && is_prime(*i))
        .collect()
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    (2..=largest_candidate(n)).all(|i| n % i != 0)
}

fn largest_candidate(n: u64) -> u64 {
    ((n as f64).sqrt() as u64) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(
            (2..=11).map(|i| (i, is_prime(i))).collect::<Vec<_>>(),
            vec![
                (2, true),
                (3, true),
                (4, false),
                (5, true),
                (6, false),
                (7, true),
                (8, false),
                (9, false),
                (10, false),
                (11, true)
            ]
        );
    }

    #[test]
    fn test_prime_iter() {
        let mut primes = PrimeIter::new();

        assert_eq!(primes.next(), Some(2));
        assert_eq!(primes.next(), Some(3));
        assert_eq!(primes.next(), Some(5));
        assert_eq!(primes.next(), Some(7));
        assert_eq!(primes.next(), Some(11));
        assert_eq!(primes.next(), Some(13));
    }
}
