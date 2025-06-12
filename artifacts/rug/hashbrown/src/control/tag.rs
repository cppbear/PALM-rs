use core::{fmt, mem};

/// Single tag in a control group.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct Tag(pub(super) u8);
impl Tag {
    /// Control tag value for an empty bucket.
    pub(crate) const EMPTY: Tag = Tag(0b1111_1111);

    /// Control tag value for a deleted bucket.
    pub(crate) const DELETED: Tag = Tag(0b1000_0000);

    /// Checks whether a control tag represents a full bucket (top bit is clear).
    #[inline]
    pub(crate) const fn is_full(self) -> bool {
        self.0 & 0x80 == 0
    }

    /// Checks whether a control tag represents a special value (top bit is set).
    #[inline]
    pub(crate) const fn is_special(self) -> bool {
        self.0 & 0x80 != 0
    }

    /// Checks whether a special control value is EMPTY (just check 1 bit).
    #[inline]
    pub(crate) const fn special_is_empty(self) -> bool {
        debug_assert!(self.is_special());
        self.0 & 0x01 != 0
    }

    /// Creates a control tag representing a full bucket with the given hash.
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) const fn full(hash: u64) -> Tag {
        // Constant for function that grabs the top 7 bits of the hash.
        const MIN_HASH_LEN: usize = if mem::size_of::<usize>() < mem::size_of::<u64>() {
            mem::size_of::<usize>()
        } else {
            mem::size_of::<u64>()
        };

        // Grab the top 7 bits of the hash. While the hash is normally a full 64-bit
        // value, some hash functions (such as FxHash) produce a usize result
        // instead, which means that the top 32 bits are 0 on 32-bit platforms.
        // So we use MIN_HASH_LEN constant to handle this.
        let top7 = hash >> (MIN_HASH_LEN * 8 - 7);
        Tag((top7 & 0x7f) as u8) // truncation
    }
}
impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_special() {
            if self.special_is_empty() {
                f.pad("EMPTY")
            } else {
                f.pad("DELETED")
            }
        } else {
            f.debug_tuple("full").field(&(self.0 & 0x7F)).finish()
        }
    }
}

/// Extension trait for slices of tags.
pub(crate) trait TagSliceExt {
    /// Fills the control with the given tag.
    fn fill_tag(&mut self, tag: Tag);

    /// Clears out the control.
    #[inline]
    fn fill_empty(&mut self) {
        self.fill_tag(Tag::EMPTY)
    }
}
impl TagSliceExt for [Tag] {
    #[inline]
    fn fill_tag(&mut self, tag: Tag) {
        // SAFETY: We have access to the entire slice, so, we can write to the entire slice.
        unsafe { self.as_mut_ptr().write_bytes(tag.0, self.len()) }
    }
}

#[cfg(test)]
mod tests_llm_16_10 {
    use super::*;

use crate::*;
    use control::tag::{Tag, TagSliceExt}; // Adjust the import path as necessary

    #[test]
    fn test_fill_tag() {
        let mut tags: [Tag; 5] = [Tag::EMPTY; 5];
        let new_tag = Tag::full(42); // Replace 42 with an actual hash you want to test
        tags.fill_tag(new_tag);
        
        for tag in &tags {
            assert_eq!(*tag, new_tag);
        }
    }
    
    #[test]
    fn test_fill_tag_with_different_tag() {
        let mut tags: [Tag; 3] = [Tag::EMPTY; 3];
        let new_tag = Tag::DELETED;
        tags.fill_tag(new_tag);

        for tag in &tags {
            assert_eq!(*tag, new_tag);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_244 {
    use super::*;

use crate::*;
    use control::tag::Tag;

    #[test]
    fn test_full() {
        let hash_values = [
            (0, Tag(0)),                   // Hash 0 results in Tag(0)
            (1, Tag(0)),                   // Hash 1 results in Tag(0)
            (127, Tag(0b0111_1111)),       // Hash 127 results in Tag(0b0111_1111)
            (128, Tag(0b0000_0000)),       // Hash 128 results in Tag(0b0000_0000)
            (255, Tag(0b0000_0111)),       // Hash 255 results in Tag(0b0000_0111)
            (511, Tag(0b0000_1111)),       // Hash 511 results in Tag(0b0000_1111)
            (1023, Tag(0b0001_1111)),      // Hash 1023 results in Tag(0b0001_1111)
            (u64::MAX, Tag(0b0111_1111)),  // Hash u64::MAX results in Tag(0b0111_1111)
        ];

        for (hash, expected_tag) in hash_values.iter() {
            let tag = Tag::full(*hash);
            assert_eq!(tag, *expected_tag, "Expected Tag for hash {} does not match", hash);
            assert!(tag.is_full(), "Tag for hash {} should be full", hash);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_245 {
    use crate::control::tag::Tag;

    #[test]
    fn test_is_full() {
        // Test with a full tag
        let full_tag = Tag::full(0x7f); // Using a hash that results in 0x7f
        assert!(full_tag.is_full());

        // Test with an empty tag
        let empty_tag = Tag::EMPTY;
        assert!(!empty_tag.is_full());

        // Test with a deleted tag
        let deleted_tag = Tag::DELETED;
        assert!(!deleted_tag.is_full());

        // Test with a custom tag that is full
        let custom_full_tag = Tag(0b0111_1111); // Full tag with top bit clear
        assert!(custom_full_tag.is_full());

        // Test with a custom tag that is not full
        let custom_not_full_tag = Tag(0b1111_1111); // Full tag with top bit set
        assert!(!custom_not_full_tag.is_full());
    }
}

#[cfg(test)]
mod tests_llm_16_246 {
    use crate::control::tag::Tag;

    #[test]
    fn test_is_special() {
        assert!(!Tag::EMPTY.is_special());
        assert!(Tag::DELETED.is_special());
        assert!(!Tag(0b0111_1111).is_special());
        assert!(Tag(0b1111_1111).is_special());
    }
}

#[cfg(test)]
mod tests_llm_16_247 {
    use crate::control::tag::Tag;

    #[test]
    fn test_special_is_empty() {
        // Test for EMPTY tag
        let empty_tag = Tag::EMPTY;
        assert!(empty_tag.is_special());
        assert!(empty_tag.special_is_empty());

        // Test for DELETED tag
        let deleted_tag = Tag::DELETED;
        assert!(deleted_tag.is_special());
        assert!(!deleted_tag.special_is_empty());

        // Test for another special tag
        let special_tag = Tag(0b1000_0001); // special and empty
        assert!(special_tag.is_special());
        assert!(special_tag.special_is_empty());
        
        let another_special_tag = Tag(0b1000_0010); // special and not empty
        assert!(another_special_tag.is_special());
        assert!(!another_special_tag.special_is_empty());
    }
}
