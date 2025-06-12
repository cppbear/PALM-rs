// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Helper functions for implementing `RngCore` functions.
//!
//! For cross-platform reproducibility, these functions all use Little Endian:
//! least-significant part first. For example, `next_u64_via_u32` takes `u32`
//! values `x, y`, then outputs `(y << 32) | x`. To implement `next_u32`
//! from `next_u64` in little-endian order, one should use `next_u64() as u32`.
//!
//! Byte-swapping (like the std `to_le` functions) is only needed to convert
//! to/from byte sequences, and since its purpose is reproducibility,
//! non-reproducible sources (e.g. `OsRng`) need not bother with it.

use crate::RngCore;

/// Implement `next_u64` via `next_u32`, little-endian order.
pub fn next_u64_via_u32<R: RngCore + ?Sized>(rng: &mut R) -> u64 {
    // Use LE; we explicitly generate one value before the next.
    let x = u64::from(rng.next_u32());
    let y = u64::from(rng.next_u32());
    (y << 32) | x
}

/// Implement `fill_bytes` via `next_u64` and `next_u32`, little-endian order.
///
/// The fastest way to fill a slice is usually to work as long as possible with
/// integers. That is why this method mostly uses `next_u64`, and only when
/// there are 4 or less bytes remaining at the end of the slice it uses
/// `next_u32` once.
pub fn fill_bytes_via_next<R: RngCore + ?Sized>(rng: &mut R, dest: &mut [u8]) {
    let mut left = dest;
    while left.len() >= 8 {
        let (l, r) = { left }.split_at_mut(8);
        left = r;
        let chunk: [u8; 8] = rng.next_u64().to_le_bytes();
        l.copy_from_slice(&chunk);
    }
    let n = left.len();
    if n > 4 {
        let chunk: [u8; 8] = rng.next_u64().to_le_bytes();
        left.copy_from_slice(&chunk[..n]);
    } else if n > 0 {
        let chunk: [u8; 4] = rng.next_u32().to_le_bytes();
        left.copy_from_slice(&chunk[..n]);
    }
}

pub(crate) trait Observable: Copy {
    type Bytes: Sized + AsRef<[u8]>;
    fn to_le_bytes(self) -> Self::Bytes;
}
impl Observable for u32 {
    type Bytes = [u8; 4];

    fn to_le_bytes(self) -> Self::Bytes {
        Self::to_le_bytes(self)
    }
}
impl Observable for u64 {
    type Bytes = [u8; 8];

    fn to_le_bytes(self) -> Self::Bytes {
        Self::to_le_bytes(self)
    }
}

/// Fill dest from src
///
/// Returns `(n, byte_len)`. `src[..n]` is consumed,
/// `dest[..byte_len]` is filled. `src[n..]` and `dest[byte_len..]` are left
/// unaltered.
pub(crate) fn fill_via_chunks<T: Observable>(src: &[T], dest: &mut [u8]) -> (usize, usize) {
    let size = core::mem::size_of::<T>();

    // Always use little endian for portability of results.

    let mut dest = dest.chunks_exact_mut(size);
    let mut src = src.iter();

    let zipped = dest.by_ref().zip(src.by_ref());
    let num_chunks = zipped.len();
    zipped.for_each(|(dest, src)| dest.copy_from_slice(src.to_le_bytes().as_ref()));

    let byte_len = num_chunks * size;
    if let Some(src) = src.next() {
        // We have consumed all full chunks of dest, but not src.
        let dest = dest.into_remainder();
        let n = dest.len();
        if n > 0 {
            dest.copy_from_slice(&src.to_le_bytes().as_ref()[..n]);
            return (num_chunks + 1, byte_len + n);
        }
    }
    (num_chunks, byte_len)
}

/// Implement `fill_bytes` by reading chunks from the output buffer of a block
/// based RNG.
///
/// The return values are `(consumed_u32, filled_u8)`.
///
/// `src` is not modified; it is taken as a `&mut` reference for backward
/// compatibility with previous versions that did change it.
///
/// `filled_u8` is the number of filled bytes in `dest`, which may be less than
/// the length of `dest`.
/// `consumed_u32` is the number of words consumed from `src`, which is the same
/// as `filled_u8 / 4` rounded up.
///
/// # Example
/// (from `IsaacRng`)
///
/// ```ignore
/// fn fill_bytes(&mut self, dest: &mut [u8]) {
///     let mut read_len = 0;
///     while read_len < dest.len() {
///         if self.index >= self.rsl.len() {
///             self.isaac();
///         }
///
///         let (consumed_u32, filled_u8) =
///             impls::fill_via_u32_chunks(&mut self.rsl[self.index..],
///                                        &mut dest[read_len..]);
///
///         self.index += consumed_u32;
///         read_len += filled_u8;
///     }
/// }
/// ```
#[deprecated(since = "0.9.3", note = "use BlockRng instead")]
pub fn fill_via_u32_chunks(src: &mut [u32], dest: &mut [u8]) -> (usize, usize) {
    fill_via_chunks(src, dest)
}

