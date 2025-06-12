// Answer 0

#[test]
fn test_at_valid_index_zero() {
    let input = ByteInput { text: b"test", only_utf8: true };
    let result = input.at(0);
}

#[test]
fn test_at_valid_index_middle() {
    let input = ByteInput { text: b"test", only_utf8: true };
    let result = input.at(2);
}

#[test]
fn test_at_valid_index_last() {
    let input = ByteInput { text: b"test", only_utf8: true };
    let result = input.at(3);
}

#[test]
fn test_at_empty_input() {
    let input = ByteInput { text: b"", only_utf8: true };
    let result = input.at(0);
}

#[should_panic]
fn test_at_out_of_bounds_high() {
    let input = ByteInput { text: b"test", only_utf8: true };
    let result = input.at(usize::MAX);
}

#[should_panic]
fn test_at_out_of_bounds_negative() {
    let input = ByteInput { text: b"test", only_utf8: true };
    let result = input.at(usize::MAX - 1);
}

