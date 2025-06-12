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
pub struct Bytes {
    ptr: *const u8,
    len: usize,
    data: AtomicPtr<()>,
    vtable: &'static Vtable,
}
struct Shared {
    buf: *mut u8,
    cap: usize,
    ref_cnt: AtomicUsize,
}
unsafe fn promotable_even_clone(
    data: &AtomicPtr<()>,
    ptr: *const u8,
    len: usize,
) -> Bytes {
    let shared = data.load(Ordering::Acquire);
    let kind = shared as usize & KIND_MASK;
    if kind == KIND_ARC {
        shallow_clone_arc(shared.cast(), ptr, len)
    } else {
        debug_assert_eq!(kind, KIND_VEC);
        let buf = ptr_map(shared.cast(), |addr| addr & !KIND_MASK);
        shallow_clone_vec(data, shared, buf, ptr, len)
    }
}
#[cfg(not(miri))]
fn ptr_map<F>(ptr: *mut u8, f: F) -> *mut u8
where
    F: FnOnce(usize) -> usize,
{
    let old_addr = ptr as usize;
    let new_addr = f(old_addr);
    new_addr as *mut u8
}
unsafe fn shallow_clone_arc(shared: *mut Shared, ptr: *const u8, len: usize) -> Bytes {
    let old_size = (*shared).ref_cnt.fetch_add(1, Ordering::Relaxed);
    if old_size > usize::MAX >> 1 {
        crate::abort();
    }
    Bytes {
        ptr,
        len,
        data: AtomicPtr::new(shared as _),
        vtable: &SHARED_VTABLE,
    }
}
#[cold]
unsafe fn shallow_clone_vec(
    atom: &AtomicPtr<()>,
    ptr: *const (),
    buf: *mut u8,
    offset: *const u8,
    len: usize,
) -> Bytes {
    let shared = Box::new(Shared {
        buf,
        cap: offset_from(offset, buf) + len,
        ref_cnt: AtomicUsize::new(2),
    });
    let shared = Box::into_raw(shared);
    debug_assert!(
        0 == (shared as usize & KIND_MASK),
        "internal: Box<Shared> should have an aligned pointer",
    );
    match atom
        .compare_exchange(ptr as _, shared as _, Ordering::AcqRel, Ordering::Acquire)
    {
        Ok(actual) => {
            debug_assert!(actual as usize == ptr as usize);
            Bytes {
                ptr: offset,
                len,
                data: AtomicPtr::new(shared as _),
                vtable: &SHARED_VTABLE,
            }
        }
        Err(actual) => {
            let shared = Box::from_raw(shared);
            mem::forget(*shared);
            shallow_clone_arc(actual as _, offset, len)
        }
    }
}
