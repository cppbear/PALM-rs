// Answer 0

#[test]
fn test_max_size_reached_display() {
    let error = MaxSizeReached { _priv: () };
    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    error.fmt(&mut formatter);
}

#[test]
fn test_max_size_reached_display_empty() {
    let error = MaxSizeReached { _priv: () };
    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    error.fmt(&mut formatter);
}

