fn put_uint_le(&mut self, n: u64, nbytes: usize) {
        let slice = n.to_le_bytes();
        let slice = match slice.get(..nbytes) {
            Some(slice) => slice,
            None => panic_does_not_fit(nbytes, slice.len()),
        };

        self.put_slice(slice);
    }