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
pub trait Buf {
    fn remaining(&self) -> usize;
    #[cfg_attr(docsrs, doc(alias = "bytes"))]
    fn chunk(&self) -> &[u8];
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize;
    fn advance(&mut self, cnt: usize);
    fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }
    fn copy_to_slice(&mut self, dst: &mut [u8]);
    fn get_u8(&mut self) -> u8;
    fn get_i8(&mut self) -> i8;
    fn get_u16(&mut self) -> u16;
    fn get_u16_le(&mut self) -> u16;
    fn get_u16_ne(&mut self) -> u16;
    fn get_i16(&mut self) -> i16;
    fn get_i16_le(&mut self) -> i16;
    fn get_i16_ne(&mut self) -> i16;
    fn get_u32(&mut self) -> u32;
    fn get_u32_le(&mut self) -> u32;
    fn get_u32_ne(&mut self) -> u32;
    fn get_i32(&mut self) -> i32;
    fn get_i32_le(&mut self) -> i32;
    fn get_i32_ne(&mut self) -> i32;
    fn get_u64(&mut self) -> u64;
    fn get_u64_le(&mut self) -> u64;
    fn get_u64_ne(&mut self) -> u64;
    fn get_i64(&mut self) -> i64;
    fn get_i64_le(&mut self) -> i64;
    fn get_i64_ne(&mut self) -> i64;
    fn get_u128(&mut self) -> u128;
    fn get_u128_le(&mut self) -> u128;
    fn get_u128_ne(&mut self) -> u128;
    fn get_i128(&mut self) -> i128;
    fn get_i128_le(&mut self) -> i128;
    fn get_i128_ne(&mut self) -> i128;
    fn get_uint(&mut self, nbytes: usize) -> u64;
    fn get_uint_le(&mut self, nbytes: usize) -> u64;
    fn get_uint_ne(&mut self, nbytes: usize) -> u64;
    fn get_int(&mut self, nbytes: usize) -> i64;
    fn get_int_le(&mut self, nbytes: usize) -> i64;
    fn get_int_ne(&mut self, nbytes: usize) -> i64;
    fn get_f32(&mut self) -> f32;
    fn get_f32_le(&mut self) -> f32;
    fn get_f32_ne(&mut self) -> f32;
    fn get_f64(&mut self) -> f64;
    fn get_f64_le(&mut self) -> f64;
    fn get_f64_ne(&mut self) -> f64;
    fn try_copy_to_slice(&mut self, mut dst: &mut [u8]) -> Result<(), TryGetError>;
    fn try_get_u8(&mut self) -> Result<u8, TryGetError>;
    fn try_get_i8(&mut self) -> Result<i8, TryGetError>;
    fn try_get_u16(&mut self) -> Result<u16, TryGetError>;
    fn try_get_u16_le(&mut self) -> Result<u16, TryGetError>;
    fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError>;
    fn try_get_i16(&mut self) -> Result<i16, TryGetError>;
    fn try_get_i16_le(&mut self) -> Result<i16, TryGetError>;
    fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError>;
    fn try_get_u32(&mut self) -> Result<u32, TryGetError>;
    fn try_get_u32_le(&mut self) -> Result<u32, TryGetError>;
    fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError>;
    fn try_get_i32(&mut self) -> Result<i32, TryGetError>;
    fn try_get_i32_le(&mut self) -> Result<i32, TryGetError>;
    fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError>;
    fn try_get_u64(&mut self) -> Result<u64, TryGetError>;
    fn try_get_u64_le(&mut self) -> Result<u64, TryGetError>;
    fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError>;
    fn try_get_i64(&mut self) -> Result<i64, TryGetError>;
    fn try_get_i64_le(&mut self) -> Result<i64, TryGetError>;
    fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError>;
    fn try_get_u128(&mut self) -> Result<u128, TryGetError>;
    fn try_get_u128_le(&mut self) -> Result<u128, TryGetError>;
    fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError>;
    fn try_get_i128(&mut self) -> Result<i128, TryGetError>;
    fn try_get_i128_le(&mut self) -> Result<i128, TryGetError>;
    fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError>;
    fn try_get_uint(&mut self, nbytes: usize) -> Result<u64, TryGetError>;
    fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError>;
    fn try_get_uint_ne(&mut self, nbytes: usize) -> Result<u64, TryGetError>;
    fn try_get_int(&mut self, nbytes: usize) -> Result<i64, TryGetError>;
    fn try_get_int_le(&mut self, nbytes: usize) -> Result<i64, TryGetError>;
    fn try_get_int_ne(&mut self, nbytes: usize) -> Result<i64, TryGetError>;
    fn try_get_f32(&mut self) -> Result<f32, TryGetError>;
    fn try_get_f32_le(&mut self) -> Result<f32, TryGetError>;
    fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError>;
    fn try_get_f64(&mut self) -> Result<f64, TryGetError>;
    fn try_get_f64_le(&mut self) -> Result<f64, TryGetError>;
    fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError>;
    fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes;
    fn take(self, limit: usize) -> Take<Self>
    where
        Self: Sized,
    {
        take::new(self, limit)
    }
    fn chain<U: Buf>(self, next: U) -> Chain<Self, U>
    where
        Self: Sized,
    {
        Chain::new(self, next)
    }
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn reader(self) -> Reader<Self>
    where
        Self: Sized,
    {
        reader::new(self)
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
unsafe impl BufMut for BytesMut {
    #[inline]
    fn remaining_mut(&self) -> usize {}
    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {}
    #[inline]
    fn chunk_mut(&mut self) -> &mut UninitSlice {}
    fn put<T: Buf>(&mut self, mut src: T)
    where
        Self: Sized,
    {
        while src.has_remaining() {
            let s = src.chunk();
            let l = s.len();
            self.extend_from_slice(s);
            src.advance(l);
        }
    }
    fn put_slice(&mut self, src: &[u8]) {}
    fn put_bytes(&mut self, val: u8, cnt: usize) {}
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
    pub fn reserve(&mut self, additional: usize) {}
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
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<u8>] {}
}
