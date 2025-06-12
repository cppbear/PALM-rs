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
pub trait Fill {
    fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R);
}
trait UInt: Copy + PartialOrd + Ord + PartialEq + Eq + SampleUniform + Hash + AddAssign {
    fn zero() -> Self;
    #[cfg_attr(feature = "alloc", allow(dead_code))]
    fn one() -> Self;
    fn as_usize(self) -> usize;
}
pub trait SliceRandom: IndexedMutRandom {
    fn shuffle<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized;
    fn partial_shuffle<R>(
        &mut self,
        rng: &mut R,
        amount: usize,
    ) -> (&mut [Self::Output], &mut [Self::Output])
    where
        Self::Output: Sized,
        R: Rng + ?Sized;
}
pub trait IndexedRandom: Index<usize> {
    fn len(&self) -> usize;
    #[inline]
    fn is_empty(&self) -> bool;
    fn choose<R>(&self, rng: &mut R) -> Option<&Self::Output>
    where
        R: Rng + ?Sized,
    {
        if self.is_empty() { None } else { Some(&self[rng.random_range(..self.len())]) }
    }
    #[cfg(feature = "alloc")]
    fn choose_multiple<R>(
        &self,
        rng: &mut R,
        amount: usize,
    ) -> SliceChooseIter<Self, Self::Output>
    where
        Self::Output: Sized,
        R: Rng + ?Sized,
    {
        let amount = core::cmp::min(amount, self.len());
        SliceChooseIter {
            slice: self,
            _phantom: Default::default(),
            indices: index::sample(rng, self.len(), amount).into_iter(),
        }
    }
    fn choose_multiple_array<R, const N: usize>(
        &self,
        rng: &mut R,
    ) -> Option<[Self::Output; N]>
    where
        Self::Output: Clone + Sized,
        R: Rng + ?Sized,
    {
        let indices = index::sample_array(rng, self.len())?;
        Some(indices.map(|index| self[index].clone()))
    }
    #[cfg(feature = "alloc")]
    fn choose_weighted<R, F, B, X>(
        &self,
        rng: &mut R,
        weight: F,
    ) -> Result<&Self::Output, WeightError>
    where
        R: Rng + ?Sized,
        F: Fn(&Self::Output) -> B,
        B: SampleBorrow<X>,
        X: SampleUniform + Weight + PartialOrd<X>,
    {
        use crate::distr::{weighted::WeightedIndex, Distribution};
        let distr = WeightedIndex::new((0..self.len()).map(|idx| weight(&self[idx])))?;
        Ok(&self[distr.sample(rng)])
    }
    #[cfg(feature = "std")]
    fn choose_multiple_weighted<R, F, X>(
        &self,
        rng: &mut R,
        amount: usize,
        weight: F,
    ) -> Result<SliceChooseIter<Self, Self::Output>, WeightError>
    where
        Self::Output: Sized,
        R: Rng + ?Sized,
        F: Fn(&Self::Output) -> X,
        X: Into<f64>,
    {
        let amount = core::cmp::min(amount, self.len());
        Ok(SliceChooseIter {
            slice: self,
            _phantom: Default::default(),
            indices: index::sample_weighted(
                    rng,
                    self.len(),
                    |idx| weight(&self[idx]).into(),
                    amount,
                )?
                .into_iter(),
        })
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
pub(crate) trait BoolAsSIMD: Sized {
    fn any(self) -> bool;
}
pub trait SampleBorrow<Borrowed> {
    fn borrow(&self) -> &Borrowed;
}
impl<T, const N: usize> Fill for [T; N]
where
    [T]: Fill,
{
    fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        <[T] as Fill>::fill(self, rng)
    }
}
