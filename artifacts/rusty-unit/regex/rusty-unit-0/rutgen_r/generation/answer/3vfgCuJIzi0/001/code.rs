// Answer 0

#[test]
fn test_fmt_with_valid_span() {
    struct Span {
        start: usize,
        end: usize,
    }

    impl Span {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Span({:?}, {:?})", self.start, self.end)
        }
    }

    let span = Span { start: 0, end: 10 };
    
    let mut buffer = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut buffer);
        span.fmt(&mut formatter).expect("Formatting failed");
    }

    assert_eq!(buffer, "Span(0, 10)");
}

#[test]
fn test_fmt_with_zero_span() {
    struct Span {
        start: usize,
        end: usize,
    }

    impl Span {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Span({:?}, {:?})", self.start, self.end)
        }
    }

    let span = Span { start: 5, end: 5 };
    
    let mut buffer = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut buffer);
        span.fmt(&mut formatter).expect("Formatting failed");
    }

    assert_eq!(buffer, "Span(5, 5)");
}

#[test]
fn test_fmt_with_reversed_span() {
    struct Span {
        start: usize,
        end: usize,
    }

    impl Span {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Span({:?}, {:?})", self.start, self.end)
        }
    }

    let span = Span { start: 10, end: 0 };

    let mut buffer = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut buffer);
        span.fmt(&mut formatter).expect("Formatting failed");
    }

    assert_eq!(buffer, "Span(10, 0)");
}

