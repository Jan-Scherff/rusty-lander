//! Lesson 6 — Fire & Debris.  Run: cargo test --test lesson_06
use rust_lander::lander::Lander;
use rust_lander::particles::*;
use rust_lander::vec2::Vec2;

fn spark(at: Vec2, life: f32) -> Particle {
    Particle::new(at, Vec2::ZERO, life, ParticleKind::Debris)
}

#[test]
fn emit_calls_the_closure_count_times() {
    let mut system = ParticleSystem::new(1);
    let mut calls = 0;
    system.emit(7, |_rng| {
        calls += 1; // FnMut: the closure may change what it captured
        spark(Vec2::new(calls as f32, 0.0), 1.0)
    });
    assert_eq!(calls, 7);
    assert_eq!(system.particles.len(), 7);
    assert_eq!(
        system.particles[0].position.x, 1.0,
        "keep them in creation order"
    );
    assert_eq!(system.particles[6].position.x, 7.0);
}

#[test]
fn emit_adds_to_existing_particles() {
    let mut system = ParticleSystem::new(1);
    system.emit(2, |_| spark(Vec2::ZERO, 1.0));
    system.emit(3, |_| spark(Vec2::ZERO, 1.0));
    assert_eq!(system.particles.len(), 5);
}

#[test]
fn emit_passes_the_random_generator() {
    let mut system = ParticleSystem::new(1);
    system.emit(20, |rng| spark(Vec2::new(rng.range(0.0, 1.0), 0.0), 1.0));
    let first = system.particles[0].position.x;
    assert!(
        system.particles.iter().any(|p| p.position.x != first),
        "the rng should give different numbers"
    );
}

#[test]
fn exhaust_uses_emit() {
    let mut system = ParticleSystem::new(1);
    system.exhaust(&Lander::new(Vec2::new(10.0, 10.0)));
    assert_eq!(system.count(ParticleKind::Exhaust), 2);
}

#[test]
fn explode() {
    let mut system = ParticleSystem::new(3);
    let at = Vec2::new(40.0, 12.0);
    system.explode(at, 100);
    assert_eq!(system.particles.len(), 100);
    for p in &system.particles {
        assert_eq!(p.position, at);
        assert_eq!(p.kind, ParticleKind::Debris);
        let speed = p.velocity.length();
        assert!((3.0..=18.0).contains(&speed), "speed {speed} out of 3..18");
        assert!(
            (1.5..=3.5).contains(&p.life),
            "life {} out of 1.5..3.5",
            p.life
        );
        assert_eq!(p.life, p.max_life, "use Particle::new so max_life is set");
    }
    let left = system
        .particles
        .iter()
        .filter(|p| p.velocity.x < 0.0)
        .count();
    let down = system
        .particles
        .iter()
        .filter(|p| p.velocity.y < 0.0)
        .count();
    assert!(
        (20..80).contains(&left),
        "debris should fly in all directions ({left}/100 went left)"
    );
    assert!(
        (20..80).contains(&down),
        "debris should fly in all directions ({down}/100 went down)"
    );
}

#[test]
fn update_moves_and_ages() {
    let mut system = ParticleSystem::new(1);
    system.emit(1, |_| {
        Particle::new(
            Vec2::new(0.0, 10.0),
            Vec2::new(2.0, 0.0),
            1.0,
            ParticleKind::Exhaust,
        )
    });
    system.update(0.5, 2.0);
    let p = system.particles[0];
    assert_eq!(
        p.velocity,
        Vec2::new(2.0, -1.0),
        "gravity pulls the velocity down"
    );
    assert_eq!(
        p.position,
        Vec2::new(1.0, 9.5),
        "position moves by the *new* velocity"
    );
    assert_eq!(p.life, 0.5);
}

#[test]
fn update_removes_dead_particles() {
    let mut system = ParticleSystem::new(1);
    let lives = [0.1, 1.0, 0.2, 2.0, 0.05];
    let mut i = 0;
    system.emit(lives.len(), |_| {
        i += 1;
        spark(Vec2::ZERO, lives[i - 1])
    });
    system.update(0.2, 0.0);
    let remaining: Vec<f32> = system.particles.iter().map(|p| p.max_life).collect();
    assert_eq!(
        remaining,
        vec![1.0, 2.0],
        "particles with life <= 0 are removed, order kept"
    );
}

#[test]
fn count_by_kind() {
    let mut system = ParticleSystem::new(1);
    system.explode(Vec2::ZERO, 5);
    system.exhaust(&Lander::new(Vec2::new(0.0, 10.0)));
    assert_eq!(system.count(ParticleKind::Debris), 5);
    assert_eq!(system.count(ParticleKind::Exhaust), 2);
}
