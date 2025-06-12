use core::fmt;
use core::mem::MaybeUninit;
use core::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
    RangeToInclusive,
};
macro_rules! impl_index {
    ($($t:ty),*) => {
        $(impl Index <$t > for UninitSlice { type Output = UninitSlice; #[inline] fn
        index(& self, index : $t) -> & UninitSlice { UninitSlice::uninit_ref(& self
        .0[index]) } } impl IndexMut <$t > for UninitSlice { #[inline] fn index_mut(& mut
        self, index : $t) -> & mut UninitSlice { UninitSlice::uninit(& mut self.0[index])
        } })*
    };
}
impl_index!(
    Range < usize >, RangeFrom < usize >, RangeFull, RangeInclusive < usize >, RangeTo <
    usize >, RangeToInclusive < usize >
);
#[repr(transparent)]
pub struct UninitSlice([MaybeUninit<u8>]);
impl UninitSlice {
    #[inline]
    pub fn new(slice: &mut [u8]) -> &mut UninitSlice {}
    #[inline]
    pub fn uninit(slice: &mut [MaybeUninit<u8>]) -> &mut UninitSlice {
        unsafe { &mut *(slice as *mut [MaybeUninit<u8>] as *mut UninitSlice) }
    }
    fn uninit_ref(slice: &[MaybeUninit<u8>]) -> &UninitSlice {}
    #[inline]
    pub unsafe fn from_raw_parts_mut<'a>(
        ptr: *mut u8,
        len: usize,
    ) -> &'a mut UninitSlice {
        let maybe_init: &mut [MaybeUninit<u8>] = core::slice::from_raw_parts_mut(
            ptr as *mut _,
            len,
        );
        Self::uninit(maybe_init)
    }
    #[inline]
    pub fn write_byte(&mut self, index: usize, byte: u8) {}
    #[inline]
    pub fn copy_from_slice(&mut self, src: &[u8]) {}
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {}
    #[inline]
    pub unsafe fn as_uninit_slice_mut(&mut self) -> &mut [MaybeUninit<u8>] {}
    #[inline]
    pub fn len(&self) -> usize {}
}
