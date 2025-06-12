// Answer 0

#[test]
fn test_child_union_literal() {
    let span = Span::new(0, 5);
    let literal = ClassSetItem::Literal(Literal::from_char('a'));
    let class_set = ClassSet::Item(literal);
    let class_frame = ClassFrame::Union {
        head: &class_set,
        tail: &[],
    };
    class_frame.child();
}

#[test]
fn test_child_union_range() {
    let span = Span::new(0, 5);
    let range = ClassSetItem::Range(ClassSetRange::new('a', 'z'));
    let class_set = ClassSet::Item(range);
    let class_frame = ClassFrame::Union {
        head: &class_set,
        tail: &[],
    };
    class_frame.child();
}

#[test]
fn test_child_union_ascii() {
    let span = Span::new(0, 5);
    let ascii_class = ClassSetItem::Ascii(ClassAscii::Alnum);
    let class_set = ClassSet::Item(ascii_class);
    let class_frame = ClassFrame::Union {
        head: &class_set,
        tail: &[],
    };
    class_frame.child();
}

#[test]
fn test_child_union_unicode() {
    let span = Span::new(0, 5);
    let unicode_class = ClassSetItem::Unicode(ClassUnicode::new("L"));
    let class_set = ClassSet::Item(unicode_class);
    let class_frame = ClassFrame::Union {
        head: &class_set,
        tail: &[],
    };
    class_frame.child();
}

#[test]
fn test_child_union_perl() {
    let span = Span::new(0, 5);
    let perl_class = ClassSetItem::Perl(ClassPerl::Digit);
    let class_set = ClassSet::Item(perl_class);
    let class_frame = ClassFrame::Union {
        head: &class_set,
        tail: &[],
    };
    class_frame.child();
}

#[test]
fn test_child_union_bracketed() {
    let span = Span::new(0, 5);
    let bracketed_class = ClassSetItem::Bracketed(Box::new(ClassBracketed::new(vec![])));
    let class_set = ClassSet::Item(bracketed_class);
    let class_frame = ClassFrame::Union {
        head: &class_set,
        tail: &[],
    };
    class_frame.child();
}

#[test]
fn test_child_union_union() {
    let span = Span::new(0, 5);
    let union_class = ClassSetItem::Union(ClassSetUnion::new(vec![]));
    let class_set = ClassSet::Item(union_class);
    let class_frame = ClassFrame::Union {
        head: &class_set,
        tail: &[],
    };
    class_frame.child();
}

