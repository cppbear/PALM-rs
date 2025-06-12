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
pub(crate) struct RawIterRange<T> {
    current_group: BitMaskIter,
    data: Bucket<T>,
    next_ctrl: *const u8,
    end: *const u8,
}
#[derive(Copy, Clone)]
pub(crate) struct BitMaskIter(pub(crate) BitMask);
#[derive(Copy, Clone)]
pub(crate) struct BitMask(pub(crate) BitMaskWord);
pub struct Bucket<T> {
    ptr: NonNull<T>,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub(crate) struct Tag(u8);
impl<T> RawIterRange<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    unsafe fn new(ctrl: *const u8, data: Bucket<T>, len: usize) -> Self {
        debug_assert_ne!(len, 0);
        debug_assert_eq!(ctrl as usize % Group::WIDTH, 0);
        let end = ctrl.add(len);
        let current_group = Group::load_aligned(ctrl.cast()).match_full();
        let next_ctrl = ctrl.add(Group::WIDTH);
        Self {
            current_group: current_group.into_iter(),
            data,
            next_ctrl,
            end,
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    pub(crate) fn split(mut self) -> (Self, Option<RawIterRange<T>>) {
        unsafe {
            if self.end <= self.next_ctrl {
                (self, None)
            } else {
                let len = offset_from(self.end, self.next_ctrl);
                debug_assert_eq!(len % Group::WIDTH, 0);
                let mid = (len / 2) & !(Group::WIDTH - 1);
                let tail = Self::new(
                    self.next_ctrl.add(mid),
                    self.data.next_n(Group::WIDTH).next_n(mid),
                    len - mid,
                );
                debug_assert_eq!(
                    self.data.next_n(Group::WIDTH).next_n(mid).ptr, tail.data.ptr
                );
                debug_assert_eq!(self.end, tail.end);
                self.end = self.next_ctrl.add(mid);
                debug_assert_eq!(self.end.add(Group::WIDTH), tail.next_ctrl);
                (self, Some(tail))
            }
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    unsafe fn next_impl<const DO_CHECK_PTR_RANGE: bool>(&mut self) -> Option<Bucket<T>> {}
    #[allow(clippy::while_let_on_iterator)]
    #[cfg_attr(feature = "inline-more", inline)]
    unsafe fn fold_impl<F, B>(mut self, mut n: usize, mut acc: B, mut f: F) -> B
    where
        F: FnMut(B, Bucket<T>) -> B,
    {
        loop {
            while let Some(index) = self.current_group.next() {
                debug_assert!(n != 0);
                let bucket = self.data.next_n(index);
                acc = f(acc, bucket);
                n -= 1;
            }
            if n == 0 {
                return acc;
            }
            self.current_group = Group::load_aligned(self.next_ctrl.cast())
                .match_full()
                .into_iter();
            self.data = self.data.next_n(Group::WIDTH);
            self.next_ctrl = self.next_ctrl.add(Group::WIDTH);
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
    unsafe fn to_base_index(&self, base: NonNull<T>) -> usize {}
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
