use std::fmt;

use crate::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use crate::distribution::Distribution;
use crate::utils::*;

#[cfg(feature="serde1")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature="serde1", derive(Serialize, Deserialize))]
pub struct WeightIndex<X: SampleUniform + PartialOrd> {
    cumulative_weights: Vec<X>,
    total_weight: X,
    weight_distribution: X::Sampler,
}

impl<X: SampleUniform + PartialOrd> WeightIndex<X> {
    pub fn new<I>(weights: I) -> Result<WeightIndex<X>, WeightedError>
    where 
        I: IntoIterator,
        I::Item: SampleBorrow<X>,
        X: for<'a> ::core::ops::AddAssign<&'a X> + Clone + Default,
    {
        let mut iter = weights.into_iter();
        let mut total_weight: X = iter.next().ok_or(WeightedError::NoItem)?.borrow().clone();

        let zero= <X as Default>::default();
        if !(total_weight >= zero) {
            return Err(WeightedError::InvalidWeight);
        }

        let mut weights = Vec::<X>::with_capacity(iter.size_hint().0);
        for w in iter {
            if !(w.borrow() >= &zero) {
                return  Err(WeightedError::InvalidWeight);
            }
            weights.push(total_weight.clone());
            total_weight += w.borrow();
        }

        if total_weight == zero {
            return Err(WeightedError::AllWeightsZero);
        }
        let dist = X::Sampler::new(zero, total_weight.clone()).unwrap();
        Ok(WeightIndex { 
            cumulative_weights: weights,
             total_weight: total_weight, 
             weight_distribution: dist 
        })
    }

    pub fn update_weights(&mut self, new_weights: &[(usize, &X)]) -> Result<(), WeightedError>
    where X: for<'a> ::core::ops::AddAssign<&'a X>
        + for<'a> ::core::ops::SubAssign<&'a X>
        + Clone
        + Default
        {
            if new_weights.is_empty() {
                return  Ok(());
            }

            let zero = <X as Default>::default();
            let mut total_weight = self.total_weight.clone();

            let mut prev_i = None;
            for &(i, w) in new_weights {
                if let Some(old_i) = prev_i {
                    if old_i >= i {
                        return Err(WeightedError::InvalidWeight);
                    }
                }
                if !(*w >= zero) {
                    return  Err(WeightedError::InvalidWeight);
                }
                if i > self.cumulative_weights.len() {
                    return  Err(WeightedError::TooMany);
                }
                let mut old_w = if i < self.cumulative_weights.len() {
                    self.cumulative_weights[i].clone()
                } else {
                    self.total_weight.clone()
                };

                if i > 0 {
                    old_w -= &self.cumulative_weights[i - 1];
                }

                total_weight -= &old_w;
                total_weight += w;
                prev_i = Some(i);
            }
            if total_weight <= zero {
                return  Err(WeightedError::AllWeightsZero);
            }

            let mut iter = new_weights.iter();
            let mut prev_weight = zero.clone();
            let mut next_new_weight = iter.next();
            let &(first_new_index, _) = next_new_weight.unwrap();
            let mut cumulative_weight = if first_new_index > 0 {
                self.cumulative_weights[first_new_index - 1].clone()
            } else {
                zero.clone()
            };

            for i in first_new_index..self.cumulative_weights.len() {
                match next_new_weight {
                    Some(&(j, w)) if i == j => {
                        cumulative_weight += w;
                        next_new_weight = iter.next();
                    }
                    _ => {
                        let mut tmp = self.cumulative_weights[i].clone();
                        tmp -= &prev_weight;
                        cumulative_weight += &tmp;
                    }
                }
                prev_weight = cumulative_weight.clone();
                core::mem::swap(&mut prev_weight, &mut self.cumulative_weights[i]);
            }
            self.total_weight = total_weight;
            self.weight_distribution = X::Sampler::new(zero, self.total_weight.clone()).unwrap();
            Ok(())
        }
}

impl<X> Distribution<usize> for WeightIndex<X>
where X: SampleUniform + PartialOrd 
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        let choose_weight = self.weight_distribution.sample(rng);

        self.cumulative_weights.partition_point(|w| w <= &choose_weight)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightedError {
    /// The provided weight collection contains no items.
    NoItem,

    /// A weight is either less than zero, greater than the supported maximum,
    /// NaN, or otherwise invalid.
    InvalidWeight,

    /// All items in the provided weight collection are zero.
    AllWeightsZero,

    /// Too many weights are provided (length greater than `u32::MAX`)
    TooMany,
}

impl fmt::Display for WeightedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            WeightedError::NoItem => "No weights provided in distribution",
            WeightedError::InvalidWeight => "A weight is invalid in distribution",
            WeightedError::AllWeightsZero => "All weights are zero in distribution",
            WeightedError::TooMany => "Too many weights (hit u32::MAX) in distribution",
        })
    }
}