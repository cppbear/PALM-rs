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
fn _assert_trait_object(_b: &dyn BufMut) {}
