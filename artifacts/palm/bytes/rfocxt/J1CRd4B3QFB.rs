#[cfg(feature = "std")]
use crate::buf::{reader, Reader};
use crate::buf::{take, Chain, Take};
#[cfg(feature = "std")]
use crate::{min_u64_usize, saturating_sub_usize_u64};
use crate::{panic_advance, panic_does_not_fit, TryGetError};
#[cfg(feature = "std")]
use std::io::IoSlice;
use alloc::boxed::Box;
macro_rules! buf_try_get_impl {
    ($this:ident, $typ:tt ::$conv:tt) => {
        { const SIZE : usize = core::mem::size_of::<$typ > (); if $this .remaining() <
        SIZE { return Err(TryGetError { requested : SIZE, available : $this .remaining(),
        }); } let ret = $this .chunk().get(..SIZE).map(| src | unsafe { $typ ::$conv (*
        (src as * const _ as * const [_; SIZE])) }); if let Some(ret) = ret { $this
        .advance(SIZE); return Ok(ret); } else { let mut buf = [0; SIZE]; $this
        .copy_to_slice(& mut buf); return Ok($typ ::$conv (buf)); } }
    };
    (le => $this:ident, $typ:tt, $len_to_read:expr) => {
        { const SIZE : usize = core::mem::size_of::<$typ > (); let mut buf = [0; SIZE];
        let subslice = match buf.get_mut(..$len_to_read) { Some(subslice) => subslice,
        None => panic_does_not_fit(SIZE, $len_to_read), }; $this
        .try_copy_to_slice(subslice) ?; return Ok($typ ::from_le_bytes(buf)); }
    };
    (be => $this:ident, $typ:tt, $len_to_read:expr) => {
        { const SIZE : usize = core::mem::size_of::<$typ > (); let slice_at = match SIZE
        .checked_sub($len_to_read) { Some(slice_at) => slice_at, None =>
        panic_does_not_fit(SIZE, $len_to_read), }; let mut buf = [0; SIZE]; $this
        .try_copy_to_slice(& mut buf[slice_at..]) ?; return Ok($typ
        ::from_be_bytes(buf)); }
    };
}
macro_rules! buf_get_impl {
    ($this:ident, $typ:tt ::$conv:tt) => {
        { return (|| buf_try_get_impl!($this, $typ ::$conv)) ().unwrap_or_else(| error |
        panic_advance(& error)); }
    };
    (le => $this:ident, $typ:tt, $len_to_read:expr) => {
        { return (|| buf_try_get_impl!(le => $this, $typ, $len_to_read)) ()
        .unwrap_or_else(| error | panic_advance(& error)); }
    };
    (be => $this:ident, $typ:tt, $len_to_read:expr) => {
        { return (|| buf_try_get_impl!(be => $this, $typ, $len_to_read)) ()
        .unwrap_or_else(| error | panic_advance(& error)); }
    };
}
macro_rules! deref_forward_buf {
    () => {
        #[inline] fn remaining(& self) -> usize { (** self).remaining() } #[inline] fn
        chunk(& self) -> & [u8] { (** self).chunk() } #[cfg(feature = "std")] #[inline]
        fn chunks_vectored <'b > (&'b self, dst : & mut [IoSlice <'b >]) -> usize { (**
        self).chunks_vectored(dst) } #[inline] fn advance(& mut self, cnt : usize) { (**
        self).advance(cnt) } #[inline] fn has_remaining(& self) -> bool { (** self)
        .has_remaining() } #[inline] fn copy_to_slice(& mut self, dst : & mut [u8]) { (**
        self).copy_to_slice(dst) } #[inline] fn get_u8(& mut self) -> u8 { (** self)
        .get_u8() } #[inline] fn get_i8(& mut self) -> i8 { (** self).get_i8() }
        #[inline] fn get_u16(& mut self) -> u16 { (** self).get_u16() } #[inline] fn
        get_u16_le(& mut self) -> u16 { (** self).get_u16_le() } #[inline] fn
        get_u16_ne(& mut self) -> u16 { (** self).get_u16_ne() } #[inline] fn get_i16(&
        mut self) -> i16 { (** self).get_i16() } #[inline] fn get_i16_le(& mut self) ->
        i16 { (** self).get_i16_le() } #[inline] fn get_i16_ne(& mut self) -> i16 { (**
        self).get_i16_ne() } #[inline] fn get_u32(& mut self) -> u32 { (** self)
        .get_u32() } #[inline] fn get_u32_le(& mut self) -> u32 { (** self).get_u32_le()
        } #[inline] fn get_u32_ne(& mut self) -> u32 { (** self).get_u32_ne() } #[inline]
        fn get_i32(& mut self) -> i32 { (** self).get_i32() } #[inline] fn get_i32_le(&
        mut self) -> i32 { (** self).get_i32_le() } #[inline] fn get_i32_ne(& mut self)
        -> i32 { (** self).get_i32_ne() } #[inline] fn get_u64(& mut self) -> u64 { (**
        self).get_u64() } #[inline] fn get_u64_le(& mut self) -> u64 { (** self)
        .get_u64_le() } #[inline] fn get_u64_ne(& mut self) -> u64 { (** self)
        .get_u64_ne() } #[inline] fn get_i64(& mut self) -> i64 { (** self).get_i64() }
        #[inline] fn get_i64_le(& mut self) -> i64 { (** self).get_i64_le() } #[inline]
        fn get_i64_ne(& mut self) -> i64 { (** self).get_i64_ne() } #[inline] fn
        get_u128(& mut self) -> u128 { (** self).get_u128() } #[inline] fn get_u128_le(&
        mut self) -> u128 { (** self).get_u128_le() } #[inline] fn get_u128_ne(& mut
        self) -> u128 { (** self).get_u128_ne() } #[inline] fn get_i128(& mut self) ->
        i128 { (** self).get_i128() } #[inline] fn get_i128_le(& mut self) -> i128 { (**
        self).get_i128_le() } #[inline] fn get_i128_ne(& mut self) -> i128 { (** self)
        .get_i128_ne() } #[inline] fn get_uint(& mut self, nbytes : usize) -> u64 { (**
        self).get_uint(nbytes) } #[inline] fn get_uint_le(& mut self, nbytes : usize) ->
        u64 { (** self).get_uint_le(nbytes) } #[inline] fn get_uint_ne(& mut self, nbytes
        : usize) -> u64 { (** self).get_uint_ne(nbytes) } #[inline] fn get_int(& mut
        self, nbytes : usize) -> i64 { (** self).get_int(nbytes) } #[inline] fn
        get_int_le(& mut self, nbytes : usize) -> i64 { (** self).get_int_le(nbytes) }
        #[inline] fn get_int_ne(& mut self, nbytes : usize) -> i64 { (** self)
        .get_int_ne(nbytes) } #[inline] fn get_f32(& mut self) -> f32 { (** self)
        .get_f32() } #[inline] fn get_f32_le(& mut self) -> f32 { (** self).get_f32_le()
        } #[inline] fn get_f32_ne(& mut self) -> f32 { (** self).get_f32_ne() } #[inline]
        fn get_f64(& mut self) -> f64 { (** self).get_f64() } #[inline] fn get_f64_le(&
        mut self) -> f64 { (** self).get_f64_le() } #[inline] fn get_f64_ne(& mut self)
        -> f64 { (** self).get_f64_ne() } #[inline] fn try_copy_to_slice(& mut self, dst
        : & mut [u8]) -> Result < (), TryGetError > { (** self).try_copy_to_slice(dst) }
        #[inline] fn try_get_u8(& mut self) -> Result < u8, TryGetError > { (** self)
        .try_get_u8() } #[inline] fn try_get_i8(& mut self) -> Result < i8, TryGetError >
        { (** self).try_get_i8() } #[inline] fn try_get_u16(& mut self) -> Result < u16,
        TryGetError > { (** self).try_get_u16() } #[inline] fn try_get_u16_le(& mut self)
        -> Result < u16, TryGetError > { (** self).try_get_u16_le() } #[inline] fn
        try_get_u16_ne(& mut self) -> Result < u16, TryGetError > { (** self)
        .try_get_u16_ne() } #[inline] fn try_get_i16(& mut self) -> Result < i16,
        TryGetError > { (** self).try_get_i16() } #[inline] fn try_get_i16_le(& mut self)
        -> Result < i16, TryGetError > { (** self).try_get_i16_le() } #[inline] fn
        try_get_i16_ne(& mut self) -> Result < i16, TryGetError > { (** self)
        .try_get_i16_ne() } #[inline] fn try_get_u32(& mut self) -> Result < u32,
        TryGetError > { (** self).try_get_u32() } #[inline] fn try_get_u32_le(& mut self)
        -> Result < u32, TryGetError > { (** self).try_get_u32_le() } #[inline] fn
        try_get_u32_ne(& mut self) -> Result < u32, TryGetError > { (** self)
        .try_get_u32_ne() } #[inline] fn try_get_i32(& mut self) -> Result < i32,
        TryGetError > { (** self).try_get_i32() } #[inline] fn try_get_i32_le(& mut self)
        -> Result < i32, TryGetError > { (** self).try_get_i32_le() } #[inline] fn
        try_get_i32_ne(& mut self) -> Result < i32, TryGetError > { (** self)
        .try_get_i32_ne() } #[inline] fn try_get_u64(& mut self) -> Result < u64,
        TryGetError > { (** self).try_get_u64() } #[inline] fn try_get_u64_le(& mut self)
        -> Result < u64, TryGetError > { (** self).try_get_u64_le() } #[inline] fn
        try_get_u64_ne(& mut self) -> Result < u64, TryGetError > { (** self)
        .try_get_u64_ne() } #[inline] fn try_get_i64(& mut self) -> Result < i64,
        TryGetError > { (** self).try_get_i64() } #[inline] fn try_get_i64_le(& mut self)
        -> Result < i64, TryGetError > { (** self).try_get_i64_le() } #[inline] fn
        try_get_i64_ne(& mut self) -> Result < i64, TryGetError > { (** self)
        .try_get_i64_ne() } #[inline] fn try_get_u128(& mut self) -> Result < u128,
        TryGetError > { (** self).try_get_u128() } #[inline] fn try_get_u128_le(& mut
        self) -> Result < u128, TryGetError > { (** self).try_get_u128_le() } #[inline]
        fn try_get_u128_ne(& mut self) -> Result < u128, TryGetError > { (** self)
        .try_get_u128_ne() } #[inline] fn try_get_i128(& mut self) -> Result < i128,
        TryGetError > { (** self).try_get_i128() } #[inline] fn try_get_i128_le(& mut
        self) -> Result < i128, TryGetError > { (** self).try_get_i128_le() } #[inline]
        fn try_get_i128_ne(& mut self) -> Result < i128, TryGetError > { (** self)
        .try_get_i128_ne() } #[inline] fn try_get_uint(& mut self, nbytes : usize) ->
        Result < u64, TryGetError > { (** self).try_get_uint(nbytes) } #[inline] fn
        try_get_uint_le(& mut self, nbytes : usize) -> Result < u64, TryGetError > { (**
        self).try_get_uint_le(nbytes) } #[inline] fn try_get_uint_ne(& mut self, nbytes :
        usize) -> Result < u64, TryGetError > { (** self).try_get_uint_ne(nbytes) }
        #[inline] fn try_get_int(& mut self, nbytes : usize) -> Result < i64, TryGetError
        > { (** self).try_get_int(nbytes) } #[inline] fn try_get_int_le(& mut self,
        nbytes : usize) -> Result < i64, TryGetError > { (** self).try_get_int_le(nbytes)
        } #[inline] fn try_get_int_ne(& mut self, nbytes : usize) -> Result < i64,
        TryGetError > { (** self).try_get_int_ne(nbytes) } #[inline] fn try_get_f32(& mut
        self) -> Result < f32, TryGetError > { (** self).try_get_f32() } #[inline] fn
        try_get_f32_le(& mut self) -> Result < f32, TryGetError > { (** self)
        .try_get_f32_le() } #[inline] fn try_get_f32_ne(& mut self) -> Result < f32,
        TryGetError > { (** self).try_get_f32_ne() } #[inline] fn try_get_f64(& mut self)
        -> Result < f64, TryGetError > { (** self).try_get_f64() } #[inline] fn
        try_get_f64_le(& mut self) -> Result < f64, TryGetError > { (** self)
        .try_get_f64_le() } #[inline] fn try_get_f64_ne(& mut self) -> Result < f64,
        TryGetError > { (** self).try_get_f64_ne() } #[inline] fn copy_to_bytes(& mut
        self, len : usize) -> crate ::Bytes { (** self).copy_to_bytes(len) }
    };
}
fn sign_extend(val: u64, nbytes: usize) -> i64 {
    let shift = (8 - nbytes) * 8;
    (val << shift) as i64 >> shift
}
