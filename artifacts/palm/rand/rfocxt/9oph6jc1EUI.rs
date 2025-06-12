use crate::distr::uniform::{SampleRange, SampleUniform};
use crate::distr::{self, Distribution, StandardUniform};
use core::num::Wrapping;
use core::{mem, slice};
use rand_core::RngCore;
const _: () = unsafe { impl_fill!(u16, u32, u64, u128,) };
const _: () = unsafe { impl_fill!(i8, i16, i32, i64, i128,) };
macro_rules! impl_fill_each {
    () => {};
    ($t:ty) => {
        impl Fill for [$t] { fn fill < R : Rng + ? Sized > (& mut self, rng : & mut R) {
        for elt in self.iter_mut() { * elt = rng.random(); } } }
    };
    ($t:ty, $($tt:ty,)*) => {
        impl_fill_each!($t); impl_fill_each!($($tt,)*);
    };
}
impl_fill_each!(bool, char, f32, f64,);
macro_rules! impl_fill {
    () => {};
    ($t:ty) => {
        { __unsafe(); impl Fill for [$t] { fn fill < R : Rng + ? Sized > (& mut self, rng
        : & mut R) { if self.len() > 0 { let size = mem::size_of_val(self); rng
        .fill_bytes(unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as * mut u8,
        size) }); for x in self { * x = x.to_le(); } } } } impl Fill for [Wrapping <$t >]
        { fn fill < R : Rng + ? Sized > (& mut self, rng : & mut R) { if self.len() > 0 {
        let size = self.len() * mem::size_of::<$t > (); rng.fill_bytes(unsafe {
        slice::from_raw_parts_mut(self.as_mut_ptr() as * mut u8, size) }); for x in self
        { * x = Wrapping(x.0.to_le()); } } } } }
    };
    ($t:ty, $($tt:ty,)*) => {
        { impl_fill!($t); impl_fill!($($tt,)*); }
    };
}
const unsafe fn __unsafe() {}
