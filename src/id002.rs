#[derive(Debug, Default)]
pub struct Fib {
    prev: Option<usize>,
    two_ago: Option<usize>,
}

impl Fib {
    pub fn new() -> Self {
        Self {
            prev: None,
            two_ago: None,
        }
    }
}

impl Iterator for Fib {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self {
                prev: None,
                two_ago: None,
            } => self.prev = Some(1),
            Self {
                prev: Some(1),
                two_ago: None,
            } => {
                self.two_ago = Some(1);
                self.prev = Some(2);
            }
            Self {
                prev: Some(p),
                two_ago: Some(other),
            } => {
                let tmp = *other;
                self.two_ago = Some(*p);
                self.prev = Some(*p + tmp);
            }
            _ => panic!("bad fibonacci state {self:?}"),
        }

        self.prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            Fib::new().take(10).collect::<Vec<_>>(),
            vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89]
        );
    }

    #[test]
    fn solution() {
        assert_eq!(
            Fib::new()
                .take_while(|i| i <= &4_000_000)
                .filter(|i| i % 2 == 0)
                .sum::<usize>(),
            4613732
        );
    }
}
