use std::iter;

use crate::utils::Rng;

pub trait Distribution<T> {
    ///generate a random value of 'T, in current stage using 'Rng' as the source of randomness.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;

    ///create an iterator that generates random values of 'T', using 'Rng' as 
    /// the source of randomness.
    /// 
    fn sample_iter<R>(self,rng: R) -> DistIter<Self, R, T>
    where
        R: Rng,
        Self: Sized,
    {
        DistIter {
            dist: self,
            rng,
            phantom: ::core::marker::PhantomData,
        }
    }

    ///create a distribution of values of 'S' by mapping the output of 'Self'
    /// through the closure 'F'
    /// 
    fn map<F, S>(self, func: F) -> DistMap<Self, F, T, S> 
    where
        F: Fn(T) -> S,
        Self: Sized,
    {
        DistMap {
            dist: self,
            func,
            phantom: ::core::marker::PhantomData,
        }
    }
}

impl<'a, T, D:Distribution<T> + ?Sized> Distribution<T> for &'a D {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        (*self).sample(rng)
    }
}

#[derive(Debug)]
pub struct DistIter<D,R, T> {
    dist: D,
    rng: R,
    phantom: ::core::marker::PhantomData<T>,
}

impl<D, R, T> Iterator for DistIter<D, R, T>
where 
    D: Distribution<T>,
    R: Rng,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        Some(self.dist.sample(&mut self.rng))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::min_value(), None)
    }
}

impl<D, R, T> iter::FusedIterator for DistIter<D, R, T>
where 
    D: Distribution<T>,
    R: Rng,
{

}

#[derive(Debug)]
pub struct DistMap<D, F, T, S> {
    dist: D,
    func: F,
    phantom: ::core::marker::PhantomData<fn(T) -> S>,
}

impl<D, F, T, S> Distribution<S> for DistMap<D, F, T, S>
where
    D: Distribution<T>,
    F: Fn(T) -> S,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> S {
        (self.func)(self.dist.sample(rng))
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Standard;
