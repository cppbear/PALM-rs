fn add(&mut self, span: ast::Span) {
        // This is grossly inefficient since we sort after each add, but right
        // now, we only ever add two spans at most.
        if span.is_one_line() {
            let i = span.start.line - 1; // because lines are 1-indexed
            self.by_line[i].push(span);
            self.by_line[i].sort();
        } else {
            self.multi_line.push(span);
            self.multi_line.sort();
        }
    }