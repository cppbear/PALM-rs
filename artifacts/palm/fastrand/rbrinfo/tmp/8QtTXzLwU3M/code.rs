pub fn fill(&mut self, slice: &mut [u8]) {
        // We fill the slice by chunks of 8 bytes, or one block of
        // WyRand output per new state.
        let mut chunks = slice.chunks_exact_mut(core::mem::size_of::<u64>());
        for chunk in chunks.by_ref() {
            let n = self.gen_u64().to_ne_bytes();
            // Safe because the chunks are always 8 bytes exactly.
            chunk.copy_from_slice(&n);
        }

        let remainder = chunks.into_remainder();

        // Any remainder will always be less than 8 bytes.
        if !remainder.is_empty() {
            // Generate one last block of 8 bytes of entropy
            let n = self.gen_u64().to_ne_bytes();

            // Use the remaining length to copy from block
            remainder.copy_from_slice(&n[..remainder.len()]);
        }
    }