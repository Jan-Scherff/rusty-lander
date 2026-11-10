//! Lesson 5 — Touchdown or Crash.  Run: cargo test --test lesson_05
use rust_lander::collision::*;
use rust_lander::lander::{Lander, LanderState};
use rust_lander::terrain::Terrain;
use rust_lander::vec2::Vec2;

/// Slope, a 10 m pad at height 10 from x=20 to x=30 (multiplier 3), slope.
fn terrain() -> Terrain {
    Terrain::from_points(vec![
        Vec2::new(0.0, 30.0),
        Vec2::new(20.0, 10.0),
        Vec2::new(30.0, 10.0),
        Vec2::new(50.0, 40.0),
    ])
}

/// A lander whose feet (2 m below the centre) are `gap` metres above y=10.
fn lander_at(x: f32, gap: f32) -> Lander {
    let mut l = Lander::new(Vec2::new(x, 12.0 + gap));
    l.velocity = Vec2::new(0.0, -1.0);
    l
}

#[test]
fn display_messages() {
    assert_eq!(
        CrashReason::MissedPad.to_string(),
        "You missed the landing pad"
    );
    assert_eq!(
        CrashReason::TooFast { speed: 4.25 }.to_string(),
        "Too fast: 4.2 m/s descent (max 3.0)"
    );
    assert_eq!(
        CrashReason::Drifting { speed: 2.0 }.to_string(),
        "Drifting sideways at 2.0 m/s (max 1.5)"
    );
    assert_eq!(
        CrashReason::Tilted { degrees: 15.4 }.to_string(),
        "Tilted 15 degrees (max 10)"
    );
    assert_eq!(CrashReason::LostInSpace.to_string(), "Lost in space");
    // Display also powers format!:
    assert_eq!(
        format!("CRASH: {}", CrashReason::LostInSpace),
        "CRASH: Lost in space"
    );
}

#[test]
fn touching_ground() {
    let t = terrain();
    assert!(
        !is_touching_ground(&lander_at(25.0, 0.5), &t),
        "hovering 0.5 m above the pad"
    );
    assert!(
        is_touching_ground(&lander_at(25.0, 0.0), &t),
        "feet exactly on the pad"
    );
    assert!(
        is_touching_ground(&lander_at(25.0, -0.2), &t),
        "feet slightly in the pad"
    );
    // Over the left slope (ground at x=10 is y=20): the body hits it.
    assert!(is_touching_ground(&Lander::new(Vec2::new(10.0, 21.0)), &t));
}

#[test]
fn points_outside_the_terrain_touch_nothing() {
    let t = terrain();
    assert!(!is_touching_ground(
        &Lander::new(Vec2::new(80.0, -100.0)),
        &t
    ));
}

#[test]
fn perfect_landing() {
    assert_eq!(check_landing(&lander_at(25.0, 0.0), &terrain()), Ok(3));
}

#[test]
fn missed_pad() {
    let t = terrain();
    assert_eq!(
        check_landing(&lander_at(10.0, 0.0), &t),
        Err(CrashReason::MissedPad),
        "nowhere near"
    );
    assert_eq!(
        check_landing(&lander_at(29.0, 0.0), &t),
        Err(CrashReason::MissedPad),
        "centre is over the pad, but the right foot (x=31) hangs over the edge"
    );
}

#[test]
fn too_tilted() {
    let mut l = lander_at(25.0, 0.0);
    l.angle = -0.3;
    match check_landing(&l, &terrain()) {
        Err(CrashReason::Tilted { degrees }) => {
            assert!(
                (degrees - 17.19).abs() < 0.1,
                "absolute tilt in degrees, got {degrees}"
            )
        }
        other => panic!("expected Tilted, got {other:?}"),
    }
}

#[test]
fn too_fast() {
    let mut l = lander_at(25.0, 0.0);
    l.velocity = Vec2::new(0.0, -5.0);
    assert_eq!(
        check_landing(&l, &terrain()),
        Err(CrashReason::TooFast { speed: 5.0 })
    );
    l.velocity = Vec2::new(0.0, -MAX_VERTICAL_SPEED);
    assert_eq!(
        check_landing(&l, &terrain()),
        Ok(3),
        "exactly the limit is fine"
    );
}

#[test]
fn drifting() {
    let mut l = lander_at(25.0, 0.0);
    l.velocity = Vec2::new(-2.5, -1.0);
    assert_eq!(
        check_landing(&l, &terrain()),
        Err(CrashReason::Drifting { speed: 2.5 })
    );
}

#[test]
fn checks_happen_in_order() {
    let mut l = lander_at(25.0, 0.0);
    l.angle = 0.5;
    l.velocity = Vec2::new(5.0, -9.0);
    assert!(
        matches!(
            check_landing(&l, &terrain()),
            Err(CrashReason::Tilted { .. })
        ),
        "tilt before speed"
    );
    l.angle = 0.0;
    assert!(
        matches!(
            check_landing(&l, &terrain()),
            Err(CrashReason::TooFast { .. })
        ),
        "descent before drift"
    );
}

#[test]
fn scoring() {
    assert_eq!(landing_score(0.0, 1), 50);
    assert_eq!(landing_score(400.0, 1), 150);
    assert_eq!(
        landing_score(399.0, 2),
        2 * (50 + 99),
        "fuel bonus rounds down"
    );
    assert_eq!(landing_score(100.0, 5), 375);
}

#[test]
fn bounds() {
    let t = terrain();
    assert_eq!(
        check_bounds(&Lander::new(Vec2::new(25.0, 100.0)), &t),
        Ok(())
    );
    assert_eq!(
        check_bounds(&Lander::new(Vec2::new(-1.0, 100.0)), &t),
        Err(CrashReason::LostInSpace)
    );
    assert_eq!(
        check_bounds(&Lander::new(Vec2::new(51.0, 100.0)), &t),
        Err(CrashReason::LostInSpace)
    );
    assert_eq!(
        check_bounds(&Lander::new(Vec2::new(25.0, CEILING + 1.0)), &t),
        Err(CrashReason::LostInSpace)
    );
}

#[test]
fn resolve_contact_lands() {
    let mut l = lander_at(25.0, -0.05);
    l.fuel = 100.0;
    resolve_contact(&mut l, &terrain());
    assert_eq!(
        l.state,
        LanderState::Landed {
            score: landing_score(100.0, 3)
        }
    );
}

#[test]
fn resolve_contact_crashes() {
    let mut l = lander_at(25.0, -0.05);
    l.velocity = Vec2::new(0.0, -8.0);
    resolve_contact(&mut l, &terrain());
    assert_eq!(
        l.state,
        LanderState::Crashed {
            reason: CrashReason::TooFast { speed: 8.0 }
        }
    );

    let mut l = Lander::new(Vec2::new(-5.0, 100.0));
    resolve_contact(&mut l, &terrain());
    assert_eq!(
        l.state,
        LanderState::Crashed {
            reason: CrashReason::LostInSpace
        }
    );
}

#[test]
fn resolve_contact_leaves_flying_landers_alone() {
    let mut l = lander_at(25.0, 5.0);
    let before = l.clone();
    resolve_contact(&mut l, &terrain());
    assert_eq!(l, before);
}

#[test]
fn resolve_contact_ignores_landers_that_already_landed() {
    let mut l = lander_at(25.0, -0.05);
    l.state = LanderState::Landed { score: 7 };
    l.velocity = Vec2::new(0.0, -50.0);
    resolve_contact(&mut l, &terrain());
    assert_eq!(l.state, LanderState::Landed { score: 7 });
}
