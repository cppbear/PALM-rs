// Answer 0

#[derive(Debug)]
struct ClassInductWrapper(ClassInduct);

#[derive(Debug)]
enum ClassInduct {
    Item(Box<ast::ClassSetItem>),
    BinaryOp(BinaryOp),
}

#[derive(Debug)]
struct BinaryOp {
    kind: ast::ClassSetBinaryOpKind,
}

#[derive(Debug)]
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
    pub enum ClassSetBinaryOpKind {
        Intersection,
        Difference,
        SymmetricDifference,
    }
}

use std::fmt;

impl fmt::Display for ClassInductWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match &*self.0 {
            ClassInduct::Item(it) => {
                match &**it {
                    ast::ClassSetItem::Empty(_) => "Item(Empty)",
                    ast::ClassSetItem::Literal(_) => "Item(Literal)",
                    ast::ClassSetItem::Range(_) => "Item(Range)",
                    ast::ClassSetItem::Ascii(_) => "Item(Ascii)",
                    ast::ClassSetItem::Perl(_) => "Item(Perl)",
                    ast::ClassSetItem::Unicode(_) => "Item(Unicode)",
                    ast::ClassSetItem::Bracketed(_) => "Item(Bracketed)",
                    ast::ClassSetItem::Union(_) => "Item(Union)",
                }
            }
            ClassInduct::BinaryOp(it) => {
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

#[test]
fn test_fmt_class_induct_item_union() {
    let item = ast::ClassSetItem::Union();
    let induct = ClassInductWrapper(ClassInduct::Item(Box::new(item)));
    let mut output = String::new();
    let result = write!(&mut output, "{}", induct);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Union)");
}

