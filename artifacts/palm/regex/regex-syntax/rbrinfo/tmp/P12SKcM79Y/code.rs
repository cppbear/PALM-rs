pub fn build(&self) -> Parser {
        Parser {
            ast: self.ast.build(),
            hir: self.hir.build(),
        }
    }