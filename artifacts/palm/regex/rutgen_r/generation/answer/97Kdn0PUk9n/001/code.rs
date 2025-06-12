// Answer 0

#[test]
fn test_next_after_empty_valid_input() {
    let text = "";
    let index = 0;
    let result = next_after_empty(text, index);
    assert_eq!(result, 0);
}

#[test]
fn test_next_after_empty_index_beyond_text_length() {
    let text = "Hello";
    let index = 5; // index equal to the text length
    let result = next_after_empty(text, index);
    assert_eq!(result, 5);
}

#[test]
fn test_next_after_empty_index_in_middle() {
    let text = "Hello";
    let index = 1; // valid index within the text
    let result = next_after_empty(text, index);
    assert_eq!(result, 2);
}

#[test]
#[should_panic]
fn test_next_after_empty_negative_index() {
    let text = "Hello";
    let index = usize::MAX; // trigger panic for out-of-bounds access
    let _ = next_after_empty(text, index);
}

#[test]
fn test_next_after_empty_empty_string() {
    let text = "";
    let index = 0; // should return immediately as the string is empty
    let result = next_after_empty(text, index);
    assert_eq!(result, 0);
}

