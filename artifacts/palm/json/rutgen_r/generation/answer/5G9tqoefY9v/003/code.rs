// Answer 0

#[test]
fn test_skip_to_escape_valid_input_no_escape() {
    struct TestInput {
        index: usize,
        slice: Vec<u8>,
    }

    impl TestInput {
        fn new(index: usize, slice: Vec<u8>) -> Self {
            Self { index, slice }
        }
        fn skip_to_escape(&mut self, forbid_control_characters: bool) {
            // The function implementation as provided in the initial context
            if self.index == self.slice.len()
                || is_escape(self.slice[self.index], forbid_control_characters)
            {
                return;
            }
            self.index += 1;

            let rest = &self.slice[self.index..];

            if !forbid_control_characters {
                self.index += memchr::memchr2(b'"', b'\\', rest).unwrap_or(rest.len());
                return;
            }

            #[cfg(fast_arithmetic = "64")]
            type Chunk = u64;
            #[cfg(fast_arithmetic = "32")]
            type Chunk = u32;

            const STEP: usize = mem::size_of::<Chunk>();
            const ONE_BYTES: Chunk = Chunk::MAX / 255;

            for chunk in rest.chunks_exact(STEP) {
                let chars = Chunk::from_le_bytes(chunk.try_into().unwrap());
                let contains_ctrl = chars.wrapping_sub(ONE_BYTES * 0x20) & !chars;
                let chars_quote = chars ^ (ONE_BYTES * Chunk::from(b'"'));
                let contains_quote = chars_quote.wrapping_sub(ONE_BYTES) & !chars_quote;
                let chars_backslash = chars ^ (ONE_BYTES * Chunk::from(b'\\'));
                let contains_backslash = chars_backslash.wrapping_sub(ONE_BYTES) & !chars_backslash;
                let masked = (contains_ctrl | contains_quote | contains_backslash) & (ONE_BYTES << 7);
                if masked != 0 {
                    self.index = unsafe { chunk.as_ptr().offset_from(self.slice.as_ptr()) } as usize
                        + masked.trailing_zeros() as usize / 8;
                    return;
                }
            }

            self.index += rest.len() / STEP * STEP;
            self.skip_to_escape_slow();
        }
    }

    let mut input = TestInput::new(0, b"Hello, World!".to_vec());
    input.skip_to_escape(false);
    assert_eq!(input.index, 13); // Entire slice processed
}

#[test]
#[should_panic]
fn test_skip_to_escape_empty_slice() {
    struct TestInput {
        index: usize,
        slice: Vec<u8>,
    }

    impl TestInput {
        fn new(index: usize, slice: Vec<u8>) -> Self {
            Self { index, slice }
        }
        fn skip_to_escape(&mut self, forbid_control_characters: bool) {
            if self.index == self.slice.len()
                || is_escape(self.slice[self.index], forbid_control_characters)
            {
                return;
            }
            self.index += 1;

            let rest = &self.slice[self.index..];

            if !forbid_control_characters {
                self.index += memchr::memchr2(b'"', b'\\', rest).unwrap_or(rest.len());
                return;
            }

            #[cfg(fast_arithmetic = "64")]
            type Chunk = u64;
            #[cfg(fast_arithmetic = "32")]
            type Chunk = u32;

            const STEP: usize = mem::size_of::<Chunk>();
            const ONE_BYTES: Chunk = Chunk::MAX / 255;

            for chunk in rest.chunks_exact(STEP) {
                let chars = Chunk::from_le_bytes(chunk.try_into().unwrap());
                let contains_ctrl = chars.wrapping_sub(ONE_BYTES * 0x20) & !chars;
                let chars_quote = chars ^ (ONE_BYTES * Chunk::from(b'"'));
                let contains_quote = chars_quote.wrapping_sub(ONE_BYTES) & !chars_quote;
                let chars_backslash = chars ^ (ONE_BYTES * Chunk::from(b'\\'));
                let contains_backslash = chars_backslash.wrapping_sub(ONE_BYTES) & !chars_backslash;
                let masked = (contains_ctrl | contains_quote | contains_backslash) & (ONE_BYTES << 7);
                if masked != 0 {
                    self.index = unsafe { chunk.as_ptr().offset_from(self.slice.as_ptr()) } as usize
                        + masked.trailing_zeros() as usize / 8;
                    return;
                }
            }

            self.index += rest.len() / STEP * STEP;
            self.skip_to_escape_slow();
        }
    }

    let mut input = TestInput::new(0, vec![]);
    input.skip_to_escape(false);
}

#[test]
fn test_skip_to_escape_control_characters() {
    struct TestInput {
        index: usize,
        slice: Vec<u8>,
    }

    impl TestInput {
        fn new(index: usize, slice: Vec<u8>) -> Self {
            Self { index, slice }
        }
        fn skip_to_escape(&mut self, forbid_control_characters: bool) {
            if self.index == self.slice.len()
                || is_escape(self.slice[self.index], forbid_control_characters)
            {
                return;
            }
            self.index += 1;

            let rest = &self.slice[self.index..];

            if !forbid_control_characters {
                self.index += memchr::memchr2(b'"', b'\\', rest).unwrap_or(rest.len());
                return;
            }

            #[cfg(fast_arithmetic = "64")]
            type Chunk = u64;
            #[cfg(fast_arithmetic = "32")]
            type Chunk = u32;

            const STEP: usize = mem::size_of::<Chunk>();
            const ONE_BYTES: Chunk = Chunk::MAX / 255;

            for chunk in rest.chunks_exact(STEP) {
                let chars = Chunk::from_le_bytes(chunk.try_into().unwrap());
                let contains_ctrl = chars.wrapping_sub(ONE_BYTES * 0x20) & !chars;
                let chars_quote = chars ^ (ONE_BYTES * Chunk::from(b'"'));
                let contains_quote = chars_quote.wrapping_sub(ONE_BYTES) & !chars_quote;
                let chars_backslash = chars ^ (ONE_BYTES * Chunk::from(b'\\'));
                let contains_backslash = chars_backslash.wrapping_sub(ONE_BYTES) & !chars_backslash;
                let masked = (contains_ctrl | contains_quote | contains_backslash) & (ONE_BYTES << 7);
                if masked != 0 {
                    self.index = unsafe { chunk.as_ptr().offset_from(self.slice.as_ptr()) } as usize
                        + masked.trailing_zeros() as usize / 8;
                    return;
                }
            }

            self.index += rest.len() / STEP * STEP;
            self.skip_to_escape_slow();
        }
    }

    let mut input = TestInput::new(0, b"Hello\x0AWorld".to_vec()); // Contains control character (LF)
    input.skip_to_escape(true); // Forbidding control characters
    assert_eq!(input.index, 5); // Index stops before the control character
}

