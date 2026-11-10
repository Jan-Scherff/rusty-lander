//! # Lesson 4 — Lunar Terrain
//!
//! The Moon's surface is a list of points joined by straight lines. Flat
//! segments that are wide enough are *landing pads*. Narrower pads are harder
//! to hit, so they award a bigger score multiplier.

use crate::rng::Rng;
use crate::vec2::Vec2;

/// Width of the playing field in metres.
pub const WORLD_WIDTH: f32 = 200.0;
/// A flat segment must be at least this wide to count as a landing pad.
pub const MIN_PAD_WIDTH: f32 = 5.0;
/// Two heights closer than this count as "equal" (never compare floats
/// with `==` after doing arithmetic on them!).
pub const FLAT_TOLERANCE: f32 = 0.001;

/// A flat piece of ground the lander may land on.
#[derive(Debug, Clone, PartialEq)]
pub struct LandingPad {
    pub x_start: f32,
    pub x_end: f32,
    pub y: f32,
    pub multiplier: u32,
}

impl LandingPad {
    /// How wide the pad is.
    pub fn width(&self) -> f32 {
        self.x_end - self.x_start
    }

    /// The x coordinate of the pad's middle.
    pub fn center_x(&self) -> f32 {
        (self.x_start + self.x_end) / 2.0
    }
}

/// The ground: points sorted from left to right (every point has a strictly
/// larger `x` than the one before it), plus the landing pads found in them.
#[derive(Debug, Clone)]
pub struct Terrain {
    pub points: Vec<Vec2>,
    pub pads: Vec<LandingPad>,
}

impl Terrain {
    /// The score multiplier for a pad of the given width:
    /// - narrower than 8 m  → 5
    /// - narrower than 12 m → 3
    /// - anything wider     → 2
    pub fn pad_multiplier(width: f32) -> u32 {
        // ✏️ LESSON 4: an `if / else if / else` *expression*, or a match with guards
        todo!("Lesson 4")
    }

    /// Builds a terrain from its points, *finding the landing pads*.
    ///
    /// Look at every pair of neighbouring points (`points.windows(2)`).
    /// A pair is a landing pad when both heights are equal (within
    /// `FLAT_TOLERANCE`) and it is at least `MIN_PAD_WIDTH` wide.
    ///
    /// Note the signature: this function takes the `Vec` *by value* —
    /// ownership moves into the function and then into the new `Terrain`.
    pub fn from_points(points: Vec<Vec2>) -> Terrain {
        // ✏️ LESSON 4: iterate with windows(2), filter, map, collect
        todo!("Lesson 4")
    }

    /// How wide the terrain is: the `x` of the last point, or `0.0` if there
    /// are no points at all.
    pub fn width(&self) -> f32 {
        // ✏️ LESSON 4: `self.points.last()` returns an Option<&Vec2>
        todo!("Lesson 4")
    }

    /// The height of the ground at `x`, or `None` if `x` lies outside the
    /// terrain.
    ///
    /// Find the pair of neighbouring points `a`, `b` with `a.x <= x <= b.x`,
    /// then interpolate: `t = (x - a.x) / (b.x - a.x)`, height =
    /// `a.y + (b.y - a.y) * t`.
    pub fn height_at(&self, x: f32) -> Option<f32> {
        // ✏️ LESSON 4: loop over windows(2) and `return Some(..)` early
        todo!("Lesson 4")
    }

    /// The landing pad directly below `x`, if there is one.
    ///
    /// The return type borrows from `self`: the `&LandingPad` points *into*
    /// our `pads` vector, so it can't outlive the terrain.
    pub fn pad_at(&self, x: f32) -> Option<&LandingPad> {
        // ✏️ LESSON 4: `.iter().find(|pad| ...)`
        todo!("Lesson 4")
    }

    /// The `y` of the highest point (or `f32::MIN` without points).
    ///
    /// Careful: `f32` is not `Ord` (because of NaN), so `.max()` on an
    /// iterator of floats won't compile. Try `fold` with `f32::max`.
    pub fn highest_point(&self) -> f32 {
        // ✏️ LESSON 4: map to y, then fold
        todo!("Lesson 4")
    }

    // ----------------------------- Provided ---------------------------------

    /// Generates a random moon surface with three landing pads.
    pub fn generate(seed: u64) -> Terrain {
        Terrain::from_points(Terrain::generate_points(seed))
    }

    /// The random point generator behind [`Terrain::generate`].
    ///
    /// Walks from left to right, taking random steps up and down, and puts
    /// three flat pads (6 m, 9 m and 14 m wide) into three regions of the map.
    pub fn generate_points(seed: u64) -> Vec<Vec2> {
        let mut rng = Rng::new(seed);

        // Shuffle the pad widths so every level looks different.
        let mut widths = [6.0, 9.0, 14.0];
        for i in (1..widths.len()).rev() {
            widths.swap(i, rng.index(i + 1));
        }
        // One pad in each third of the map: (start, width) pairs.
        let mut pads = Vec::new();
        for (region, width) in widths.into_iter().enumerate() {
            let region_start = 12.0 + region as f32 * 62.0;
            let start = region_start + rng.range(0.0, 50.0 - width);
            pads.push((start, width));
        }

        let mut points = vec![Vec2::new(0.0, rng.range(30.0, 70.0))];
        let mut next_pad = 0;
        loop {
            let last = *points.last().expect("points is never empty");
            if last.x >= WORLD_WIDTH {
                break;
            }
            let step = rng.range(3.0, 8.0);

            if let Some(&(pad_start, pad_width)) = pads.get(next_pad) {
                if last.x + step >= pad_start {
                    // Flatten: a slope down/up to the pad, then the pad itself.
                    let pad_y = (last.y + rng.range(-10.0, 6.0)).clamp(4.0, 60.0);
                    let pad_y = if (pad_y - last.y).abs() < 1.0 {
                        pad_y + 2.0
                    } else {
                        pad_y
                    };
                    points.push(Vec2::new(pad_start.max(last.x + 1.0), pad_y));
                    points.push(Vec2::new(pad_start.max(last.x + 1.0) + pad_width, pad_y));
                    next_pad += 1;
                    continue;
                }
            }

            let x = (last.x + step).min(WORLD_WIDTH);
            let mut dy = rng.range(-14.0, 14.0);
            if dy.abs() < 1.0 {
                dy = dy.signum() * 2.0; // keep non-pad segments clearly sloped
            }
            // Bounce off the floor/ceiling so the walk stays on screen.
            let mut y = last.y + dy;
            if !(4.0..=90.0).contains(&y) {
                y = last.y - dy;
            }
            points.push(Vec2::new(x, y));
        }
        points
    }
}
