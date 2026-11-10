//! Lesson 4 — Lunar Terrain.  Run: cargo test --test lesson_04
use rust_lander::terrain::*;
use rust_lander::vec2::Vec2;

/// A hand-made surface:
///
/// ```text
///  y
/// 20 |\                 ______(14 m pad)
/// 10 | \____(6 m pad)  /
///  0 |      \_________/  (a 30 m *dip* that is not flat)
///    0  10  16       46  56     70
/// ```
fn sample() -> Terrain {
    Terrain::from_points(vec![
        Vec2::new(0.0, 20.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(16.0, 10.0), // 6 m flat: pad (x5)
        Vec2::new(31.0, 0.0),
        Vec2::new(46.0, 1.0),
        Vec2::new(56.0, 20.0),
        Vec2::new(70.0, 20.0), // 14 m flat: pad (x2)
    ])
}

#[test]
fn pad_multipliers() {
    assert_eq!(Terrain::pad_multiplier(5.0), 5);
    assert_eq!(Terrain::pad_multiplier(7.99), 5);
    assert_eq!(Terrain::pad_multiplier(8.0), 3);
    assert_eq!(Terrain::pad_multiplier(11.5), 3);
    assert_eq!(Terrain::pad_multiplier(12.0), 2);
    assert_eq!(Terrain::pad_multiplier(40.0), 2);
}

#[test]
fn from_points_keeps_the_points() {
    let t = sample();
    assert_eq!(t.points.len(), 7);
    assert_eq!(t.points[3], Vec2::new(31.0, 0.0));
}

#[test]
fn from_points_finds_the_pads() {
    let t = sample();
    assert_eq!(
        t.pads,
        vec![
            LandingPad {
                x_start: 10.0,
                x_end: 16.0,
                y: 10.0,
                multiplier: 5
            },
            LandingPad {
                x_start: 56.0,
                x_end: 70.0,
                y: 20.0,
                multiplier: 2
            },
        ]
    );
}

#[test]
fn narrow_flat_bits_are_not_pads() {
    let t = Terrain::from_points(vec![
        Vec2::new(0.0, 5.0),
        Vec2::new(3.0, 5.0), // only 3 m: too narrow
        Vec2::new(10.0, 8.0),
    ]);
    assert!(t.pads.is_empty());
}

#[test]
fn width() {
    assert_eq!(sample().width(), 70.0);
    assert_eq!(
        Terrain::from_points(vec![]).width(),
        0.0,
        "empty terrain has width 0"
    );
}

#[test]
fn height_at_points_and_between() {
    let t = sample();
    assert_eq!(t.height_at(0.0), Some(20.0), "left edge");
    assert_eq!(t.height_at(5.0), Some(15.0), "halfway down the first slope");
    assert_eq!(t.height_at(13.0), Some(10.0), "on the first pad");
    assert_eq!(t.height_at(70.0), Some(20.0), "right edge");
    let h = t.height_at(38.5).expect("38.5 is inside the terrain");
    assert!((h - 0.5).abs() < 1e-4, "interpolation in the dip, got {h}");
}

#[test]
fn height_outside_is_none() {
    let t = sample();
    assert_eq!(t.height_at(-0.1), None);
    assert_eq!(t.height_at(70.1), None);
}

#[test]
fn pad_at() {
    let t = sample();
    assert_eq!(t.pad_at(12.0).map(|p| p.multiplier), Some(5));
    assert_eq!(t.pad_at(60.0).map(|p| p.multiplier), Some(2));
    assert!(t.pad_at(30.0).is_none(), "no pad in the dip");
    assert!(t.pad_at(100.0).is_none());
}

#[test]
fn highest_point() {
    assert_eq!(sample().highest_point(), 20.0);
    let t = Terrain::from_points(vec![Vec2::new(0.0, -5.0), Vec2::new(1.0, -2.0)]);
    assert_eq!(t.highest_point(), -2.0, "works with negative heights too");
}

#[test]
fn generated_terrain_has_three_pads() {
    for seed in 0..50 {
        let t = Terrain::generate(seed);
        assert_eq!(t.pads.len(), 3, "seed {seed}: expected three pads");
        assert_eq!(t.width(), WORLD_WIDTH, "seed {seed}");
        let mut multipliers: Vec<u32> = t.pads.iter().map(|p| p.multiplier).collect();
        multipliers.sort();
        assert_eq!(multipliers, vec![2, 3, 5], "seed {seed}");
        for pad in &t.pads {
            assert_eq!(t.height_at(pad.center_x()), Some(pad.y), "seed {seed}");
        }
    }
}
