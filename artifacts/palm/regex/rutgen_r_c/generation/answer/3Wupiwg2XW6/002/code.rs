// Answer 0

#[test]
fn test_is_negated_not_equal_negated() {
    let span = Span { start: Position(0), end: Position(10) };
    let class = ClassUnicode { 
        span, 
        negated: true, 
        kind: ClassUnicodeKind::NamedValue { 
            op: ClassUnicodeOpKind::NotEqual, 
            name: String::from("scx"), 
            value: String::from("Katakana") 
        } 
    };
    assert_eq!(class.is_negated(), false);
}

#[test]
fn test_is_negated_not_equal_not_negated() {
    let span = Span { start: Position(0), end: Position(10) };
    let class = ClassUnicode { 
        span, 
        negated: false, 
        kind: ClassUnicodeKind::NamedValue { 
            op: ClassUnicodeOpKind::NotEqual, 
            name: String::from("scx"), 
            value: String::from("Katakana") 
        } 
    };
    assert_eq!(class.is_negated(), false);
}

#[test]
fn test_is_negated_negated() {
    let span = Span { start: Position(0), end: Position(10) };
    let class = ClassUnicode { 
        span, 
        negated: true, 
        kind: ClassUnicodeKind::NamedValue { 
            op: ClassUnicodeOpKind::Equal, 
            name: String::from("scx"), 
            value: String::from("Katakana") 
        } 
    };
    assert_eq!(class.is_negated(), true);
}

#[test]
fn test_is_negated_not_negated() {
    let span = Span { start: Position(0), end: Position(10) };
    let class = ClassUnicode { 
        span, 
        negated: false, 
        kind: ClassUnicodeKind::NamedValue { 
            op: ClassUnicodeOpKind::Equal, 
            name: String::from("scx"), 
            value: String::from("Katakana") 
        } 
    };
    assert_eq!(class.is_negated(), false);
}

