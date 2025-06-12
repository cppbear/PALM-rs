// Answer 0

#[test]
fn test_fmt_item_literal() {
    use std::fmt;
    
    struct ClassSetItem {
        kind: String,
    }

    impl ClassSetItem {
        fn literal() -> Self {
            ClassSetItem {
                kind: "Literal".to_string(),
            }
        }
    }
    
    struct ClassInduct {
        item: ClassSetItem,
    }

    impl fmt::Display for ClassInduct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let x = match &self.item {
                ClassSetItem { kind } if kind == "Empty" => "Item(Empty)",
                ClassSetItem { kind } if kind == "Literal" => "Item(Literal)",
                ClassSetItem { kind } if kind == "Range" => "Item(Range)",
                ClassSetItem { kind } if kind == "Ascii" => "Item(Ascii)",
                ClassSetItem { kind } if kind == "Perl" => "Item(Perl)",
                ClassSetItem { kind } if kind == "Unicode" => "Item(Unicode)",
                ClassSetItem { kind } if kind == "Bracketed" => "Item(Bracketed)",
                ClassSetItem { kind } if kind == "Union" => "Item(Union)",
                _ => "Invalid",
            };
            write!(f, "{}", x)
        }
    }

    let item_literal = ClassSetItem::literal();
    let class_induct = ClassInduct { item: item_literal };

    let mut output = String::new();
    {
        let writer = &mut output;
        class_induct.fmt(writer).unwrap();
    }

    assert_eq!(output, "Item(Literal)");
}

