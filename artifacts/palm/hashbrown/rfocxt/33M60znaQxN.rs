use crate::alloc::alloc::{alloc, dealloc, Layout};
use core::ptr::NonNull;
pub(crate) fn do_alloc<A: Allocator>(
    alloc: &A,
    layout: Layout,
) -> Result<NonNull<u8>, ()> {
    alloc.allocate(layout)
}
