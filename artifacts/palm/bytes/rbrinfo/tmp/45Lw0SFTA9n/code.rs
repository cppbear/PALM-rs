pub fn split_to(&mut self, at: usize) -> Self {
        if at == self.len() {
            let end_ptr = self.ptr.wrapping_add(at);
            return mem::replace(self, Bytes::new_empty_with_ptr(end_ptr));
        }

        if at == 0 {
            return Bytes::new_empty_with_ptr(self.ptr);
        }

        assert!(
            at <= self.len(),
            "split_to out of bounds: {:?} <= {:?}",
            at,
            self.len(),
        );

        let mut ret = self.clone();

        unsafe { self.inc_start(at) };

        ret.len = at;
        ret
    }