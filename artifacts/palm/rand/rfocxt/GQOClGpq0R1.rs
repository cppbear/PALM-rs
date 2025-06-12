#[cfg(feature = "alloc")]
use alloc::string::String;
use core::array;
use core::char;
use core::num::Wrapping;
#[cfg(feature = "alloc")]
use crate::distr::SampleString;
use crate::distr::{Distribution, StandardUniform, Uniform};
use crate::Rng;
#[cfg(feature = "simd_support")]
use core::simd::prelude::*;
#[cfg(feature = "simd_support")]
use core::simd::{LaneCount, MaskElement, SupportedLaneCount};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
macro_rules! tuple_impl {
    ($($tyvar:ident)*) => {
        impl < $($tyvar,)* > Distribution < ($($tyvar,)*) > for StandardUniform where
        $(StandardUniform : Distribution < $tyvar >,)* { #[inline] fn sample < R : Rng +
        ? Sized > (& self, rng : & mut R) -> ($($tyvar,)*) { let out = ($(rng
        .random::<$tyvar > (),)*); let _rng = rng; out } }
    };
}
macro_rules! tuple_impls {
    ($($tyvar:ident)*) => {
        tuple_impls! { [] $($tyvar)* }
    };
    ([$($prefix:ident)*] $head:ident $($tail:ident)*) => {
        tuple_impl! { $($prefix)* } tuple_impls! { [$($prefix)* $head] $($tail)* }
    };
    ([$($prefix:ident)*]) => {
        tuple_impl! { $($prefix)* }
    };
}
tuple_impls! {
    A B C D E F G H I J K L
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
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StandardUniform;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "X::Sampler: Serialize")))]
#[cfg_attr(
    feature = "serde",
    serde(bound(deserialize = "X::Sampler: Deserialize<'de>"))
)]
pub struct Uniform<X: SampleUniform>(X::Sampler);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `low > high`, or equal in case of exclusive range.
    EmptyRange,
    /// Input or range `high - low` is non-finite. Not relevant to integer types.
    NonFinite,
}
impl<T> Distribution<Wrapping<T>> for StandardUniform
where
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Wrapping<T> {
        Wrapping(rng.random())
    }
}
