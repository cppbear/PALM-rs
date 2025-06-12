pub fn query(&self) -> Option<&str> {
        if self.query == NONE {
            None
        } else {
            let i = self.query + 1;
            Some(&self.data[i as usize..])
        }
    }