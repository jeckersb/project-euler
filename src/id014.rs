use crate::collatz::Collatz;

pub fn solve() -> u64 {
    let mut collatz = Collatz::new();
    let mut longest = usize::MIN;
    let mut leader = 0;

    for n in 1..1_000_000 {
        let len = collatz.seq_len(n);
        if len > longest {
            leader = n;
            longest = len;
        }
    }

    leader
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {}

    #[test]
    fn solution() {
        assert_eq!(solve(), 837799);
    }
}
