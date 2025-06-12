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
    fn sample_string<R: Rng + ?Sized>(&self, rng: &mut R, len: usize) -> String {
        let mut s = String::new();
        self.append_string(rng, &mut s, len);
        s
    }
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
#[cfg(feature = "alloc")]
impl super::SampleString for Choose<'_, char> {
    fn append_string<R: crate::Rng + ?Sized>(
        &self,
        rng: &mut R,
        string: &mut String,
        len: usize,
    ) {
        let max_char_len = if self.slice.len() < 200 {
            self.slice
                .iter()
                .try_fold(
                    1,
                    |max_len, char| {
                        Some(max_len.max(char.len_utf8())).filter(|len| *len < 4)
                    },
                )
                .unwrap_or(4)
        } else {
            4
        };
        let mut extend_len = if max_char_len == 1 || len < 100 { len } else { len / 4 };
        let mut remain_len = len;
        while extend_len > 0 {
            string.reserve(max_char_len * extend_len);
            string.extend(self.sample_iter(&mut *rng).take(extend_len));
            remain_len -= extend_len;
            extend_len = extend_len.min(remain_len);
        }
    }
}
