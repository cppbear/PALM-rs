fn remaining(&self) -> usize {
        self.a.remaining().saturating_add(self.b.remaining())
    }