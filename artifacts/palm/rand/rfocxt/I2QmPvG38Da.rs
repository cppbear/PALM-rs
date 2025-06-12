use core::num::NonZeroUsize;
use crate::distr::uniform::{UniformSampler, UniformUsize};
use crate::distr::Distribution;
#[cfg(feature = "alloc")]
use alloc::string::String;
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
impl<'a, T> Choose<'a, T> {
    pub fn new(slice: &'a [T]) -> Result<Self, Empty> {
        let num_choices = NonZeroUsize::new(slice.len()).ok_or(Empty)?;
        Ok(Self {
            slice,
            range: UniformUsize::new(0, num_choices.get()).unwrap(),
            num_choices,
        })
    }
    pub fn num_choices(&self) -> NonZeroUsize {
        self.num_choices
    }
}
