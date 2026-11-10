//! A tiny, deterministic pseudo-random number generator.
//!
//! *(Provided — no lesson here, but have a read!)*
//!
//! Real projects would use the `rand` crate. We roll our own "xorshift"
//! generator so that the same `seed` always produces the same moon surface,
//! which makes tests repeatable.

/// A pseudo-random number generator. Same seed ⇒ same sequence of numbers.
#[derive(Debug, Clone)]
pub struct Rng {
    state: u64,
}

impl Rng {
    /// Creates a generator from a seed. Any seed (even 0) works.
    pub fn new(seed: u64) -> Rng {
        // Mix the seed so that similar seeds give very different sequences.
        // `wrapping_mul` multiplies and silently wraps around on overflow
        // (a plain `*` would panic on overflow in debug builds).
        let state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        Rng { state }
    }

    /// The next raw 64-bit random number (xorshift64*).
    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A random `f32` in `0.0..1.0`.
    pub fn next_f32(&mut self) -> f32 {
        // Take the top 24 bits — exactly the precision of an f32 mantissa.
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    /// A random `f32` in `min..max`.
    pub fn range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }

    /// A random index in `0..len`. Panics if `len == 0`.
    pub fn index(&mut self, len: usize) -> usize {
        assert!(len > 0, "cannot pick an index from an empty range");
        (self.next_u64() % len as u64) as usize
    }
}
