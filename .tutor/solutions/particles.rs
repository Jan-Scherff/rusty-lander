//! # Lesson 6 — Fire & Debris
//!
//! Particles are tiny, short-lived dots: engine exhaust and crash debris.
//! This lesson is all about **closures** — anonymous functions that can
//! capture variables from their surroundings — and passing them to
//! **generic** functions.

use crate::lander::Lander;
use crate::rng::Rng;
use crate::vec2::Vec2;
use std::f32::consts::{PI, TAU};

/// What a particle represents (the game picks colours based on this).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleKind {
    Exhaust,
    Debris,
}

/// A single particle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    /// Seconds left to live. The particle disappears when this reaches 0.
    pub life: f32,
    /// The particle's total lifetime (used to fade it out).
    pub max_life: f32,
    pub kind: ParticleKind,
}

impl Particle {
    /// A new particle with `life` seconds to live.
    pub fn new(position: Vec2, velocity: Vec2, life: f32, kind: ParticleKind) -> Particle {
        Particle { position, velocity, life, max_life: life, kind }
    }

    /// `1.0` when freshly spawned, fading to `0.0` as it dies.
    pub fn brightness(&self) -> f32 {
        (self.life / self.max_life).clamp(0.0, 1.0)
    }
}

/// Owns all live particles plus the random generator used to spawn them.
#[derive(Debug, Clone)]
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    rng: Rng,
}

impl ParticleSystem {
    pub fn new(seed: u64) -> ParticleSystem {
        ParticleSystem { particles: Vec::new(), rng: Rng::new(seed) }
    }

    /// Spawns `count` particles. **You** don't decide what they look like —
    /// the caller does, by passing a closure `make` that builds one particle.
    /// Call `make(&mut self.rng)` `count` times and keep every particle.
    ///
    /// `F: FnMut(&mut Rng) -> Particle` is a *trait bound*: `F` can be any
    /// closure (or function) that takes a `&mut Rng` and returns a
    /// `Particle`. `FnMut` means the closure may change things it captured.
    pub fn emit<F>(&mut self, count: usize, mut make: F)
    where
        F: FnMut(&mut Rng) -> Particle,
    {
        // ✏️ LESSON 6: a `for _ in 0..count` loop works great
        for _ in 0..count {
            let particle = make(&mut self.rng);
            self.particles.push(particle);
        }
    }

    /// Engine exhaust: **worked example** of calling `emit` with a closure.
    /// The closure *captures* `nozzle` and `lander` from this function, and
    /// receives the `rng` as its argument each time `emit` calls it.
    pub fn exhaust(&mut self, lander: &Lander) {
        let nozzle = lander.to_world(Vec2::new(0.0, -1.9));
        self.emit(2, |rng| {
            // Exhaust shoots out of the bottom: the nose angle + 180°,
            // spread by up to ±0.25 radians.
            let direction = Vec2::from_angle(lander.angle + PI + rng.range(-0.25, 0.25));
            let velocity = lander.velocity + direction * rng.range(8.0, 14.0);
            Particle::new(nozzle, velocity, rng.range(0.3, 0.7), ParticleKind::Exhaust)
        });
    }

    /// A crash! Spawn `count` `Debris` particles at `at`, flying off in
    /// random directions (`rng.range(0.0, TAU)` as the angle) at random
    /// speeds between 3 and 18 m/s, each living 1.5 to 3.5 seconds.
    pub fn explode(&mut self, at: Vec2, count: usize) {
        // ✏️ LESSON 6: call self.emit with a closure that captures `at`
        self.emit(count, |rng| {
            let direction = Vec2::from_angle(rng.range(0.0, TAU));
            let speed = rng.range(3.0, 18.0);
            Particle::new(at, direction * speed, rng.range(1.5, 3.5), ParticleKind::Debris)
        });
    }

    /// Moves every particle forward by `dt` seconds:
    /// 1. pull its velocity down by `gravity * dt`,
    /// 2. move its position by `velocity * dt`,
    /// 3. reduce its `life` by `dt`,
    ///
    /// then remove every particle whose life is `<= 0.0`
    /// (look up `Vec::retain`).
    pub fn update(&mut self, dt: f32, gravity: f32) {
        // ✏️ LESSON 6: `for p in &mut self.particles` or `.iter_mut()`
        for p in &mut self.particles {
            p.velocity.y -= gravity * dt;
            p.position += p.velocity * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
    }

    /// How many live particles are of the given kind?
    pub fn count(&self, kind: ParticleKind) -> usize {
        // ✏️ LESSON 6: iter, filter, count
        self.particles.iter().filter(|p| p.kind == kind).count()
    }
}
