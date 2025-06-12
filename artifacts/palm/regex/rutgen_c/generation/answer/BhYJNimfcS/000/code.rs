// Answer 0

#[test]
fn test_fmt_empty_literals() {
    let literals = Literals {
        lits: vec![],
        limit_size: 0,
        limit_class: 0,
    };
    let mut buffer = vec![];
    let result = write!(&mut buffer, "{:?}", literals);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&buffer), "Literals { lits: [], limit_size: 0, limit_class: 0 }");
}

#[test]
fn test_fmt_single_unicode_literal() {
    let literals = Literals {
        lits: vec![Literal::Unicode('a')],
        limit_size: 1,
        limit_class: 1,
    };
    let mut buffer = vec![];
    let result = write!(&mut buffer, "{:?}", literals);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&buffer), "Literals { lits: [Unicode('a')], limit_size: 1, limit_class: 1 }");
}

#[test]
fn test_fmt_single_byte_literal() {
    let literals = Literals {
        lits: vec![Literal::Byte(97)],
        limit_size: 1,
        limit_class: 1,
    };
    let mut buffer = vec![];
    let result = write!(&mut buffer, "{:?}", literals);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&buffer), "Literals { lits: [Byte(97)], limit_size: 1, limit_class: 1 }");
}

#[test]
fn test_fmt_multiple_literals() {
    let literals = Literals {
        lits: vec![Literal::Unicode('a'), Literal::Byte(98)],
        limit_size: 2,
        limit_class: 2,
    };
    let mut buffer = vec![];
    let result = write!(&mut buffer, "{:?}", literals);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&buffer), "Literals { lits: [Unicode('a'), Byte(98)], limit_size: 2, limit_class: 2 }");
}

#[test]
fn test_fmt_large_limits() {
    let literals = Literals {
        lits: vec![Literal::Unicode('x')],
        limit_size: usize::MAX,
        limit_class: usize::MAX,
    };
    let mut buffer = vec![];
    let result = write!(&mut buffer, "{:?}", literals);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&buffer), "Literals { lits: [Unicode('x')], limit_size: 18446744073709551615, limit_class: 18446744073709551615 }");
}

