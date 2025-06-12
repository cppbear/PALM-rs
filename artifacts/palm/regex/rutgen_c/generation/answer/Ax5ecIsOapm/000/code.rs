// Answer 0

#[test]
fn test_repeat_char_zero_count() {
    let result = repeat_char('a', 0);
    assert_eq!(result, "");
}

#[test]
fn test_repeat_char_one_count() {
    let result = repeat_char('b', 1);
    assert_eq!(result, "b");
}

#[test]
fn test_repeat_char_five_count() {
    let result = repeat_char('c', 5);
    assert_eq!(result, "ccccc");
}

#[test]
fn test_repeat_char_ten_count() {
    let result = repeat_char('d', 10);
    assert_eq!(result, "dddddddddd");
}

#[test]
fn test_repeat_char_large_count() {
    let result = repeat_char('e', 100);
    assert_eq!(result.len(), 100);
    assert_eq!(result.chars().next().unwrap(), 'e');
}

