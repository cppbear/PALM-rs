// Answer 0

#[test]
fn test_fmt_map() {
    let unexpected_map = Unexpected::Map;
    let mut buffer = Vec::new();
    let mut formatter = fmt::Formatter::new(&mut buffer);
    unexpected_map.fmt(&mut formatter);
}

#[test]
fn test_fmt_map_with_empty() {
    let unexpected_map = Unexpected::Map;
    let mut buffer = Vec::new();
    let mut formatter = fmt::Formatter::new(&mut buffer);
    unexpected_map.fmt(&mut formatter);
}

