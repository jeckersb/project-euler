pub fn largest_palindrome(start: u64, end: u64) -> u64 {
    let mut max = u64::MIN;

    for i in start..=end {
        for j in i..=end {
            let product = i * j;

            if is_palindrome(product) {
                max = max.max(product);
            }
        }
    }

    max
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes();

    bytes.iter().eq(bytes.iter().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(largest_palindrome(10, 99), 9009);
    }

    #[test]
    fn solution() {
        assert_eq!(largest_palindrome(100, 999), 906609);
    }
}
