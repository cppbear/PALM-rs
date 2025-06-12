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
mod rusty_tests {
	use crate::*;
	use std::iter::ExactSizeIterator;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::iter::Iterator;
	use std::cmp::Eq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6() {
    rusty_monitor::set_test_id(6);
    let mut u64_0: u64 = 3541u64;
    let mut itermut_0: crate::map::IterMut<isize, isize> = crate::map::IterMut::default();
    let mut itermut_0_ref_0: &crate::map::IterMut<isize, isize> = &mut itermut_0;
    let mut iter_0: crate::table::Iter<std::result::Result<crate::raw::RawTableInner, TryReserveError>> = crate::table::Iter::default();
    let mut iter_0_ref_0: &crate::table::Iter<std::result::Result<crate::raw::RawTableInner, TryReserveError>> = &mut iter_0;
    let mut iterhash_0: crate::table::IterHash<isize> = crate::table::IterHash::default();
    let mut iterhash_0_ref_0: &crate::table::IterHash<isize> = &mut iterhash_0;
    let mut iter_1: crate::table::Iter<isize> = crate::table::Iter::default();
    let mut iter_1_ref_0: &crate::table::Iter<isize> = &mut iter_1;
    let mut iter_2: crate::table::Iter<isize> = crate::table::Iter::default();
    let mut iter_2_ref_0: &crate::table::Iter<isize> = &mut iter_2;
    let mut rawiter_0: crate::raw::RawIter<isize> = crate::raw::RawIter::default();
    let mut rawiter_0_ref_0: &mut crate::raw::RawIter<isize> = &mut rawiter_0;
    let mut valuesmut_0: crate::map::ValuesMut<isize, std::result::Result<crate::raw::Bucket<isize>, crate::raw::InsertSlot>> = crate::map::ValuesMut::default();
    let mut valuesmut_0_ref_0: &crate::map::ValuesMut<isize, std::result::Result<crate::raw::Bucket<isize>, crate::raw::InsertSlot>> = &mut valuesmut_0;
    let mut u64_1: u64 = 9755u64;
    let mut tag_0: crate::control::tag::Tag = crate::control::tag::Tag::full(u64_1);
    let mut values_0: crate::map::Values<isize, isize> = crate::map::Values::default();
    let mut values_0_ref_0: &crate::map::Values<isize, isize> = &mut values_0;
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::Values::size_hint(values_0_ref_0);
    let mut bool_0: bool = crate::control::tag::Tag::is_special(tag_0);
    let mut option_0: std::option::Option<crate::raw::Bucket<isize>> = crate::raw::RawIter::next(rawiter_0_ref_0);
    let mut bucket_0: crate::raw::Bucket<isize> = std::option::Option::unwrap(option_0);
    let mut usize_0: usize = crate::table::Iter::len(iter_2_ref_0);
    let mut usize_1: usize = crate::table::Iter::len(iter_1_ref_0);
    let mut iterhashmut_0: crate::table::IterHashMut<isize> = crate::table::IterHashMut::default();
    let mut iterhash_1: crate::table::IterHash<isize> = crate::table::IterHash::clone(iterhash_0_ref_0);
    let mut bucket_0_ref_0: &crate::raw::Bucket<isize> = &mut bucket_0;
    let mut usize_2: usize = crate::map::IterMut::len(itermut_0_ref_0);
    let mut tag_1: crate::control::tag::Tag = crate::control::tag::Tag::full(u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut u64_0: u64 = 6090u64;
    let mut tag_0: crate::control::tag::Tag = crate::control::tag::Tag::full(u64_0);
    let mut tag_0_ref_0: &crate::control::tag::Tag = &mut tag_0;
    let mut u64_1: u64 = 4712u64;
    let mut tag_1: crate::control::tag::Tag = crate::control::tag::Tag::full(u64_1);
    let mut tag_1_ref_0: &crate::control::tag::Tag = &mut tag_1;
    let mut rawiter_0: crate::raw::RawIter<isize> = crate::raw::RawIter::default();
    let mut rawiter_0_ref_0: &mut crate::raw::RawIter<isize> = &mut rawiter_0;
    let mut u64_2: u64 = 7314u64;
    let mut tag_2: crate::control::tag::Tag = crate::control::tag::Tag::full(u64_2);
    let mut valuesmut_0: crate::map::ValuesMut<isize, isize> = crate::map::ValuesMut::default();
    let mut valuesmut_0_ref_0: &mut crate::map::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut u64_3: u64 = 548u64;
    let mut tag_3: crate::control::tag::Tag = crate::control::tag::Tag::full(u64_3);
    let mut rawiter_1: crate::raw::RawIter<isize> = crate::raw::RawIter::default();
    let mut rawiter_1_ref_0: &crate::raw::RawIter<isize> = &mut rawiter_1;
    let mut tryreserveerror_0: TryReserveError = crate::TryReserveError::CapacityOverflow;
    let mut tryreserveerror_0_ref_0: &TryReserveError = &mut tryreserveerror_0;
    let mut iter_0: crate::set::Iter<std::string::String> = crate::set::Iter::default();
    let mut iter_0_ref_0: &crate::set::Iter<std::string::String> = &mut iter_0;
    let mut u64_4: u64 = 264u64;
    let mut tag_4: crate::control::tag::Tag = crate::control::tag::Tag::full(u64_4);
    let mut bool_0: bool = crate::control::tag::Tag::special_is_empty(tag_4);
    let mut tuple_0: () = crate::TryReserveError::assert_receiver_is_total_eq(tryreserveerror_0_ref_0);
    let mut iter_1: crate::table::Iter<isize> = crate::table::Iter::default();
    let mut iter_1_ref_0: &mut crate::table::Iter<isize> = &mut iter_1;
    let mut option_0: std::option::Option<&isize> = crate::table::Iter::next(iter_1_ref_0);
    let mut isize_0: &isize = std::option::Option::unwrap(option_0);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::raw::RawIter::size_hint(rawiter_1_ref_0);
    let mut bool_1: bool = crate::control::tag::Tag::is_special(tag_3);
    let mut option_1: std::option::Option<&mut isize> = crate::map::ValuesMut::next(valuesmut_0_ref_0);
    let mut isize_1: &mut isize = std::option::Option::unwrap(option_1);
    let mut bool_2: bool = crate::control::tag::Tag::is_special(tag_2);
    let mut option_2: std::option::Option<crate::raw::Bucket<isize>> = crate::raw::RawIter::next(rawiter_0_ref_0);
    let mut bucket_0: crate::raw::Bucket<isize> = std::option::Option::unwrap(option_2);
    let mut bool_3: bool = crate::control::tag::Tag::eq(tag_1_ref_0, tag_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut itermut_0: crate::map::IterMut<isize, isize> = crate::map::IterMut::default();
    let mut itermut_0_ref_0: &mut crate::map::IterMut<isize, isize> = &mut itermut_0;
    let mut itermut_1: crate::map::IterMut<std::result::Result<crate::raw::RawTableInner, TryReserveError>, std::string::String> = crate::map::IterMut::default();
    let mut itermut_1_ref_0: &crate::map::IterMut<std::result::Result<crate::raw::RawTableInner, TryReserveError>, std::string::String> = &mut itermut_1;
    let mut u64_0: u64 = 4559u64;
    let mut itermut_2: crate::table::IterMut<isize> = crate::table::IterMut::default();
    let mut itermut_2_ref_0: &mut crate::table::IterMut<isize> = &mut itermut_2;
    let mut u64_1: u64 = 2805u64;
    let mut tag_0: crate::control::tag::Tag = crate::control::tag::Tag::full(u64_1);
    let mut iter_0: crate::set::Iter<isize> = crate::set::Iter::default();
    let mut iter_0_ref_0: &crate::set::Iter<isize> = &mut iter_0;
    let mut values_0: crate::map::Values<isize, isize> = crate::map::Values::default();
    let mut values_0_ref_0: &crate::map::Values<isize, isize> = &mut values_0;
    let mut iter_1: crate::table::Iter<isize> = crate::table::Iter::default();
    let mut iter_1_ref_0: &crate::table::Iter<isize> = &mut iter_1;
    let mut rawiter_0: crate::raw::RawIter<isize> = crate::raw::RawIter::default();
    let mut iter_2: crate::table::Iter<isize> = crate::table::Iter::clone(iter_1_ref_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::Values::size_hint(values_0_ref_0);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::set::Iter::size_hint(iter_0_ref_0);
    let mut iter_2_ref_0: &crate::table::Iter<isize> = &mut iter_2;
    let mut usize_0: usize = crate::table::Iter::len(iter_2_ref_0);
    let mut rawiter_0_ref_0: &crate::raw::RawIter<isize> = &mut rawiter_0;
    let mut tuple_2: (usize, std::option::Option<usize>) = crate::raw::RawIter::size_hint(rawiter_0_ref_0);
    let mut bool_0: bool = crate::control::tag::Tag::is_special(tag_0);
    let mut option_0: std::option::Option<&mut isize> = crate::table::IterMut::next(itermut_2_ref_0);
    let mut rawtable_0: crate::raw::RawTable<isize, allocator_api2::alloc::Global> = crate::raw::RawTable::new();
    let mut tag_1: crate::control::tag::Tag = crate::control::tag::Tag::full(u64_0);
    let mut tag_1_ref_0: &crate::control::tag::Tag = &mut tag_1;
    let mut tag_2: crate::control::tag::Tag = crate::control::tag::Tag::clone(tag_1_ref_0);
    let mut option_1: std::option::Option<(&isize, &mut isize)> = crate::map::IterMut::next(itermut_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_32() {
//     rusty_monitor::set_test_id(32);
//     let mut itermut_0: crate::table::IterMut<std::result::Result<crate::scopeguard::ScopeGuard<crate::raw::RawTableInner>, TryReserveError>> = crate::table::IterMut::default();
//     let mut itermut_0_ref_0: &crate::table::IterMut<std::result::Result<crate::scopeguard::ScopeGuard<crate::raw::RawTableInner>, TryReserveError>> = &mut itermut_0;
//     let mut values_0: crate::map::Values<isize, isize> = crate::map::Values::default();
//     let mut values_0_ref_0: &crate::map::Values<isize, isize> = &mut values_0;
//     let mut values_1: crate::map::Values<isize, isize> = crate::map::Values::default();
//     let mut values_1_ref_0: &crate::map::Values<isize, isize> = &mut values_1;
//     let mut u64_0: u64 = 286u64;
//     let mut tag_0: crate::control::tag::Tag = crate::control::tag::Tag::full(u64_0);
//     let mut iterhash_0: crate::table::IterHash<std::result::Result<crate::raw::Bucket<isize>, crate::raw::InsertSlot>> = crate::table::IterHash::default();
//     let mut iterhash_0_ref_0: &crate::table::IterHash<std::result::Result<crate::raw::Bucket<isize>, crate::raw::InsertSlot>> = &mut iterhash_0;
//     let mut values_2: crate::map::Values<isize, isize> = crate::map::Values::default();
//     let mut values_2_ref_0: &mut crate::map::Values<isize, isize> = &mut values_2;
//     let mut tryreserveerror_0: TryReserveError = crate::TryReserveError::CapacityOverflow;
//     let mut tryreserveerror_0_ref_0: &TryReserveError = &mut tryreserveerror_0;
//     let mut iter_0: crate::table::Iter<isize> = crate::table::Iter::default();
//     let mut iter_0_ref_0: &crate::table::Iter<isize> = &mut iter_0;
//     let mut iter_1: crate::set::Iter<std::result::Result<std::ptr::NonNull<u8>, ()>> = crate::set::Iter::default();
//     let mut iter_1_ref_0: &crate::set::Iter<std::result::Result<std::ptr::NonNull<u8>, ()>> = &mut iter_1;
//     let mut u64_1: u64 = 1947u64;
//     let mut tag_1: crate::control::tag::Tag = crate::control::tag::Tag::full(u64_1);
//     let mut bool_0: bool = crate::control::tag::Tag::is_full(tag_1);
//     let mut tuple_0: (usize, std::option::Option<usize>) = crate::table::Iter::size_hint(iter_0_ref_0);
//     let mut iterhashmut_0: crate::table::IterHashMut<isize> = crate::table::IterHashMut::default();
//     let mut option_0: std::option::Option<&isize> = crate::map::Values::next(values_2_ref_0);
//     let mut iterhashmut_0_ref_0: &mut crate::table::IterHashMut<isize> = &mut iterhashmut_0;
//     let mut option_1: std::option::Option<&mut isize> = crate::table::IterHashMut::next(iterhashmut_0_ref_0);
//     let mut bool_1: bool = crate::control::tag::Tag::is_special(tag_0);
//     let mut usize_0: usize = crate::map::Values::len(values_1_ref_0);
//     let mut isize_0: &isize = std::option::Option::unwrap(option_0);
//     let mut isize_1: &mut isize = std::option::Option::unwrap(option_1);
//     let mut values_3: crate::map::Values<isize, isize> = crate::map::Values::clone(values_0_ref_0);
//     panic!("From RustyUnit with love");
// }
}