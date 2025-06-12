// Answer 0

#[test]
fn test_is_negated_with_not_equal_and_negated() {
    let span = Span { start: Position(0), end: Position(5) };
    let class_unicode = ClassUnicode {
        span,
        negated: true,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: "script".to_string(),
            value: "Latin".to_string(),
        },
    };
    class_unicode.is_negated();
}

#[test]
fn test_is_negated_with_not_equal_and_not_negated() {
    let span = Span { start: Position(0), end: Position(5) };
    let class_unicode = ClassUnicode {
        span,
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: "script".to_string(),
            value: "Cyrillic".to_string(),
        },
    };
    class_unicode.is_negated();
}

#[test]
fn test_is_negated_with_not_equal_and_negated_another_case() {
    let span = Span { start: Position(0), end: Position(5) };
    let class_unicode = ClassUnicode {
        span,
        negated: true,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: "script".to_string(),
            value: "Katakana".to_string(),
        },
    };
    class_unicode.is_negated();
}

#[test]
fn test_is_negated_with_not_equal_empty_name_and_not_negated() {
    let span = Span { start: Position(0), end: Position(5) };
    let class_unicode = ClassUnicode {
        span,
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: "".to_string(),
            value: "".to_string(),
        },
    };
    class_unicode.is_negated();
}

