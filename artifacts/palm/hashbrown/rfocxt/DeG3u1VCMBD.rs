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
struct RawTableInner {
    bucket_mask: usize,
    ctrl: NonNull<u8>,
    growth_left: usize,
    items: usize,
}
impl RawTableInner {
    #[cfg_attr(feature = "inline-more", inline)]
    unsafe fn new_uninitialized<A>(
        alloc: &A,
        table_layout: TableLayout,
        buckets: usize,
        fallibility: Fallibility,
    ) -> Result<Self, TryReserveError>
    where
        A: Allocator,
    {
        debug_assert!(buckets.is_power_of_two());
        let (layout, ctrl_offset) = match table_layout.calculate_layout_for(buckets) {
            Some(lco) => lco,
            None => return Err(fallibility.capacity_overflow()),
        };
        let ptr: NonNull<u8> = match do_alloc(alloc, layout) {
            Ok(block) => block.cast(),
            Err(_) => return Err(fallibility.alloc_err(layout)),
        };
        let ctrl = NonNull::new_unchecked(ptr.as_ptr().add(ctrl_offset));
        Ok(Self {
            ctrl,
            bucket_mask: buckets - 1,
            items: 0,
            growth_left: bucket_mask_to_capacity(buckets - 1),
        })
    }
    #[inline]
    fn fallible_with_capacity<A>(
        alloc: &A,
        table_layout: TableLayout,
        capacity: usize,
        fallibility: Fallibility,
    ) -> Result<Self, TryReserveError>
    where
        A: Allocator,
    {
        if capacity == 0 {
            Ok(Self::NEW)
        } else {
            unsafe {
                let buckets = capacity_to_buckets(capacity)
                    .ok_or_else(|| fallibility.capacity_overflow())?;
                let result = Self::new_uninitialized(
                    alloc,
                    table_layout,
                    buckets,
                    fallibility,
                )?;
                result.ctrl(0).write_bytes(Tag::EMPTY.0, result.num_ctrl_bytes());
                Ok(result)
            }
        }
    }
    fn with_capacity<A>(alloc: &A, table_layout: TableLayout, capacity: usize) -> Self
    where
        A: Allocator,
    {
        match Self::fallible_with_capacity(
            alloc,
            table_layout,
            capacity,
            Fallibility::Infallible,
        ) {
            Ok(table_inner) => table_inner,
            Err(_) => unsafe { hint::unreachable_unchecked() }
        }
    }
    #[inline]
    unsafe fn fix_insert_slot(&self, mut index: usize) -> InsertSlot {}
    #[inline]
    fn find_insert_slot_in_group(
        &self,
        group: &Group,
        probe_seq: &ProbeSeq,
    ) -> Option<usize> {}
    #[inline]
    unsafe fn find_or_find_insert_slot_inner(
        &self,
        hash: u64,
        eq: &mut dyn FnMut(usize) -> bool,
    ) -> Result<usize, InsertSlot> {}
    #[inline]
    unsafe fn prepare_insert_slot(&mut self, hash: u64) -> (usize, Tag) {}
    #[inline]
    unsafe fn find_insert_slot(&self, hash: u64) -> InsertSlot {}
    #[inline(always)]
    unsafe fn find_inner(
        &self,
        hash: u64,
        eq: &mut dyn FnMut(usize) -> bool,
    ) -> Option<usize> {}
    #[allow(clippy::mut_mut)]
    #[inline]
    unsafe fn prepare_rehash_in_place(&mut self) {}
    #[inline]
    unsafe fn iter<T>(&self) -> RawIter<T> {}
    unsafe fn drop_elements<T>(&mut self) {}
    unsafe fn drop_inner_table<T, A: Allocator>(
        &mut self,
        alloc: &A,
        table_layout: TableLayout,
    ) {}
    #[inline]
    unsafe fn bucket<T>(&self, index: usize) -> Bucket<T> {}
    #[inline]
    unsafe fn bucket_ptr(&self, index: usize, size_of: usize) -> *mut u8 {
        debug_assert_ne!(self.bucket_mask, 0);
        debug_assert!(index < self.buckets());
        let base: *mut u8 = self.data_end().as_ptr();
        base.sub((index + 1) * size_of)
    }
    #[inline]
    fn data_end<T>(&self) -> NonNull<T> {
        self.ctrl.cast()
    }
    #[inline]
    fn probe_seq(&self, hash: u64) -> ProbeSeq {}
    #[inline]
    unsafe fn record_item_insert_at(&mut self, index: usize, old_ctrl: Tag, hash: u64) {}
    #[inline]
    fn is_in_same_group(&self, i: usize, new_i: usize, hash: u64) -> bool {}
    #[inline]
    unsafe fn set_ctrl_hash(&mut self, index: usize, hash: u64) {}
    #[inline]
    unsafe fn replace_ctrl_hash(&mut self, index: usize, hash: u64) -> Tag {}
    #[inline]
    unsafe fn set_ctrl(&mut self, index: usize, ctrl: Tag) {}
    #[inline]
    unsafe fn ctrl(&self, index: usize) -> *mut Tag {}
    #[inline]
    fn buckets(&self) -> usize {
        self.bucket_mask + 1
    }
    #[inline]
    unsafe fn is_bucket_full(&self, index: usize) -> bool {}
    #[inline]
    fn num_ctrl_bytes(&self) -> usize {}
    #[inline]
    fn is_empty_singleton(&self) -> bool {}
    #[allow(clippy::mut_mut)]
    #[inline]
    fn prepare_resize<'a, A>(
        &self,
        alloc: &'a A,
        table_layout: TableLayout,
        capacity: usize,
        fallibility: Fallibility,
    ) -> Result<
        crate::scopeguard::ScopeGuard<Self, impl FnMut(&mut Self) + 'a>,
        TryReserveError,
    >
    where
        A: Allocator,
    {
        debug_assert!(self.items <= capacity);
        let new_table = RawTableInner::fallible_with_capacity(
            alloc,
            table_layout,
            capacity,
            fallibility,
        )?;
        Ok(
            guard(
                new_table,
                move |self_| {
                    if !self_.is_empty_singleton() {
                        unsafe { self_.free_buckets(alloc, table_layout) };
                    }
                },
            ),
        )
    }
    #[allow(clippy::inline_always)]
    #[inline(always)]
    unsafe fn reserve_rehash_inner<A>(
        &mut self,
        alloc: &A,
        additional: usize,
        hasher: &dyn Fn(&mut Self, usize) -> u64,
        fallibility: Fallibility,
        layout: TableLayout,
        drop: Option<unsafe fn(*mut u8)>,
    ) -> Result<(), TryReserveError>
    where
        A: Allocator,
    {}
    #[inline(always)]
    unsafe fn full_buckets_indices(&self) -> FullBucketsIndices {}
    #[allow(clippy::inline_always)]
    #[inline(always)]
    unsafe fn resize_inner<A>(
        &mut self,
        alloc: &A,
        capacity: usize,
        hasher: &dyn Fn(&mut Self, usize) -> u64,
        fallibility: Fallibility,
        layout: TableLayout,
    ) -> Result<(), TryReserveError>
    where
        A: Allocator,
    {}
    #[allow(clippy::inline_always)]
    #[cfg_attr(feature = "inline-more", inline(always))]
    #[cfg_attr(not(feature = "inline-more"), inline)]
    unsafe fn rehash_in_place(
        &mut self,
        hasher: &dyn Fn(&mut Self, usize) -> u64,
        size_of: usize,
        drop: Option<unsafe fn(*mut u8)>,
    ) {}
    #[inline]
    unsafe fn free_buckets<A>(&mut self, alloc: &A, table_layout: TableLayout)
    where
        A: Allocator,
    {}
    #[inline]
    unsafe fn allocation_info(
        &self,
        table_layout: TableLayout,
    ) -> (NonNull<u8>, Layout) {}
    #[inline]
    unsafe fn allocation_size_or_zero(&self, table_layout: TableLayout) -> usize {}
    #[inline]
    fn clear_no_drop(&mut self) {}
    #[inline]
    unsafe fn erase(&mut self, index: usize) {}
}
