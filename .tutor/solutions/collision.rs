//! # Lesson 5 — Touchdown or Crash
//!
//! When the lander touches the ground we must decide: a gentle, upright
//! landing on a pad, or a crash? This is a perfect job for `Result`:
//! `Ok(multiplier)` for a landing, `Err(reason)` for a crash.

use crate::lander::{Lander, LanderState};
use crate::terrain::Terrain;
use crate::vec2::Vec2;
use std::fmt;

/// Fastest safe downward speed at touchdown, in m/s.
pub const MAX_VERTICAL_SPEED: f32 = 3.0;
/// Fastest safe sideways speed at touchdown, in m/s.
pub const MAX_HORIZONTAL_SPEED: f32 = 1.5;
/// Largest safe tilt at touchdown, in radians (≈ 10°).
pub const MAX_LANDING_ANGLE: f32 = 0.18;
/// Fly higher than this and you drift off into space.
pub const CEILING: f32 = 250.0;
/// Points for any successful landing, before bonuses.
pub const BASE_SCORE: u32 = 50;

/// Why did we crash? Some variants carry the measurement that was too big.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrashReason {
    /// Touched the ground somewhere that isn't a landing pad (or with a
    /// foot hanging over the edge of one).
    MissedPad,
    /// Came down too fast.
    TooFast { speed: f32 },
    /// Slid sideways too fast.
    Drifting { speed: f32 },
    /// Wasn't upright enough.
    Tilted { degrees: f32 },
    /// Left the playing field.
    LostInSpace,
}

/// `Display` is the trait behind `{}` in `format!` / `println!`.
/// (`Debug` — what `{:?}` uses — is for programmers; `Display` is for users.)
///
/// Write these messages (the tests check them exactly):
/// - `MissedPad`             → `You missed the landing pad`
/// - `TooFast { speed }`     → `Too fast: 4.2 m/s descent (max 3.0)`
/// - `Drifting { speed }`    → `Drifting sideways at 2.0 m/s (max 1.5)`
/// - `Tilted { degrees }`    → `Tilted 15 degrees (max 10)`
/// - `LostInSpace`           → `Lost in space`
///
/// Speeds use one decimal place (`{:.1}`), degrees none (`{:.0}`).
/// The "max" numbers come from the constants above (convert the angle
/// with `.to_degrees()`).
impl fmt::Display for CrashReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ✏️ LESSON 5: match on self and use write!(f, "...")
        match self {
            CrashReason::MissedPad => write!(f, "You missed the landing pad"),
            CrashReason::TooFast { speed } => write!(
                f,
                "Too fast: {speed:.1} m/s descent (max {MAX_VERTICAL_SPEED:.1})"
            ),
            CrashReason::Drifting { speed } => write!(
                f,
                "Drifting sideways at {speed:.1} m/s (max {MAX_HORIZONTAL_SPEED:.1})"
            ),
            CrashReason::Tilted { degrees } => write!(
                f,
                "Tilted {degrees:.0} degrees (max {:.0})",
                MAX_LANDING_ANGLE.to_degrees()
            ),
            CrashReason::LostInSpace => write!(f, "Lost in space"),
        }
    }
}

/// Does any of the lander's `contact_points()` touch the ground?
///
/// A point touches the ground when there *is* ground below it
/// (`height_at` returns `Some(h)`) and the point is at or below that height.
/// Points outside the terrain never touch anything.
pub fn is_touching_ground(lander: &Lander, terrain: &Terrain) -> bool {
    // ✏️ LESSON 5: a for loop with a match guard: `Some(h) if ...`
    for point in lander.contact_points() {
        match terrain.height_at(point.x) {
            Some(ground) if point.y <= ground => return true,
            _ => {}
        }
    }
    false
}

