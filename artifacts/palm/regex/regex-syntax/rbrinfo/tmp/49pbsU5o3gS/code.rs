fn reset(&self) {
        // These settings should be in line with the construction
        // in `ParserBuilder::build`.
        self.pos.set(Position { offset: 0, line: 1, column: 1});
        self.ignore_whitespace.set(self.initial_ignore_whitespace);
        self.comments.borrow_mut().clear();
        self.stack_group.borrow_mut().clear();
        self.stack_class.borrow_mut().clear();
    }