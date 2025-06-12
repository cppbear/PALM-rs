// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The ChaCha random number generator.

use crate::guts::ChaCha;
use core::fmt;
use rand_core::block::{BlockRng, BlockRngCore, CryptoBlockRng};
use rand_core::{CryptoRng, RngCore, SeedableRng};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// NB. this must remain consistent with some currently hard-coded numbers in this module
const BUF_BLOCKS: u8 = 4;
// number of 32-bit words per ChaCha block (fixed by algorithm definition)
const BLOCK_WORDS: u8 = 16;

#[repr(transparent)]
pub struct Array64<T>([T; 64]);
impl<T> Default for Array64<T>
where
    T: Default,
{
    #[rustfmt::skip]
    fn default() -> Self {
        Self([
            T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(),
            T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(),
            T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(),
            T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(),
            T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(),
            T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(),
            T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(),
            T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(), T::default(),
        ])
    }
}
impl<T> AsRef<[T]> for Array64<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}
impl<T> AsMut<[T]> for Array64<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}
impl<T> Clone for Array64<T>
where
    T: Copy + Default,
{
    fn clone(&self) -> Self {
        let mut new = Self::default();
        new.0.copy_from_slice(&self.0);
        new
    }
}
impl<T> fmt::Debug for Array64<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Array64 {{}}")
    }
}

