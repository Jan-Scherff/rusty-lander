//! Lesson 3 — Taking Control.  Run: cargo test --test lesson_03
use rust_lander::collision::CrashReason;
use rust_lander::lander::*;
use rust_lander::vec2::Vec2;

const DT: f32 = 0.1;

fn lander() -> Lander {
    Lander::new(Vec2::new(50.0, 100.0))
}

#[test]
fn rotate_direction() {
    assert_eq!(Rotate::Left.direction(), -1.0);
    assert_eq!(Rotate::Right.direction(), 1.0);
    assert_eq!(Rotate::None.direction(), 0.0);
}

#[test]
fn is_flying() {
    let mut l = lander();
    assert!(l.is_flying());
    l.state = LanderState::Landed { score: 10 };
    assert!(!l.is_flying());
    l.state = LanderState::Crashed {
        reason: CrashReason::MissedPad,
    };
    assert!(!l.is_flying());
}

#[test]
fn update_without_input_just_falls() {
    let mut l = lander();
    l.update(&Controls::default(), DT);
    assert!(l.velocity.y < 0.0, "gravity applied");
    assert!(l.position.y < 100.0, "position integrated");
    assert_eq!(l.angle, 0.0);
    assert!(!l.thrusting);
    assert_eq!(l.fuel, START_FUEL);
}

#[test]
fn update_with_thrust() {
    let mut l = lander();
    let controls = Controls {
        thrust: true,
        rotate: Rotate::None,
    };
    l.update(&controls, DT);
    assert!(l.thrusting);
    assert!(l.fuel < START_FUEL);
    // THRUST_ACCEL (4.0) beats GRAVITY (1.62), so we go up.
    assert!(
        l.velocity.y > 0.0,
        "thrust should overcome gravity, vy = {}",
        l.velocity.y
    );
}

#[test]
fn update_turns_off_engine_when_key_released() {
    let mut l = lander();
    l.update(
        &Controls {
            thrust: true,
            rotate: Rotate::None,
        },
        DT,
    );
    assert!(l.thrusting);
    l.update(&Controls::default(), DT);
    assert!(!l.thrusting, "thrusting must be reset each update");
}

#[test]
fn update_rotates() {
    let mut l = lander();
    l.update(
        &Controls {
            thrust: false,
            rotate: Rotate::Right,
        },
        DT,
    );
    assert!(l.angle > 0.0);
    let mut l = lander();
    l.update(
        &Controls {
            thrust: false,
            rotate: Rotate::Left,
        },
        DT,
    );
    assert!(l.angle < 0.0);
}

#[test]
fn landed_or_crashed_landers_dont_move() {
    for state in [
        LanderState::Landed { score: 100 },
        LanderState::Crashed {
            reason: CrashReason::LostInSpace,
        },
    ] {
        let mut l = lander();
        l.state = state;
        l.thrusting = true;
        let before = l.clone();
        l.update(
            &Controls {
                thrust: true,
                rotate: Rotate::Left,
            },
            DT,
        );
        assert_eq!(l.position, before.position, "{state:?}: must not move");
        assert_eq!(
            l.velocity, before.velocity,
            "{state:?}: must not accelerate"
        );
        assert_eq!(l.fuel, before.fuel, "{state:?}: must not burn fuel");
        assert_eq!(l.angle, before.angle, "{state:?}: must not rotate");
        assert!(!l.thrusting, "{state:?}: engine must be off");
    }
}

#[test]
fn status_messages() {
    let mut l = lander();
    assert_eq!(l.status_message(), None);
    l.state = LanderState::Landed { score: 250 };
    assert_eq!(
        l.status_message().as_deref(),
        Some("The Eagle has landed! +250 points")
    );
    l.state = LanderState::Crashed {
        reason: CrashReason::MissedPad,
    };
    assert_eq!(
        l.status_message().as_deref(),
        Some("Houston, we have a problem.")
    );
}
