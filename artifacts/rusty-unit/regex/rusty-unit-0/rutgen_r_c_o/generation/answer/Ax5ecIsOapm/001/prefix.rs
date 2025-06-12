// Answer 0

#[test]
fn test_repeat_char_with_zero_count() {
    let result = repeat_char('a', 0);
}

#[test]
fn test_repeat_char_with_minimum_count() {
    let result = repeat_char('b', 1);
}

#[test]
fn test_repeat_char_with_medium_count() {
    let result = repeat_char('c', 50);
}

#[test]
fn test_repeat_char_with_maximum_count() {
    let result = repeat_char('d', 100);
}

#[test]
fn test_repeat_char_with_different_characters() {
    let result1 = repeat_char('e', 25);
    let result2 = repeat_char('f', 10);
    let result3 = repeat_char('g', 75);
} 

#[test]
fn test_repeat_char_with_non_printable_character() {
    let result = repeat_char('\n', 10);
}

