fn hir_literal(&self, lit: &ast::Literal) -> Result<Hir> {
        let ch = match self.literal_to_char(lit)? {
            byte @ hir::Literal::Byte(_) => return Ok(Hir::literal(byte)),
            hir::Literal::Unicode(ch) => ch,
        };
        if self.flags().case_insensitive() {
            self.hir_from_char_case_insensitive(lit.span, ch)
        } else {
            self.hir_from_char(lit.span, ch)
        }
    }