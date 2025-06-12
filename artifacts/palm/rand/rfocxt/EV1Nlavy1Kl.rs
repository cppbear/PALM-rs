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
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Alphabetic;
impl Distribution<u8> for Alphabetic {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u8 = 26 + 26;
        let offset = rng.random_range(0..RANGE) + b'A';
        offset + (offset > b'Z') as u8 * (b'a' - b'Z' - 1)
    }
}
