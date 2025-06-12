// Answer 0

#[test]
fn test_fill_with_full_chunk() {
    struct Rng {
        seed: u64,
    }

    impl Rng {
        fn gen_u64(&self) -> u64 {
            self.seed
        }

        fn fill(&mut self, slice: &mut [u8]) {
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

    let mut rng = Rng { seed: 42 };
    let mut data = [0u8; 16]; // Multiple of 8
    rng.fill(&mut data);
    assert_eq!(data, [42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42]);
}

#[test]
fn test_fill_with_partial_chunk() {
    struct Rng {
        seed: u64,
    }

    impl Rng {
        fn gen_u64(&self) -> u64 {
            self.seed
        }

        fn fill(&mut self, slice: &mut [u8]) {
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

    let mut rng = Rng { seed: 100 };
    let mut data = [0u8; 10]; // Length not a multiple of 8
    rng.fill(&mut data);
    assert_eq!(data, [100, 100, 100, 100, 100, 100, 100, 100, 100, 100]);
}

#[should_panic]
#[test]
fn test_fill_with_empty_slice() {
    struct Rng {
        seed: u64,
    }

    impl Rng {
        fn gen_u64(&self) -> u64 {
            self.seed
        }

        fn fill(&mut self, slice: &mut [u8]) {
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

    let mut rng = Rng { seed: 200 };
    let mut data: &mut [u8] = &mut [];
    rng.fill(data); // This should panic.
}

