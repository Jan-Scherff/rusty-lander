//! Lesson 1 — Vectors in Space.  Run: cargo test --test lesson_01
use rust_lander::vec2::Vec2;
use std::f32::consts::{FRAC_PI_2, PI};

fn close(a: Vec2, b: Vec2) -> bool {
    (a.x - b.x).abs() < 1e-4 && (a.y - b.y).abs() < 1e-4
}

#[test]
fn new_sets_both_fields() {
    let v = Vec2::new(3.0, -4.5);
    assert_eq!(v.x, 3.0);
    assert_eq!(v.y, -4.5);
}

#[test]
fn length_of_a_3_4_5_triangle() {
    assert_eq!(Vec2::new(3.0, 4.0).length(), 5.0);
    assert_eq!(
        Vec2::new(-3.0, -4.0).length(),
        5.0,
        "length is never negative"
    );
    assert_eq!(Vec2::ZERO.length(), 0.0);
}

#[test]
fn add_and_sub() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(10.0, 20.0);
    assert_eq!(a + b, Vec2::new(11.0, 22.0));
    assert_eq!(b - a, Vec2::new(9.0, 18.0));
    // Vec2 is Copy, so `a` and `b` are still usable after being "used" above.
    assert_eq!(a - b, Vec2::new(-9.0, -18.0));
}

#[test]
fn multiply_by_a_number() {
    assert_eq!(Vec2::new(1.5, -2.0) * 2.0, Vec2::new(3.0, -4.0));
    assert_eq!(Vec2::new(1.5, -2.0) * 0.0, Vec2::ZERO);
}

#[test]
fn add_assign_changes_in_place() {
    let mut position = Vec2::new(1.0, 1.0);
    position += Vec2::new(0.5, -2.0);
    assert_eq!(position, Vec2::new(1.5, -1.0));
}

#[test]
fn from_angle_points_the_right_way() {
    assert!(
        close(Vec2::from_angle(0.0), Vec2::new(0.0, 1.0)),
        "angle 0 points up"
    );
    assert!(
        close(Vec2::from_angle(FRAC_PI_2), Vec2::new(1.0, 0.0)),
        "+90° points right"
    );
    assert!(
        close(Vec2::from_angle(-FRAC_PI_2), Vec2::new(-1.0, 0.0)),
        "-90° points left"
    );
    assert!(
        close(Vec2::from_angle(PI), Vec2::new(0.0, -1.0)),
        "180° points down"
    );
    assert!(
        (Vec2::from_angle(1.234).length() - 1.0).abs() < 1e-5,
        "always length 1"
    );
}

#[test]
fn rotated_turns_clockwise() {
    let up = Vec2::new(0.0, 1.0);
    assert!(
        close(up.rotated(FRAC_PI_2), Vec2::new(1.0, 0.0)),
        "up turned 90° clockwise is right"
    );
    let right = Vec2::new(2.0, 0.0);
    assert!(
        close(right.rotated(FRAC_PI_2), Vec2::new(0.0, -2.0)),
        "right turned 90° clockwise is down"
    );
    // Rotating keeps the length.
    let v = Vec2::new(3.0, 4.0).rotated(0.7);
    assert!((v.length() - 5.0).abs() < 1e-4);
    // Rotating `up` matches from_angle.
    assert!(close(up.rotated(0.4), Vec2::from_angle(0.4)));
}

#[test]
fn operators_work_together() {
    // lerp (provided) uses +, - and * — a good integration check.
    let a = Vec2::new(0.0, 0.0);
    let b = Vec2::new(10.0, -10.0);
    assert_eq!(a.lerp(b, 0.5), Vec2::new(5.0, -5.0));
    assert_eq!(-b, Vec2::new(-10.0, 10.0));
}