macro_rules! chacha_impl {
    ($ChaChaXCore:ident, $ChaChaXRng:ident, $rounds:expr, $doc:expr, $abst:ident,) => {
        #[doc=$doc]
        #[derive(Clone, PartialEq, Eq)]
        pub struct $ChaChaXCore {
            state: ChaCha,
        }

        // Custom Debug implementation that does not expose the internal state
        impl fmt::Debug for $ChaChaXCore {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "ChaChaXCore {{}}")
            }
        }

        impl BlockRngCore for $ChaChaXCore {
            type Item = u32;
            type Results = Array64<u32>;

            #[inline]
            fn generate(&mut self, r: &mut Self::Results) {
                self.state.refill4($rounds, &mut r.0);
            }
        }

        impl SeedableRng for $ChaChaXCore {
            type Seed = [u8; 32];

            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                $ChaChaXCore {
                    state: ChaCha::new(&seed, &[0u8; 8]),
                }
            }
        }

        impl CryptoBlockRng for $ChaChaXCore {}

        /// A cryptographically secure random number generator that uses the ChaCha algorithm.
        ///
        /// ChaCha is a stream cipher designed by Daniel J. Bernstein[^1], that we use as an RNG. It is
        /// an improved variant of the Salsa20 cipher family, which was selected as one of the "stream
        /// ciphers suitable for widespread adoption" by eSTREAM[^2].
        ///
        /// ChaCha uses add-rotate-xor (ARX) operations as its basis. These are safe against timing
        /// attacks, although that is mostly a concern for ciphers and not for RNGs. We provide a SIMD
        /// implementation to support high throughput on a variety of common hardware platforms.
        ///
        /// With the ChaCha algorithm it is possible to choose the number of rounds the core algorithm
        /// should run. The number of rounds is a tradeoff between performance and security, where 8
        /// rounds is the minimum potentially secure configuration, and 20 rounds is widely used as a
        /// conservative choice.
        ///
        /// We use a 64-bit counter and 64-bit stream identifier as in Bernstein's implementation[^1]
        /// except that we use a stream identifier in place of a nonce. A 64-bit counter over 64-byte
        /// (16 word) blocks allows 1 ZiB of output before cycling, and the stream identifier allows
        /// 2<sup>64</sup> unique streams of output per seed. Both counter and stream are initialized
        /// to zero but may be set via the `set_word_pos` and `set_stream` methods.
        ///
        /// The word layout is:
        ///
        /// ```text
        /// constant  constant  constant  constant
        /// seed      seed      seed      seed
        /// seed      seed      seed      seed
        /// counter   counter   stream_id stream_id
        /// ```
        ///
        /// This implementation uses an output buffer of sixteen `u32` words, and uses
        /// [`BlockRng`] to implement the [`RngCore`] methods.
        ///
        /// [^1]: D. J. Bernstein, [*ChaCha, a variant of Salsa20*](
        ///       https://cr.yp.to/chacha.html)
        ///
        /// [^2]: [eSTREAM: the ECRYPT Stream Cipher Project](
        ///       http://www.ecrypt.eu.org/stream/)
        #[derive(Clone, Debug)]
        pub struct $ChaChaXRng {
            rng: BlockRng<$ChaChaXCore>,
        }

        impl SeedableRng for $ChaChaXRng {
            type Seed = [u8; 32];

            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                let core = $ChaChaXCore::from_seed(seed);
                Self {
                    rng: BlockRng::new(core),
                }
            }
        }

        impl RngCore for $ChaChaXRng {
            #[inline]
            fn next_u32(&mut self) -> u32 {
                self.rng.next_u32()
            }

            #[inline]
            fn next_u64(&mut self) -> u64 {
                self.rng.next_u64()
            }

            #[inline]
            fn fill_bytes(&mut self, bytes: &mut [u8]) {
                self.rng.fill_bytes(bytes)
            }
        }

        impl $ChaChaXRng {
            // The buffer is a 4-block window, i.e. it is always at a block-aligned position in the
            // stream but if the stream has been sought it may not be self-aligned.

            /// Get the offset from the start of the stream, in 32-bit words.
            ///
            /// Since the generated blocks are 16 words (2<sup>4</sup>) long and the
            /// counter is 64-bits, the offset is a 68-bit number. Sub-word offsets are
            /// not supported, hence the result can simply be multiplied by 4 to get a
            /// byte-offset.
            #[inline]
            pub fn get_word_pos(&self) -> u128 {
                let buf_start_block = {
                    let buf_end_block = self.rng.core.state.get_block_pos();
                    u64::wrapping_sub(buf_end_block, BUF_BLOCKS.into())
                };
                let (buf_offset_blocks, block_offset_words) = {
                    let buf_offset_words = self.rng.index() as u64;
                    let blocks_part = buf_offset_words / u64::from(BLOCK_WORDS);
                    let words_part = buf_offset_words % u64::from(BLOCK_WORDS);
                    (blocks_part, words_part)
                };
                let pos_block = u64::wrapping_add(buf_start_block, buf_offset_blocks);
                let pos_block_words = u128::from(pos_block) * u128::from(BLOCK_WORDS);
                pos_block_words + u128::from(block_offset_words)
            }

            /// Set the offset from the start of the stream, in 32-bit words.
            ///
            /// As with `get_word_pos`, we use a 68-bit number. Since the generator
            /// simply cycles at the end of its period (1 ZiB), we ignore the upper
            /// 60 bits.
            #[inline]
            pub fn set_word_pos(&mut self, word_offset: u128) {
                let block = (word_offset / u128::from(BLOCK_WORDS)) as u64;
                self.rng.core.state.set_block_pos(block);
                self.rng
                    .generate_and_set((word_offset % u128::from(BLOCK_WORDS)) as usize);
            }

            /// Set the stream number.
            ///
            /// This is initialized to zero; 2<sup>64</sup> unique streams of output
            /// are available per seed/key.
            ///
            /// Note that in order to reproduce ChaCha output with a specific 64-bit
            /// nonce, one can convert that nonce to a `u64` in little-endian fashion
            /// and pass to this function. In theory a 96-bit nonce can be used by
            /// passing the last 64-bits to this function and using the first 32-bits as
            /// the most significant half of the 64-bit counter (which may be set
            /// indirectly via `set_word_pos`), but this is not directly supported.
            #[inline]
            pub fn set_stream(&mut self, stream: u64) {
                self.rng.core.state.set_nonce(stream);
                if self.rng.index() != 64 {
                    let wp = self.get_word_pos();
                    self.set_word_pos(wp);
                }
            }

            /// Get the stream number.
            #[inline]
            pub fn get_stream(&self) -> u64 {
                self.rng.core.state.get_nonce()
            }

            /// Get the seed.
            #[inline]
            pub fn get_seed(&self) -> [u8; 32] {
                self.rng.core.state.get_seed()
            }
        }

        impl CryptoRng for $ChaChaXRng {}

        impl From<$ChaChaXCore> for $ChaChaXRng {
            fn from(core: $ChaChaXCore) -> Self {
                $ChaChaXRng {
                    rng: BlockRng::new(core),
                }
            }
        }

        impl PartialEq<$ChaChaXRng> for $ChaChaXRng {
            fn eq(&self, rhs: &$ChaChaXRng) -> bool {
                let a: $abst::$ChaChaXRng = self.into();
                let b: $abst::$ChaChaXRng = rhs.into();
                a == b
            }
        }
        impl Eq for $ChaChaXRng {}

        #[cfg(feature = "serde")]
        impl Serialize for $ChaChaXRng {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                $abst::$ChaChaXRng::from(self).serialize(s)
            }
        }
        #[cfg(feature = "serde")]
        impl<'de> Deserialize<'de> for $ChaChaXRng {
            fn deserialize<D>(d: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                $abst::$ChaChaXRng::deserialize(d).map(|x| Self::from(&x))
            }
        }

        mod $abst {
            #[cfg(feature = "serde")]
            use serde::{Deserialize, Serialize};

            // The abstract state of a ChaCha stream, independent of implementation choices. The
            // comparison and serialization of this object is considered a semver-covered part of
            // the API.
            #[derive(Debug, PartialEq, Eq)]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            pub(crate) struct $ChaChaXRng {
                seed: [u8; 32],
                stream: u64,
                word_pos: u128,
            }

            impl From<&super::$ChaChaXRng> for $ChaChaXRng {
                // Forget all information about the input except what is necessary to determine the
                // outputs of any sequence of pub API calls.
                fn from(r: &super::$ChaChaXRng) -> Self {
                    Self {
                        seed: r.get_seed(),
                        stream: r.get_stream(),
                        word_pos: r.get_word_pos(),
                    }
                }
            }

            impl From<&$ChaChaXRng> for super::$ChaChaXRng {
                // Construct one of the possible concrete RNGs realizing an abstract state.
                fn from(a: &$ChaChaXRng) -> Self {
                    use rand_core::SeedableRng;
                    let mut r = Self::from_seed(a.seed);
                    r.set_stream(a.stream);
                    r.set_word_pos(a.word_pos);
                    r
                }
            }
        }
    };
}

