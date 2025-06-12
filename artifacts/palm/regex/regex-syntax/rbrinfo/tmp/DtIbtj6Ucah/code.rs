fn class_literal_byte(&self, ast: &ast::Literal) -> Result<u8> {
        match self.literal_to_char(ast)? {
            hir::Literal::Byte(byte) => Ok(byte),
            hir::Literal::Unicode(ch) => {
                if ch <= 0x7F as char {
                    Ok(ch as u8)
                } else {
                    // We can't feasibly support Unicode in
                    // byte oriented classes. Byte classes don't
                    // do Unicode case folding.
                    Err(self.error(ast.span, ErrorKind::UnicodeNotAllowed))
                }
            }
        }
    }