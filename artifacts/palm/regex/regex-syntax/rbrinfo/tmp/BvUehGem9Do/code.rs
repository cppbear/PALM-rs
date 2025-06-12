pub fn translate(&mut self, pattern: &str, ast: &Ast) -> Result<Hir> {
        ast::visit(ast, TranslatorI::new(self, pattern))
    }