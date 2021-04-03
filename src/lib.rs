use std::cmp::Ordering;

const THOUSANDS_SEPARATOR: char = ',';
const DECIMAL_PT: char = '.';

// Compare a and b as numbers without parsing them to f64.
// This improves performance and allows to compare numbers of arbitrary length.
pub fn numeric_str_cmp(a: &str, b: &str) -> Ordering {
    // First, check for a difference in the sign
    let (a, sign_a) = parse_sign(a);
    let (b, sign_b) = parse_sign(b);

    let sign_cmp = sign_a.cmp(&sign_b);
    if sign_cmp != Ordering::Equal {
        let a_contains_invalid = contains_invalid_chars(&mut a.chars(), &mut false);
        let b_contains_invalid = contains_invalid_chars(&mut b.chars(), &mut false);

        return ordering_from_invalid_chars(a_contains_invalid, b_contains_invalid)
            .unwrap_or(sign_cmp);
    }

    let minus = sign_a == -1;

    let mut ordering_if_same_len = Ordering::Equal;

    let mut a_chars = a.chars().filter(|&c| c != THOUSANDS_SEPARATOR);
    let mut b_chars = b.chars().filter(|&c| c != THOUSANDS_SEPARATOR);

    let mut a_contains_invalid = false;
    let mut b_contains_invalid = false;

    let mut both_had_decimal_pt = false;

    // These flags are needed to detect duplicate decimal points
    let mut a_had_decimal_pt = false;
    let mut b_had_decimal_pt = false;

    let orderng = 'outer: loop {
        let a_next = a_chars.next();
        let b_next = b_chars.next();
        if a_next.map_or(false, |c| is_invalid_char(c, &mut a_had_decimal_pt)) {
            a_contains_invalid = true
        }
        if b_next.map_or(false, |c| is_invalid_char(c, &mut b_had_decimal_pt)) {
            b_contains_invalid = true
        }
        match (a_next, b_next) {
            (None, None) => break ordering_if_same_len,
            (Some(DECIMAL_PT), Some(DECIMAL_PT)) => {
                both_had_decimal_pt = true;
            }
            (Some(_), Some(DECIMAL_PT)) => break Ordering::Greater,
            (Some(DECIMAL_PT), Some(_)) => break Ordering::Less,
            (Some(c_a), None) => {
                if both_had_decimal_pt && c_a == '0' {
                    for a_char in &mut a_chars {
                        if is_invalid_char(a_char, &mut a_had_decimal_pt) {
                            a_contains_invalid = true;
                            // this value will not be used because the a_contains_invalid flag is set.
                            break 'outer Ordering::Less;
                        } else if a_char != '0' {
                            break 'outer Ordering::Greater;
                        }
                    }
                    break Ordering::Equal;
                } else {
                    break Ordering::Greater;
                };
            }
            (None, Some(c_b)) => {
                if both_had_decimal_pt && c_b == '0' {
                    for b_char in &mut b_chars {
                        if is_invalid_char(b_char, &mut b_had_decimal_pt) {
                            b_contains_invalid = true;
                            // this value will not be used because the b_contains_invalid flag is set.
                            break 'outer Ordering::Less;
                        } else if b_char != '0' {
                            break 'outer Ordering::Less;
                        }
                    }
                    break Ordering::Equal;
                } else {
                    break Ordering::Less;
                };
            }
            (Some(c_a), Some(c_b)) => {
                if ordering_if_same_len == Ordering::Equal {
                    ordering_if_same_len = c_a.cmp(&c_b);
                    if both_had_decimal_pt && ordering_if_same_len != Ordering::Equal {
                        break ordering_if_same_len;
                    }
                }
            }
        }
    };

    a_contains_invalid =
        a_contains_invalid || contains_invalid_chars(&mut a_chars, &mut a_had_decimal_pt);
    b_contains_invalid =
        b_contains_invalid || contains_invalid_chars(&mut b_chars, &mut b_had_decimal_pt);

    if let Some(ordering) = ordering_from_invalid_chars(a_contains_invalid, b_contains_invalid) {
        return ordering;
    }

    if minus {
        orderng.reverse()
    } else {
        orderng
    }
}

