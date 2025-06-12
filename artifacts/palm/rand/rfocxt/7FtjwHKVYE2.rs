use super::increasing_uniform::IncreasingUniform;
use super::index;
#[cfg(feature = "alloc")]
use crate::distr::uniform::{SampleBorrow, SampleUniform};
#[cfg(feature = "alloc")]
use crate::distr::weighted::{Error as WeightError, Weight};
use crate::Rng;
use core::ops::{Index, IndexMut};
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
trait UInt: Copy + PartialOrd + Ord + PartialEq + Eq + SampleUniform + Hash + AddAssign {
    fn zero() -> Self;
    #[cfg_attr(feature = "alloc", allow(dead_code))]
    fn one() -> Self;
    fn as_usize(self) -> usize;
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
pub trait Fill {
    fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R);
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
pub trait IndexedMutRandom: IndexedRandom + IndexMut<usize> {
    fn choose_mut<R>(&mut self, rng: &mut R) -> Option<&mut Self::Output>
    where
        R: Rng + ?Sized,
    {
        if self.is_empty() {
            None
        } else {
            let len = self.len();
            Some(&mut self[rng.random_range(..len)])
        }
    }
    #[cfg(feature = "alloc")]
    fn choose_weighted_mut<R, F, B, X>(
        &mut self,
        rng: &mut R,
        weight: F,
    ) -> Result<&mut Self::Output, WeightError>
    where
        R: Rng + ?Sized,
        F: Fn(&Self::Output) -> B,
        B: SampleBorrow<X>,
        X: SampleUniform + Weight + PartialOrd<X>,
    {
        use crate::distr::{weighted::WeightedIndex, Distribution};
        let distr = WeightedIndex::new((0..self.len()).map(|idx| weight(&self[idx])))?;
        let index = distr.sample(rng);
        Ok(&mut self[index])
    }
}
pub(crate) struct IncreasingUniform<R: RngCore> {
    pub rng: R,
    n: u32,
    chunk: u32,
    chunk_remaining: u8,
}
impl<T> SliceRandom for [T] {
    fn shuffle<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {}
    fn partial_shuffle<R>(&mut self, rng: &mut R, amount: usize) -> (&mut [T], &mut [T])
    where
        R: Rng + ?Sized,
    {
        let m = self.len().saturating_sub(amount);
        if self.len() < (u32::MAX as usize) {
            let mut chooser = IncreasingUniform::new(rng, m as u32);
            for i in m..self.len() {
                let index = chooser.next_index();
                self.swap(i, index);
            }
        } else {
            for i in m..self.len() {
                let index = rng.random_range(..i + 1);
                self.swap(i, index);
            }
        }
        let r = self.split_at_mut(m);
        (r.1, r.0)
    }
}
impl<R: RngCore> IncreasingUniform<R> {
    pub fn new(rng: R, n: u32) -> Self {
        let chunk_remaining = if n == 0 { 1 } else { 0 };
        Self {
            rng,
            n,
            chunk: 0,
            chunk_remaining,
        }
    }
    #[inline]
    pub fn next_index(&mut self) -> usize {
        let next_n = self.n + 1;
        let next_chunk_remaining = self
            .chunk_remaining
            .checked_sub(1)
            .unwrap_or_else(|| {
                let (bound, remaining) = calculate_bound_u32(next_n);
                self.chunk = self.rng.random_range(..bound);
                remaining - 1
            });
        let result = if next_chunk_remaining == 0 {
            self.chunk as usize
        } else {
            let r = self.chunk % next_n;
            self.chunk /= next_n;
            r as usize
        };
        self.chunk_remaining = next_chunk_remaining;
        self.n = next_n;
        result
    }
}
