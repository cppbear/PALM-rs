fn visit_class_set_item_pre(
        &mut self,
        ast: &ast::ClassSetItem,
    ) -> Result<()> {
        match *ast {
            ast::ClassSetItem::Bracketed(_) => {
                if self.flags().unicode() {
                    let cls = hir::ClassUnicode::empty();
                    self.push(HirFrame::ClassUnicode(cls));
                } else {
                    let cls = hir::ClassBytes::empty();
                    self.push(HirFrame::ClassBytes(cls));
                }
            }
            // We needn't handle the Union case here since the visitor will
            // do it for us.
            _ => {}
        }
        Ok(())
    }