fn push(&self, frame: HirFrame) {
        self.trans().stack.borrow_mut().push(frame);
    }