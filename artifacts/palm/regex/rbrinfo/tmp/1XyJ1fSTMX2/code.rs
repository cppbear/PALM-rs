fn at(&self, i: usize) -> InputAt {
        InputAt {
            pos: i,
            c: None.into(),
            byte: self.get(i).cloned(),
            len: 1,
        }
    }