//! # Lessons 2 & 3 — The Lander
//!
//! **Lesson 2 (Newton on the Moon):** the physics — gravity pulls the lander
//! down, the engine pushes it along its nose, and velocity moves it.
//!
//! **Lesson 3 (Taking Control):** enums and `match` — the lander can be
//! flying, landed or crashed, and the player can steer left or right.

use crate::collision::CrashReason;
use crate::vec2::Vec2;
use std::f32::consts::FRAC_PI_2;

/// Gravity on the Moon in m/s² (about 1/6 of Earth's 9.81).
pub const GRAVITY: f32 = 1.62;
/// Acceleration from the main engine at full power, in m/s².
pub const THRUST_ACCEL: f32 = 4.0;
/// How fast the lander turns, in radians per second.
pub const ROTATION_SPEED: f32 = 1.8;
/// The lander can tilt at most 90° to either side.
pub const MAX_TILT: f32 = FRAC_PI_2;
/// Fuel units burned per second while the engine fires.
pub const FUEL_BURN_RATE: f32 = 10.0;
/// Fuel in a full tank.
pub const START_FUEL: f32 = 400.0;

// ---------------------------------------------------------------------------
// Lesson 3 types
// ---------------------------------------------------------------------------

/// What is the lander doing right now?
///
/// Enum variants can carry data! `Landed` holds a named field (like a struct)
/// and `Crashed` holds the reason for the crash.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LanderState {
    Flying,
    Landed { score: u32 },
    Crashed { reason: CrashReason },
}

/// Which way the player wants to turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Rotate {
    /// `#[default]` marks the variant `Rotate::default()` returns.
    #[default]
    None,
    /// Counter-clockwise (nose to the left).
    Left,
    /// Clockwise (nose to the right).
    Right,
}

/// What the player is pressing this frame.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Controls {
    pub thrust: bool,
    pub rotate: Rotate,
}

