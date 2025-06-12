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
    pub fn len(&self) -> usize {
        self.len
    }
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn capacity(&self) -> usize {
        self.cap
    }
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
    fn reserve_inner(&mut self, additional: usize, allocate: bool) -> bool {
        let len = self.len();
        let kind = self.kind();
        if kind == KIND_VEC {
            unsafe {
                let off = self.get_vec_pos();
                if self.capacity() - self.len() + off >= additional && off >= self.len()
                {
                    let base_ptr = self.ptr.as_ptr().sub(off);
                    ptr::copy_nonoverlapping(self.ptr.as_ptr(), base_ptr, self.len);
                    self.ptr = vptr(base_ptr);
                    self.set_vec_pos(0);
                    self.cap += off;
                } else {
                    if !allocate {
                        return false;
                    }
                    let mut v = ManuallyDrop::new(
                        rebuild_vec(self.ptr.as_ptr(), self.len, self.cap, off),
                    );
                    v.reserve(additional);
                    self.ptr = vptr(v.as_mut_ptr().add(off));
                    self.cap = v.capacity() - off;
                    debug_assert_eq!(self.len, v.len() - off);
                }
                return true;
            }
        }
        debug_assert_eq!(kind, KIND_ARC);
        let shared: *mut Shared = self.data;
        let mut new_cap = match len.checked_add(additional) {
            Some(new_cap) => new_cap,
            None if !allocate => return false,
            None => panic!("overflow"),
        };
        unsafe {
            if (*shared).is_unique() {
                let v = &mut (*shared).vec;
                let v_capacity = v.capacity();
                let ptr = v.as_mut_ptr();
                let offset = offset_from(self.ptr.as_ptr(), ptr);
                if v_capacity >= new_cap + offset {
                    self.cap = new_cap;
                } else if v_capacity >= new_cap && offset >= len {
                    ptr::copy_nonoverlapping(self.ptr.as_ptr(), ptr, len);
                    self.ptr = vptr(ptr);
                    self.cap = v.capacity();
                } else {
                    if !allocate {
                        return false;
                    }
                    let off = (self.ptr.as_ptr() as usize) - (v.as_ptr() as usize);
                    new_cap = new_cap.checked_add(off).expect("overflow");
                    let double = v.capacity().checked_shl(1).unwrap_or(new_cap);
                    new_cap = cmp::max(double, new_cap);
                    debug_assert!(off + len <= v.capacity());
                    v.set_len(off + len);
                    v.reserve(new_cap - v.len());
                    self.ptr = vptr(v.as_mut_ptr().add(off));
                    self.cap = v.capacity() - off;
                }
                return true;
            }
        }
        if !allocate {
            return false;
        }
        let original_capacity_repr = unsafe { (*shared).original_capacity_repr };
        let original_capacity = original_capacity_from_repr(original_capacity_repr);
        new_cap = cmp::max(new_cap, original_capacity);
        let mut v = ManuallyDrop::new(Vec::with_capacity(new_cap));
        v.extend_from_slice(self.as_ref());
        unsafe { release_shared(shared) };
        let data = (original_capacity_repr << ORIGINAL_CAPACITY_OFFSET) | KIND_VEC;
        self.data = invalid_ptr(data);
        self.ptr = vptr(v.as_mut_ptr());
        self.cap = v.capacity();
        debug_assert_eq!(self.len, v.len());
        return true;
    }
    #[inline]
    #[must_use = "consider BytesMut::reserve if you need an infallible reservation"]
    pub fn try_reclaim(&mut self, additional: usize) -> bool {
        let len = self.len();
        let rem = self.capacity() - len;
        if additional <= rem {
            return true;
        }
        self.reserve_inner(additional, false)
    }
    #[inline]
    pub fn extend_from_slice(&mut self, extend: &[u8]) {}
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
