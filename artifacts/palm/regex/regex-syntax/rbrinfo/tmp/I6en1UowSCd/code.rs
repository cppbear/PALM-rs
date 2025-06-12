fn pop(&self) -> Option<HirFrame> {
        self.trans().stack.borrow_mut().pop()
    }