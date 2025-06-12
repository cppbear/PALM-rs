fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut read_len = 0;
        self.half_used = false;
        while read_len < dest.len() {
            if self.index >= self.results.as_ref().len() {
                self.core.generate(&mut self.results);
                self.index = 0;
            }

            let (consumed_u64, filled_u8) =
                fill_via_chunks(&self.results.as_mut()[self.index..], &mut dest[read_len..]);

            self.index += consumed_u64;
            read_len += filled_u8;
        }
    }