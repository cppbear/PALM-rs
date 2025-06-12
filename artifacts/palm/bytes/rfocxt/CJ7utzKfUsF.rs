use crate::buf::{limit, Chain, Limit, UninitSlice};
#[cfg(feature = "std")]
use crate::buf::{writer, Writer};
use crate::{panic_advance, panic_does_not_fit, TryGetError};
use core::{mem, ptr, usize};
use alloc::{boxed::Box, vec::Vec};
macro_rules! deref_forward_bufmut {
    () => {
        #[inline] fn remaining_mut(& self) -> usize { (** self).remaining_mut() }
        #[inline] fn chunk_mut(& mut self) -> & mut UninitSlice { (** self).chunk_mut() }
        #[inline] unsafe fn advance_mut(& mut self, cnt : usize) { (** self)
        .advance_mut(cnt) } #[inline] fn put_slice(& mut self, src : & [u8]) { (** self)
        .put_slice(src) } #[inline] fn put_u8(& mut self, n : u8) { (** self).put_u8(n) }
        #[inline] fn put_i8(& mut self, n : i8) { (** self).put_i8(n) } #[inline] fn
        put_u16(& mut self, n : u16) { (** self).put_u16(n) } #[inline] fn put_u16_le(&
        mut self, n : u16) { (** self).put_u16_le(n) } #[inline] fn put_u16_ne(& mut
        self, n : u16) { (** self).put_u16_ne(n) } #[inline] fn put_i16(& mut self, n :
        i16) { (** self).put_i16(n) } #[inline] fn put_i16_le(& mut self, n : i16) { (**
        self).put_i16_le(n) } #[inline] fn put_i16_ne(& mut self, n : i16) { (** self)
        .put_i16_ne(n) } #[inline] fn put_u32(& mut self, n : u32) { (** self).put_u32(n)
        } #[inline] fn put_u32_le(& mut self, n : u32) { (** self).put_u32_le(n) }
        #[inline] fn put_u32_ne(& mut self, n : u32) { (** self).put_u32_ne(n) }
        #[inline] fn put_i32(& mut self, n : i32) { (** self).put_i32(n) } #[inline] fn
        put_i32_le(& mut self, n : i32) { (** self).put_i32_le(n) } #[inline] fn
        put_i32_ne(& mut self, n : i32) { (** self).put_i32_ne(n) } #[inline] fn
        put_u64(& mut self, n : u64) { (** self).put_u64(n) } #[inline] fn put_u64_le(&
        mut self, n : u64) { (** self).put_u64_le(n) } #[inline] fn put_u64_ne(& mut
        self, n : u64) { (** self).put_u64_ne(n) } #[inline] fn put_i64(& mut self, n :
        i64) { (** self).put_i64(n) } #[inline] fn put_i64_le(& mut self, n : i64) { (**
        self).put_i64_le(n) } #[inline] fn put_i64_ne(& mut self, n : i64) { (** self)
        .put_i64_ne(n) }
    };
}
pub unsafe trait BufMut {
    fn remaining_mut(&self) -> usize;
    unsafe fn advance_mut(&mut self, cnt: usize);
    #[inline]
    fn has_remaining_mut(&self) -> bool {
        self.remaining_mut() > 0
    }
    #[cfg_attr(docsrs, doc(alias = "bytes_mut"))]
    fn chunk_mut(&mut self) -> &mut UninitSlice;
    #[inline]
    fn put<T: super::Buf>(&mut self, mut src: T)
    where
        Self: Sized,
    {
        if self.remaining_mut() < src.remaining() {
            panic_advance(
                &TryGetError {
                    requested: src.remaining(),
                    available: self.remaining_mut(),
                },
            );
        }
        while src.has_remaining() {
            let s = src.chunk();
            let d = self.chunk_mut();
            let cnt = usize::min(s.len(), d.len());
            d[..cnt].copy_from_slice(&s[..cnt]);
            unsafe { self.advance_mut(cnt) };
            src.advance(cnt);
        }
    }
    #[inline]
    fn put_slice(&mut self, mut src: &[u8]) {
        if self.remaining_mut() < src.len() {
            panic_advance(
                &TryGetError {
                    requested: src.len(),
                    available: self.remaining_mut(),
                },
            );
        }
        while !src.is_empty() {
            let dst = self.chunk_mut();
            let cnt = usize::min(src.len(), dst.len());
            dst[..cnt].copy_from_slice(&src[..cnt]);
            src = &src[cnt..];
            unsafe { self.advance_mut(cnt) };
        }
    }
    #[inline]
    fn put_bytes(&mut self, val: u8, mut cnt: usize) {
        if self.remaining_mut() < cnt {
            panic_advance(
                &TryGetError {
                    requested: cnt,
                    available: self.remaining_mut(),
                },
            )
        }
        while cnt > 0 {
            let dst = self.chunk_mut();
            let dst_len = usize::min(dst.len(), cnt);
            unsafe { core::ptr::write_bytes(dst.as_mut_ptr(), val, dst_len) };
            unsafe { self.advance_mut(dst_len) };
            cnt -= dst_len;
        }
    }
    #[inline]
    fn put_u8(&mut self, n: u8) {
        let src = [n];
        self.put_slice(&src);
    }
    #[inline]
    fn put_i8(&mut self, n: i8) {
        let src = [n as u8];
        self.put_slice(&src)
    }
    #[inline]
    fn put_u16(&mut self, n: u16) {
        self.put_slice(&n.to_be_bytes())
    }
    #[inline]
    fn put_u16_le(&mut self, n: u16) {
        self.put_slice(&n.to_le_bytes())
    }
    #[inline]
    fn put_u16_ne(&mut self, n: u16) {
        self.put_slice(&n.to_ne_bytes())
    }
    #[inline]
    fn put_i16(&mut self, n: i16) {
        self.put_slice(&n.to_be_bytes())
    }
    #[inline]
    fn put_i16_le(&mut self, n: i16) {
        self.put_slice(&n.to_le_bytes())
    }
    #[inline]
    fn put_i16_ne(&mut self, n: i16) {
        self.put_slice(&n.to_ne_bytes())
    }
    #[inline]
    fn put_u32(&mut self, n: u32) {
        self.put_slice(&n.to_be_bytes())
    }
    #[inline]
    fn put_u32_le(&mut self, n: u32) {
        self.put_slice(&n.to_le_bytes())
    }
    #[inline]
    fn put_u32_ne(&mut self, n: u32) {
        self.put_slice(&n.to_ne_bytes())
    }
    #[inline]
    fn put_i32(&mut self, n: i32) {
        self.put_slice(&n.to_be_bytes())
    }
    #[inline]
    fn put_i32_le(&mut self, n: i32) {
        self.put_slice(&n.to_le_bytes())
    }
    #[inline]
    fn put_i32_ne(&mut self, n: i32) {
        self.put_slice(&n.to_ne_bytes())
    }
    #[inline]
    fn put_u64(&mut self, n: u64) {
        self.put_slice(&n.to_be_bytes())
    }
    #[inline]
    fn put_u64_le(&mut self, n: u64) {
        self.put_slice(&n.to_le_bytes())
    }
    #[inline]
    fn put_u64_ne(&mut self, n: u64) {
        self.put_slice(&n.to_ne_bytes())
    }
    #[inline]
    fn put_i64(&mut self, n: i64) {
        self.put_slice(&n.to_be_bytes())
    }
    #[inline]
    fn put_i64_le(&mut self, n: i64) {
        self.put_slice(&n.to_le_bytes())
    }
    #[inline]
    fn put_i64_ne(&mut self, n: i64) {
        self.put_slice(&n.to_ne_bytes())
    }
    #[inline]
    fn put_u128(&mut self, n: u128) {
        self.put_slice(&n.to_be_bytes())
    }
    #[inline]
    fn put_u128_le(&mut self, n: u128) {
        self.put_slice(&n.to_le_bytes())
    }
    #[inline]
    fn put_u128_ne(&mut self, n: u128) {
        self.put_slice(&n.to_ne_bytes())
    }
    #[inline]
    fn put_i128(&mut self, n: i128) {
        self.put_slice(&n.to_be_bytes())
    }
    #[inline]
    fn put_i128_le(&mut self, n: i128) {
        self.put_slice(&n.to_le_bytes())
    }
    #[inline]
    fn put_i128_ne(&mut self, n: i128) {
        self.put_slice(&n.to_ne_bytes())
    }
    #[inline]
    fn put_uint(&mut self, n: u64, nbytes: usize) {
        let start = match mem::size_of_val(&n).checked_sub(nbytes) {
            Some(start) => start,
            None => panic_does_not_fit(nbytes, mem::size_of_val(&n)),
        };
        self.put_slice(&n.to_be_bytes()[start..]);
    }
    #[inline]
    fn put_uint_le(&mut self, n: u64, nbytes: usize) {
        let slice = n.to_le_bytes();
        let slice = match slice.get(..nbytes) {
            Some(slice) => slice,
            None => panic_does_not_fit(nbytes, slice.len()),
        };
        self.put_slice(slice);
    }
    #[inline]
    fn put_uint_ne(&mut self, n: u64, nbytes: usize) {
        if cfg!(target_endian = "big") {
            self.put_uint(n, nbytes)
        } else {
            self.put_uint_le(n, nbytes)
        }
    }
    #[inline]
    fn put_int(&mut self, n: i64, nbytes: usize) {
        let start = match mem::size_of_val(&n).checked_sub(nbytes) {
            Some(start) => start,
            None => panic_does_not_fit(nbytes, mem::size_of_val(&n)),
        };
        self.put_slice(&n.to_be_bytes()[start..]);
    }
    #[inline]
    fn put_int_le(&mut self, n: i64, nbytes: usize) {
        let slice = n.to_le_bytes();
        let slice = match slice.get(..nbytes) {
            Some(slice) => slice,
            None => panic_does_not_fit(nbytes, slice.len()),
        };
        self.put_slice(slice);
    }
    #[inline]
    fn put_int_ne(&mut self, n: i64, nbytes: usize) {
        if cfg!(target_endian = "big") {
            self.put_int(n, nbytes)
        } else {
            self.put_int_le(n, nbytes)
        }
    }
    #[inline]
    fn put_f32(&mut self, n: f32) {
        self.put_u32(n.to_bits());
    }
    #[inline]
    fn put_f32_le(&mut self, n: f32) {
        self.put_u32_le(n.to_bits());
    }
    #[inline]
    fn put_f32_ne(&mut self, n: f32) {
        self.put_u32_ne(n.to_bits());
    }
    #[inline]
    fn put_f64(&mut self, n: f64) {
        self.put_u64(n.to_bits());
    }
    #[inline]
    fn put_f64_le(&mut self, n: f64) {
        self.put_u64_le(n.to_bits());
    }
    #[inline]
    fn put_f64_ne(&mut self, n: f64) {
        self.put_u64_ne(n.to_bits());
    }
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
pub trait Buf {
    fn remaining(&self) -> usize;
    #[cfg_attr(docsrs, doc(alias = "bytes"))]
    fn chunk(&self) -> &[u8];
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize;
    fn advance(&mut self, cnt: usize);
    fn has_remaining(&self) -> bool;
    fn copy_to_slice(&mut self, dst: &mut [u8]);
    fn get_u8(&mut self) -> u8;
    fn get_i8(&mut self) -> i8;
    fn get_u16(&mut self) -> u16;
    fn get_u16_le(&mut self) -> u16;
    fn get_u16_ne(&mut self) -> u16;
    fn get_i16(&mut self) -> i16;
    fn get_i16_le(&mut self) -> i16;
    fn get_i16_ne(&mut self) -> i16;
    fn get_u32(&mut self) -> u32;
    fn get_u32_le(&mut self) -> u32;
    fn get_u32_ne(&mut self) -> u32;
    fn get_i32(&mut self) -> i32;
    fn get_i32_le(&mut self) -> i32;
    fn get_i32_ne(&mut self) -> i32;
    fn get_u64(&mut self) -> u64;
    fn get_u64_le(&mut self) -> u64;
    fn get_u64_ne(&mut self) -> u64;
    fn get_i64(&mut self) -> i64;
    fn get_i64_le(&mut self) -> i64;
    fn get_i64_ne(&mut self) -> i64;
    fn get_u128(&mut self) -> u128;
    fn get_u128_le(&mut self) -> u128;
    fn get_u128_ne(&mut self) -> u128;
    fn get_i128(&mut self) -> i128;
    fn get_i128_le(&mut self) -> i128;
    fn get_i128_ne(&mut self) -> i128;
    fn get_uint(&mut self, nbytes: usize) -> u64;
    fn get_uint_le(&mut self, nbytes: usize) -> u64;
    fn get_uint_ne(&mut self, nbytes: usize) -> u64;
    fn get_int(&mut self, nbytes: usize) -> i64;
    fn get_int_le(&mut self, nbytes: usize) -> i64;
    fn get_int_ne(&mut self, nbytes: usize) -> i64;
    fn get_f32(&mut self) -> f32;
    fn get_f32_le(&mut self) -> f32;
    fn get_f32_ne(&mut self) -> f32;
    fn get_f64(&mut self) -> f64;
    fn get_f64_le(&mut self) -> f64;
    fn get_f64_ne(&mut self) -> f64;
    fn try_copy_to_slice(&mut self, mut dst: &mut [u8]) -> Result<(), TryGetError>;
    fn try_get_u8(&mut self) -> Result<u8, TryGetError>;
    fn try_get_i8(&mut self) -> Result<i8, TryGetError>;
    fn try_get_u16(&mut self) -> Result<u16, TryGetError>;
    fn try_get_u16_le(&mut self) -> Result<u16, TryGetError>;
    fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError>;
    fn try_get_i16(&mut self) -> Result<i16, TryGetError>;
    fn try_get_i16_le(&mut self) -> Result<i16, TryGetError>;
    fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError>;
    fn try_get_u32(&mut self) -> Result<u32, TryGetError>;
    fn try_get_u32_le(&mut self) -> Result<u32, TryGetError>;
    fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError>;
    fn try_get_i32(&mut self) -> Result<i32, TryGetError>;
    fn try_get_i32_le(&mut self) -> Result<i32, TryGetError>;
    fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError>;
    fn try_get_u64(&mut self) -> Result<u64, TryGetError>;
    fn try_get_u64_le(&mut self) -> Result<u64, TryGetError>;
    fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError>;
    fn try_get_i64(&mut self) -> Result<i64, TryGetError>;
    fn try_get_i64_le(&mut self) -> Result<i64, TryGetError>;
    fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError>;
    fn try_get_u128(&mut self) -> Result<u128, TryGetError>;
    fn try_get_u128_le(&mut self) -> Result<u128, TryGetError>;
    fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError>;
    fn try_get_i128(&mut self) -> Result<i128, TryGetError>;
    fn try_get_i128_le(&mut self) -> Result<i128, TryGetError>;
    fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError>;
    fn try_get_uint(&mut self, nbytes: usize) -> Result<u64, TryGetError>;
    fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError>;
    fn try_get_uint_ne(&mut self, nbytes: usize) -> Result<u64, TryGetError>;
    fn try_get_int(&mut self, nbytes: usize) -> Result<i64, TryGetError>;
    fn try_get_int_le(&mut self, nbytes: usize) -> Result<i64, TryGetError>;
    fn try_get_int_ne(&mut self, nbytes: usize) -> Result<i64, TryGetError>;
    fn try_get_f32(&mut self) -> Result<f32, TryGetError>;
    fn try_get_f32_le(&mut self) -> Result<f32, TryGetError>;
    fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError>;
    fn try_get_f64(&mut self) -> Result<f64, TryGetError>;
    fn try_get_f64_le(&mut self) -> Result<f64, TryGetError>;
    fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError>;
    fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes;
    fn take(self, limit: usize) -> Take<Self>
    where
        Self: Sized,
    {
        take::new(self, limit)
    }
    fn chain<U: Buf>(self, next: U) -> Chain<Self, U>
    where
        Self: Sized,
    {
        Chain::new(self, next)
    }
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn reader(self) -> Reader<Self>
    where
        Self: Sized,
    {
        reader::new(self)
    }
}
#[repr(transparent)]
pub struct UninitSlice([MaybeUninit<u8>]);
#[derive(Debug, PartialEq, Eq)]
pub struct TryGetError {
    /// The number of bytes necessary to get the value
    pub requested: usize,
    /// The number of bytes available in the buffer
    pub available: usize,
}
unsafe impl BufMut for &mut [core::mem::MaybeUninit<u8>] {
    #[inline]
    fn remaining_mut(&self) -> usize {}
    #[inline]
    fn chunk_mut(&mut self) -> &mut UninitSlice {}
    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {}
    #[inline]
    fn put_slice(&mut self, src: &[u8]) {
        if self.len() < src.len() {
            panic_advance(
                &TryGetError {
                    requested: src.len(),
                    available: self.len(),
                },
            );
        }
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr().cast(), src.len());
            self.advance_mut(src.len());
        }
    }
    #[inline]
    fn put_bytes(&mut self, val: u8, cnt: usize) {}
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
    pub fn copy_from_slice(&mut self, src: &[u8]) {
        use core::ptr;
        assert_eq!(self.len(), src.len());
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr(), self.len());
        }
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {}
    #[inline]
    pub unsafe fn as_uninit_slice_mut(&mut self) -> &mut [MaybeUninit<u8>] {}
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
#[cold]
fn panic_advance(error_info: &TryGetError) -> ! {
    panic!(
        "advance out of bounds: the len is {} but advancing by {}", error_info.available,
        error_info.requested
    );
}
