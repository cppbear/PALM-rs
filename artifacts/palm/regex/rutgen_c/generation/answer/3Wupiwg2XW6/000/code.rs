// Answer 0

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassUnicodeOpKind {
    NotEqual,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position(usize);

#[test]
fn test_is_negated_simple_negated() {
    let span = Span {
        start: Position(0),
        end: Position(1),
    };
    let class = ClassUnicode {
        span,
        negated: true,
        kind: ClassUnicodeKind::OneLetter('N'),
    };
    assert!(class.is_negated());
}

#[test]
fn test_is_negated_simple_non_negated() {
    let span = Span {
        start: Position(0),
        end: Position(1),
    };
    let class = ClassUnicode {
        span,
        negated: false,
        kind: ClassUnicodeKind::OneLetter('N'),
    };
    assert!(!class.is_negated());
}

#[test]
fn test_is_negated_named_value_negated() {
    let span = Span {
        start: Position(0),
        end: Position(1),
    };
    let class = ClassUnicode {
        span,
        negated: true,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: String::from("scx"),
            value: String::from("Katakana"),
        },
    };
    assert!(!class.is_negated());
}

#[test]
fn test_is_negated_named_value_non_negated() {
    let span = Span {
        start: Position(0),
        end: Position(1),
    };
    let class = ClassUnicode {
        span,
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: String::from("scx"),
            value: String::from("Katakana"),
        },
    };
    assert!(!class.is_negated());
}

#[test]
fn test_is_negated_named_value_non_negated_with_negated_true() {
    let span = Span {
        start: Position(0),
        end: Position(1),
    };
    let class = ClassUnicode {
        span,
        negated: true,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: String::from("scx"),
            value: String::from("Katakana"),
        },
    };
    assert!(!class.is_negated());
}

