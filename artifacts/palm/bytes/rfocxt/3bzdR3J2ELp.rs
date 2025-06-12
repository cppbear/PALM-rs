use crate::buf::UninitSlice;
use crate::BufMut;
use core::cmp;
pub unsafe trait BufMut {
    fn remaining_mut(&self) -> usize;
    unsafe fn advance_mut(&mut self, cnt: usize);
    #[inline]
    fn has_remaining_mut(&self) -> bool;
    #[cfg_attr(docsrs, doc(alias = "bytes_mut"))]
    fn chunk_mut(&mut self) -> &mut UninitSlice;
    #[inline]
    fn put<T: super::Buf>(&mut self, mut src: T)
    where
        Self: Sized;
    #[inline]
    fn put_slice(&mut self, mut src: &[u8]);
    #[inline]
    fn put_bytes(&mut self, val: u8, mut cnt: usize);
    #[inline]
    fn put_u8(&mut self, n: u8);
    #[inline]
    fn put_i8(&mut self, n: i8);
    #[inline]
    fn put_u16(&mut self, n: u16);
    #[inline]
    fn put_u16_le(&mut self, n: u16);
    #[inline]
    fn put_u16_ne(&mut self, n: u16);
    #[inline]
    fn put_i16(&mut self, n: i16);
    #[inline]
    fn put_i16_le(&mut self, n: i16);
    #[inline]
    fn put_i16_ne(&mut self, n: i16);
    #[inline]
    fn put_u32(&mut self, n: u32);
    #[inline]
    fn put_u32_le(&mut self, n: u32);
    #[inline]
    fn put_u32_ne(&mut self, n: u32);
    #[inline]
    fn put_i32(&mut self, n: i32);
    #[inline]
    fn put_i32_le(&mut self, n: i32);
    #[inline]
    fn put_i32_ne(&mut self, n: i32);
    #[inline]
    fn put_u64(&mut self, n: u64);
    #[inline]
    fn put_u64_le(&mut self, n: u64);
    #[inline]
    fn put_u64_ne(&mut self, n: u64);
    #[inline]
    fn put_i64(&mut self, n: i64);
    #[inline]
    fn put_i64_le(&mut self, n: i64);
    #[inline]
    fn put_i64_ne(&mut self, n: i64);
    #[inline]
    fn put_u128(&mut self, n: u128);
    #[inline]
    fn put_u128_le(&mut self, n: u128);
    #[inline]
    fn put_u128_ne(&mut self, n: u128);
    #[inline]
    fn put_i128(&mut self, n: i128);
    #[inline]
    fn put_i128_le(&mut self, n: i128);
    #[inline]
    fn put_i128_ne(&mut self, n: i128);
    #[inline]
    fn put_uint(&mut self, n: u64, nbytes: usize);
    #[inline]
    fn put_uint_le(&mut self, n: u64, nbytes: usize);
    #[inline]
    fn put_uint_ne(&mut self, n: u64, nbytes: usize);
    #[inline]
    fn put_int(&mut self, n: i64, nbytes: usize);
    #[inline]
    fn put_int_le(&mut self, n: i64, nbytes: usize);
    #[inline]
    fn put_int_ne(&mut self, n: i64, nbytes: usize);
    #[inline]
    fn put_f32(&mut self, n: f32);
    #[inline]
    fn put_f32_le(&mut self, n: f32);
    #[inline]
    fn put_f32_ne(&mut self, n: f32);
    #[inline]
    fn put_f64(&mut self, n: f64);
    #[inline]
    fn put_f64_le(&mut self, n: f64);
    #[inline]
    fn put_f64_ne(&mut self, n: f64);
    #[inline]
    fn limit(self, limit: usize) -> Limit<Self>
    where
        Self: Sized,
    {
        limit::new(self, limit)
    }
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    #[inline]
    fn writer(self) -> Writer<Self>
    where
        Self: Sized,
    {
        writer::new(self)
    }
    #[inline]
    fn chain_mut<U: BufMut>(self, next: U) -> Chain<Self, U>
    where
        Self: Sized,
    {
        Chain::new(self, next)
    }
}
#[derive(Debug)]
pub struct Limit<T> {
    inner: T,
    limit: usize,
}
#[repr(transparent)]
pub struct UninitSlice([MaybeUninit<u8>]);
unsafe impl<T: BufMut> BufMut for Limit<T> {
    fn remaining_mut(&self) -> usize {}
    fn chunk_mut(&mut self) -> &mut UninitSlice {
        let bytes = self.inner.chunk_mut();
        let end = cmp::min(bytes.len(), self.limit);
        &mut bytes[..end]
    }
    unsafe fn advance_mut(&mut self, cnt: usize) {}
}
impl UninitSlice {
    #[inline]
    pub fn new(slice: &mut [u8]) -> &mut UninitSlice {}
    #[inline]
    pub fn uninit(slice: &mut [MaybeUninit<u8>]) -> &mut UninitSlice {}
    fn uninit_ref(slice: &[MaybeUninit<u8>]) -> &UninitSlice {}
    #[inline]
    pub unsafe fn from_raw_parts_mut<'a>(
        ptr: *mut u8,
        len: usize,
    ) -> &'a mut UninitSlice {}
    #[inline]
    pub fn write_byte(&mut self, index: usize, byte: u8) {}
    #[inline]
    pub fn copy_from_slice(&mut self, src: &[u8]) {}
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {}
    #[inline]
    pub unsafe fn as_uninit_slice_mut(&mut self) -> &mut [MaybeUninit<u8>] {}
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
