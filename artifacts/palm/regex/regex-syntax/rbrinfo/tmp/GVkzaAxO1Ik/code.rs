pub fn cut(&mut self) {
        for lit in &mut self.lits {
            lit.cut();
        }
    }