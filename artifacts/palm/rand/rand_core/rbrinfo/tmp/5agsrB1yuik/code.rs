fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut read_len = 0;
        while read_len < dest.len() {
            if self.index >= self.results.as_ref().len() {
                self.generate_and_set(0);
            }
            let (consumed_u32, filled_u8) =
                fill_via_chunks(&self.results.as_mut()[self.index..], &mut dest[read_len..]);

            self.index += consumed_u32;
            read_len += filled_u8;
        }
    }