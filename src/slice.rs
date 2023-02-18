use crate::distribution::Distribution;
use crate::uniform::{Uniform, UniformFloat};
use crate::utils::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Slice<'a, T> {
    slice: &'a [T],
    range: Uniform<usize>,
}

impl<'a, T> Slice<'a, T> {
    // create a new 'slice' instance
    pub fn new(slice: &'a [T]) -> Result<Self, EmptySlice> {
        match slice.len() {
            0 => Err(EmptySlice),
            len => Ok(Self {
                slice,
                range: Uniform::new(0, len).unwrap(),
            }),
        }
    }
}

impl<'a, T> Distribution<&'a T> for Slice<'a, T>  {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> &'a T {
        let idx = self.range.sample(rng);
        debug_assert!(
            idx < self.slice.len(),
            "Uniform::new(0, {}) somehow returned {}",
            self.slice.len(),
            idx
        );

        //safety: at construction time, it was ensured that the slice was
        // non-empty, and that the 'Uniform' range produces values in range
        // for the slice
        unsafe { self.slice.get_unchecked(idx) }
    }
}

#[derive(Debug, Clone,Copy)]
pub struct EmptySlice;

impl core::fmt::Display for EmptySlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tried to create a slice with empty slice"
        )
    }
}