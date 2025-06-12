fn finish(self) -> Result<Hir> {
        if self.trans().stack.borrow().is_empty() {
            // This can happen if the Ast given consists of a single set of
            // flags. e.g., `(?i)`. /shrug
            return Ok(Hir::empty());
        }
        // ... otherwise, we should have exactly one HIR on the stack.
        assert_eq!(self.trans().stack.borrow().len(), 1);
        Ok(self.pop().unwrap().unwrap_expr())
    }