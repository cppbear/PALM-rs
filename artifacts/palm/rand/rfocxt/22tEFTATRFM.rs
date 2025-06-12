use crate::Rng;
#[cfg(feature = "alloc")]
use alloc::string::String;
use core::iter;
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
#[derive(Debug)]
pub struct Map<D, F, T, S> {
    distr: D,
    func: F,
    phantom: core::marker::PhantomData<fn(T) -> S>,
}
impl<D, F, T, S> Distribution<S> for Map<D, F, T, S>
where
    D: Distribution<T>,
    F: Fn(T) -> S,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> S {
        (self.func)(self.distr.sample(rng))
    }
}