chacha_impl!(
    ChaCha20Core,
    ChaCha20Rng,
    10,
    "ChaCha with 20 rounds",
    abstract20,
);
chacha_impl!(
    ChaCha12Core,
    ChaCha12Rng,
    6,
    "ChaCha with 12 rounds",
    abstract12,
);
chacha_impl!(
    ChaCha8Core,
    ChaCha8Rng,
    4,
    "ChaCha with 8 rounds",
    abstract8,
);

#[cfg(test)]
mod test {
    use rand_core::{RngCore, SeedableRng};

    #[cfg(feature = "serde")]
    use super::{ChaCha12Rng, ChaCha20Rng, ChaCha8Rng};

    type ChaChaRng = super::ChaCha20Rng;

    #[cfg(feature = "serde")]
    #[test]
    fn test_chacha_serde_roundtrip() {
        let seed = [
            1, 0, 52, 0, 0, 0, 0, 0, 1, 0, 10, 0, 22, 32, 0, 0, 2, 0, 55, 49, 0, 11, 0, 0, 3, 0, 0,
            0, 0, 0, 2, 92,
        ];
        let mut rng1 = ChaCha20Rng::from_seed(seed);
        let mut rng2 = ChaCha12Rng::from_seed(seed);
        let mut rng3 = ChaCha8Rng::from_seed(seed);

        let encoded1 = serde_json::to_string(&rng1).unwrap();
        let encoded2 = serde_json::to_string(&rng2).unwrap();
        let encoded3 = serde_json::to_string(&rng3).unwrap();

        let mut decoded1: ChaCha20Rng = serde_json::from_str(&encoded1).unwrap();
        let mut decoded2: ChaCha12Rng = serde_json::from_str(&encoded2).unwrap();
        let mut decoded3: ChaCha8Rng = serde_json::from_str(&encoded3).unwrap();

        assert_eq!(rng1, decoded1);
        assert_eq!(rng2, decoded2);
        assert_eq!(rng3, decoded3);

        assert_eq!(rng1.next_u32(), decoded1.next_u32());
        assert_eq!(rng2.next_u32(), decoded2.next_u32());
        assert_eq!(rng3.next_u32(), decoded3.next_u32());
    }

    // This test validates that:
    // 1. a hard-coded serialization demonstrating the format at time of initial release can still
    //    be deserialized to a ChaChaRng
    // 2. re-serializing the resultant object produces exactly the original string
    //
    // Condition 2 is stronger than necessary: an equivalent serialization (e.g. with field order
    // permuted, or whitespace differences) would also be admissible, but would fail this test.
    // However testing for equivalence of serialized data is difficult, and there shouldn't be any
    // reason we need to violate the stronger-than-needed condition, e.g. by changing the field
    // definition order.
    #[cfg(feature = "serde")]
    #[test]
    fn test_chacha_serde_format_stability() {
        let j = r#"{"seed":[4,8,15,16,23,42,4,8,15,16,23,42,4,8,15,16,23,42,4,8,15,16,23,42,4,8,15,16,23,42,4,8],"stream":27182818284,"word_pos":314159265359}"#;
        let r: ChaChaRng = serde_json::from_str(j).unwrap();
        let j1 = serde_json::to_string(&r).unwrap();
        assert_eq!(j, j1);
    }

    #[test]
    fn test_chacha_construction() {
        let seed = [
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0,
            0, 0, 0,
        ];
        let mut rng1 = ChaChaRng::from_seed(seed);
        assert_eq!(rng1.next_u32(), 137206642);

        let mut rng2 = ChaChaRng::from_rng(&mut rng1);
        assert_eq!(rng2.next_u32(), 1325750369);
    }

