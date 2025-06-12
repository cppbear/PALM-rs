fn from_set(ast: &'a ast::ClassSet) -> ClassInduct<'a> {
        match *ast {
            ast::ClassSet::Item(ref item) => ClassInduct::Item(item),
            ast::ClassSet::BinaryOp(ref op) => ClassInduct::BinaryOp(op),
        }
    }