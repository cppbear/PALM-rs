use super::super::{BitMask, Tag};
use core::mem;
use core::num::NonZeroU16;

#[cfg(target_arch = "x86")]
use core::arch::x86;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as x86;

pub(crate) type BitMaskWord = u16;
pub(crate) type NonZeroBitMaskWord = NonZeroU16;
pub(crate) const BITMASK_STRIDE: usize = 1;
pub(crate) const BITMASK_MASK: BitMaskWord = 0xffff;
pub(crate) const BITMASK_ITER_MASK: BitMaskWord = !0;

/// Abstraction over a group of control tags which can be scanned in
/// parallel.
///
/// This implementation uses a 128-bit SSE value.
#[derive(Copy, Clone)]
pub(crate) struct Group(x86::__m128i);

// FIXME: https://github.com/rust-lang/rust-clippy/issues/3859
#[allow(clippy::use_self)]
impl Group {
    /// Number of bytes in the group.
    pub(crate) const WIDTH: usize = mem::size_of::<Self>();

    /// Returns a full group of empty tags, suitable for use as the initial
    /// value for an empty hash table.
    ///
    /// This is guaranteed to be aligned to the group size.
    #[inline]
    #[allow(clippy::items_after_statements)]
    pub(crate) const fn static_empty() -> &'static [Tag; Group::WIDTH] {
        #[repr(C)]
        struct AlignedTags {
            _align: [Group; 0],
            tags: [Tag; Group::WIDTH],
        }
        const ALIGNED_TAGS: AlignedTags = AlignedTags {
            _align: [],
            tags: [Tag::EMPTY; Group::WIDTH],
        };
        &ALIGNED_TAGS.tags
    }

    /// Loads a group of tags starting at the given address.
    #[inline]
    #[allow(clippy::cast_ptr_alignment)] // unaligned load
    pub(crate) unsafe fn load(ptr: *const Tag) -> Self {
        Group(x86::_mm_loadu_si128(ptr.cast()))
    }

    /// Loads a group of tags starting at the given address, which must be
    /// aligned to `mem::align_of::<Group>()`.
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    pub(crate) unsafe fn load_aligned(ptr: *const Tag) -> Self {
        debug_assert_eq!(ptr.align_offset(mem::align_of::<Self>()), 0);
        Group(x86::_mm_load_si128(ptr.cast()))
    }

    /// Stores the group of tags to the given address, which must be
    /// aligned to `mem::align_of::<Group>()`.
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    pub(crate) unsafe fn store_aligned(self, ptr: *mut Tag) {
        debug_assert_eq!(ptr.align_offset(mem::align_of::<Self>()), 0);
        x86::_mm_store_si128(ptr.cast(), self.0);
    }

    /// Returns a `BitMask` indicating all tags in the group which have
    /// the given value.
    #[inline]
    pub(crate) fn match_tag(self, tag: Tag) -> BitMask {
        #[allow(
            clippy::cast_possible_wrap, // tag.0: Tag as i8
            // tag: i32 as u16
            //   note: _mm_movemask_epi8 returns a 16-bit mask in a i32, the
            //   upper 16-bits of the i32 are zeroed:
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation
        )]
        unsafe {
            let cmp = x86::_mm_cmpeq_epi8(self.0, x86::_mm_set1_epi8(tag.0 as i8));
            BitMask(x86::_mm_movemask_epi8(cmp) as u16)
        }
    }

    /// Returns a `BitMask` indicating all tags in the group which are
    /// `EMPTY`.
    #[inline]
    pub(crate) fn match_empty(self) -> BitMask {
        self.match_tag(Tag::EMPTY)
    }

    /// Returns a `BitMask` indicating all tags in the group which are
    /// `EMPTY` or `DELETED`.
    #[inline]
    pub(crate) fn match_empty_or_deleted(self) -> BitMask {
        #[allow(
            // tag: i32 as u16
            //   note: _mm_movemask_epi8 returns a 16-bit mask in a i32, the
            //   upper 16-bits of the i32 are zeroed:
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation
        )]
        unsafe {
            // A tag is EMPTY or DELETED iff the high bit is set
            BitMask(x86::_mm_movemask_epi8(self.0) as u16)
        }
    }

    /// Returns a `BitMask` indicating all tags in the group which are full.
    #[inline]
    pub(crate) fn match_full(&self) -> BitMask {
        self.match_empty_or_deleted().invert()
    }

    /// Performs the following transformation on all tags in the group:
    /// - `EMPTY => EMPTY`
    /// - `DELETED => EMPTY`
    /// - `FULL => DELETED`
    #[inline]
    pub(crate) fn convert_special_to_empty_and_full_to_deleted(self) -> Self {
        // Map high_bit = 1 (EMPTY or DELETED) to 1111_1111
        // and high_bit = 0 (FULL) to 1000_0000
        //
        // Here's this logic expanded to concrete values:
        //   let special = 0 > tag = 1111_1111 (true) or 0000_0000 (false)
        //   1111_1111 | 1000_0000 = 1111_1111
        //   0000_0000 | 1000_0000 = 1000_0000
        #[allow(
            clippy::cast_possible_wrap, // tag: Tag::DELETED.0 as i8
        )]
        unsafe {
            let zero = x86::_mm_setzero_si128();
            let special = x86::_mm_cmpgt_epi8(zero, self.0);
            Group(x86::_mm_or_si128(
                special,
                x86::_mm_set1_epi8(Tag::DELETED.0 as i8),
            ))
        }
    }
}