    #[test]
    fn test_chacha_true_values_a() {
        // Test vectors 1 and 2 from
        // https://tools.ietf.org/html/draft-nir-cfrg-chacha20-poly1305-04
        let seed = [0u8; 32];
        let mut rng = ChaChaRng::from_seed(seed);

        let mut results = [0u32; 16];
        for i in results.iter_mut() {
            *i = rng.next_u32();
        }
        let expected = [
            0xade0b876, 0x903df1a0, 0xe56a5d40, 0x28bd8653, 0xb819d2bd, 0x1aed8da0, 0xccef36a8,
            0xc70d778b, 0x7c5941da, 0x8d485751, 0x3fe02477, 0x374ad8b8, 0xf4b8436a, 0x1ca11815,
            0x69b687c3, 0x8665eeb2,
        ];
        assert_eq!(results, expected);

        for i in results.iter_mut() {
            *i = rng.next_u32();
        }
        let expected = [
            0xbee7079f, 0x7a385155, 0x7c97ba98, 0x0d082d73, 0xa0290fcb, 0x6965e348, 0x3e53c612,
            0xed7aee32, 0x7621b729, 0x434ee69c, 0xb03371d5, 0xd539d874, 0x281fed31, 0x45fb0a51,
            0x1f0ae1ac, 0x6f4d794b,
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_chacha_true_values_b() {
        // Test vector 3 from
        // https://tools.ietf.org/html/draft-nir-cfrg-chacha20-poly1305-04
        let seed = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ];
        let mut rng = ChaChaRng::from_seed(seed);

        // Skip block 0
        for _ in 0..16 {
            rng.next_u32();
        }

        let mut results = [0u32; 16];
        for i in results.iter_mut() {
            *i = rng.next_u32();
        }
        let expected = [
            0x2452eb3a, 0x9249f8ec, 0x8d829d9b, 0xddd4ceb1, 0xe8252083, 0x60818b01, 0xf38422b8,
            0x5aaa49c9, 0xbb00ca8e, 0xda3ba7b4, 0xc4b592d1, 0xfdf2732f, 0x4436274e, 0x2561b3c8,
            0xebdd4aa6, 0xa0136c00,
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_chacha_true_values_c() {
        // Test vector 4 from
        // https://tools.ietf.org/html/draft-nir-cfrg-chacha20-poly1305-04
        let seed = [
            0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];
        let expected = [
            0xfb4dd572, 0x4bc42ef1, 0xdf922636, 0x327f1394, 0xa78dea8f, 0x5e269039, 0xa1bebbc1,
            0xcaf09aae, 0xa25ab213, 0x48a6b46c, 0x1b9d9bcb, 0x092c5be6, 0x546ca624, 0x1bec45d5,
            0x87f47473, 0x96f0992e,
        ];
        let expected_end = 3 * 16;
        let mut results = [0u32; 16];

        // Test block 2 by skipping block 0 and 1
        let mut rng1 = ChaChaRng::from_seed(seed);
        for _ in 0..32 {
            rng1.next_u32();
        }
        for i in results.iter_mut() {
            *i = rng1.next_u32();
        }
        assert_eq!(results, expected);
        assert_eq!(rng1.get_word_pos(), expected_end);

        // Test block 2 by using `set_word_pos`
        let mut rng2 = ChaChaRng::from_seed(seed);
        rng2.set_word_pos(2 * 16);
        for i in results.iter_mut() {
            *i = rng2.next_u32();
        }
        assert_eq!(results, expected);
        assert_eq!(rng2.get_word_pos(), expected_end);

        // Test skipping behaviour with other types
        let mut buf = [0u8; 32];
        rng2.fill_bytes(&mut buf[..]);
        assert_eq!(rng2.get_word_pos(), expected_end + 8);
        rng2.fill_bytes(&mut buf[0..25]);
        assert_eq!(rng2.get_word_pos(), expected_end + 15);
        rng2.next_u64();
        assert_eq!(rng2.get_word_pos(), expected_end + 17);
        rng2.next_u32();
        rng2.next_u64();
        assert_eq!(rng2.get_word_pos(), expected_end + 20);
        rng2.fill_bytes(&mut buf[0..1]);
        assert_eq!(rng2.get_word_pos(), expected_end + 21);
    }

    #[test]
    fn test_chacha_multiple_blocks() {
        let seed = [
            0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7,
            0, 0, 0,
        ];
        let mut rng = ChaChaRng::from_seed(seed);

        // Store the 17*i-th 32-bit word,
        // i.e., the i-th word of the i-th 16-word block
        let mut results = [0u32; 16];
        for i in results.iter_mut() {
            *i = rng.next_u32();
            for _ in 0..16 {
                rng.next_u32();
            }
        }
        let expected = [
            0xf225c81a, 0x6ab1be57, 0x04d42951, 0x70858036, 0x49884684, 0x64efec72, 0x4be2d186,
            0x3615b384, 0x11cfa18e, 0xd3c50049, 0x75c775f6, 0x434c6530, 0x2c5bad8f, 0x898881dc,
            0x5f1c86d9, 0xc1f8e7f4,
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_chacha_true_bytes() {
        let seed = [0u8; 32];
        let mut rng = ChaChaRng::from_seed(seed);
        let mut results = [0u8; 32];
        rng.fill_bytes(&mut results);
        let expected = [
            118, 184, 224, 173, 160, 241, 61, 144, 64, 93, 106, 229, 83, 134, 189, 40, 189, 210,
            25, 184, 160, 141, 237, 26, 168, 54, 239, 204, 139, 119, 13, 199,
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_chacha_nonce() {
        // Test vector 5 from
        // https://tools.ietf.org/html/draft-nir-cfrg-chacha20-poly1305-04
        // Although we do not support setting a nonce, we try it here anyway so
        // we can use this test vector.
        let seed = [0u8; 32];
        let mut rng = ChaChaRng::from_seed(seed);
        // 96-bit nonce in LE order is: 0,0,0,0, 0,0,0,0, 0,0,0,2
        rng.set_stream(2u64 << (24 + 32));

        let mut results = [0u32; 16];
        for i in results.iter_mut() {
            *i = rng.next_u32();
        }
        let expected = [
            0x374dc6c2, 0x3736d58c, 0xb904e24a, 0xcd3f93ef, 0x88228b1a, 0x96a4dfb3, 0x5b76ab72,
            0xc727ee54, 0x0e0e978a, 0xf3145c95, 0x1b748ea8, 0xf786c297, 0x99c28f5f, 0x628314e8,
            0x398a19fa, 0x6ded1b53,
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_chacha_clone_streams() {
        let seed = [
            0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7,
            0, 0, 0,
        ];
        let mut rng = ChaChaRng::from_seed(seed);
        let mut clone = rng.clone();
        for _ in 0..16 {
            assert_eq!(rng.next_u64(), clone.next_u64());
        }

        rng.set_stream(51);
        for _ in 0..7 {
            assert!(rng.next_u32() != clone.next_u32());
        }
        clone.set_stream(51); // switch part way through block
        for _ in 7..16 {
            assert_eq!(rng.next_u32(), clone.next_u32());
        }
    }

    #[test]
    fn test_chacha_word_pos_wrap_exact() {
        use super::{BLOCK_WORDS, BUF_BLOCKS};
        let mut rng = ChaChaRng::from_seed(Default::default());
        // refilling the buffer in set_word_pos will wrap the block counter to 0
        let last_block = (1 << 68) - u128::from(BUF_BLOCKS * BLOCK_WORDS);
        rng.set_word_pos(last_block);
        assert_eq!(rng.get_word_pos(), last_block);
    }

    #[test]
    fn test_chacha_word_pos_wrap_excess() {
        use super::BLOCK_WORDS;
        let mut rng = ChaChaRng::from_seed(Default::default());
        // refilling the buffer in set_word_pos will wrap the block counter past 0
        let last_block = (1 << 68) - u128::from(BLOCK_WORDS);
        rng.set_word_pos(last_block);
        assert_eq!(rng.get_word_pos(), last_block);
    }

    #[test]
    fn test_chacha_word_pos_zero() {
        let mut rng = ChaChaRng::from_seed(Default::default());
        assert_eq!(rng.get_word_pos(), 0);
        rng.set_word_pos(0);
        assert_eq!(rng.get_word_pos(), 0);
    }

    #[test]
    fn test_trait_objects() {
        use rand_core::CryptoRng;

        let mut rng1 = ChaChaRng::from_seed(Default::default());
        let rng2 = &mut rng1.clone() as &mut dyn CryptoRng;
        for _ in 0..1000 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }
}

#[cfg(test)]
mod tests_llm_16_1 {
    use super::*;

use crate::*;
    use crate::chacha::Array64;

    #[test]
    fn test_clone() {
        // Setup
        let original = Array64([1u8; 64]); // Array filled with 1s
        let cloned = original.clone();

        // Assert that the original and cloned contents are the same
        assert_eq!(original.as_ref(), cloned.as_ref());

        // Assert that the original and cloned are different instances
        assert!(!std::ptr::eq(&original, &cloned));
    }

    #[test]
    fn test_clone_with_default() {
        // Setup
        let original = Array64::<u8>::default(); // Array with default values (0s)
        let cloned = original.clone();

        // Assert that the original and cloned contents are the same
        assert_eq!(original.as_ref(), cloned.as_ref());

        // Assert that the original and cloned are different instances
        assert!(!std::ptr::eq(&original, &cloned));
    }

    #[test]
    fn test_clone_empty() {
        // Setup
        let original = Array64::<u8>([0; 64]); // Array filled with 0s
        let cloned = original.clone();

        // Assert that the original and cloned contents are the same
        assert_eq!(original.as_ref(), cloned.as_ref());

        // Assert that the original and cloned are different instances
        assert!(!std::ptr::eq(&original, &cloned));
    }
}

#[cfg(test)]
mod tests_llm_16_2 {
    use super::*;

use crate::*;
    use crate::chacha::Array64;

    #[test]
    fn test_as_mut() {
        let mut array = Array64([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64]);
        
        let slice: &mut [i32] = array.as_mut();
        slice[0] = 42;

        assert_eq!(array.0[0], 42);
    }
}

#[cfg(test)]
mod tests_llm_16_3 {
    use super::*;

use crate::*;
    use crate::chacha::Array64;

    #[test]
    fn test_as_ref() {
        let array = Array64([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                             17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
                             29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                             41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
                             53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64]);
        let slice: &[i32] = array.as_ref();
        assert_eq!(slice.len(), 64);
        assert_eq!(slice[0], 1);
        assert_eq!(slice[63], 64);
    }

    #[test]
    fn test_as_ref_empty() {
        let array: Array64<u8> = Array64([0; 64]);
        let slice: &[u8] = array.as_ref();
        assert_eq!(slice.len(), 64);
        assert!(slice.iter().all(|&x| x == 0));
    }
}

#[cfg(test)]
mod tests_llm_16_4 {
    use super::*;

use crate::*;
    use std::default::Default;

    #[test]
    fn test_array64_default() {
        let array: Array64<u32> = Default::default();
        let expected = Array64([0u32; 64]);
        assert_eq!(array.as_ref(), expected.as_ref());

        let array_f64: Array64<f64> = Default::default();
        let expected_f64 = Array64([0.0; 64]);
        assert_eq!(array_f64.as_ref(), expected_f64.as_ref());
    }
}

#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_from_seed() {
        let seed: [u8; 32] = [0; 32]; // Example seed for testing
        let rng = chacha::ChaCha12Core::from_seed(seed);
        
        // Assert properties of the rng, such as it being initialized
        assert_eq!(rng.state.get_seed(), seed);
    }
}

#[cfg(test)]
mod tests_llm_16_14 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_from_cha_cha_12_core() {
        let seed: [u8; 32] = [0u8; 32]; // Replace with a test seed if needed
        let core = chacha::ChaCha12Core::from_seed(seed);
        let rng: chacha::ChaCha12Rng = chacha::ChaCha12Rng::from(core);
        
        assert_eq!(rng.get_seed(), seed);
    }

    #[test]
    fn test_from_with_different_seed() {
        let seed1: [u8; 32] = [1u8; 32];
        let seed2: [u8; 32] = [2u8; 32];
        let core1 = chacha::ChaCha12Core::from_seed(seed1);
        let core2 = chacha::ChaCha12Core::from_seed(seed2);

        let rng1: chacha::ChaCha12Rng = chacha::ChaCha12Rng::from(core1);
        let rng2: chacha::ChaCha12Rng = chacha::ChaCha12Rng::from(core2);

        assert_ne!(rng1.get_seed(), rng2.get_seed());
    }
}

#[cfg(test)]
mod tests_llm_16_15 {
    use super::*;

use crate::*;
    use crate::ChaCha20Core;

    #[test]
    fn test_from_seed() {
        let seed: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        
        let rng = ChaCha20Core::from_seed(seed);
        
        // Check if the rng state is correctly initialized
        assert_eq!(rng.state.get_seed(), seed);
    }
}

#[cfg(test)]
mod tests_llm_16_18 {
    use super::*;

use crate::*;
    use crate::ChaCha20Rng;

    #[test]
    fn test_fill_bytes() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = ChaCha20Rng::from_seed(seed);
        let mut bytes = [0; 16];

        rng.fill_bytes(&mut bytes);

        assert_ne!(bytes, [0; 16], "The filled bytes should not be all zeros.");
    }

    #[test]
    fn test_fill_bytes_multiple() {
        let seed: [u8; 32] = [1; 32];
        let mut rng = ChaCha20Rng::from_seed(seed);
        let mut bytes1 = [0; 16];
        let mut bytes2 = [0; 16];

        rng.fill_bytes(&mut bytes1);
        rng.fill_bytes(&mut bytes2);

        assert_ne!(bytes1, bytes2, "The filled bytes should not be the same for different calls.");
    }
}

#[cfg(test)]
mod tests_llm_16_19 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_next_u32() {
        let seed: [u8; 32] = [0; 32]; // Using a fixed seed for determinism
        let mut rng = ChaCha20Rng::from_seed(seed);

        let value1 = rng.next_u32();
        let value2 = rng.next_u32();

        // Ensure we get different values on consecutive calls
        assert_ne!(value1, value2);

        // Verify that values are within the proper range
        assert!(value1 <= u32::MAX);
        assert!(value2 <= u32::MAX);
    }
}

#[cfg(test)]
mod tests_llm_16_20 {
    use super::*;

use crate::*;
    use crate::ChaCha20Rng;
    use rand_core::SeedableRng;

    #[test]
    fn test_next_u64() {
        // Arrange
        let seed: [u8; 32] = [0; 32]; // Example seed
        let mut rng = ChaCha20Rng::from_seed(seed);
        
        // Act
        let value = rng.next_u64();
        
        // Assert
        assert!(value != 0); // Assert that the value is not zero, indicating randomness
    }
}

#[cfg(test)]
mod tests_llm_16_21 {
    use super::*;

use crate::*;
    use crate::ChaCha20Rng;

    #[test]
    fn test_from_seed() {
        let seed: [u8; 32] = [0; 32];
        let rng = ChaCha20Rng::from_seed(seed);
        
        assert_eq!(rng.get_seed(), seed);
    }

    #[test]
    fn test_from_seed_non_zero() {
        let seed: [u8; 32] = [1; 32];
        let rng = ChaCha20Rng::from_seed(seed);
        
        assert_eq!(rng.get_seed(), seed);
    }

    #[test]
    fn test_from_seed_different_seeds() {
        let seed1: [u8; 32] = [0; 32];
        let seed2: [u8; 32] = [1; 32];
        let rng1 = ChaCha20Rng::from_seed(seed1);
        let rng2 = ChaCha20Rng::from_seed(seed2);

        assert_ne!(rng1.get_seed(), rng2.get_seed());
    }
}

#[cfg(test)]
mod tests_llm_16_22 {
    use super::*;

use crate::*;
    use crate::ChaCha20Rng;

    #[test]
    fn test_eq_identical() {
        let seed: [u8; 32] = [0u8; 32];
        let rng1 = ChaCha20Rng::from_seed(seed);
        let rng2 = ChaCha20Rng::from_seed(seed);
        assert!(rng1.eq(&rng2));
    }

    #[test]
    fn test_eq_different() {
        let seed1: [u8; 32] = [0u8; 32];
        let seed2: [u8; 32] = [1u8; 32];
        let rng1 = ChaCha20Rng::from_seed(seed1);
        let rng2 = ChaCha20Rng::from_seed(seed2);
        assert!(!rng1.eq(&rng2));
    }

    #[test]
    fn test_eq_different_states() {
        let seed: [u8; 32] = [0u8; 32];
        let mut rng1 = ChaCha20Rng::from_seed(seed);
        let mut rng2 = ChaCha20Rng::from_seed(seed);
        rng1.next_u32(); // Advance rng1
        assert!(!rng1.eq(&rng2));
    }

    #[test]
    fn test_eq_reflexivity() {
        let seed: [u8; 32] = [0u8; 32];
        let rng = ChaCha20Rng::from_seed(seed);
        assert!(rng.eq(&rng));
    }
}

#[cfg(test)]
mod tests_llm_16_24 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_from_seed() {
        let seed: [u8; 32] = [1; 32]; // Example seed
        let rng = chacha::ChaCha8Core::from_seed(seed);
        
        // Validate that the RNG is created and has the expected properties
        assert_eq!(rng.state.get_seed(), seed);
    }
}

#[cfg(test)]
mod tests_llm_16_29 {
    use super::*;

use crate::*;
    use crate::ChaCha8Rng;

    #[test]
    fn test_next_u64() {
        let seed: [u8; 32] = [0u8; 32];
        let mut rng = ChaCha8Rng::from_seed(seed);
        
        let value = rng.next_u64();
        
        assert!(value <= u64::MAX);
    }
}

#[cfg(test)]
mod tests_llm_16_36 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;
    use crate::ChaCha12Rng;

    #[test]
    fn test_get_seed() {
        let seed: [u8; 32] = [0u8; 32];
        let rng = ChaCha12Rng::from_seed(seed);
        let retrieved_seed = rng.get_seed();
        assert_eq!(retrieved_seed, seed);
    }
}

#[cfg(test)]
mod tests_llm_16_37 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_get_stream() {
        let seed: [u8; 32] = [0; 32]; // Example seed
        let rng = ChaCha12Rng::from_seed(seed);
        let stream_number = rng.get_stream();
        assert_eq!(stream_number, 0); // The initial stream number should be 0
    }

    #[test]
    fn test_set_stream() {
        let seed: [u8; 32] = [0; 32]; // Example seed
        let mut rng = ChaCha12Rng::from_seed(seed);
        rng.set_stream(42);
        let stream_number = rng.get_stream();
        assert_eq!(stream_number, 42); // The stream number should be set to 42
    }

    #[test]
    fn test_get_stream_multiple() {
        let seed: [u8; 32] = [0; 32]; // Example seed
        let mut rng = ChaCha12Rng::from_seed(seed);
        rng.set_stream(135);
        assert_eq!(rng.get_stream(), 135); // Ensure stream number is correctly set
        rng.set_stream(256);
        assert_eq!(rng.get_stream(), 256); // Ensure stream number is correctly set again
    }
}

#[cfg(test)]
mod tests_llm_16_38 {
    use super::*;

use crate::*;
    use crate::{ChaCha12Rng, rand_core::SeedableRng};

