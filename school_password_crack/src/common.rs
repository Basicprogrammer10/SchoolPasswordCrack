use std::collections::HashMap;

/// Use Dice Coefficient to calculate the similarity between two strings.
pub fn similarity(str1: &str, str2: &str) -> f64 {
    let a = str1.replace(" ", "");
    let b = str2.replace(" ", "");

    // Check some simple cases
    if a == b {
        return 1.0;
    }
    if a.len() < 2 || b.len() < 2 {
        return 0.0;
    }

    let mut firstBigrams: HashMap<&str, i32> = HashMap::new();
    for i in 0..a.len() - 1 {
        let bigram = &a[i..i + 2];
        let count = firstBigrams.get(bigram).unwrap_or(&0) + 1;
        firstBigrams.insert(bigram, count);
    }

    let mut intersectionSize = 0;
    for i in 0..b.len() - 1 {
        let bigram = &b[i..i + 2];
        let count = firstBigrams.get(bigram).unwrap_or(&0).clone();
        if count > 0 {
            firstBigrams.insert(bigram, count - 1);
            intersectionSize += 1;
        }
    }

    (2.0 * intersectionSize as f64) / (str1.len() + str2.len() - 2) as f64
}

pub fn upper_first_char(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity() {
        assert_eq!(similarity("abc", "abc"), 1.0);
        assert_eq!(similarity("hello", "world"), 0.0);
        assert_eq!(similarity("hello", "helll"), 0.75);
    }

    #[test]
    fn test_upper_first_char() {
        assert_eq!(upper_first_char("hello"), "Hello".to_string());
        assert_eq!(upper_first_char("HELLO"), "HELLO".to_string());
        assert_eq!(upper_first_char(""), "".to_string());
    }
}