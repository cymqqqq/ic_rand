#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rand32 {
    state: u64,
    inc: u64,
}

impl Rand32 {
    ///the default value for `increment`
    /// this is basically arbitrary, it comes from the
    /// PCG reference C implementation;
    pub const DEFAULT_INC: u64 = 1442695040888963407;
    
    ///this is the number that you have to really get right 
    ///the value used here is from the PCG C implementation
    pub(crate) const MULTIPLIER: u64 = 6364136223846793005;

    ///creates a new PRNG with a given seed and a default increment
    pub fn new(seed: u64) -> Self {
        Self::new_inc(seed, Self::DEFAULT_INC)
    }

    ///create a new PRNG. the two inputs, `seed` and `increment`
    /// determine what you get; `increment` basically selects which
    /// sequence of all those possible the PRNG will produce, and 
    /// the `seed` selects where in the sequence youstart
    /// 
    /// both are arbitrary; increment must be an odd number but this
    /// handles that for you
    pub fn new_inc(seed: u64, increment: u64) -> Self {
        let mut rng = Self {
            state: 0,
            inc: increment.wrapping_shl(1) | 1,
        };

        let _ = rng.rand_u32();
        rng.state = rng.state.wrapping_add(seed);
        let _ = rng.rand_u32();
        rng
    }

    // /returns the internal state of the PRNG. this alllow
    // / you to save a PRNG and create a new one that will 
    // / resume from the same spot in the sequence
    // pub fn state(&self) -> (u64, u64) {
    //     (self.state, self.state)
    // }

    // /createa a new PRNG from a saved state from 
    // / Rand32::state() 
    // / this is NOT quite the same as `new_inc` because
    // /`new_inc()` does a little extra setup work to
    // / initialize the state

    // pub fn from_state(state: (u64, u64)) -> Self {
    //     let (state, inc) = state;
    //     Self { state, inc }
    // }

    //produces a random `u32` in the range
    // `[0, u32::MAX]`
    // pub fn rand_u32(&mut self) -> u32 {
    //     let oldstate = self.state;
    //     self.state = oldstate
    //     .wrapping_mul(Self::MULTIPLIER)
    //     .wrapping_add(self.inc);

    //     let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
    //     let rot = (oldstate >> 59) as u32;
    //     xorshifted.rotate_right(rot)
    // }

    //produces a random `i32` in the range `[i32::MIN, i32::MAX]`
    //
    // pub fn rand_i32(&mut self) -> i32 {
    //     self.rand_u32() as i32
    // }

    // pub fn rand_f32(&mut self) -> f32 {
    //     // This impl was taken more or less from `rand`, see
    //     // <https://docs.rs/rand/0.7.0/src/rand/distributions/float.rs.html#104-117>
    //     // There MAY be better ways to do this, see:
    //     // https://mumble.net/~campbell/2014/04/28/uniform-random-float
    //     // https://mumble.net/~campbell/2014/04/28/random_real.c
    //     // https://github.com/Lokathor/randomize/issues/34
    //     const TOTAL_BITS: u32 = 32;
    //     const PRECISION: u32 = std::f32::MANTISSA_DIGITS + 1;
    //     const MANTISSA_SCALE: f32 = 1.0 / ((1u32 << PRECISION) as f32);
    //     let mut u = self.rand_u32();
    //     u >>= TOTAL_BITS - PRECISION;
    //     u as f32 * MANTISSA_SCALE
    // }

    // pub fn gen_range(&mut self, low: f32, high: f32) -> f32 {
    //     let r = self.rand_f32();
    //     let result = low + (high - low) * r;
    //     result
    // }
}

pub trait Rng {
    fn state(&self) -> (u64, u64);

    fn from_state(state: (u64, u64)) -> Self;

    fn rand_u32(&mut self) -> u32;

    fn rand_i32(&mut self) -> i32;

    fn rand_f32(&mut self) -> f32;

    fn gen_range(&mut self, low: f32, high: f32) -> f32;
}

impl Rng for Rand32 {
    fn state(&self) -> (u64, u64) {
        (self.state, self.state)
    }

    fn from_state(state: (u64, u64)) -> Self {
        let (state, inc) = state;
        Self { state, inc }
    }

    fn rand_u32(&mut self) -> u32 {
        let oldstate = self.state;
        self.state = oldstate
        .wrapping_mul(Self::MULTIPLIER)
        .wrapping_add(self.inc);

        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot = (oldstate >> 59) as u32;
        xorshifted.rotate_right(rot)
    }

    fn rand_i32(&mut self) -> i32 {
        self.rand_u32() as i32
    }


    fn rand_f32(&mut self) -> f32 {
        // This impl was taken more or less from `rand`, see
        // <https://docs.rs/rand/0.7.0/src/rand/distributions/float.rs.html#104-117>
        // There MAY be better ways to do this, see:
        // https://mumble.net/~campbell/2014/04/28/uniform-random-float
        // https://mumble.net/~campbell/2014/04/28/random_real.c
        // https://github.com/Lokathor/randomize/issues/34
        const TOTAL_BITS: u32 = 32;
        const PRECISION: u32 = std::f32::MANTISSA_DIGITS + 1;
        const MANTISSA_SCALE: f32 = 1.0 / ((1u32 << PRECISION) as f32);
        let mut u = self.rand_u32();
        u >>= TOTAL_BITS - PRECISION;
        u as f32 * MANTISSA_SCALE
    }

    fn gen_range(&mut self, low: f32, high: f32) -> f32 {
        let r = self.rand_f32();
        let result = low + (high - low) * r;
        result
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct  Rand64 {
    state: u128,
    inc: u128,
}

impl Rand64 {
    pub const DEFAULT_INC: u128 = 0x2FE0E169_FFBD06E3_5BC307BD_4D2F814F;

    pub(crate) const MULTIPLIER: u128 = 47026247687942121848144207491837523525;

    pub fn new(seed: u128) -> Self {
        Self::new_inc(seed, Self::DEFAULT_INC)
    }

    pub fn new_inc(seed: u128, increment: u128) -> Self {
        let mut rng = Self {
            state: 0,
            inc: increment.wrapping_shl(1) | 1,
        };

        let _ = rng.rand_u64();
        rng.state = rng.state.wrapping_add(seed);
        let _ = rng.rand_u64();
        rng
    }

    pub fn state(&self) -> (u128, u128) {
        (self.state, self.inc)
    }

    pub fn from_state(state: (u128, u128)) -> Self {
        let (state, inc) = state;
        Self { state, inc }
    }

    pub fn rand_u64(&mut self) -> u64 {
        let oldstate: u128 = self.state;
        self.state = oldstate
        .wrapping_mul(Self::MULTIPLIER)
        .wrapping_add(self.inc);
        
        let xorshifted: u64 = (((oldstate >> 29) ^ oldstate) >> 58) as u64;
        let rot: u32 = (oldstate >> 122) as u32;
        xorshifted.rotate_right(rot)
    }

    pub fn rand_i64(&mut self) -> i64 {
        self.rand_u64() as i64 
    }

    pub fn rand_f64(&mut self) -> f64 {
        const TOTAL_BITS: u32 = 64;
        const PRECISION: u32 = core::f64::MANTISSA_DIGITS + 1;
        const MANTISSA_SCALE: f64 = 1.0 / ((1u64 << PRECISION) as f64);

        let mut u = self.rand_u64();
        u >>= TOTAL_BITS - PRECISION;
        u as f64 * MANTISSA_SCALE
    }

    
}