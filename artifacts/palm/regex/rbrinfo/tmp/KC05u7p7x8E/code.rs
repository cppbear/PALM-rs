fn replace_append(&mut self, caps: &Captures, dst: &mut String) {
        dst.push_str(&(*self)(caps));
    }