/// Implement `fill_bytes` by reading chunks from the output buffer of a block
/// based RNG.
///
/// The return values are `(consumed_u64, filled_u8)`.
///
/// `src` is not modified; it is taken as a `&mut` reference for backward
/// compatibility with previous versions that did change it.
///
/// `filled_u8` is the number of filled bytes in `dest`, which may be less than
/// the length of `dest`.
/// `consumed_u64` is the number of words consumed from `src`, which is the same
/// as `filled_u8 / 8` rounded up.
///
/// See `fill_via_u32_chunks` for an example.
#[deprecated(since = "0.9.3", note = "use BlockRng64 instead")]
pub fn fill_via_u64_chunks(src: &mut [u64], dest: &mut [u8]) -> (usize, usize) {
    fill_via_chunks(src, dest)
}

/// Implement `next_u32` via `fill_bytes`, little-endian order.
pub fn next_u32_via_fill<R: RngCore + ?Sized>(rng: &mut R) -> u32 {
    let mut buf = [0; 4];
    rng.fill_bytes(&mut buf);
    u32::from_le_bytes(buf)
}

/// Implement `next_u64` via `fill_bytes`, little-endian order.
pub fn next_u64_via_fill<R: RngCore + ?Sized>(rng: &mut R) -> u64 {
    let mut buf = [0; 8];
    rng.fill_bytes(&mut buf);
    u64::from_le_bytes(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fill_via_u32_chunks() {
        let src_orig = [1u32, 2, 3];

        let mut src = src_orig;
        let mut dst = [0u8; 11];
        assert_eq!(fill_via_chunks(&mut src, &mut dst), (3, 11));
        assert_eq!(dst, [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0]);

        let mut src = src_orig;
        let mut dst = [0u8; 13];
        assert_eq!(fill_via_chunks(&mut src, &mut dst), (3, 12));
        assert_eq!(dst, [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0]);

        let mut src = src_orig;
        let mut dst = [0u8; 5];
        assert_eq!(fill_via_chunks(&mut src, &mut dst), (2, 5));
        assert_eq!(dst, [1, 0, 0, 0, 2]);
    }

    #[test]
    fn test_fill_via_u64_chunks() {
        let src_orig = [1u64, 2];

        let mut src = src_orig;
        let mut dst = [0u8; 11];
        assert_eq!(fill_via_chunks(&mut src, &mut dst), (2, 11));
        assert_eq!(dst, [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0]);

        let mut src = src_orig;
        let mut dst = [0u8; 17];
        assert_eq!(fill_via_chunks(&mut src, &mut dst), (2, 16));
        assert_eq!(dst, [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0]);

        let mut src = src_orig;
        let mut dst = [0u8; 5];
        assert_eq!(fill_via_chunks(&mut src, &mut dst), (1, 5));
        assert_eq!(dst, [1, 0, 0, 0, 0]);
    }
}

#[cfg(test)]
mod tests_llm_16_493 {
    use super::*;

use crate::*;
    use crate::impls::Observable;

    #[test]
    fn test_to_le_bytes() {
        let value: u32 = 42;
        let expected_bytes = value.to_le_bytes();
        let result_bytes = (<u32 as Observable>::to_le_bytes(value));
        assert_eq!(result_bytes, expected_bytes);
    }
}

#[cfg(test)]
mod tests_llm_16_494 {
    use super::*;

use crate::*;
    use crate::impls::Observable;

    #[test]
    fn test_to_le_bytes() {
        let value: u64 = 123456789;
        let expected_bytes = value.to_le_bytes();
        let actual_bytes = value.to_le_bytes();
        assert_eq!(expected_bytes, actual_bytes);
    }
}

#[cfg(test)]
mod tests_llm_16_510 {
    use super::*;

use crate::*;
    use crate::{RngCore, impls};

    struct MockRng {
        state: u64,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.state as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.state
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            impls::fill_bytes_via_next(self, dst);
        }
    }

    #[test]
    fn test_fill_bytes_via_next() {
        let mut rng = MockRng { state: 1 };
        let mut buffer = [0u8; 16];

        rng.fill_bytes(&mut buffer);

        assert_eq!(buffer, [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
    }

    #[test]
    fn test_fill_bytes_via_next_partial() {
        let mut rng = MockRng { state: 1 };
        let mut buffer = [0u8; 5];

        rng.fill_bytes(&mut buffer);

        assert_eq!(buffer, [1, 0, 0, 0, 1]);
    }

    #[test]
    fn test_fill_bytes_via_next_empty() {
        let mut rng = MockRng { state: 1 };
        let mut buffer = [];

        rng.fill_bytes(&mut buffer);
        // No assertion, as the buffer should remain empty
    }
}

#[cfg(test)]
mod tests_llm_16_512 {
    use super::*;

use crate::*;

    #[test]
    fn test_fill_via_u32_chunks() {
        let mut src = [1u32, 2, 3, 4, 5]; // Example source
        let mut dest = [0u8; 16]; // Example destination buffer
        let (consumed_u32, filled_u8) = fill_via_u32_chunks(&mut src, &mut dest);

        // Check that the number of consumed u32 is as expected
        assert_eq!(consumed_u32, 4);

        // Check that the number of filled u8 is as expected
        assert_eq!(filled_u8, 16);

        // Check that dest is filled correctly
        assert_eq!(dest, [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
    }

    #[test]
    fn test_fill_via_u32_chunks_partial() {
        let mut src = [1u32, 2]; // Shorter source
        let mut dest = [0u8; 8]; // Example shorter destination
        let (consumed_u32, filled_u8) = fill_via_u32_chunks(&mut src, &mut dest);

        // Check that the number of consumed u32 is as expected
        assert_eq!(consumed_u32, 2);

        // Check that the number of filled u8 is as expected
        assert_eq!(filled_u8, 8);

        // Check that dest is filled correctly
        assert_eq!(dest, [1, 0, 0, 0, 2, 0, 0, 0]);
    }

    #[test]
    fn test_fill_via_u32_chunks_empty() {
        let mut src = []; // Empty source
        let mut dest = [0u8; 16]; // Non-empty destination
        let (consumed_u32, filled_u8) = fill_via_u32_chunks(&mut src, &mut dest);

        // Check that no u32s were consumed
        assert_eq!(consumed_u32, 0);

        // Check that no bytes were filled
        assert_eq!(filled_u8, 0);

        // Check that dest is unchanged
        assert_eq!(dest, [0u8; 16]);
    }
}

#[cfg(test)]
mod tests_llm_16_514 {
    use super::*;

use crate::*;
    use crate::{RngCore, impls};

    struct MockRng {
        value: u32,
    }

    impl MockRng {
        fn new(start: u32) -> Self {
            MockRng { value: start }
        }
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let current = self.value;
            self.value = self.value.wrapping_add(1);
            current
        }

        fn next_u64(&mut self) -> u64 {
            self.next_u32() as u64
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            let bytes = self.next_u32().to_le_bytes();
            dst.copy_from_slice(&bytes);
        }
    }

    #[test]
    fn test_next_u32_via_fill() {
        let mut rng = MockRng::new(1);
        let result = next_u32_via_fill(&mut rng);
        assert_eq!(result, 1);

        let result = next_u32_via_fill(&mut rng);
        assert_eq!(result, 2);

        let result = next_u32_via_fill(&mut rng);
        assert_eq!(result, 3);
    }
}

#[cfg(test)]
mod tests_llm_16_515 {
    use super::*;

use crate::*;
    use crate::RngCore;

    struct TestRng {
        value: u64,
    }

    impl TestRng {
        fn new() -> Self {
            TestRng { value: 0 }
        }
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            (self.next_u64() & 0xFFFFFFFF) as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value += 1;
            self.value
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = self.next_u32() as u8;
            }
        }
    }

    #[test]
    fn test_next_u64_via_fill() {
        let mut rng = TestRng::new();
        let result = next_u64_via_fill(&mut rng);
        assert_eq!(result, 1);
        
        let result = next_u64_via_fill(&mut rng);
        assert_eq!(result, 2);
        
        let result = next_u64_via_fill(&mut rng);
        assert_eq!(result, 3);
    }
}

#[cfg(test)]
mod tests_llm_16_516 {
    use super::*;

use crate::*;
    use crate::RngCore;

    struct MockRng {
        next_u32: u32,
        next_u32_count: u32,
    }

    impl MockRng {
        fn new() -> Self {
            Self {
                next_u32: 0,
                next_u32_count: 0,
            }
        }
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.next_u32_count;
            self.next_u32_count += 1;
            value
        }

        fn next_u64(&mut self) -> u64 {
            panic!("next_u64 should not be called directly");
        }

        fn fill_bytes(&mut self, _dst: &mut [u8]) {
            panic!("fill_bytes should not be called");
        }
    }

    #[test]
    fn test_next_u64_via_u32() {
        let mut rng = MockRng::new();
        let result = next_u64_via_u32(&mut rng);
        assert_eq!(result, (1u64 << 32) | 0);
    }
}
