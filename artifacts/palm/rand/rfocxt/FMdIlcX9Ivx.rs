#[cfg(feature = "simd_support")]
use core::simd::prelude::*;
#[cfg(feature = "simd_support")]
use core::simd::{LaneCount, SimdElement, SupportedLaneCount};
macro_rules! wmul_impl {
    ($ty:ty, $wide:ty, $shift:expr) => {
        impl WideningMultiply for $ty { type Output = ($ty, $ty); #[inline(always)] fn
        wmul(self, x : $ty) -> Self::Output { let tmp = (self as $wide) * (x as $wide);
        ((tmp >> $shift) as $ty, tmp as $ty) } }
    };
    ($(($ty:ident, $wide:ty),)+, $shift:expr) => {
        $(impl WideningMultiply for $ty { type Output = ($ty, $ty); #[inline(always)] fn
        wmul(self, x : $ty) -> Self::Output { let y : $wide = self.cast(); let x : $wide
        = x.cast(); let tmp = y * x; let hi : $ty = (tmp >> Simd::splat($shift)).cast();
        let lo : $ty = tmp.cast(); (hi, lo) } })+
    };
}
wmul_impl! {
    u8, u16, 8
}
wmul_impl! {
    u16, u32, 16
}
wmul_impl! {
    u32, u64, 32
}
wmul_impl! {
    u64, u128, 64
}
macro_rules! wmul_impl_large {
    ($ty:ty, $half:expr) => {
        impl WideningMultiply for $ty { type Output = ($ty, $ty); #[inline(always)] fn
        wmul(self, b : $ty) -> Self::Output { const LOWER_MASK : $ty = ! 0 >> $half; let
        mut low = (self & LOWER_MASK).wrapping_mul(b & LOWER_MASK); let mut t = low >>
        $half; low &= LOWER_MASK; t += (self >> $half).wrapping_mul(b & LOWER_MASK); low
        += (t & LOWER_MASK) << $half; let mut high = t >> $half; t = low >> $half; low &=
        LOWER_MASK; t += (b >> $half).wrapping_mul(self & LOWER_MASK); low += (t &
        LOWER_MASK) << $half; high += t >> $half; high += (self >> $half).wrapping_mul(b
        >> $half); (high, low) } }
    };
    (($($ty:ty,)+) $scalar:ty, $half:expr) => {
        $(impl WideningMultiply for $ty { type Output = ($ty, $ty); #[inline(always)] fn
        wmul(self, b : $ty) -> Self::Output { let lower_mask = <$ty >::splat(! 0 >>
        $half); let half = <$ty >::splat($half); let mut low = (self & lower_mask) * (b &
        lower_mask); let mut t = low >> half; low &= lower_mask; t += (self >> half) * (b
        & lower_mask); low += (t & lower_mask) << half; let mut high = t >> half; t = low
        >> half; low &= lower_mask; t += (b >> half) * (self & lower_mask); low += (t &
        lower_mask) << half; high += t >> half; high += (self >> half) * (b >> half);
        (high, low) } })+
    };
}
wmul_impl_large! {
    u128, 64
}
macro_rules! wmul_impl_usize {
    ($ty:ty) => {
        impl WideningMultiply for usize { type Output = (usize, usize); #[inline(always)]
        fn wmul(self, x : usize) -> Self::Output { let (high, low) = (self as $ty).wmul(x
        as $ty); (high as usize, low as usize) } }
    };
}
#[cfg(target_pointer_width = "16")]
wmul_impl_usize! {
    u16
}
#[cfg(target_pointer_width = "32")]
wmul_impl_usize! {
    u32
}
#[cfg(target_pointer_width = "64")]
wmul_impl_usize! {
    u64
}
macro_rules! scalar_float_impl {
    ($ty:ident, $uty:ident) => {
        impl FloatSIMDUtils for $ty { type Mask = bool; type UInt = $uty;
        #[inline(always)] fn all_lt(self, other : Self) -> bool { self < other }
        #[inline(always)] fn all_le(self, other : Self) -> bool { self <= other }
        #[inline(always)] fn all_finite(self) -> bool { self.is_finite() }
        #[inline(always)] fn gt_mask(self, other : Self) -> Self::Mask { self > other }
        #[inline(always)] fn decrease_masked(self, mask : Self::Mask) -> Self {
        debug_assert!(mask, "At least one lane must be set"); <$ty >::from_bits(self
        .to_bits() - 1) } #[inline] fn cast_from_int(i : Self::UInt) -> Self { i as $ty }
        } #[cfg(test)] impl FloatSIMDScalarUtils for $ty { type Scalar = $ty; #[inline]
        fn replace(self, index : usize, new_value : Self::Scalar) -> Self {
        debug_assert_eq!(index, 0); new_value } #[inline] fn extract_lane(self, index :
        usize) -> Self::Scalar { debug_assert_eq!(index, 0); self } } impl FloatAsSIMD
        for $ty {}
    };
}
scalar_float_impl!(f32, u32);
scalar_float_impl!(f64, u64);
#[cfg(feature = "simd_support")]
macro_rules! simd_impl {
    ($fty:ident, $uty:ident) => {
        impl < const LANES : usize > FloatSIMDUtils for Simd <$fty, LANES > where
        LaneCount < LANES >: SupportedLaneCount, { type Mask = Mask <<$fty as SimdElement
        >::Mask, LANES >; type UInt = Simd <$uty, LANES >; #[inline(always)] fn
        all_lt(self, other : Self) -> bool { self.simd_lt(other).all() }
        #[inline(always)] fn all_le(self, other : Self) -> bool { self.simd_le(other)
        .all() } #[inline(always)] fn all_finite(self) -> bool { self.is_finite().all() }
        #[inline(always)] fn gt_mask(self, other : Self) -> Self::Mask { self
        .simd_gt(other) } #[inline(always)] fn decrease_masked(self, mask : Self::Mask)
        -> Self { debug_assert!(mask.any(), "At least one lane must be set");
        Self::from_bits(self.to_bits() + mask.to_int().cast()) } #[inline] fn
        cast_from_int(i : Self::UInt) -> Self { i.cast() } } #[cfg(test)] impl < const
        LANES : usize > FloatSIMDScalarUtils for Simd <$fty, LANES > where LaneCount <
        LANES >: SupportedLaneCount, { type Scalar = $fty; #[inline] fn replace(mut self,
        index : usize, new_value : Self::Scalar) -> Self { self.as_mut_array() [index] =
        new_value; self } #[inline] fn extract_lane(self, index : usize) -> Self::Scalar
        { self.as_array() [index] } }
    };
}
#[cfg(feature = "simd_support")]
simd_impl!(f32, u32);
#[cfg(feature = "simd_support")]
simd_impl!(f64, u64);
pub(crate) trait BoolAsSIMD: Sized {
    fn any(self) -> bool;
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
pub trait SampleBorrow<Borrowed> {
    fn borrow(&self) -> &Borrowed;
}
impl BoolAsSIMD for bool {
    #[inline(always)]
    fn any(self) -> bool {
        self
    }
}
