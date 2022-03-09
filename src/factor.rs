pub fn num_factors(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }

    let mut factors = 2; // [1,n]

    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            if n / i == i {
                //sqrt
                factors += 1;
            } else {
                factors += 2;
            }
        }
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_factors() {
        assert_eq!(num_factors(0), 1);
        assert_eq!(num_factors(1), 1);
        assert_eq!(num_factors(2), 2);
        assert_eq!(num_factors(3), 2);
        assert_eq!(num_factors(6), 4);
        assert_eq!(num_factors(10), 4);
        assert_eq!(num_factors(15), 4);
        assert_eq!(num_factors(21), 4);
        assert_eq!(num_factors(28), 6);
        assert_eq!(num_factors(100), 9); // make sure 10*10 comes out as only one
    }
}
