fn check(self, ast: &Ast) -> Result<()> {
        ast::visit(ast, self)
    }