impl Rotate {
    /// The sign of the rotation: `Left` → `-1.0`, `Right` → `1.0`,
    /// `None` → `0.0`. (Remember: positive angles are clockwise.)
    pub fn direction(self) -> f32 {
        // ✏️ LESSON 3: `match self { ... }` with one arm per variant
        match self {
            Rotate::Left => -1.0,
            Rotate::Right => 1.0,
            Rotate::None => 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// The lander itself
// ---------------------------------------------------------------------------

/// Our spacecraft.
///
/// `position` is the centre of the body. The feet are 2 m below it.
#[derive(Debug, Clone, PartialEq)]
pub struct Lander {
    pub position: Vec2,
    pub velocity: Vec2,
    /// Tilt in radians. 0 = upright, positive = tilted clockwise (right).
    pub angle: f32,
    pub fuel: f32,
    /// Was the engine firing during the last update? (Used for drawing the
    /// flame and spawning exhaust particles.)
    pub thrusting: bool,
    pub state: LanderState,
}

impl Lander {
    // ----------------------------- Lesson 2 ---------------------------------

    /// A fresh lander at `position`: not moving, upright, full tank,
    /// engine off, flying.
    pub fn new(position: Vec2) -> Lander {
        // ✏️ LESSON 2: return a `Lander { ... }` with every field set
        Lander {
            position,
            velocity: Vec2::ZERO,
            angle: 0.0,
            fuel: START_FUEL,
            thrusting: false,
            state: LanderState::Flying,
        }
    }

    /// Gravity changes the *velocity*: every second, the downward speed grows
    /// by `GRAVITY` m/s. Over a small time step `dt` (in seconds) that's
    /// `GRAVITY * dt`.
    pub fn apply_gravity(&mut self, dt: f32) {
        // ✏️ LESSON 2: decrease self.velocity.y
        self.velocity.y -= GRAVITY * dt;
    }

    /// Fires the main engine for `dt` seconds.
    ///
    /// - With fuel left: push the velocity in the direction the nose points
    ///   (`Vec2::from_angle(self.angle)`) by `THRUST_ACCEL * dt`, burn
    ///   `FUEL_BURN_RATE * dt` fuel (never going below 0) and set
    ///   `thrusting` to `true`.
    /// - With an empty tank: nothing happens, and `thrusting` is `false`.
    pub fn fire_engine(&mut self, dt: f32) {
        // ✏️ LESSON 2: check the fuel, then accelerate and burn
        if self.fuel <= 0.0 {
            self.thrusting = false;
            return;
        }
        self.velocity += Vec2::from_angle(self.angle) * (THRUST_ACCEL * dt);
        self.fuel = (self.fuel - FUEL_BURN_RATE * dt).max(0.0);
        self.thrusting = true;
    }

    /// Turns the lander. `direction` is `-1.0` (left), `1.0` (right) or `0.0`.
    /// The angle changes by `direction * ROTATION_SPEED * dt` and must stay
    /// within `-MAX_TILT..=MAX_TILT` (look up `f32::clamp`).
    pub fn rotate(&mut self, direction: f32, dt: f32) {
        // ✏️ LESSON 2: change and clamp self.angle
        self.angle = (self.angle + direction * ROTATION_SPEED * dt).clamp(-MAX_TILT, MAX_TILT);
    }

    /// Velocity changes the *position*: move by `velocity * dt`.
    pub fn integrate(&mut self, dt: f32) {
        // ✏️ LESSON 2: one line, using the operators from Lesson 1
        self.position += self.velocity * dt;
    }

    /// The current speed in m/s (the length of the velocity vector).
    pub fn speed(&self) -> f32 {
        // ✏️ LESSON 2: you wrote this in Lesson 1, in a way
        self.velocity.length()
    }

    // ----------------------------- Lesson 3 ---------------------------------

    /// `true` while the state is `LanderState::Flying`.
    pub fn is_flying(&self) -> bool {
        // ✏️ LESSON 3: try the `matches!` macro, or a `match`
        matches!(self.state, LanderState::Flying)
    }

    /// Advances the lander by one frame of `dt` seconds.
    ///
    /// 1. If the lander is not flying, set `thrusting` to `false` and return.
    /// 2. Rotate according to `controls.rotate`.
    /// 3. Set `thrusting` to `false`, then fire the engine if
    ///    `controls.thrust` is pressed.
    /// 4. Apply gravity, then integrate the position.
    pub fn update(&mut self, controls: &Controls, dt: f32) {
        // ✏️ LESSON 3: glue together your Lesson 2 methods
        if !self.is_flying() {
            self.thrusting = false;
            return;
        }
        self.rotate(controls.rotate.direction(), dt);
        self.thrusting = false;
        if controls.thrust {
            self.fire_engine(dt);
        }
        self.apply_gravity(dt);
        self.integrate(dt);
    }

    /// A message for the big text in the middle of the screen:
    /// - `Flying`            → `None` (nothing to say)
    /// - `Landed { score }`  → `Some("The Eagle has landed! +<score> points")`
    /// - `Crashed { .. }`    → `Some("Houston, we have a problem.")`
    pub fn status_message(&self) -> Option<String> {
        // ✏️ LESSON 3: match on self.state, use format! for the score
        match self.state {
            LanderState::Flying => None,
            LanderState::Landed { score } => {
                Some(format!("The Eagle has landed! +{score} points"))
            }
            LanderState::Crashed { .. } => Some("Houston, we have a problem.".to_string()),
        }
    }

    // ----------------------------- Provided ---------------------------------

    /// Points (in world coordinates) that can touch the ground: two feet,
    /// two shoulders and the top of the cabin. Used for collisions.
    pub fn contact_points(&self) -> [Vec2; 5] {
        // `.map` on an array applies a closure to each element (Lesson 4/6).
        [
            Vec2 { x: -2.0, y: -2.0 }, // left foot
            Vec2 { x: 2.0, y: -2.0 },  // right foot
            Vec2 { x: -1.6, y: -0.7 }, // left shoulder
            Vec2 { x: 1.6, y: -0.7 },  // right shoulder
            Vec2 { x: 0.0, y: 1.8 },   // top
        ]
        .map(|local| self.to_world(local))
    }

    /// Just the two feet: `[left, right]`.
    pub fn feet(&self) -> [Vec2; 2] {
        let points = self.contact_points();
        [points[0], points[1]]
    }

    /// Converts a point from the lander's own coordinate system (origin at
    /// its centre, unrotated) into world coordinates.
    pub fn to_world(&self, local: Vec2) -> Vec2 {
        self.position + local.rotated(self.angle)
    }
}
