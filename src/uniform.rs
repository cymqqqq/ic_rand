use std::fmt;
use std::ops::{RangeInclusive, Range};

use crate::distribution::Distribution;
use crate::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    EmptyRange,
    NonFinite,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::EmptyRange => "low > high (for equal if exclusive) in uniform distribution",
            Error::NonFinite => "non-finite range in uniform distribution",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde1", serde(bound(serialize = "X::Sampler: Serialize")))]
#[cfg_attr(feature = "serde1", serde(bound(deserialize = "X::Sampler: Deserialize<'de>")))]
pub struct  Uniform<X: SampleUniform>(X::Sampler);

impl<X: SampleUniform> Uniform<X> {
    // create a new uniform instance, which samples uniformly from the half
    // open range (low, high) 
    // fails if low >= high or if 'low', 'high' or the range 'high - low' is
    // non-finite, 
    pub fn new<B1, B2>(low: B1, high: B2) -> Result<Uniform<X>, Error>
    where
        B1: SampleBorrow<X> + Sized,
        B2: SampleBorrow<X> + Sized,
    {
        X::Sampler::new(low, high).map(Uniform)
    }

    // create a new 'uniform' instance, which samples unformly from the closed
    // range '(low, high)' 
    pub fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Uniform<X>, Error> 
    where
        B1: SampleBorrow<X> + Sized,
        B2: SampleBorrow<X> + Sized,
    {
        X::Sampler::new_inclusive(low, high).map(Uniform)
    }
}

impl<X: SampleUniform> Distribution<X> for Uniform<X> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> X {
        self.0.sample(rng)
    }
}

pub trait SampleUniform: Sized {
    //UniformeSampler
    type Sampler: UniformSampler<X = Self>;
}

pub trait  UniformSampler: Sized {
    type X;

    fn new<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized;
    
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Self, Error> 
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized;

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X;

    fn sample_single<R: Rng + ?Sized, B1, B2>(low: B1, high: B2, rng: &mut R) -> Result<Self::X, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let uniform: Self = UniformSampler::new(low, high)?;
        Ok(uniform.sample(rng))
    }

    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low: B1, high: B2, rng: &mut R) 
    -> Result<Self::X, Error> 
    where 
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized
    {
        let uniform: Self =UniformSampler::new_inclusive(low, high)?;
        Ok(uniform.sample(rng))
    }
}

impl<X: SampleUniform> TryFrom<Range<X>> for Uniform<X> {
    type Error = Error;

    fn try_from(value: ::core::ops::Range<X>) -> Result<Uniform<X>, Self::Error> {
        Uniform::new(value.start, value.end)
    }
}

impl<X: SampleUniform> TryFrom<RangeInclusive<X>> for Uniform<X> {
    type Error = Error;

    fn try_from(value: ::core::ops::RangeInclusive<X>) -> Result<Uniform<X>, Error> {
        Uniform::new_inclusive(value.start(), value.end())
    }
}

pub trait SampleBorrow<Borrowed> {
    fn borrow(&self) -> &Borrowed;
}

impl<Borrowed> SampleBorrow<Borrowed> for Borrowed
where Borrowed: SampleUniform
{
    fn borrow(&self) -> &Borrowed {
        self
    }
}

impl<'a, Borrowed> SampleBorrow<Borrowed> for &'a Borrowed
where Borrowed: SampleUniform
{
    fn borrow(&self) -> &Borrowed {
        self
    }
}

pub trait SampleRange<T> {
    fn sample_single<R: Rng + ?Sized> (self, rng: &mut R) -> Result<T, Error>;

    fn is_empty(&self) -> bool;
}

impl<T: SampleUniform + PartialOrd> SampleRange<T> for Range<T> {
    fn sample_single<R: Rng + ?Sized> (self, rng: &mut R) -> Result<T, Error> {
        T::Sampler::sample_single(self.start, self.end, rng)
    }

    fn is_empty(&self) -> bool {
        !(self.start < self.end)
    }
}

impl<T: SampleUniform + PartialOrd> SampleRange<T> for RangeInclusive<T> {
    fn sample_single<R: Rng + ?Sized> (self, rng: &mut R) -> Result<T, Error> {
        T::Sampler::sample_single_inclusive(self.start(), self.end(), rng)
    }

    fn is_empty(&self) -> bool {
        !(self.start() <= self.end())
    }
}

#[derive(Clone, Copy, Debug ,PartialEq)]
#[cfg_attr(feature="serde1", derive(Serialize, Deserialize))]
pub struct UniformInt<X> {
    low: X,
    range: X,
    z: X,
}

// macro_rules! uniform_int_impl {
//     ($ty: ty, $unsigned: ident, $u_large: ident) => {
//         impl SampleUniform for $ty {
//             type Sampler = UniformInt<$ty>;
//         }

//         implUniformeSampler for UniformInt<$ty> {
//             type X = $ty;

//             #[inline]
//             fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
//             where
//                 B1: SampleBorrow<Self::X> + Sized,
//                 B2: SampleBorrow<Self::X> + Sized,
//             {
//                 let low = *low_b.borrow();
//                 let high = *high_b.borrow();
//                 if !(low < high) {
//                     return Err(Error::EmptyRange);
//                 }
//                UniformeSampler::new_inclusive(low, high - 1)
//             }

//             #[inline]
//             fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
//             where
//                 B1: SampleBorrow<Self::X> + Sized,
//                 B2: SampleBorrow<Self::X> + Sized,
//             {
//                 let low = *low_b.borrow();
//                 let high = *high_b.borrow();

//                 if !(low <= high) {
//                     return Err(Error::EmptyRange);
//                 }
//                 let unsigned_max = ::core::$u_large::MAX;

