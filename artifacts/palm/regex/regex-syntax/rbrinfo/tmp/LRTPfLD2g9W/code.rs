fn visit_pre(&mut self, ast: &Ast) -> Result<()> {
        match *ast {
            Ast::Class(ast::Class::Bracketed(_)) => {
                if self.flags().unicode() {
                    let cls = hir::ClassUnicode::empty();
                    self.push(HirFrame::ClassUnicode(cls));
                } else {
                    let cls = hir::ClassBytes::empty();
                    self.push(HirFrame::ClassBytes(cls));
                }
            }
            Ast::Group(ref x) => {
                let old_flags = x.flags().map(|ast| self.set_flags(ast));
                self.push(HirFrame::Group {
                    old_flags: old_flags,
                });
            }
            Ast::Concat(ref x) if x.asts.is_empty() => {}
            Ast::Concat(_) => {
                self.push(HirFrame::Concat);
            }
            Ast::Alternation(ref x) if x.asts.is_empty() => {}
            Ast::Alternation(_) => {
                self.push(HirFrame::Alternation);
            }
            _ => {}
        }
        Ok(())
    }