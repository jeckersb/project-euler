use num_bigint::ToBigUint;

pub fn solve(exp: usize) -> u64 {
    let mut n = 2_u8.to_biguint().unwrap();
    let two = n.clone();

    for _ in 1..exp {
        n *= &two;
    }

    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(15), 26);
    }

    #[test]
    fn solution() {
        assert_eq!(solve(1000), 1366);
    }
}
