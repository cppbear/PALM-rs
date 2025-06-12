pub fn push(&mut self, item: ClassSetItem) {
        if self.items.is_empty() {
            self.span.start = item.span().start;
        }
        self.span.end = item.span().end;
        self.items.push(item);
    }