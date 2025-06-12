// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn bigrams(s: &str) -> Vec<(char, char)> {
        let chars: Vec<_> = s.chars().collect();
        chars.windows(2).filter_map(|pair| {
            if let [a, b] = pair {
                Some((*a, *b))
            } else {
                None
            }
        }).collect()
    }

    #[test]
    fn test_sorensen_dice_identical_strings() {
        assert_eq!(1.0, sorensen_dice("ab", "ab"));
    }

    #[test]
    fn test_sorensen_dice_no_common_bigrams() {
        assert_eq!(0.0, sorensen_dice("ab", "cd"));
    }

    #[test]
    fn test_sorensen_dice_exact_bigrams() {
        assert_eq!(1.0, sorensen_dice("ab", "ab"));
    }

    #[test]
    fn test_sorensen_dice_partial_overlap() {
        assert_eq!(0.5, sorensen_dice("ab", "ac"));
    }

    #[test]
    fn test_sorensen_dice_more_parameters() {
        assert_eq!(0.5, sorensen_dice("cd", "ac"));
    }

    #[test]
    fn test_sorensen_dice_non_contiguous_bigrams() {
        assert_eq!(0.0, sorensen_dice("abc", "def"));
    }

    #[test]
    fn test_sorensen_dice_with_spaces() {
        assert_eq!(0.0, sorensen_dice("a b", "c d"));
    }

    #[test]
    fn test_sorensen_dice_with_same_characters_diff_order() {
        assert_eq!(0.0, sorensen_dice("ab", "ba"));
    }
}