/// Judges a touchdown. Returns the pad's multiplier if the landing is good,
/// otherwise the *first* problem found, checking in this order:
///
/// 1. There must be a pad under `lander.position.x`, else `MissedPad`
///    (hint: `Option::ok_or` turns an Option into a Result, then use `?`).
/// 2. Both feet must be over that pad (`x_start <= foot.x <= x_end`),
///    else `MissedPad`.
/// 3. `|angle|` must not exceed `MAX_LANDING_ANGLE`, else `Tilted`
///    (with the absolute tilt in degrees).
/// 4. The downward speed (`-velocity.y`) must not exceed
///    `MAX_VERTICAL_SPEED`, else `TooFast`.
/// 5. `|velocity.x|` must not exceed `MAX_HORIZONTAL_SPEED`, else
///    `Drifting`.
pub fn check_landing(lander: &Lander, terrain: &Terrain) -> Result<u32, CrashReason> {
    // ✏️ LESSON 5: early returns with `?` and `return Err(...)`
    let pad = terrain
        .pad_at(lander.position.x)
        .ok_or(CrashReason::MissedPad)?;

    let on_pad = |foot: Vec2| pad.x_start <= foot.x && foot.x <= pad.x_end;
    if !lander.feet().into_iter().all(on_pad) {
        return Err(CrashReason::MissedPad);
    }
    if lander.angle.abs() > MAX_LANDING_ANGLE {
        return Err(CrashReason::Tilted { degrees: lander.angle.abs().to_degrees() });
    }
    let descent = -lander.velocity.y;
    if descent > MAX_VERTICAL_SPEED {
        return Err(CrashReason::TooFast { speed: descent });
    }
    let drift = lander.velocity.x.abs();
    if drift > MAX_HORIZONTAL_SPEED {
        return Err(CrashReason::Drifting { speed: drift });
    }
    Ok(pad.multiplier)
}

/// Score for a landing: `(BASE_SCORE + fuel / 4) * multiplier`, where the
/// fuel bonus is rounded *down* to a whole number (an `as u32` cast does that).
pub fn landing_score(fuel: f32, multiplier: u32) -> u32 {
    // ✏️ LESSON 5: mind the types — fuel is f32, the rest u32
    (BASE_SCORE + (fuel / 4.0) as u32) * multiplier
}

/// `Err(LostInSpace)` when the lander's x is outside `0..=terrain.width()`
/// or its y is above `CEILING`; `Ok(())` otherwise.
///
/// `()` is the "unit" type — an empty value. `Result<(), E>` means
/// "either it worked (and there's nothing to return) or here's an error".
pub fn check_bounds(lander: &Lander, terrain: &Terrain) -> Result<(), CrashReason> {
    // ✏️ LESSON 5: a range's `.contains(&x)` reads nicely here
    let Vec2 { x, y } = lander.position;
    if !(0.0..=terrain.width()).contains(&x) || y > CEILING {
        return Err(CrashReason::LostInSpace);
    }
    Ok(())
}

/// Called every frame by the game. If the lander is flying:
///
/// 1. If `check_bounds` fails, the state becomes `Crashed` with that reason.
/// 2. Otherwise, if it touches the ground, use `check_landing` to set the
///    state to `Landed { score }` (via `landing_score` with the remaining
///    fuel) or `Crashed { reason }`.
///
/// In every other case, leave the lander alone.
pub fn resolve_contact(lander: &mut Lander, terrain: &Terrain) {
    // ✏️ LESSON 5: `if let Err(reason) = ...` and a `match` on the Result
    if !lander.is_flying() {
        return;
    }
    if let Err(reason) = check_bounds(lander, terrain) {
        lander.state = LanderState::Crashed { reason };
        return;
    }
    if !is_touching_ground(lander, terrain) {
        return;
    }
    lander.state = match check_landing(lander, terrain) {
        Ok(multiplier) => LanderState::Landed { score: landing_score(lander.fuel, multiplier) },
        Err(reason) => LanderState::Crashed { reason },
    };
}
