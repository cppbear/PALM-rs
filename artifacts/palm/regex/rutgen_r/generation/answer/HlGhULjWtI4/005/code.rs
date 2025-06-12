// Answer 0

#[test]
fn test_fmt_class_induct_item_bracketed() {
    use std::fmt;

    struct MockClassSetItem {
        kind: ast::ClassSetItem,
    }

    struct MockClassInduct {
        item: MockClassSetItem,
    }

    impl fmt::Display for MockClassInduct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let x = match &self.item.kind {
                ast::ClassSetItem::Empty(_) => "Item(Empty)",
                ast::ClassSetItem::Literal(_) => "Item(Literal)",
                ast::ClassSetItem::Range(_) => "Item(Range)",
                ast::ClassSetItem::Ascii(_) => "Item(Ascii)",
                ast::ClassSetItem::Perl(_) => "Item(Perl)",
                ast::ClassSetItem::Unicode(_) => "Item(Unicode)",
                ast::ClassSetItem::Bracketed(_) => "Item(Bracketed)",
                ast::ClassSetItem::Union(_) => "Item(Union)",
            };
            write!(f, "{}", x)
        }
    }

    enum ClassInduct {
        Item(MockClassInduct),
    }

    impl fmt::Display for ClassInduct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let x = match *self {
                ClassInduct::Item(ref it) => it.fmt(f)?,
            };
            Ok(x)
        }
    }

    let bracketed_item = ast::ClassSetItem::Bracketed(vec![]); // Assuming dummy data for the bracketed item
    let mock_item = MockClassSetItem { kind: bracketed_item };
    let class_induct = ClassInduct::Item(MockClassInduct { item: mock_item });

    let result = format!("{}", class_induct);
    assert_eq!(result, "Item(Bracketed)");
}