fn ordering_from_invalid_chars(
    a_contains_invalid: bool,
    b_contains_invalid: bool,
) -> Option<Ordering> {
    // treat numbers containing invalid characters as -infinity, therefore they are smaller.
    match (a_contains_invalid, b_contains_invalid) {
        (false, false) => None,
        (true, false) => Some(Ordering::Less),
        (false, true) => Some(Ordering::Greater),
        (true, true) => Some(Ordering::Equal),
    }
}

fn parse_sign(str: &str) -> (&str, i8) {
    if let Some(remaining) = str.strip_prefix('-') {
        (remaining, -1)
    } else if let Some(remaining) = str.strip_prefix('+') {
        (remaining, 1)
    } else {
        // Parse + for no sign
        (str, 1)
    }
}

fn contains_invalid_chars(
    iter: &mut impl Iterator<Item = char>,
    had_decimal_pt: &mut bool,
) -> bool {
    iter.any(|c| is_invalid_char(c, had_decimal_pt))
}

fn is_invalid_char(c: char, had_decimal_pt: &mut bool) -> bool {
    if c == DECIMAL_PT {
        if *had_decimal_pt {
            // this is a decimal pt but we already had one, so it is invalid
            true
        } else {
            *had_decimal_pt = true;
            false
        }
    } else {
        !c.is_ascii_digit() && c != DECIMAL_PT && c != THOUSANDS_SEPARATOR
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_single_digit() {
        assert_eq!(numeric_str_cmp("1", "2"), Ordering::Less);
        assert_eq!(numeric_str_cmp("2", "1"), Ordering::Greater);
        assert_eq!(numeric_str_cmp("0", "0"), Ordering::Equal);
    }
    #[test]
    fn test_minus() {
        assert_eq!(numeric_str_cmp("-1", "-2"), Ordering::Greater);
        assert_eq!(numeric_str_cmp("-2", "-1"), Ordering::Less);
        assert_eq!(numeric_str_cmp("-0", "-0"), Ordering::Equal);
    }
    #[test]
    fn test_different_len() {
        assert_eq!(numeric_str_cmp("-20", "-100"), Ordering::Greater);
        assert_eq!(numeric_str_cmp("10.0", "2.000000"), Ordering::Greater);
    }
    #[test]
    fn test_decimal_digits() {
        assert_eq!(numeric_str_cmp("20.1", "20.2"), Ordering::Less);
        assert_eq!(numeric_str_cmp("20.1", "20.15"), Ordering::Less);
        assert_eq!(numeric_str_cmp("-20.1", "+20.15"), Ordering::Less);
        assert_eq!(numeric_str_cmp("-20.1", "-20"), Ordering::Less);
    }
    #[test]
    fn test_trailing_zeroes() {
        assert_eq!(numeric_str_cmp("20.00000", "20.1"), Ordering::Less);
        assert_eq!(numeric_str_cmp("20.00000", "20.0"), Ordering::Equal);
    }
    #[test]
    fn test_invalid_digits() {
        assert_eq!(numeric_str_cmp("foo", "bar"), Ordering::Equal);
        assert_eq!(numeric_str_cmp("20.1", "a"), Ordering::Greater);
        assert_eq!(numeric_str_cmp("-20.1", "a"), Ordering::Greater);
        assert_eq!(numeric_str_cmp("a", "0.15"), Ordering::Less);
    }
    #[test]
    fn test_multiple_decimal_pts() {
        assert_eq!(numeric_str_cmp("10.0.0", "50.0.0"), Ordering::Equal);
        assert_eq!(numeric_str_cmp("0.1.", "0.2.0"), Ordering::Equal);
        assert_eq!(numeric_str_cmp("1.1.", "0"), Ordering::Less);
        assert_eq!(numeric_str_cmp("1.1.", "-0"), Ordering::Less);
    }

    #[test]
    fn minus_zero() {
        // This matches GNU sort behavior.
        assert_eq!(numeric_str_cmp("-0", "0"), Ordering::Less);
    }
}
