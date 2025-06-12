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
pub struct RawIterHash<T> {
    inner: RawIterHashInner,
    _marker: PhantomData<T>,
}
#[derive(Clone)]
struct RawIterHashInner {
    bucket_mask: usize,
    ctrl: NonNull<u8>,
    tag_hash: Tag,
    probe_seq: ProbeSeq,
    group: Group,
    bitmask: BitMaskIter,
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
impl<T> RawIterHash<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    unsafe fn new<A: Allocator>(table: &RawTable<T, A>, hash: u64) -> Self {
        RawIterHash {
            inner: RawIterHashInner::new(&table.table, hash),
            _marker: PhantomData,
        }
    }
}
impl RawIterHashInner {
    #[cfg_attr(feature = "inline-more", inline)]
    unsafe fn new(table: &RawTableInner, hash: u64) -> Self {
        let tag_hash = Tag::full(hash);
        let probe_seq = table.probe_seq(hash);
        let group = Group::load(table.ctrl(probe_seq.pos));
        let bitmask = group.match_tag(tag_hash).into_iter();
        RawIterHashInner {
            bucket_mask: table.bucket_mask,
            ctrl: table.ctrl,
            tag_hash,
            probe_seq,
            group,
            bitmask,
        }
    }
}
