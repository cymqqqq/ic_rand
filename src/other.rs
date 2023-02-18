use core::num::Wrapping;
use std::mem::{MaybeUninit, self};

use crate::distribution::{Distribution, Standard};
use crate::uniform::{Uniform, UniformFloat};
use crate::utils::Rng;

#[cfg(feature="serde1")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature="serde1", derive(Serialize, Deserialize))]
pub struct Alphanumeric;

// impl Distribution<char> for Standard {
//     #[inline]
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
//         // A valid `char` is either in the interval `[0, 0xD800)` or
//         // `(0xDFFF, 0x11_0000)`. All `char`s must therefore be in
//         // `[0, 0x11_0000)` but not in the "gap" `[0xD800, 0xDFFF]` which is
//         // reserved for surrogates. This is the size of that gap.
//         const GAP_SIZE: u32 = 0xDFFF - 0xD800 + 1;

//         // Uniform::new(0, 0x11_0000 - GAP_SIZE) can also be used, but it
//         // seemed slower.
//         let range = Uniform::new(GAP_SIZE, 0x11_0000).unwrap();

//         let mut n = range.sample(rng);
//         if n <= 0xDFFF {
//             n -= GAP_SIZE;
//         }
//         unsafe { char::from_u32_unchecked(n) }
//     }
// }

impl Distribution<u8> for Alphanumeric {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 26 + 10;
        const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789";
        // We can pick from 62 characters. This is so close to a power of 2, 64,
        // that we can do better than `Uniform`. Use a simple bitshift and
        // rejection sampling. We do not use a bitmask, because for small RNGs
        // the most significant bits are usually of higher quality.
        loop {
            let var = rng.rand_u32() >> (32 - 6);
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize];
            }
        }
    }
}

impl Distribution<bool> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        rng.rand_i32() < 0
    }
}

// macro_rules! tuple_impl {
//     ($($tyvar: ident), *) => {
//         //the trailing commas are for the 1 tuple
//         impl<$( $tyvar), *>
//             Distribution<($( $tyvar ),* , )>
//             for Standard
//             where $( Standard: Distribution<$tyvar> ),*
//         {
//             #[inline]
//             fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> ( $( $tyvar ),* , ) {
//                 $(
//                     _rng.gen::<$tyvar>()
//                 ),*
//                 ,
//             }
//         }
//     }
// }

impl Distribution<()> for Standard {
    #[allow(clippy::unused_unit)]
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> () {
        ()
    }
}

// tuple_impl! {A}
// tuple_impl! {A, B}
// tuple_impl! {A, B, C}
// tuple_impl! {A, B, C, D}
// tuple_impl! {A, B, C, D, E}
// tuple_impl! {A, B, C, D, E, F}
// tuple_impl! {A, B, C, D, E, F, G}
// tuple_impl! {A, B, C, D, E, F, G, H}
// tuple_impl! {A, B, C, D, E, F, G, H, I}
// tuple_impl! {A, B, C, D, E, F, G, H, I, J}
// tuple_impl! {A, B, C, D, E, F, G, H, I, J, K}
// tuple_impl! {A, B, C, D, E, F, G, H, I, J, K, L}

// impl<T, const N: usize> Distribution<[T; N]> for Standard
// where Standard: Distribution<T>
// {
//     #[inline]
//     fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> [T; N] {
//         let mut buff: [MaybeUninit<T>; N] = unsafe {
//             MaybeUninit::uninit().assume_init()
//         };

//         for elem in &mut buff {
//             *elem = MaybeUninit::new(_rng.gen());
//         }
//         unsafe { mem::transmute_copy::<_, _>(&buff)}
//     }

// }

// impl<T> Distribution<Option<T>> for Standard
// where Standard: Distribution<T>
// {
//     #[inline]
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<T> {
//         if rng.gen::<bool>() {
//             Some(rng.gen())
//         } else {
//             None
//         }
//     }
// }

// impl<T> Distribution<Wrapping<T>> for Standard
// where Standard: Distribution<T>
// {
//     #[inline]
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Wrapping<T> {
//         Wrapping(rng.gen())
//     }
// }