    #[test]
    fn test_get_word_pos() {
        // Arrange
        let seed: [u8; 32] = [0; 32];
        let mut rng = ChaCha12Rng::from_seed(seed);
        
        // Act
        let word_pos = rng.get_word_pos();
        
        // Assert
        assert_eq!(word_pos, 0); // Adjust the expected value based on the initial state of the RNG
    }

    #[test]
    fn test_get_word_pos_after_generation() {
        // Arrange
        let seed: [u8; 32] = [0; 32];
        let mut rng = ChaCha12Rng::from_seed(seed);
        let initial_pos = rng.get_word_pos();
        
        // Generate some random numbers to change the state
        rng.next_u32();
        
        // Act
        let new_pos = rng.get_word_pos();
        
        // Assert
        assert!(new_pos > initial_pos);
    }

    #[test]
    fn test_get_word_pos_with_different_seeds() {
        // Arrange
        let seed1: [u8; 32] = [1; 32];
        let seed2: [u8; 32] = [2; 32];
        
        let mut rng1 = ChaCha12Rng::from_seed(seed1);
        let mut rng2 = ChaCha12Rng::from_seed(seed2);
        
        // Act
        let pos1 = rng1.get_word_pos();
        let pos2 = rng2.get_word_pos();
        
        // Assert
        assert_ne!(pos1, pos2);
    }
}

#[cfg(test)]
mod tests_llm_16_39 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_set_stream() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = ChaCha12Rng::from_seed(seed);

