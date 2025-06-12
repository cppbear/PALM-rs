use core::iter::FromIterator;
use core::mem::{self, ManuallyDrop, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::ptr::{self, NonNull};
use core::{cmp, fmt, hash, isize, slice, usize};
use alloc::{
    borrow::{Borrow, BorrowMut},
    boxed::Box, string::String, vec, vec::Vec,
};
use crate::buf::{IntoIter, UninitSlice};
use crate::bytes::Vtable;
#[allow(unused)]
use crate::loom::sync::atomic::AtomicMut;
use crate::loom::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use crate::{offset_from, Buf, BufMut, Bytes, TryGetError};
static SHARED_VTABLE: Vtable = Vtable {
    clone: shared_v_clone,
    into_vec: shared_v_to_vec,
    into_mut: shared_v_to_mut,
    is_unique: shared_v_is_unique,
    drop: shared_v_drop,
};
const _: [(); 0 - mem::align_of::<Shared>() % 2] = [];
const KIND_ARC: usize = 0b0;
const KIND_VEC: usize = 0b1;
const KIND_MASK: usize = 0b1;
const MAX_ORIGINAL_CAPACITY_WIDTH: usize = 17;
const MIN_ORIGINAL_CAPACITY_WIDTH: usize = 10;
const ORIGINAL_CAPACITY_MASK: usize = 0b11100;
const ORIGINAL_CAPACITY_OFFSET: usize = 2;
const VEC_POS_OFFSET: usize = 5;
const MAX_VEC_POS: usize = usize::MAX >> VEC_POS_OFFSET;
const NOT_VEC_POS_MASK: usize = 0b11111;
#[cfg(target_pointer_width = "64")]
const PTR_WIDTH: usize = 64;
#[cfg(target_pointer_width = "32")]
const PTR_WIDTH: usize = 32;
pub unsafe trait BufMut {
    fn remaining_mut(&self) -> usize;
    unsafe fn advance_mut(&mut self, cnt: usize);
    #[inline]
    fn has_remaining_mut(&self) -> bool;
    #[cfg_attr(docsrs, doc(alias = "bytes_mut"))]
    fn chunk_mut(&mut self) -> &mut UninitSlice;
    #[inline]
    fn put<T: super::Buf>(&mut self, mut src: T)
    where
        Self: Sized;
    #[inline]
    fn put_slice(&mut self, mut src: &[u8]);
    #[inline]
    fn put_bytes(&mut self, val: u8, mut cnt: usize);
    #[inline]
    fn put_u8(&mut self, n: u8);
    #[inline]
    fn put_i8(&mut self, n: i8);
    #[inline]
    fn put_u16(&mut self, n: u16);
    #[inline]
    fn put_u16_le(&mut self, n: u16);
    #[inline]
    fn put_u16_ne(&mut self, n: u16);
    #[inline]
    fn put_i16(&mut self, n: i16);
    #[inline]
    fn put_i16_le(&mut self, n: i16);
    #[inline]
    fn put_i16_ne(&mut self, n: i16);
    #[inline]
    fn put_u32(&mut self, n: u32);
    #[inline]
    fn put_u32_le(&mut self, n: u32);
    #[inline]
    fn put_u32_ne(&mut self, n: u32);
    #[inline]
    fn put_i32(&mut self, n: i32);
    #[inline]
    fn put_i32_le(&mut self, n: i32);
    #[inline]
    fn put_i32_ne(&mut self, n: i32);
    #[inline]
    fn put_u64(&mut self, n: u64);
    #[inline]
    fn put_u64_le(&mut self, n: u64);
    #[inline]
    fn put_u64_ne(&mut self, n: u64);
    #[inline]
    fn put_i64(&mut self, n: i64);
    #[inline]
    fn put_i64_le(&mut self, n: i64);
    #[inline]
    fn put_i64_ne(&mut self, n: i64);
    #[inline]
    fn put_u128(&mut self, n: u128);
    #[inline]
    fn put_u128_le(&mut self, n: u128);
    #[inline]
    fn put_u128_ne(&mut self, n: u128);
    #[inline]
    fn put_i128(&mut self, n: i128);
    #[inline]
    fn put_i128_le(&mut self, n: i128);
    #[inline]
    fn put_i128_ne(&mut self, n: i128);
    #[inline]
    fn put_uint(&mut self, n: u64, nbytes: usize);
    #[inline]
    fn put_uint_le(&mut self, n: u64, nbytes: usize);
    #[inline]
    fn put_uint_ne(&mut self, n: u64, nbytes: usize);
    #[inline]
    fn put_int(&mut self, n: i64, nbytes: usize);
    #[inline]
    fn put_int_le(&mut self, n: i64, nbytes: usize);
    #[inline]
    fn put_int_ne(&mut self, n: i64, nbytes: usize);
    #[inline]
    fn put_f32(&mut self, n: f32);
    #[inline]
    fn put_f32_le(&mut self, n: f32);
    #[inline]
    fn put_f32_ne(&mut self, n: f32);
    #[inline]
    fn put_f64(&mut self, n: f64);
    #[inline]
    fn put_f64_le(&mut self, n: f64);
    #[inline]
    fn put_f64_ne(&mut self, n: f64);
    #[inline]
    fn limit(self, limit: usize) -> Limit<Self>
    where
        Self: Sized,
    {
        limit::new(self, limit)
    }
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    #[inline]
    fn writer(self) -> Writer<Self>
    where
        Self: Sized,
    {
        writer::new(self)
    }
    #[inline]
    fn chain_mut<U: BufMut>(self, next: U) -> Chain<Self, U>
    where
        Self: Sized,
    {
        Chain::new(self, next)
    }
}
pub struct BytesMut {
    ptr: NonNull<u8>,
    len: usize,
    cap: usize,
    data: *mut Shared,
}
struct Shared {
    buf: *mut u8,
    cap: usize,
    ref_cnt: AtomicUsize,
}
struct Shared {
    vec: Vec<u8>,
    original_capacity_repr: usize,
    ref_count: AtomicUsize,
}
impl BytesMut {
    #[inline]
    pub fn with_capacity(capacity: usize) -> BytesMut {}
    #[inline]
    pub fn new() -> BytesMut {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn capacity(&self) -> usize {}
    #[inline]
    pub fn freeze(self) -> Bytes {}
    pub fn zeroed(len: usize) -> BytesMut {}
    #[must_use = "consider BytesMut::truncate if you don't need the other half"]
    pub fn split_off(&mut self, at: usize) -> BytesMut {}
    #[must_use = "consider BytesMut::clear if you don't need the other half"]
    pub fn split(&mut self) -> BytesMut {}
    #[must_use = "consider BytesMut::advance if you don't need the other half"]
    pub fn split_to(&mut self, at: usize) -> BytesMut {}
    pub fn truncate(&mut self, len: usize) {}
    pub fn clear(&mut self) {}
    pub fn resize(&mut self, new_len: usize, value: u8) {}
    #[inline]
    pub unsafe fn set_len(&mut self, len: usize) {}
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        let len = self.len();
        let rem = self.capacity() - len;
        if additional <= rem {
            return;
        }
        let _ = self.reserve_inner(additional, true);
    }
    fn reserve_inner(&mut self, additional: usize, allocate: bool) -> bool {}
    #[inline]
    #[must_use = "consider BytesMut::reserve if you need an infallible reservation"]
    pub fn try_reclaim(&mut self, additional: usize) -> bool {}
    #[inline]
    pub fn extend_from_slice(&mut self, extend: &[u8]) {
        let cnt = extend.len();
        self.reserve(cnt);
        unsafe {
            let dst = self.spare_capacity_mut();
            debug_assert!(dst.len() >= cnt);
            ptr::copy_nonoverlapping(extend.as_ptr(), dst.as_mut_ptr().cast(), cnt);
        }
        unsafe {
            self.advance_mut(cnt);
        }
    }
    pub fn unsplit(&mut self, other: BytesMut) {}
    #[inline]
    pub(crate) fn from_vec(vec: Vec<u8>) -> BytesMut {}
    #[inline]
    fn as_slice(&self) -> &[u8] {}
    #[inline]
    fn as_slice_mut(&mut self) -> &mut [u8] {}
    pub(crate) unsafe fn advance_unchecked(&mut self, count: usize) {}
    fn try_unsplit(&mut self, other: BytesMut) -> Result<(), BytesMut> {}
    #[inline]
    fn kind(&self) -> usize {}
    unsafe fn promote_to_shared(&mut self, ref_cnt: usize) {}
    #[inline]
    unsafe fn shallow_clone(&mut self) -> BytesMut {}
    #[inline]
    unsafe fn get_vec_pos(&self) -> usize {}
    #[inline]
    unsafe fn set_vec_pos(&mut self, pos: usize) {}
    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        unsafe {
            let ptr = self.ptr.as_ptr().add(self.len);
            let len = self.cap - self.len;
            slice::from_raw_parts_mut(ptr.cast(), len)
        }
    }
}
