fn next_u64(&mut self) -> u64 {
        let res = self.v;
        self.v = self.v.wrapping_add(self.a);
        res
    }