        let initial_stream = rng.get_stream();
        let new_stream: u64 = 42;

        rng.set_stream(new_stream);
        let updated_stream = rng.get_stream();

        assert_ne!(initial_stream, updated_stream);
        assert_eq!(updated_stream, new_stream);
    }

    #[test]
    fn test_set_stream_no_change() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = ChaCha12Rng::from_seed(seed);

        let initial_stream = rng.get_stream();
        rng.set_stream(initial_stream);
        let updated_stream = rng.get_stream();

        assert_eq!(updated_stream, initial_stream);
    }
}

#[cfg(test)]
mod tests_llm_16_40 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_set_word_pos() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = ChaCha12Rng::from_seed(seed);
        
        // Setting a known position
        let word_pos: u128 = 100;

        // Before setting position, get the current position
        let initial_pos = rng.get_word_pos();
        
        // Set new word position
        rng.set_word_pos(word_pos);
        
        // Check if the new position reflects the expected value
        let new_pos = rng.get_word_pos();
        assert_eq!(new_pos, word_pos);
        
        // Ensure the position has actually changed
        assert_ne!(initial_pos, new_pos);
    }

    #[test]
    fn test_set_word_pos_large_value() {
        let seed: [u8; 32] = [1; 32];
        let mut rng = ChaCha12Rng::from_seed(seed);
        
        // Setting a word position that is significantly larger than the maximum block
        let word_pos: u128 = u128::MAX;

        // Set new word position
        rng.set_word_pos(word_pos);
        
        // Check if the new position reflects the expected value
        let new_pos = rng.get_word_pos();
        assert_eq!(new_pos, word_pos);
    }
}

