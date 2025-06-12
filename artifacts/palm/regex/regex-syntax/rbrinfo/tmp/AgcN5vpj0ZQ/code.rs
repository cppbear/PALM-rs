pub fn to_empty(&self) -> Literals {
        let mut lits = Literals::empty();
        lits.set_limit_size(self.limit_size)
            .set_limit_class(self.limit_class);
        lits
    }