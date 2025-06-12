use core::iter::FromIterator;
use core::mem::{self, ManuallyDrop};
use core::ops::{Deref, RangeBounds};
use core::ptr::NonNull;
use core::{cmp, fmt, hash, ptr, slice, usize};
use alloc::{
    alloc::{dealloc, Layout},
    borrow::Borrow, boxed::Box, string::String, vec::Vec,
};
use crate::buf::IntoIter;
#[allow(unused)]
use crate::loom::sync::atomic::AtomicMut;
use crate::loom::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use crate::{offset_from, Buf, BytesMut};
static OWNED_VTABLE: Vtable = Vtable {
    clone: owned_clone,
    into_vec: owned_to_vec,
    into_mut: owned_to_mut,
    is_unique: owned_is_unique,
    drop: owned_drop,
};
static PROMOTABLE_EVEN_VTABLE: Vtable = Vtable {
    clone: promotable_even_clone,
    into_vec: promotable_even_to_vec,
    into_mut: promotable_even_to_mut,
    is_unique: promotable_is_unique,
    drop: promotable_even_drop,
};
static PROMOTABLE_ODD_VTABLE: Vtable = Vtable {
    clone: promotable_odd_clone,
    into_vec: promotable_odd_to_vec,
    into_mut: promotable_odd_to_mut,
    is_unique: promotable_is_unique,
    drop: promotable_odd_drop,
};
static SHARED_VTABLE: Vtable = Vtable {
    clone: shared_clone,
    into_vec: shared_to_vec,
    into_mut: shared_to_mut,
    is_unique: shared_is_unique,
    drop: shared_drop,
};
const STATIC_VTABLE: Vtable = Vtable {
    clone: static_clone,
    into_vec: static_to_vec,
    into_mut: static_to_mut,
    is_unique: static_is_unique,
    drop: static_drop,
};
const _: [(); 0 - mem::align_of::<Shared>() % 2] = [];
const KIND_ARC: usize = 0b0;
const KIND_VEC: usize = 0b1;
const KIND_MASK: usize = 0b1;
pub struct BytesMut {
    ptr: NonNull<u8>,
    len: usize,
    cap: usize,
    data: *mut Shared,
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
    pub fn extend_from_slice(&mut self, extend: &[u8]) {}
    pub fn unsplit(&mut self, other: BytesMut) {}
    #[inline]
    pub(crate) fn from_vec(vec: Vec<u8>) -> BytesMut {
        let mut vec = ManuallyDrop::new(vec);
        let ptr = vptr(vec.as_mut_ptr());
        let len = vec.len();
        let cap = vec.capacity();
        let original_capacity_repr = original_capacity_to_repr(cap);
        let data = (original_capacity_repr << ORIGINAL_CAPACITY_OFFSET) | KIND_VEC;
        BytesMut {
            ptr,
            len,
            cap,
            data: invalid_ptr(data),
        }
    }
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
unsafe fn owned_to_mut(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> BytesMut {
    BytesMut::from_vec(owned_to_vec(data, ptr, len))
}
unsafe fn owned_to_vec(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Vec<u8> {
    let slice = slice::from_raw_parts(ptr, len);
    let vec = slice.to_vec();
    owned_drop_impl(data.load(Ordering::Relaxed));
    vec
}
