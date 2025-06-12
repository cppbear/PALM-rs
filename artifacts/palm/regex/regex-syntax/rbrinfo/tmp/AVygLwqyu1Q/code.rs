fn visit_class_set_item_post(
        &mut self,
        ast: &ast::ClassSetItem,
    ) -> Result<()> {
        match *ast {
            ast::ClassSetItem::Empty(_)
            | ast::ClassSetItem::Literal(_)
            | ast::ClassSetItem::Range(_)
            | ast::ClassSetItem::Ascii(_)
            | ast::ClassSetItem::Unicode(_)
            | ast::ClassSetItem::Perl(_) => {
                // These are all base cases, so we don't decrement depth.
                Ok(())
            }
            ast::ClassSetItem::Bracketed(_)
            | ast::ClassSetItem::Union(_) => {
                self.decrement_depth();
                Ok(())
            }
        }
    }