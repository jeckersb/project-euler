fn letter_count(n: usize) -> usize {
    assert!((0..=1000).contains(&n));
    match n {
        0 => 0,
        1 | 2 | 6 | 10 => 3,
        4 | 5 | 9 => 4,
        3 | 7 | 8 => 5,
        11 | 12 => 6,
        15 | 16 => 7,
        13 | 14 | 18 | 19 => 8,
        17 => 9,
        20..=99 => two_digit(n),
        100..=999 => three_digit(n),
        1000 => 11,
        _ => unreachable!(),
    }
}

fn two_digit(n: usize) -> usize {
    assert!((20..=99).contains(&n));

    let prefix = match n {
        40..=69 => 5,
        20..=39 | 80..=99 => 6,
        70..=79 => 7,
        _ => unreachable!(),
    };

    prefix + letter_count(n - ((n / 10) * 10))
}

fn three_digit(n: usize) -> usize {
    let hundreds = n / 100;

    let hundreds_letters = if n % 100 == 0 {
        7 // "hundred"
    } else {
        10 // "hundred and"
    } + letter_count(hundreds);

    let rest = letter_count(n - (hundreds * 100));

    hundreds_letters + rest
}

pub fn solve(limit: usize) -> usize {
    (1..=limit).map(letter_count).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(5), 19);

        assert_eq!(letter_count(342), 23);
        assert_eq!(letter_count(115), 20);
    }

    #[test]
    fn solution() {
        assert_eq!(solve(1000), 21124);
    }
}
