use std::collections::HashMap;

pub struct Collatz(HashMap<u64, usize>);

impl Collatz {
    pub fn new() -> Self {
        Self(HashMap::from([(1, 1)]))
    }

    pub fn seq_len(&mut self, n: u64) -> usize {
        let mut target = n;
        let mut seen = Vec::new();

        loop {
            if let Some(memo) = self.0.get(&target).copied() {
                let mut seen_count = memo;

                while let Some(i) = seen.pop() {
                    seen_count += 1;
                    self.0.insert(i, seen_count);
                }

                return seen_count;
            }

            seen.push(target);

            target = if target % 2 == 0 {
                target / 2
            } else {
                3 * target + 1
            };
        }
    }
}

impl Default for Collatz {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Collatz::new().seq_len(13), 10);
    }
}
