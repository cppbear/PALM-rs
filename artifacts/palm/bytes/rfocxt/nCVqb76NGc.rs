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
#[derive(Debug, PartialEq, Eq)]
pub struct TryGetError {
    /// The number of bytes necessary to get the value
    pub requested: usize,
    /// The number of bytes available in the buffer
    pub available: usize,
}
unsafe impl BufMut for Vec<u8> {
    #[inline]
    fn remaining_mut(&self) -> usize {}
    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        let len = self.len();
        let remaining = self.capacity() - len;
        if remaining < cnt {
            panic_advance(
                &TryGetError {
                    requested: cnt,
                    available: remaining,
                },
            );
        }
        self.set_len(len + cnt);
    }
    #[inline]
    fn chunk_mut(&mut self) -> &mut UninitSlice {}
    #[inline]
    fn put<T: super::Buf>(&mut self, mut src: T)
    where
        Self: Sized,
    {}
    #[inline]
    fn put_slice(&mut self, src: &[u8]) {}
    #[inline]
    fn put_bytes(&mut self, val: u8, cnt: usize) {}
}
#[cold]
fn panic_advance(error_info: &TryGetError) -> ! {
    panic!(
        "advance out of bounds: the len is {} but advancing by {}", error_info.available,
        error_info.requested
    );
}
