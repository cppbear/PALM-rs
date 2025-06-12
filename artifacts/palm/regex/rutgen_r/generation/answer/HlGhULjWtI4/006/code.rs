// Answer 0

fn test_fmt_class_induct_item_perl() {
    use std::fmt;

    mod ast {
        #[derive(Debug)]
        pub enum ClassSetItem {
            Empty(),
            Literal(),
            Range(),
            Ascii(),
            Perl(),
            Unicode(),
            Bracketed(),
            Union(),
        }

        #[derive(Debug)]
        pub struct ClassSetBinaryOp {
            pub kind: ClassSetBinaryOpKind,
        }

        #[derive(Debug)]
        pub enum ClassSetBinaryOpKind {
            Intersection,
            Difference,
            SymmetricDifference,
        }
        
        #[derive(Debug)]
        pub enum ClassInduct {
            Item(ClassSetItem),
            BinaryOp(ClassSetBinaryOp),
        }
    }

    impl fmt::Display for ast::ClassInduct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let x = match *self {
                ast::ClassInduct::Item(it) => {
                    match it {
                        ast::ClassSetItem::Empty() => "Item(Empty)",
                        ast::ClassSetItem::Literal() => "Item(Literal)",
                        ast::ClassSetItem::Range() => "Item(Range)",
                        ast::ClassSetItem::Ascii() => "Item(Ascii)",
                        ast::ClassSetItem::Perl() => "Item(Perl)",
                        ast::ClassSetItem::Unicode() => "Item(Unicode)",
                        ast::ClassSetItem::Bracketed() => "Item(Bracketed)",
                        ast::ClassSetItem::Union() => "Item(Union)",
                    }
                }
                ast::ClassInduct::BinaryOp(it) => {
                    match it.kind {
                        ast::ClassSetBinaryOpKind::Intersection => {
                            "BinaryOp(Intersection)"
                        }
                        ast::ClassSetBinaryOpKind::Difference => {
                            "BinaryOp(Difference)"
                        }
                        ast::ClassSetBinaryOpKind::SymmetricDifference => {
                            "BinaryOp(SymmetricDifference)"
                        }
                    }
                }
            };
            write!(f, "{}", x)
        }
    }

    let item_perl = ast::ClassSetItem::Perl();
    let class_induct_item = ast::ClassInduct::Item(item_perl);
    
    let result = format!("{}", class_induct_item);
    assert_eq!(result, "Item(Perl)");
}

