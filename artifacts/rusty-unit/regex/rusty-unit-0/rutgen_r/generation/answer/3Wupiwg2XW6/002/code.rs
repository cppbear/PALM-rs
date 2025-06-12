// Answer 0

#[test]
fn test_is_negated_not_equal_false() {
    struct ClassUnicodeKind {
        op: ClassUnicodeOpKind,
    }

    enum ClassUnicodeOpKind {
        NotEqual,
    }

    struct ClassUnicode {
        kind: ClassUnicodeKind,
        negated: bool,
    }

    let unicode_class = ClassUnicode {
        kind: ClassUnicodeKind {
            op: ClassUnicodeOpKind::NotEqual,
        },
        negated: false,
    };

    assert_eq!(unicode_class.is_negated(), true);
}

#[test]
fn test_is_negated_not_equal_true() {
    struct ClassUnicodeKind {
        op: ClassUnicodeOpKind,
    }

    enum ClassUnicodeOpKind {
        NotEqual,
    }

    struct ClassUnicode {
        kind: ClassUnicodeKind,
        negated: bool,
    }

    let unicode_class = ClassUnicode {
        kind: ClassUnicodeKind {
            op: ClassUnicodeOpKind::NotEqual,
        },
        negated: true,
    };

    assert_eq!(unicode_class.is_negated(), false);
}

#[test]
fn test_is_negated_other_kind() {
    struct ClassUnicodeKind {
        op: ClassUnicodeOpKind,
    }

    enum ClassUnicodeOpKind {
        Equal,
        NotEqual,
    }

    struct ClassUnicode {
        kind: ClassUnicodeKind,
        negated: bool,
    }

    let unicode_class = ClassUnicode {
        kind: ClassUnicodeKind {
            op: ClassUnicodeOpKind::Equal,
        },
        negated: true,
    };

    assert_eq!(unicode_class.is_negated(), true);
}

