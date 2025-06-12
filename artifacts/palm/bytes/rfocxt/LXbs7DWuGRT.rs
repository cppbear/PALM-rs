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
pub struct Bytes {
    ptr: *const u8,
    len: usize,
    data: AtomicPtr<()>,
    vtable: &'static Vtable,
}
struct Shared {
    vec: Vec<u8>,
    original_capacity_repr: usize,
    ref_count: AtomicUsize,
}
pub(crate) struct Vtable {
    /// fn(data, ptr, len)
    pub clone: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Bytes,
    /// fn(data, ptr, len)
    ///
    /// `into_*` consumes the `Bytes`, returning the respective value.
    pub into_vec: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Vec<u8>,
    pub into_mut: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> BytesMut,
    /// fn(data)
    pub is_unique: unsafe fn(&AtomicPtr<()>) -> bool,
    /// fn(data, ptr, len)
    pub drop: unsafe fn(&mut AtomicPtr<()>, *const u8, usize),
}
impl Bytes {
    #[inline]
    #[cfg(not(all(loom, test)))]
    pub const fn new() -> Self {
        const EMPTY: &[u8] = &[];
        Bytes::from_static(EMPTY)
    }
    #[cfg(all(loom, test))]
    pub fn new() -> Self {
        const EMPTY: &[u8] = &[];
        Bytes::from_static(EMPTY)
    }
    #[inline]
    #[cfg(not(all(loom, test)))]
    pub const fn from_static(bytes: &'static [u8]) -> Self {
        Bytes {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
            data: AtomicPtr::new(ptr::null_mut()),
            vtable: &STATIC_VTABLE,
        }
    }
    #[cfg(all(loom, test))]
    pub fn from_static(bytes: &'static [u8]) -> Self {
        Bytes {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
            data: AtomicPtr::new(ptr::null_mut()),
            vtable: &STATIC_VTABLE,
        }
    }
    fn new_empty_with_ptr(ptr: *const u8) -> Self {
        debug_assert!(! ptr.is_null());
        let ptr = without_provenance(ptr as usize);
        Bytes {
            ptr,
            len: 0,
            data: AtomicPtr::new(ptr::null_mut()),
            vtable: &STATIC_VTABLE,
        }
    }
    pub fn from_owner<T>(owner: T) -> Self
    where
        T: AsRef<[u8]> + Send + 'static,
    {
        let owned = Box::into_raw(
            Box::new(Owned {
                lifetime: OwnedLifetime {
                    ref_cnt: AtomicUsize::new(1),
                    drop: owned_box_and_drop::<T>,
                },
                owner,
            }),
        );
        let mut ret = Bytes {
            ptr: NonNull::dangling().as_ptr(),
            len: 0,
            data: AtomicPtr::new(owned.cast()),
            vtable: &OWNED_VTABLE,
        };
        let buf = unsafe { &*owned }.owner.as_ref();
        ret.ptr = buf.as_ptr();
        ret.len = buf.len();
        ret
    }
    #[inline]
    pub const fn len(&self) -> usize {}
    #[inline]
    pub const fn is_empty(&self) -> bool {}
    pub fn is_unique(&self) -> bool {}
    pub fn copy_from_slice(data: &[u8]) -> Self {
        data.to_vec().into()
    }
    pub fn slice(&self, range: impl RangeBounds<usize>) -> Self {
        use core::ops::Bound;
        let len = self.len();
        let begin = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n.checked_add(1).expect("out of range"),
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&n) => n.checked_add(1).expect("out of range"),
            Bound::Excluded(&n) => n,
            Bound::Unbounded => len,
        };
        assert!(
            begin <= end, "range start must not be greater than end: {:?} <= {:?}",
            begin, end,
        );
        assert!(end <= len, "range end out of bounds: {:?} <= {:?}", end, len,);
        if end == begin {
            return Bytes::new();
        }
        let mut ret = self.clone();
        ret.len = end - begin;
        ret.ptr = unsafe { ret.ptr.add(begin) };
        ret
    }
    pub fn slice_ref(&self, subset: &[u8]) -> Self {
        if subset.is_empty() {
            return Bytes::new();
        }
        let bytes_p = self.as_ptr() as usize;
        let bytes_len = self.len();
        let sub_p = subset.as_ptr() as usize;
        let sub_len = subset.len();
        assert!(
            sub_p >= bytes_p,
            "subset pointer ({:p}) is smaller than self pointer ({:p})", subset.as_ptr(),
            self.as_ptr(),
        );
        assert!(
            sub_p + sub_len <= bytes_p + bytes_len,
            "subset is out of bounds: self = ({:p}, {}), subset = ({:p}, {})", self
            .as_ptr(), bytes_len, subset.as_ptr(), sub_len,
        );
        let sub_offset = sub_p - bytes_p;
        self.slice(sub_offset..(sub_offset + sub_len))
    }
    #[must_use = "consider Bytes::truncate if you don't need the other half"]
    pub fn split_off(&mut self, at: usize) -> Self {
        if at == self.len() {
            return Bytes::new_empty_with_ptr(self.ptr.wrapping_add(at));
        }
        if at == 0 {
            return mem::replace(self, Bytes::new_empty_with_ptr(self.ptr));
        }
        assert!(
            at <= self.len(), "split_off out of bounds: {:?} <= {:?}", at, self.len(),
        );
        let mut ret = self.clone();
        self.len = at;
        unsafe { ret.inc_start(at) };
        ret
    }
    #[must_use = "consider Bytes::advance if you don't need the other half"]
    pub fn split_to(&mut self, at: usize) -> Self {
        if at == self.len() {
            let end_ptr = self.ptr.wrapping_add(at);
            return mem::replace(self, Bytes::new_empty_with_ptr(end_ptr));
        }
        if at == 0 {
            return Bytes::new_empty_with_ptr(self.ptr);
        }
        assert!(
            at <= self.len(), "split_to out of bounds: {:?} <= {:?}", at, self.len(),
        );
        let mut ret = self.clone();
        unsafe { self.inc_start(at) };
        ret.len = at;
        ret
    }
    #[inline]
    pub fn truncate(&mut self, len: usize) {}
    #[inline]
    pub fn clear(&mut self) {}
    pub fn try_into_mut(self) -> Result<BytesMut, Bytes> {}
    #[inline]
    pub(crate) unsafe fn with_vtable(
        ptr: *const u8,
        len: usize,
        data: AtomicPtr<()>,
        vtable: &'static Vtable,
    ) -> Bytes {
        Bytes { ptr, len, data, vtable }
    }
    #[inline]
    fn as_slice(&self) -> &[u8] {}
    #[inline]
    unsafe fn inc_start(&mut self, by: usize) {}
}
unsafe fn shared_v_clone(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Bytes {
    let shared = data.load(Ordering::Relaxed) as *mut Shared;
    increment_shared(shared);
    let data = AtomicPtr::new(shared as *mut ());
    Bytes::with_vtable(ptr, len, data, &SHARED_VTABLE)
}
unsafe fn increment_shared(ptr: *mut Shared) {
    let old_size = (*ptr).ref_count.fetch_add(1, Ordering::Relaxed);
    if old_size > isize::MAX as usize {
        crate::abort();
    }
}
