fn literal_to_char(&self, lit: &ast::Literal) -> Result<hir::Literal> {
        if self.flags().unicode() {
            return Ok(hir::Literal::Unicode(lit.c));
        }
        let byte = match lit.byte() {
            None => return Ok(hir::Literal::Unicode(lit.c)),
            Some(byte) => byte,
        };
        if byte <= 0x7F {
            return Ok(hir::Literal::Unicode(byte as char));
        }
        if !self.trans().allow_invalid_utf8 {
            return Err(self.error(lit.span, ErrorKind::InvalidUtf8));
        }
        Ok(hir::Literal::Byte(byte))
    }