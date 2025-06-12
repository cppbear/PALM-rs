// Answer 0

#[test]
fn test_fmt_class_induct_item_ascii() {
    use std::fmt;
    use regex_syntax::ast;

    struct ClassInduct {
        item: ast::ClassSetItem,
    }

    impl fmt::Display for ClassInduct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let x = match self {
                ClassInduct { item: ast::ClassSetItem::Empty(_) } => "Item(Empty)",
                ClassInduct { item: ast::ClassSetItem::Literal(_) } => "Item(Literal)",
                ClassInduct { item: ast::ClassSetItem::Range(_) } => "Item(Range)",
                ClassInduct { item: ast::ClassSetItem::Ascii(_) } => "Item(Ascii)",
                ClassInduct { item: ast::ClassSetItem::Perl(_) } => "Item(Perl)",
                ClassInduct { item: ast::ClassSetItem::Unicode(_) } => "Item(Unicode)",
                ClassInduct { item: ast::ClassSetItem::Bracketed(_) } => "Item(Bracketed)",
                ClassInduct { item: ast::ClassSetItem::Union(_) } => "Item(Union)",
            };
            write!(f, "{}", x)
        }
    }

    let ascii_item = ast::ClassSetItem::Ascii('a'); // Example initialization
    let class_induct = ClassInduct { item: ascii_item };

    let mut output = String::new();
    let result = write!(&mut output, "{}", class_induct);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Ascii)");
}

