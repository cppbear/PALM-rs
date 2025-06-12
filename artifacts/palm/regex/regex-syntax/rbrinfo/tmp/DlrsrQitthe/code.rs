fn from_bracketed(ast: &'a ast::ClassBracketed) -> ClassInduct<'a> {
        ClassInduct::from_set(&ast.kind)
    }