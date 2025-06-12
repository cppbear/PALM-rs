fn empty_literals(&self) -> Literals {
        let mut lits = Literals::empty();
        lits.set_limit_size(self.flag_literal_limit);
        lits.set_limit_class(self.flag_class_limit);
        lits
    }