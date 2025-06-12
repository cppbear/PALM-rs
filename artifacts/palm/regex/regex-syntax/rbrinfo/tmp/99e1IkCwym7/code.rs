fn fmt_class_unicode(&mut self, ast: &ast::ClassUnicode) -> fmt::Result {
        use ast::ClassUnicodeKind::*;
        use ast::ClassUnicodeOpKind::*;

        if ast.negated {
            self.wtr.write_str(r"\P")?;
        } else {
            self.wtr.write_str(r"\p")?;
        }
        match ast.kind {
            OneLetter(c) => self.wtr.write_char(c),
            Named(ref x) => write!(self.wtr, "{{{}}}", x),
            NamedValue { op: Equal, ref name, ref value } => {
                write!(self.wtr, "{{{}={}}}", name, value)
            }
            NamedValue { op: Colon, ref name, ref value } => {
                write!(self.wtr, "{{{}:{}}}", name, value)
            }
            NamedValue { op: NotEqual, ref name, ref value } => {
                write!(self.wtr, "{{{}!={}}}", name, value)
            }
        }
    }