fn visit_class_set_item_pre(
        &mut self,
        ast: &ast::ClassSetItem,
    ) -> Result<(), Self::Err> {
        match *ast {
            ast::ClassSetItem::Bracketed(ref x) => {
                self.fmt_class_bracketed_pre(x)
            }
            _ => Ok(()),
        }
    }