#[cfg(test)]
mod tests_llm_16_236 {
    use super::*;

use crate::*;
    use std::mem;

    #[test]
    fn test_load() {
        // Prepare an array of Tags
        let tags: [Tag; Group::WIDTH] = [
            Tag::EMPTY, Tag::DELETED, Tag::full(1), Tag::full(2),
            Tag::full(3), Tag::full(4), Tag::full(5), Tag::full(6),
            Tag::full(7), Tag::full(8), Tag::full(9), Tag::full(10),
            Tag::full(11), Tag::full(12), Tag::full(13), Tag::full(14),
        ];
        
        // Get a pointer to the start of the tags array
        let tags_ptr = tags.as_ptr();

        // Use unsafe block to call the load function
        let group: Group;

        unsafe {
            group = Group::load(tags_ptr);
        }

        // Verify that the loaded group has the expected first tag
        let expected_mask = 0b1111_1111; // Corresponds to Tag::EMPTY in this example
        let tag_mask = group.match_empty_or_deleted().0;

        assert_eq!(tag_mask, expected_mask, "Loaded group did not match expected tag mask.");
    }
}

#[cfg(test)]
mod tests_llm_16_238 {
    use super::*;

use crate::*;
    use control::bitmask::BitMask;
    use control::group::sse2::Group;
    use control::tag::Tag;

    #[test]
    fn test_match_empty() {
        let empty_group = Group::static_empty();
        let group = unsafe { Group::load(empty_group.as_ptr()) };
        let bitmask = group.match_empty();
        
        // Check if the BitMask indicates all bits set to EMPTY
        assert!(bitmask.any_bit_set());
        assert_eq!(bitmask.trailing_zeros(), 0);
        assert_eq!(bitmask.leading_zeros(), 0);
        assert_eq!(bitmask.lowest_set_bit(), Some(0));
    }
}

#[cfg(test)]
mod tests_llm_16_239 {
    use super::*;

use crate::*;
    use control::bitmask::BitMask;
    use control::group::sse2::Group;
    use std::mem;

    #[test]
    fn test_match_empty_or_deleted() {
        // Assuming Tag::EMPTY is defined as 0
        let empty_tag = Tag(0);
        let deleted_tag = Tag(1 << 7); // Assuming DELETED is defined as high bit set
        let tags = [empty_tag, deleted_tag, empty_tag, empty_tag, deleted_tag, empty_tag, empty_tag, empty_tag, empty_tag, empty_tag, empty_tag, empty_tag, empty_tag, empty_tag, empty_tag, empty_tag];
        let group = unsafe { Group::load(tags.as_ptr()) };
        
        let mask = group.match_empty_or_deleted();
        
        // Here, we expect the mask to have bits set for all empty and deleted tags
        // Assuming DELETED is the high bit, the expected mask would be 0b1100000000000000
        let expected_mask = (1 << 15) | (1 << 14); // bits for 0, 1
        assert_eq!(mask.0, expected_mask as BitMaskWord);
    }
}

#[cfg(test)]
mod tests_llm_16_242 {
    use crate::control::group::sse2::Group;
    use crate::control::tag::Tag;

    #[test]
    fn test_static_empty() {
        const EMPTY_TAGS: &[Tag; Group::WIDTH] = Group::static_empty();

        // Verify that all tags are EMPTY
        for tag in EMPTY_TAGS {
            assert_eq!(*tag, Tag::EMPTY);
        }
    }
}
