use core::{fmt, num};

use crate::distribution::Distribution;
use crate::utils::Rng;

#[cfg(feature="serde1")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature="serde1", derive(Serialize, Deserialize))]
pub struct Bernoulli {
    p_int: u64,
}

const ALWAYS_TRUE: u64 = u64::MAX;

const SCALE: f64 = 2.0 * (1u64 << 63) as f64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BernoulliError {
    InvalidProbability,
}

impl fmt::Display for BernoulliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            BernoulliError::InvalidProbability => "p is outside [0, 1] in Bernoulli distribution",
        })
    }
}

impl Bernoulli {
    #[inline]
    pub fn new(p: f64) -> Result<Bernoulli, BernoulliError> {
        if !(0.0..1.0).contains(&p) {
            if p == 1.0 {
                return Ok(Bernoulli { p_int: ALWAYS_TRUE });
            }
            return Err(BernoulliError::InvalidProbability);
        }
        Ok(Bernoulli { 
            p_int: (p * SCALE) as u64,
         })
    }

    /// Construct a new `Bernoulli` with the probability of success of
    /// `numerator`-in-`denominator`. I.e. `new_ratio(2, 3)` will return
    /// a `Bernoulli` with a 2-in-3 chance, or about 67%, of returning `true`.
    ///
    /// return `true`. If `numerator == 0` it will always return `false`.
    /// For `numerator > denominator` and `denominator == 0`, this returns an
    /// error. Otherwise, for `numerator == denominator`, samples are always
    /// true; for `numerator == 0` samples are always false.
    #[inline]
    pub fn from_ratio(numerator: u32, denominator: u32) -> Result<Bernoulli, BernoulliError> {
        if numerator > denominator || denominator == 0 {
            return  Err(BernoulliError::InvalidProbability);
        }
        if numerator == denominator {
            return Ok(Bernoulli { p_int: ALWAYS_TRUE });
        }
        let p_int = ((f64::from(numerator) / f64::from(denominator)) * SCALE) as u64;
        Ok(Bernoulli { p_int: p_int })
    }
}

impl Distribution<bool> for Bernoulli {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        //make sure to always return true for p = 1.0
        if self.p_int == ALWAYS_TRUE {
            return true;
        }
        let v: u32 = rng.rand_u32();
        v < self.p_int as u8 as u32
    }
}

