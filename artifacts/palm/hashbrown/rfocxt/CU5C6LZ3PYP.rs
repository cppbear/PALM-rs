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
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub(crate) struct Tag(u8);
impl Tag {
    const EMPTY: Tag = Tag(0b1111_1111);
    const DELETED: Tag = Tag(0b1000_0000);
    #[inline]
    const fn is_full(self) -> bool {}
    #[inline]
    const fn is_special(self) -> bool {
        self.0 & 0x80 != 0
    }
    #[inline]
    const fn special_is_empty(self) -> bool {
        debug_assert!(self.is_special());
        self.0 & 0x01 != 0
    }
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    const fn full(hash: u64) -> Tag {}
}
