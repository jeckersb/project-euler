pub fn solve(sum: u64) -> u64 {
    for a in 1..(sum / 3) {
        for b in (a + 1)..=((sum - a) / 2) {
            let c = sum - a - b;

            if a >= b || b >= c {
                continue;
            }

            if (a * a) + (b * b) == (c * c) {
                return a * b * c;
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(12), 3 * 4 * 5);
    }

    #[test]
    fn solution() {
        assert_eq!(solve(1000), 31875000);
    }
}
