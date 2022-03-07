fn sum_of_squares(n: u64) -> u64 {
    (1..=n).map(|i| i * i).sum()
}

fn square_of_sums(n: u64) -> u64 {
    let sum: u64 = (1..=n).sum();
    sum * sum
}

pub fn difference(n: u64) -> u64 {
    square_of_sums(n) - sum_of_squares(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(sum_of_squares(10), 385);
        assert_eq!(square_of_sums(10), 3025);
        assert_eq!(difference(10), 2640);
    }

    #[test]
    fn solution() {
        assert_eq!(difference(100), 25164150);
    }
}
