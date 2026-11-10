//! Lesson 2 — Newton on the Moon.  Run: cargo test --test lesson_02
use rust_lander::lander::*;
use rust_lander::vec2::Vec2;
use std::f32::consts::FRAC_PI_2;

fn approx(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-4
}

#[test]
fn new_lander_is_ready_for_launch() {
    let l = Lander::new(Vec2::new(10.0, 100.0));
    assert_eq!(l.position, Vec2::new(10.0, 100.0));
    assert_eq!(l.velocity, Vec2::ZERO);
    assert_eq!(l.angle, 0.0);
    assert_eq!(l.fuel, START_FUEL);
    assert!(!l.thrusting);
    assert_eq!(l.state, LanderState::Flying);
}

#[test]
fn gravity_pulls_down() {
    let mut l = Lander::new(Vec2::new(0.0, 50.0));
    l.apply_gravity(1.0);
    assert!(
        approx(l.velocity.y, -GRAVITY),
        "after 1 s, falling at GRAVITY m/s; got {}",
        l.velocity.y
    );
    l.apply_gravity(0.5);
    assert!(
        approx(l.velocity.y, -1.5 * GRAVITY),
        "gravity adds up over time"
    );
    assert_eq!(l.velocity.x, 0.0, "gravity doesn't push sideways");
    assert_eq!(
        l.position,
        Vec2::new(0.0, 50.0),
        "gravity changes velocity, not position"
    );
}

#[test]
fn integrate_moves_by_velocity() {
    let mut l = Lander::new(Vec2::new(0.0, 50.0));
    l.velocity = Vec2::new(2.0, -4.0);
    l.integrate(0.5);
    assert_eq!(l.position, Vec2::new(1.0, 48.0));
}

#[test]
fn engine_pushes_along_the_nose() {
    let mut l = Lander::new(Vec2::new(0.0, 50.0));
    l.fire_engine(1.0);
    assert!(
        approx(l.velocity.y, THRUST_ACCEL),
        "upright: thrust goes straight up"
    );
    assert!(approx(l.velocity.x, 0.0));
    assert!(l.thrusting);

    let mut tilted = Lander::new(Vec2::new(0.0, 50.0));
    tilted.angle = FRAC_PI_2;
    tilted.fire_engine(0.5);
    assert!(
        approx(tilted.velocity.x, THRUST_ACCEL * 0.5),
        "tilted right: thrust pushes right"
    );
    assert!(approx(tilted.velocity.y, 0.0));
}

#[test]
fn engine_burns_fuel() {
    let mut l = Lander::new(Vec2::new(0.0, 50.0));
    l.fire_engine(2.0);
    assert!(approx(l.fuel, START_FUEL - 2.0 * FUEL_BURN_RATE));
}

#[test]
fn fuel_never_goes_negative() {
    let mut l = Lander::new(Vec2::new(0.0, 50.0));
    l.fuel = 1.0;
    l.fire_engine(1.0);
    assert_eq!(l.fuel, 0.0, "fuel is clamped at 0");
}

#[test]
fn empty_tank_does_nothing() {
    let mut l = Lander::new(Vec2::new(0.0, 50.0));
    l.fuel = 0.0;
    l.thrusting = true;
    l.fire_engine(1.0);
    assert_eq!(l.velocity, Vec2::ZERO, "no fuel, no push");
    assert!(!l.thrusting, "the engine is off without fuel");
    assert_eq!(l.fuel, 0.0);
}

#[test]
fn rotate_and_clamp() {
    let mut l = Lander::new(Vec2::new(0.0, 50.0));
    l.rotate(1.0, 0.1);
    assert!(
        approx(l.angle, ROTATION_SPEED * 0.1),
        "right turns clockwise (positive)"
    );
    l.rotate(-1.0, 0.3);
    assert!(approx(l.angle, -ROTATION_SPEED * 0.2), "left turns back");
    l.rotate(1.0, 100.0);
    assert_eq!(l.angle, MAX_TILT, "can't tilt past MAX_TILT");
    l.rotate(-1.0, 100.0);
    assert_eq!(l.angle, -MAX_TILT, "or past -MAX_TILT");
}

#[test]
fn speed_is_length_of_velocity() {
    let mut l = Lander::new(Vec2::new(0.0, 50.0));
    l.velocity = Vec2::new(-3.0, 4.0);
    assert_eq!(l.speed(), 5.0);
}

#[test]
fn a_falling_lander_hits_the_ground_eventually() {
    // Simulate 10 seconds of free fall at 60 frames per second.
    let mut l = Lander::new(Vec2::new(0.0, 100.0));
    for _ in 0..600 {
        l.apply_gravity(1.0 / 60.0);
        l.integrate(1.0 / 60.0);
    }
    // Physics says: y = 100 - ½·g·t² ≈ 19 m
    assert!(
        (l.position.y - 19.0).abs() < 1.0,
        "expected about 19 m, got {}",
        l.position.y
    );
}