//                 let range = high.wrapping_sub(low).wrapping_add(1) as $unsigned;
//                 let ints_to_reject = if range > 0 {
//                     let range = $u_large::from(range);
//                     (unsigned_max - range + 1) % range
//                 } else {
//                     0
//                 };

//                 Ok(UniformInt {
//                     low ,
//                     range: range as $ty,
//                     z: ints_to_reject as $unsigned as $ty,
//                 })
//             }

//             #[inline]
//             fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
//                 let range = self.range as $unsigned as $u_large;
//                 if range > 0 {
//                     let unsigned_max = ::core::$u_large::MAX;
//                     let zone = unsigned_max - (self.z as $unsigned as $u_large);
//                     loop {
//                         let v: $u_large = rng.
//                     }
//                 }
//             }
//         }
//     };

// }

// impl SampleUniform for char {
//     type Sampler = UniformChar;
// }

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature="serde1", derive(Serialize, Deserialize))]
pub struct UniformChar {
    sampler: UniformInt<u32>,
}

// UTF-16 surrogate range start
const CHAR_SURROGATE_START: u32 = 0xD800;
// UTF-16 surrogate range size
const CHAR_SURROGATE_LEN: u32 = 0xE000 - CHAR_SURROGATE_START;

//convert 'char' to compressed 'u32'
fn char_to_comp_u32(c: char) -> u32 {
    match c as u32 {
        c if c >= CHAR_SURROGATE_START => c - CHAR_SURROGATE_LEN,
        c => c,
    }
}

// impl UniformSampler for UniformChar {
//     type X = char;
//     #[inline]
//     fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
//     where
//         B1: SampleBorrow<Self::X> +  Sized,
//         B2: SampleBorrow<Self::X> + Sized,
//     {
//         let low = char_to_comp_u32(*low_b.borrow());
//         let high = char_to_comp_u32(*high_b.borrow());
//         let sampler = UniformInt::<u32>::new(low, high);
//         sampler.map(|sampler| UniformChar { sampler })
//     }

//     #[inline]
//     fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error> 
//         where
//             B1: SampleBorrow<Self::X> + Sized,
//             B2: SampleBorrow<Self::X> + Sized 
//     {
//         let low = char_to_comp_u32(*low_b.borrow());
//         let high = char_to_comp_u32(*high_b.borrow());
//         let sampler = UniformInt::<u32>::new_inclusive(low, high);
//         sampler.map(|sampler| UniformChar { sampler })
//     }

//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
//         let mut x=  self.sampler.sample(rng);
//         if x >= CHAR_SURROGATE_START {
//             x += CHAR_SURROGATE_LEN;
//         }

//         unsafe { core::char::from_u32_unchecked(x)}
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature="serde1", derive(Serialize, Deserialize))]
pub struct UniformFloat<X> {
    low: X,
    scale: X,
}

// macro_rules! uniform_float_impl {
//     ($ty: ty, $uty: ident, $f_scalar: ident, $u_scalar: ident, $bits_to_discard: expr) => {
//         impl SampleUniform for $ty {
//             type Sampler = UniformFloat<$ty>;
//         }

//         impl UniformSampler for UniformFloat<$ty> {
//             type X = $ty;

//             fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
//             where 
//                 B1: SampleBorrow<Self::X> + Sized,
//                 B2: SampleBorrow<Self::X> + Sized,
//             {
//                 let low = *low_b.borrow();
//                 let high = *high_b.borrow();
//                 #[cfg(debug_assertions)]
//                 if !(low.all_finite()) || !(high.all_finite()) {
//                     return Err(Error::NonFinite);
//                 }

//                 if !(low.all_lt(high)) {
//                     return Err(Error::EmptyRange);
//                 }

//                 let max_rand = <$ty>::splat(
//                     (::core::$u_scalar::MAX >> $bits_to_discard).into_float_with_exponent(0) - 1.0,
//                 );
//                 let mut scale = high -  low;
//                 if !(scale.all_finite()) {
//                     return Err(Error::EmptyRange);
//                 }

//                 loop {
//                     let mask = (scale * max_rand + low).ge_mask(high);
//                     if !mask.any() {
//                         break;
//                     }
//                     scale = scale.decrease_masked(mask);
//                 }
//                 debug_assert!(<$ty>::splat(0.0).all_le(scale));
//                 Ok(UniformFloat { low, scale })
//             }

//             fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
//             where
//                 B1: SampleBorrow<Self::X> + Sized,
//                 B2: SampleBorrow<Self::X> + Sized,
//             {
//                 let low = *low_b.borrow();
//                 let high = *high_b.borrow();
//                 #[cfg(debug_assertions)]
//                 if !(low.all_finite()) || !(high.all_finite()) {
//                     return Err(Error::NonFinite);
//                 }
//                 if !low.all_le(high) {
//                     return Err(Error::EmptyRange);
//                 }
//                 let max_rand = <$ty>::splat(
//                     (::core::$u_scalar::MAX >> $bits_to_discard).into_float_with_exponent(0) - 1.0,
//                 );

//                 let mut scale = (high - low) / max_rand;
//                 if !scale.all_finite() {
//                     return Err(Error::NonFinite);
//                 }

//                 loop {
//                     let mask = (scale * max_rand + low).gt_mask(high);
//                     if !mask.any() {
//                         break;
//                     }
//                     scale = scale.decrease_masked(mask);
//                 }
//                 debug_assert!(<$ty>::splat(0.0).all_le(scale));
//                 Ok(UniformFloat { low, scale })
//             }
//         }
//     };
// }

// uniform_float_impl!( f32, u32, f32, u32, 32 - 23);
// uniform_float_impl!( f64, u64, f64, u64, 64 - 52);