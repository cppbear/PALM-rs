// Answer 0

#[test]
fn test_fmt_cut() {
    // Create a new Literal instance that is cut.
    let mut literal = Literal {
        v: vec![65, 66, 67], // ASCII for 'ABC'
        cut: true,
    };

    // Attempt to format the Literal instance.
    let result = format!("{:?}", literal);
    assert_eq!(result, "Cut(ABC)");
}

#[test]
fn test_fmt_cut_with_unicode() {
    // Create a new Literal instance that is cut with Unicode characters.
    let mut literal = Literal {
        v: vec![240, 159, 152, 128], // UTF-8 for 😄 (smiling face with open mouth)
        cut: true,
    };

    // Attempt to format the Literal instance.
    let result = format!("{:?}", literal);
    assert_eq!(result, "Cut(\u{1f604})");
}

#[test]
fn test_fmt_cut_with_whitespace() {
    // Create a new Literal instance that is cut with whitespace.
    let mut literal = Literal {
        v: vec![32, 65, 32], // ASCII for ' A '
        cut: true,
    };

    // Attempt to format the Literal instance.
    let result = format!("{:?}", literal);
    assert_eq!(result, "Cut( A )");
}

#[test]
fn test_fmt_cut_empty() {
    // Create a new Literal instance that is cut and empty.
    let mut literal = Literal {
        v: vec![], // Empty Vec<u8>
        cut: true,
    };

    // Attempt to format the Literal instance.
    let result = format!("{:?}", literal);
    assert_eq!(result, "Cut( )");
}

