// Answer 0

#[test]
fn test_starts_with_digit_empty_string() {
    let input = "";
    starts_with_digit(input);
}

#[test]
fn test_starts_with_digit_space() {
    let input = " ";
    starts_with_digit(input);
}

#[test]
fn test_starts_with_digit_tab() {
    let input = "\t";
    starts_with_digit(input);
}

#[test]
fn test_starts_with_digit_newline() {
    let input = "\n";
    starts_with_digit(input);
}

#[test]
fn test_starts_with_digit_carriage_return() {
    let input = "\r";
    starts_with_digit(input);
}

