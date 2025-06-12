// Answer 0

#[test]
fn test_fmt_complete() {
    let literal = Literal {
        v: b"hello".to_vec(),
        cut: false,
    };
    let mut output = vec![];
    let result = writeln!(&mut output, "{}", literal);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Complete(hello)\n");
}

#[test]
fn test_fmt_cut() {
    let literal = Literal {
        v: b"world".to_vec(),
        cut: true,
    };
    let mut output = vec![];
    let result = writeln!(&mut output, "{}", literal);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Cut(world)\n");
}

#[test]
fn test_fmt_empty() {
    let literal = Literal {
        v: Vec::new(),
        cut: false,
    };
    let mut output = vec![];
    let result = writeln!(&mut output, "{}", literal);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Complete()\n");
}

#[test]
fn test_fmt_cut_empty() {
    let literal = Literal {
        v: Vec::new(),
        cut: true,
    };
    let mut output = vec![];
    let result = writeln!(&mut output, "{}", literal);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Cut()\n");
}

