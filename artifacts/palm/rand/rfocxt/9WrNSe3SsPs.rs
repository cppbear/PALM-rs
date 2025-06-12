use core::num::NonZeroUsize;
use crate::distr::uniform::{UniformSampler, UniformUsize};
use crate::distr::Distribution;
#[cfg(feature = "alloc")]
use alloc::string::String;
pub trait Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
    fn sample_iter<R>(self, rng: R) -> Iter<Self, R, T>
    where
        R: Rng,
        Self: Sized,
    {
        Iter {
            distr: self,
            rng,
            phantom: core::marker::PhantomData,
        }
    }
    fn map<F, S>(self, func: F) -> Map<Self, F, T, S>
    where
        F: Fn(T) -> S,
        Self: Sized,
    {
        Map {
            distr: self,
            func,
            phantom: core::marker::PhantomData,
        }
    }
}
#[cfg(feature = "alloc")]
pub trait SampleString {
    fn append_string<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        string: &mut String,
        len: usize,
    );
    #[inline]
    fn sample_string<R: Rng + ?Sized>(&self, rng: &mut R, len: usize) -> String;
}
#[derive(Debug, Clone, Copy)]
pub struct Choose<'a, T> {
    slice: &'a [T],
    range: UniformUsize,
    num_choices: NonZeroUsize,
}
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformUsize {
    low: usize,
    range: usize,
    thresh: usize,
    #[cfg(target_pointer_width = "64")]
    mode64: bool,
}
impl<'a, T> Distribution<&'a T> for Choose<'a, T> {
    fn sample<R: crate::Rng + ?Sized>(&self, rng: &mut R) -> &'a T {
        let idx = self.range.sample(rng);
        debug_assert!(
            idx < self.slice.len(), "Uniform::new(0, {}) somehow returned {}", self.slice
            .len(), idx
        );
        unsafe { self.slice.get_unchecked(idx) }
    }
}
