// Answer 0

#[test]
fn test_hamming_empty_strings() {
    hamming("", "");
}

#[test]
fn test_hamming_single_character_match() {
    hamming("a", "a");
}

#[test]
fn test_hamming_single_character_mismatch() {
    hamming("a", "b");
}

#[test]
fn test_hamming_short_strings_same_content() {
    hamming("abc", "abc");
}

#[test]
fn test_hamming_mismatch_in_length_shorter_second_string() {
    hamming("abc", "ab");
}

#[test]
fn test_hamming_mismatch_in_length_longer_second_string() {
    hamming("abc", "abcd");
}

#[test]
fn test_hamming_lengths_match_single_character_difference() {
    hamming("abcd", "abce");
}

#[test]
fn test_hamming_lengths_match_all_characters_same() {
    hamming("abcd", "abcd");
}

#[test]
fn test_hamming_same_length_multiple_differences() {
    hamming("hamming", "hammers");
}

#[test]
fn test_hamming_same_length_single_character_difference_in_longer_string() {
    hamming("longerstring", "longerstrung");
}

