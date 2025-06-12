use crate::alloc::alloc::{handle_alloc_error, Layout};
use crate::scopeguard::{guard, ScopeGuard};
use crate::TryReserveError;
use core::array;
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::mem;
use core::ptr::NonNull;
use core::{hint, ptr};
pub(crate) use self::alloc::{do_alloc, Allocator, Global};
use self::bitmask::BitMaskIter;
use self::imp::Group;
#[cfg(not(feature = "nightly"))]
use core::convert::{identity as likely, identity as unlikely};
#[cfg(feature = "nightly")]
use core::intrinsics::{likely, unlikely};
cfg_if! {
    if #[cfg(all(target_feature = "sse2", any(target_arch = "x86", target_arch =
    "x86_64"), not(miri),))] { mod sse2; use sse2 as imp; } else if #[cfg(all(target_arch
    = "aarch64", target_feature = "neon", target_endian = "little", not(miri),))] { mod
    neon; use neon as imp; } else { mod generic; use generic as imp; }
}
pub(crate) struct FullBucketsIndices {
    current_group: BitMaskIter,
    group_first_index: usize,
    ctrl: NonNull<u8>,
    items: usize,
}
#[derive(Copy, Clone)]
pub(crate) struct BitMaskIter(pub(crate) BitMask);
#[derive(Copy, Clone)]
pub(crate) struct BitMask(pub(crate) BitMaskWord);
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub(crate) struct Tag(u8);
impl FullBucketsIndices {
    #[inline(always)]
    unsafe fn next_impl(&mut self) -> Option<usize> {
        loop {
            if let Some(index) = self.current_group.next() {
                return Some(self.group_first_index + index);
            }
            self.ctrl = NonNull::new_unchecked(self.ctrl.as_ptr().add(Group::WIDTH));
            self.current_group = Group::load_aligned(self.ctrl.as_ptr().cast())
                .match_full()
                .into_iter();
            self.group_first_index += Group::WIDTH;
        }
    }
}
impl Iterator for BitMaskIter {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        let bit = self.0.lowest_set_bit()?;
        self.0 = self.0.remove_lowest_bit();
        Some(bit)
    }
}
impl IntoIterator for BitMask {
    type Item = usize;
    type IntoIter = BitMaskIter;
    #[inline]
    fn into_iter(self) -> BitMaskIter {
        BitMaskIter(BitMask(self.0 & BITMASK_ITER_MASK))
    }
}
