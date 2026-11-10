//! # Lesson 1 — Vectors in Space
//!
//! A `Vec2` is a 2D vector: an arrow with an `x` and a `y` component.
//! We use it for positions ("where is the lander?"), velocities ("how fast,
//! and in which direction?") and forces.
//!
//! **Coordinate system:** units are metres, `x` grows to the right and
//! `y` grows *upwards* (like in a physics book, not like on a screen).
//!
//! **Angles** are in radians. An angle of `0.0` means "pointing straight up";
//! positive angles rotate *clockwise* (the lander's nose tips to the right).

use std::ops::{Add, AddAssign, Mul, Neg, Sub};

/// A 2D vector with `f32` components.
///
/// The `derive` line asks the compiler to write some trait implementations
/// for us:
/// - `Debug`     → printable with `{:?}`
/// - `Clone`     → can be duplicated with `.clone()`
/// - `Copy`      → duplicated *implicitly* — assigning a `Vec2` copies it
///                 instead of moving it (cheap: it's only 8 bytes)
/// - `PartialEq` → comparable with `==`
/// - `Default`   → `Vec2::default()` gives `(0, 0)`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// The zero vector `(0, 0)`. Associated *constants* live on the type.
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    /// Creates a new vector. This is an *associated function* (no `self`),
    /// called as `Vec2::new(1.0, 2.0)`.
    pub fn new(x: f32, y: f32) -> Vec2 {
        // ✏️ LESSON 1: build and return a Vec2 from `x` and `y`
        Vec2 { x, y }
    }

    /// The length (magnitude) of the vector: √(x² + y²).
    ///
    /// This is a *method*: it takes `self`, so you call it as `v.length()`.
    /// Because `Vec2` is `Copy`, taking `self` by value is fine and cheap.
    pub fn length(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// A vector of length 1 pointing in direction `angle`.
    ///
    /// Angle `0.0` points straight up `(0, 1)`; positive angles turn
    /// clockwise, so `PI / 2` points right `(1, 0)`.
    /// Hint: x = sin(angle), y = cos(angle).
    pub fn from_angle(angle: f32) -> Vec2 {
        let x = angle.sin();
        let y = angle.cos();
        Vec2 { x, y }
    }

    /// Rotates the vector clockwise by `angle` radians.
    ///
    /// The formula for a clockwise rotation is:
    /// ```text
    /// x' =  x·cos(a) + y·sin(a)
    /// y' = -x·sin(a) + y·cos(a)
    /// ```
    /// Used to turn the lander's outline when it tilts.
    pub fn rotated(self, angle: f32) -> Vec2 {
        // ✏️ LESSON 1: apply the rotation formula from the doc comment
        let x = self.x * angle.cos() + self.y * angle.sin();
        let y = - self.x * angle.sin() + self.y * angle.cos();
        Vec2 { x, y }
    }

    /// Linear interpolation: `t = 0` gives `self`, `t = 1` gives `other`.
    /// *(Provided as an example of a method that uses the operators below.)*
    pub fn lerp(self, other: Vec2, t: f32) -> Vec2 {
        self + (other - self) * t
    }
}

// ---------------------------------------------------------------------------
// Operator overloading
//
// In Rust, `a + b` is just sugar for `Add::add(a, b)`. If we implement the
// `Add` trait for `Vec2`, we can write `position + velocity`!
// ---------------------------------------------------------------------------

/// `-v` — **worked example**: study this one, then do the others.
impl Neg for Vec2 {
    /// The type the operator produces.
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

/// `a + b`
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        // ✏️ LESSON 1: add the components
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vec2 { x, y }
    }
}

/// `a - b`
impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        // ✏️ LESSON 1: subtract the components
        let x = self.x - other.x;
        let y = self.y - other.y;
        Vec2 { x, y }
    }
}

/// `v * 2.0` — scaling a vector by a number. Note the generic parameter:
/// `Mul<f32>` means "multiply a Vec2 *by an f32*".
impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, factor: f32) -> Vec2 {
        let x = self.x * factor;
        let y = self.y * factor;
        Vec2 { x, y }
    }
}

/// `a += b` — modifies the left-hand side in place, so it takes `&mut self`
/// and returns nothing.
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        // ✏️ LESSON 1: update self.x and self.y
        self.x += other.x;
        self.y += other.y;
    }
}
