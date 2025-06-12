// Answer 0

#[test]
fn test_fill_non_empty_remainder() {
    struct Random {
        state: u64,
    }

    impl Random {
        pub fn new(state: u64) -> Self {
            Random { state }
        }

        pub fn gen_u64(&mut self) -> u64 {
            self.state += 1;
            self.state
        }

        pub fn fill(&mut self, slice: &mut [u8]) {
            let mut chunks = slice.chunks_exact_mut(core::mem::size_of::<u64>());
            for chunk in chunks.by_ref() {
                let n = self.gen_u64().to_ne_bytes();
                chunk.copy_from_slice(&n);
            }

            let remainder = chunks.into_remainder();
            if !remainder.is_empty() {
                let n = self.gen_u64().to_ne_bytes();
                remainder.copy_from_slice(&n[..remainder.len()]);
            }
        }
    }

    let mut random = Random::new(0);
    let mut buffer = [0u8; 9]; // 8 bytes for one full u64, and 1 byte for remainder
    random.fill(&mut buffer);

    // Check that the first 8 bytes are filled with incremented values
    assert_eq!(&buffer[0..8], &0u64.to_ne_bytes());
    // Check that the last byte is filled from the next generated value
    assert_eq!(buffer[8], 1u64.to_ne_bytes()[0]);
}

#[test]
#[should_panic]
fn test_fill_panic_on_chunk_split() {
    struct Random {
        state: u64,
    }

    impl Random {
        pub fn new(state: u64) -> Self {
            Random { state }
        }

        pub fn gen_u64(&mut self) -> u64 {
            self.state += 1;
            self.state
        }

        pub fn fill(&mut self, slice: &mut [u8]) {
            let mut chunks = slice.chunks_exact_mut(core::mem::size_of::<u64>());
            for chunk in chunks.by_ref() {
                let n = self.gen_u64().to_ne_bytes();
                chunk.copy_from_slice(&n);
            }

            let remainder = chunks.into_remainder();
            if !remainder.is_empty() {
                let n = self.gen_u64().to_ne_bytes();
                remainder.copy_from_slice(&n[..remainder.len()]);
            }
        }
    }

    let mut random = Random::new(0);
    let mut invalid_buffer = [0u8; 7]; // Buffer that cannot be chunked into 8 bytes
    random.fill(&mut invalid_buffer); // This should panic
}

#[test]
fn test_fill_exact_chunk_size() {
    struct Random {
        state: u64,
    }

    impl Random {
        pub fn new(state: u64) -> Self {
            Random { state }
        }

        pub fn gen_u64(&mut self) -> u64 {
            self.state += 1;
            self.state
        }

        pub fn fill(&mut self, slice: &mut [u8]) {
            let mut chunks = slice.chunks_exact_mut(core::mem::size_of::<u64>());
            for chunk in chunks.by_ref() {
                let n = self.gen_u64().to_ne_bytes();
                chunk.copy_from_slice(&n);
            }

            let remainder = chunks.into_remainder();
            if !remainder.is_empty() {
                let n = self.gen_u64().to_ne_bytes();
                remainder.copy_from_slice(&n[..remainder.len()]);
            }
        }
    }

    let mut random = Random::new(0);
    let mut buffer = [0u8; 8]; // Exactly one u64
    random.fill(&mut buffer);

    // Check that the buffer is filled with incremented values
    assert_eq!(buffer, 0u64.to_ne_bytes());
}

