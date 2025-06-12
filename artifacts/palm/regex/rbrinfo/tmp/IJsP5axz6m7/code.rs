fn replace_append(&mut self, _: &Captures, dst: &mut String) {
        dst.push_str(self.0);
    }