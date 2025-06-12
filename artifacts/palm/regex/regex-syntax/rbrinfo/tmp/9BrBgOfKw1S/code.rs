fn visit_class_set_item_pre(
        &mut self,
        ast: &ast::ClassSetItem,
    ) -> Result<()> {
        let span = match *ast {
            ast::ClassSetItem::Empty(_)
            | ast::ClassSetItem::Literal(_)
            | ast::ClassSetItem::Range(_)
            | ast::ClassSetItem::Ascii(_)
            | ast::ClassSetItem::Unicode(_)
            | ast::ClassSetItem::Perl(_) => {
                // These are all base cases, so we don't increment depth.
                return Ok(());
            }
            ast::ClassSetItem::Bracketed(ref x) => &x.span,
            ast::ClassSetItem::Union(ref x) => &x.span,
        };
        self.increment_depth(span)
    }