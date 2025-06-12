// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Span({:?}, {:?})", self.start, self.end)
    }
}

#[test]
fn test_fmt() {
    let span = Span { start: 5, end: 10 };
    let mut output = String::new();
    let result = span.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Span(5, 10)");
}

#[test]
fn test_fmt_with_boundary_values() {
    let span = Span { start: 0, end: 0 };
    let mut output = String::new();
    let result = span.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Span(0, 0)");
}

#[test]
fn test_fmt_with_large_values() {
    let span = Span { start: usize::MAX, end: usize::MAX };
    let mut output = String::new();
    let result = span.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Span(18446744073709551615, 18446744073709551615)");
}