#[cfg(test)]
mod tests_llm_16_42 {
    use super::*;

use crate::*;
    use crate::ChaCha20Rng;

    #[test]
    fn test_get_stream() {
        let seed: [u8; 32] = [0; 32]; // Example seed
        let mut rng = ChaCha20Rng::from_seed(seed);
        
        // Initially, the stream number should be 0
        assert_eq!(rng.get_stream(), 0);
        
        // Set the stream number to a known value
        let stream_value: u64 = 42;
        rng.set_stream(stream_value);
        
        // Verify the stream number
        assert_eq!(rng.get_stream(), stream_value);
        
        // Set the stream number to another value
        let another_stream_value: u64 = 100;
        rng.set_stream(another_stream_value);
        
        // Verify the stream number
        assert_eq!(rng.get_stream(), another_stream_value);
    }
}

#[cfg(test)]
mod tests_llm_16_45 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_set_word_pos() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = ChaCha20Rng::from_seed(seed);

        // Setting position to 0
        rng.set_word_pos(0);
        assert_eq!(rng.get_word_pos(), 0);

        // Setting position to a positive word offset
        let offset: u128 = 10; // arbitrary offset
        rng.set_word_pos(offset);
        assert_eq!(rng.get_word_pos(), offset);

        // Setting position to a very large word offset
        let large_offset: u128 = u128::MAX; // maximum offset
        rng.set_word_pos(large_offset);
        assert_eq!(rng.get_word_pos(), large_offset);
    }
}

