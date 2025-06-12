// Answer 0

#[test]
fn test_is_always_utf8_unicode() {
    use hir::interval::{IntervalSet, Interval};
    use ast::Span;

    let unicode_class = Class::Unicode(ClassUnicode {
        span: Span::new(0, 10),
        negated: false,
        kind: ClassUnicodeKind::SomeKind,
    });

    unicode_class.is_always_utf8();
}

#[test]
fn test_is_always_utf8_unicode_with_different_span() {
    use hir::interval::{IntervalSet, Interval};
    use ast::Span;

    let unicode_class = Class::Unicode(ClassUnicode {
        span: Span::new(5, 15),
        negated: false,
        kind: ClassUnicodeKind::SomeKind,
    });

    unicode_class.is_always_utf8();
}

#[test]
fn test_is_always_utf8_unicode_negated() {
    use hir::interval::{IntervalSet, Interval};
    use ast::Span;

    let unicode_class = Class::Unicode(ClassUnicode {
        span: Span::new(0, 10),
        negated: true,
        kind: ClassUnicodeKind::SomeKind,
    });

    unicode_class.is_always_utf8();
}

