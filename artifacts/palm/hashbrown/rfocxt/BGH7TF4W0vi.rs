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
trait RawTableClone {
    unsafe fn clone_from_spec(&mut self, source: &Self);
}
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}
pub struct RawTable<T, A: Allocator = Global> {
    table: RawTableInner,
    alloc: A,
    marker: PhantomData<T>,
}
struct RawTableInner {
    bucket_mask: usize,
    ctrl: NonNull<u8>,
    growth_left: usize,
    items: usize,
}
#[cfg(feature = "nightly")]
impl<T: Copy, A: Allocator + Clone> RawTableClone for RawTable<T, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    unsafe fn clone_from_spec(&mut self, source: &Self) {
        source
            .table
            .ctrl(0)
            .copy_to_nonoverlapping(self.table.ctrl(0), self.table.num_ctrl_bytes());
        source
            .data_start()
            .as_ptr()
            .copy_to_nonoverlapping(self.data_start().as_ptr(), self.table.buckets());
        self.table.items = source.table.items;
        self.table.growth_left = source.table.growth_left;
    }
}
impl<T: Clone, A: Allocator + Clone> RawTable<T, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    unsafe fn clone_from_impl(&mut self, source: &Self) {
        source
            .table
            .ctrl(0)
            .copy_to_nonoverlapping(self.table.ctrl(0), self.table.num_ctrl_bytes());
        let mut guard = guard(
            (0, &mut *self),
            |(index, self_)| {
                if T::NEEDS_DROP {
                    for i in 0..*index {
                        if self_.is_bucket_full(i) {
                            self_.bucket(i).drop();
                        }
                    }
                }
            },
        );
        for from in source.iter() {
            let index = source.bucket_index(&from);
            let to = guard.1.bucket(index);
            to.write(from.as_ref().clone());
            guard.0 = index + 1;
        }
        mem::forget(guard);
        self.table.items = source.table.items;
        self.table.growth_left = source.table.growth_left;
    }
}