#[cfg(test)]
mod tests_llm_16_46 {
    use super::*;

use crate::*;
    use crate::ChaCha8Rng;

    #[test]
    fn test_get_seed() {
        let seed: [u8; 32] = [0; 32]; // Test seed
        let mut rng = ChaCha8Rng::from_seed(seed);
        
        // Ensure that the seed retrieved via get_seed matches the original seed
        let retrieved_seed = rng.get_seed();
        assert_eq!(retrieved_seed, seed);
    }
}

#[cfg(test)]
mod tests_llm_16_47 {
    use super::*; // Adjust based on your module structure.

use crate::*;
    use crate::ChaCha8Rng;

    #[test]
    fn test_get_stream() {
        let seed: [u8; 32] = [0; 32];
        let rng = ChaCha8Rng::from_seed(seed);
        let stream_number = rng.get_stream();
        assert_eq!(stream_number, 0); // Assuming initial stream number is 0
    }

    #[test]
    fn test_get_stream_after_setting_stream() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = ChaCha8Rng::from_seed(seed);
        rng.set_stream(42); // Setting stream number to 42
        let stream_number = rng.get_stream();
        assert_eq!(stream_number, 42);
    }
}

#[cfg(test)]
mod tests_llm_16_48 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_get_word_pos() {
        let seed: [u8; 32] = [0u8; 32];
        let mut rng = ChaCha8Rng::from_seed(seed);
        rng.set_word_pos(100);
        let pos = rng.get_word_pos();
        assert_eq!(pos, 100);
        
        rng.set_word_pos(0);
        let pos_zero = rng.get_word_pos();
        assert_eq!(pos_zero, 0);
        
        rng.set_word_pos(16);
        let pos_sixteen = rng.get_word_pos();
        assert_eq!(pos_sixteen, 16);
        
        rng.set_word_pos(256);
        let pos_two_fifty_six = rng.get_word_pos();
        assert_eq!(pos_two_fifty_six, 256);
    }
}

#[cfg(test)]
mod tests_llm_16_49 {
    use super::*;

use crate::*;
    use crate::ChaCha8Rng;
    
    #[test]
    fn test_set_stream() {
        // Create a new ChaCha8Rng instance with a known seed
        let seed: [u8; 32] = [0u8; 32];
        let mut rng = ChaCha8Rng::from_seed(seed);

        // Set a specific stream
        let stream_id: u64 = 10;
        rng.set_stream(stream_id);

        // Verify if the stream has been set correctly
        assert_eq!(rng.get_stream(), stream_id);
    }

    #[test]
    fn test_set_stream_sequential() {
        let seed: [u8; 32] = [0u8; 32];
        let mut rng = ChaCha8Rng::from_seed(seed);

        // Set a stream, generate a number, then change stream
        rng.set_stream(0);
        let first_output = rng.next_u32();

        rng.set_stream(1);
        let second_output = rng.next_u32();

        // Verify that the outputs are different with different streams
        assert_ne!(first_output, second_output);
    }
}

#[cfg(test)]
mod tests_llm_16_50 {
    use super::*;

use crate::*;
    use crate::ChaCha8Rng;

    #[test]
    fn test_set_word_pos() {
        let seed = [0u8; 32];
        let mut rng = ChaCha8Rng::from_seed(seed);

        let initial_pos = rng.get_word_pos();
        let new_pos = initial_pos + 16; // Move 16 words ahead

        rng.set_word_pos(new_pos);
        let current_pos = rng.get_word_pos();

        assert_eq!(current_pos, new_pos);
    }

    #[test]
    fn test_set_word_pos_zero() {
        let seed = [0u8; 32];
        let mut rng = ChaCha8Rng::from_seed(seed);

        rng.set_word_pos(0);
        let current_pos = rng.get_word_pos();

        assert_eq!(current_pos, 0);
    }

    #[test]
    fn test_set_word_pos_large_value() {
        let seed = [0u8; 32];
        let mut rng = ChaCha8Rng::from_seed(seed);

        let max_word_pos = u128::MAX;
        rng.set_word_pos(max_word_pos);
        let current_pos = rng.get_word_pos();

        assert_eq!(current_pos, max_word_pos % u128::from(BLOCK_WORDS));
    }
}
