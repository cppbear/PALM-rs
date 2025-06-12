fn drop(&mut self) {
        (self.dropfn)(&mut self.value);
    }