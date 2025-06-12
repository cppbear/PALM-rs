fn fmt_literal(&mut self, ast: &ast::Literal) -> fmt::Result {
        use ast::LiteralKind::*;

        match ast.kind {
            Verbatim => self.wtr.write_char(ast.c),
            Punctuation => write!(self.wtr, r"\{}", ast.c),
            Octal => write!(self.wtr, r"\{:o}", ast.c as u32),
            HexFixed(ast::HexLiteralKind::X) => {
                write!(self.wtr, r"\x{:02X}", ast.c as u32)
            }
            HexFixed(ast::HexLiteralKind::UnicodeShort) => {
                write!(self.wtr, r"\u{:04X}", ast.c as u32)
            }
            HexFixed(ast::HexLiteralKind::UnicodeLong) => {
                write!(self.wtr, r"\U{:08X}", ast.c as u32)
            }
            HexBrace(ast::HexLiteralKind::X) => {
                write!(self.wtr, r"\x{{{:X}}}", ast.c as u32)
            }
            HexBrace(ast::HexLiteralKind::UnicodeShort) => {
                write!(self.wtr, r"\u{{{:X}}}", ast.c as u32)
            }
            HexBrace(ast::HexLiteralKind::UnicodeLong) => {
                write!(self.wtr, r"\U{{{:X}}}", ast.c as u32)
            }
            Special(ast::SpecialLiteralKind::Bell) => {
                self.wtr.write_str(r"\a")
            }
            Special(ast::SpecialLiteralKind::FormFeed) => {
                self.wtr.write_str(r"\f")
            }
            Special(ast::SpecialLiteralKind::Tab) => {
                self.wtr.write_str(r"\t")
            }
            Special(ast::SpecialLiteralKind::LineFeed) => {
                self.wtr.write_str(r"\n")
            }
            Special(ast::SpecialLiteralKind::CarriageReturn) => {
                self.wtr.write_str(r"\r")
            }
            Special(ast::SpecialLiteralKind::VerticalTab) => {
                self.wtr.write_str(r"\v")
            }
            Special(ast::SpecialLiteralKind::Space) => {
                self.wtr.write_str(r"\ ")
            }
        }
    }