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
pub struct Bucket<T> {
    ptr: NonNull<T>,
}
impl<T> Bucket<T> {
    #[inline]
    unsafe fn from_base_index(base: NonNull<T>, index: usize) -> Self {
        let ptr = if T::IS_ZERO_SIZED {
            invalid_mut(index + 1)
        } else {
            base.as_ptr().sub(index)
        };
        Self {
            ptr: NonNull::new_unchecked(ptr),
        }
    }
    #[inline]
    unsafe fn to_base_index(&self, base: NonNull<T>) -> usize {
        if T::IS_ZERO_SIZED {
            self.ptr.as_ptr() as usize - 1
        } else {
            offset_from(base.as_ptr(), self.ptr.as_ptr())
        }
    }
    #[inline]
    pub fn as_ptr(&self) -> *mut T {}
    #[inline]
    fn as_non_null(&self) -> NonNull<T> {}
    #[inline]
    unsafe fn next_n(&self, offset: usize) -> Self {
        let ptr = if T::IS_ZERO_SIZED {
            invalid_mut(self.ptr.as_ptr() as usize + offset)
        } else {
            self.ptr.as_ptr().sub(offset)
        };
        Self {
            ptr: NonNull::new_unchecked(ptr),
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) unsafe fn drop(&self) {}
    #[inline]
    pub(crate) unsafe fn read(&self) -> T {}
    #[inline]
    pub(crate) unsafe fn write(&self, val: T) {}
    #[inline]
    pub unsafe fn as_ref<'a>(&self) -> &'a T {}
    #[inline]
    pub unsafe fn as_mut<'a>(&self) -> &'a mut T {}
}
#[inline]
unsafe fn offset_from<T>(to: *const T, from: *const T) -> usize {
    to.offset_from(from) as usize
}
