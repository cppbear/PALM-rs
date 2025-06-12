fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match *self {
            ClassInduct::Item(it) => {
                match *it {
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