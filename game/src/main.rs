//! # Rust Lander — the game window (provided)
//!
//! This file draws the game and reads the keyboard. All the *logic* lives in
//! the library modules you write during the course.
//!
//! The game is built to run even while lessons are unfinished: every call into
//! your code is wrapped in a safety net (`Workshop::run`). If it hits a
//! `todo!()`, the game uses a simple fallback where it can and shows an
//! "UNDER CONSTRUCTION" note instead of crashing. As you finish lessons, more
//! of the game comes to life.
//!
//! Run it with `cargo run` from the project folder.

use macroquad::prelude::*;
use rust_lander::collision::{
    self, CrashReason, MAX_HORIZONTAL_SPEED, MAX_LANDING_ANGLE, MAX_VERTICAL_SPEED,
};
use rust_lander::highscore::HighScores;
use rust_lander::lander::{Controls, GRAVITY, Lander, LanderState, Rotate, START_FUEL};
use rust_lander::particles::{ParticleKind, ParticleSystem};
use rust_lander::rng::Rng;
use rust_lander::terrain::{LandingPad, Terrain, WORLD_WIDTH};
// This explicit import wins over macroquad's own `Vec2` from the glob above.
use rust_lander::vec2::Vec2;
use std::collections::BTreeMap;
use std::path::Path;

const HIGHSCORE_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../highscores.txt");
const MAX_ZOOM: f32 = 2.6;
const ZOOM_ALTITUDE: f32 = 28.0;
const REFUEL_PER_LANDING: f32 = 120.0;

const LESSON_NAMES: [&str; 7] = [
    "Vectors",
    "Physics",
    "Controls",
    "Terrain",
    "Collisions",
    "Particles",
    "High scores",
];

const BG: Color = Color::new(0.02, 0.02, 0.06, 1.0);
const GROUND: Color = Color::new(0.10, 0.10, 0.14, 1.0);
const OUTLINE: Color = Color::new(0.80, 0.82, 0.88, 1.0);
const ACCENT: Color = Color::new(1.00, 0.80, 0.25, 1.0);
const OK: Color = Color::new(0.35, 0.95, 0.45, 1.0);
const BAD: Color = Color::new(1.00, 0.35, 0.30, 1.0);
const WARN: Color = Color::new(1.00, 0.60, 0.15, 1.0);

// ---------------------------------------------------------------------------
// The safety net
// ---------------------------------------------------------------------------

mod guard {
    //! Catches panics (like `todo!()`) from learner code so the game can keep
    //! running, and remembers the panic message to show on screen.
    use std::cell::{Cell, RefCell};
    use std::panic::{self, AssertUnwindSafe};

    thread_local! {
        static LAST_PANIC: RefCell<Option<String>> = const { RefCell::new(None) };
        static DEPTH: Cell<u32> = const { Cell::new(0) };
    }

    pub fn install() {
        let default_hook = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if DEPTH.with(Cell::get) == 0 {
                // A panic outside the safety net: behave normally.
                default_hook(info);
                return;
            }
            let payload = info.payload();
            let message = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "unknown error".to_string());
            let location = info
                .location()
                .map(|l| format!("  [{}:{}]", l.file(), l.line()))
                .unwrap_or_default();
            LAST_PANIC.with(|p| *p.borrow_mut() = Some(format!("{message}{location}")));
        }));
    }

    pub fn attempt<T>(f: impl FnOnce() -> T) -> Result<T, String> {
        DEPTH.with(|d| d.set(d.get() + 1));
        let result = panic::catch_unwind(AssertUnwindSafe(f));
        DEPTH.with(|d| d.set(d.get() - 1));
        result.map_err(|_| {
            LAST_PANIC
                .with(|p| p.borrow_mut().take())
                .unwrap_or_else(|| "unknown error".to_string())
        })
    }
}

/// Keeps track of which lessons are still "under construction".
#[derive(Default)]
struct Workshop {
    issues: BTreeMap<u8, String>,
}

impl Workshop {
    /// Runs learner code for `lesson`. Returns `None` (and records the
    /// problem) if it panicked.
    fn run<T>(&mut self, lesson: u8, f: impl FnOnce() -> T) -> Option<T> {
        match guard::attempt(f) {
            Ok(value) => Some(value),
            Err(message) => {
                self.issues.entry(lesson).or_insert(message);
                None
            }
        }
    }

    /// Calls every function of every lesson once at start-up, so the title
    /// screen can show which systems are online.
    fn probe_all(&mut self) {
        self.run(1, || {
            let a = Vec2::new(1.0, 2.0);
            let mut b = a + a - a * 2.0 + -a;
            b += a;
            let _ = (b.length(), Vec2::from_angle(0.5).rotated(0.3));
        });
        self.run(2, || {
            let mut l = Lander::new(Vec2 { x: 50.0, y: 50.0 });
            l.apply_gravity(0.1);
            l.fire_engine(0.1);
            l.rotate(1.0, 0.1);
            l.integrate(0.1);
            let _ = l.speed();
        });
        self.run(3, || {
            let mut l = demo_lander();
            let _ = (Rotate::Left.direction(), l.is_flying(), l.status_message());
            l.update(&Controls::default(), 0.1);
        });
        self.run(4, || {
            let t = Terrain::generate(42);
            let _ = (
                t.height_at(50.0),
                t.pad_at(50.0),
                t.width(),
                t.highest_point(),
            );
            let _ = Terrain::pad_multiplier(7.0);
        });
        self.run(5, || {
            let t = flat_terrain();
            let mut l = demo_lander();
            let _ = (
                collision::is_touching_ground(&l, &t),
                collision::check_landing(&l, &t),
                collision::check_bounds(&l, &t),
                collision::landing_score(100.0, 2),
                CrashReason::MissedPad.to_string(),
            );
            collision::resolve_contact(&mut l, &t);
        });
        self.run(6, || {
            let mut p = ParticleSystem::new(1);
            p.explode(Vec2 { x: 1.0, y: 1.0 }, 10);
            p.update(0.1, GRAVITY);
            let _ = p.count(ParticleKind::Debris);
        });
        self.run(7, || {
            let mut h = HighScores::parse("AAA,10\n").unwrap_or_default();
            let _ = (h.to_text(), h.qualifies(5), h.add("BBB", 20));
            let _ = rust_lander::highscore::HighScoreError::from(std::io::Error::other("probe"));
        });
    }
}

/// A lander built directly from its fields (no lesson code needed).
fn demo_lander() -> Lander {
    Lander {
        position: Vec2 { x: 100.0, y: 110.0 },
        velocity: Vec2 { x: 0.0, y: 0.0 },
        angle: 0.0,
        fuel: START_FUEL,
        thrusting: false,
        state: LanderState::Flying,
    }
}

/// Flat ground with one big pad, built without any lesson code.
fn flat_terrain() -> Terrain {
    Terrain {
        points: vec![
            Vec2 { x: 0.0, y: 10.0 },
            Vec2 {
                x: WORLD_WIDTH,
                y: 10.0,
            },
        ],
        pads: vec![LandingPad {
            x_start: 0.0,
            x_end: WORLD_WIDTH,
            y: 10.0,
            multiplier: 1,
        }],
    }
}

// ---------------------------------------------------------------------------
// Game state
// ---------------------------------------------------------------------------

enum Phase {
    Title,
    Playing,
    Landed { timer: f32 },
    Crashed { timer: f32 },
    EnterInitials { name: String },
    Broken { message: String },
}

struct Game {
    phase: Phase,
    lander: Lander,
    terrain: Terrain,
    particles: ParticleSystem,
    stars: Vec<(f32, f32, f32)>,
    level: u32,
    score: u32,
    seed: u64,
    /// `None` while Lesson 7 is unfinished.
    highscores: Option<HighScores>,
    highscore_note: Option<String>,
    workshop: Workshop,
    zoom: f32,
    view_left: f32,
    view_bottom: f32,
    time: f32,
}

impl Game {
    fn new() -> Game {
        let mut workshop = Workshop::default();
        workshop.probe_all();

        let mut rng = Rng::new(7);
        let stars = (0..160)
            .map(|_| {
                (
                    rng.range(0.0, WORLD_WIDTH),
                    rng.range(0.0, 260.0),
                    rng.range(0.4, 1.0),
                )
            })
            .collect();

        let mut game = Game {
            phase: Phase::Title,
            lander: demo_lander(),
            terrain: flat_terrain(),
            particles: ParticleSystem::new(99),
            stars,
            level: 1,
            score: 0,
            seed: 2024,
            highscores: None,
            highscore_note: None,
            workshop,
            zoom: 1.0,
            view_left: 0.0,
            view_bottom: 0.0,
            time: 0.0,
        };
        game.terrain = game.build_terrain(game.seed);
        game.load_highscores();
        game
    }

    fn load_highscores(&mut self) {
        let path = Path::new(HIGHSCORE_FILE);
        match self.workshop.run(7, || HighScores::load(path)) {
            Some(Ok(table)) => self.highscores = Some(table),
            Some(Err(err)) => {
                self.highscore_note = Some(err.to_string());
                self.highscores = Some(HighScores::default());
            }
            None => self.highscores = None,
        }
    }

    fn build_terrain(&mut self, seed: u64) -> Terrain {
        if let Some(terrain) = self.workshop.run(4, || Terrain::generate(seed)) {
            return terrain;
        }
        // Fallback: the provided point generator, but no pads detected.
        match guard::attempt(|| Terrain::generate_points(seed)) {
            Ok(points) => Terrain {
                points,
                pads: Vec::new(),
            },
            Err(_) => flat_terrain(),
        }
    }

    fn start_level(&mut self, fuel: Option<f32>) {
        self.seed = self.seed.wrapping_mul(31).wrapping_add(17);
        self.terrain = self.build_terrain(self.seed);
        self.particles = ParticleSystem::new(self.seed);
        let from_left = self.seed % 2 == 0;
        let start = Vec2 {
            x: if from_left { 12.0 } else { WORLD_WIDTH - 12.0 },
            y: 130.0,
        };
        match guard::attempt(|| Lander::new(start)) {
            Ok(mut lander) => {
                lander.velocity = Vec2 {
                    x: if from_left { 6.0 } else { -6.0 },
                    y: 0.0,
                };
                if let Some(fuel) = fuel {
                    lander.fuel = fuel;
                }
                self.lander = lander;
                self.phase = Phase::Playing;
            }
            Err(message) => self.phase = Phase::Broken { message },
        }
    }

    /// Height of the ground under the lander (fallback: 0).
    fn ground_below(&mut self) -> f32 {
        let x = self.lander.position.x;
        let terrain = &self.terrain;
        self.workshop
            .run(4, || terrain.height_at(x))
            .flatten()
            .unwrap_or(0.0)
    }

    // ------------------------------- update ---------------------------------

    fn update(&mut self, dt: f32) {
        self.time += dt;
        let continue_pressed = is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter);

        match &mut self.phase {
            Phase::Title => {
                if continue_pressed {
                    self.score = 0;
                    self.level = 1;
                    self.start_level(None);
                }
            }
            Phase::Playing => self.update_flight(dt),
            Phase::Landed { timer } => {
                *timer += dt;
                if *timer > 1.0 && continue_pressed {
                    self.level += 1;
                    let fuel = (self.lander.fuel + REFUEL_PER_LANDING).min(START_FUEL);
                    self.start_level(Some(fuel));
                }
            }
            Phase::Crashed { timer } => {
                *timer += dt;
                if *timer > 1.5 && continue_pressed {
                    let score = self.score;
                    let qualifies = match &self.highscores {
                        Some(table) => self
                            .workshop
                            .run(7, || table.qualifies(score))
                            .unwrap_or(false),
                        None => false,
                    };
                    self.phase = if qualifies {
                        Phase::EnterInitials {
                            name: String::new(),
                        }
                    } else {
                        Phase::Title
                    };
                }
            }
            Phase::EnterInitials { name } => {
                while let Some(c) = get_char_pressed() {
                    if c.is_ascii_alphanumeric() && name.len() < 3 {
                        name.push(c.to_ascii_uppercase());
                    }
                }
                if is_key_pressed(KeyCode::Backspace) {
                    name.pop();
                }
                if is_key_pressed(KeyCode::Enter) && !name.is_empty() {
                    let name = name.clone();
                    self.save_highscore(&name);
                    self.phase = Phase::Title;
                }
            }
            Phase::Broken { .. } => {}
        }

        // Particles keep moving in every phase.
        let particles = &mut self.particles;
        self.workshop.run(6, || particles.update(dt, GRAVITY));
        self.update_camera(dt);
    }

    fn update_flight(&mut self, dt: f32) {
        let controls = read_controls();
        let lander = &mut self.lander;

        if self
            .workshop
            .run(3, || lander.update(&controls, dt))
            .is_none()
        {
            // Lesson 3 unfinished: drive the Lesson 2 physics directly.
            let direction = match controls.rotate {
                Rotate::Left => -1.0,
                Rotate::Right => 1.0,
                Rotate::None => 0.0,
            };
            let lander = &mut self.lander;
            let physics = self.workshop.run(2, || {
                lander.rotate(direction, dt);
                lander.thrusting = false;
                if controls.thrust {
                    lander.fire_engine(dt);
                }
                lander.apply_gravity(dt);
                lander.integrate(dt);
            });
            if physics.is_none() {
                let message = self.workshop.issues.get(&2).cloned().unwrap_or_default();
                self.phase = Phase::Broken { message };
                return;
            }
        }

        if self.lander.thrusting {
            let (particles, lander) = (&mut self.particles, &self.lander);
            self.workshop.run(6, || particles.exhaust(lander));
        }

        let (lander, terrain) = (&mut self.lander, &self.terrain);
        if self
            .workshop
            .run(5, || collision::resolve_contact(lander, terrain))
            .is_none()
        {
            // Lesson 5 unfinished: no collisions, you fall through the Moon...
            let p = self.lander.position;
            if p.y < -15.0 || p.x < -20.0 || p.x > WORLD_WIDTH + 20.0 || p.y > 300.0 {
                self.lander.state = LanderState::Crashed {
                    reason: CrashReason::LostInSpace,
                };
            }
        }

        match self.lander.state {
            LanderState::Flying => {}
            LanderState::Landed { score } => {
                self.score += score;
                self.lander.thrusting = false;
                self.phase = Phase::Landed { timer: 0.0 };
            }
            LanderState::Crashed { .. } => {
                self.lander.thrusting = false;
                let (particles, at) = (&mut self.particles, self.lander.position);
                self.workshop.run(6, || particles.explode(at, 150));
                self.phase = Phase::Crashed { timer: 0.0 };
            }
        }
    }

    fn save_highscore(&mut self, name: &str) {
        let score = self.score;
        let path = Path::new(HIGHSCORE_FILE);
        if let Some(table) = &mut self.highscores {
            let result = self.workshop.run(7, || {
                table.add(name, score);
                table.save(path)
            });
            self.highscore_note = match result {
                Some(Err(err)) => Some(err.to_string()),
                _ => None,
            };
        }
    }

    fn update_camera(&mut self, dt: f32) {
        let flying_scene = !matches!(self.phase, Phase::Title | Phase::Broken { .. });
        let ground = self.ground_below();
        let altitude = self.lander.position.y - ground;
        let target_zoom = if flying_scene && altitude < ZOOM_ALTITUDE {
            MAX_ZOOM
        } else {
            1.0
        };

        let smoothing = 1.0 - (-3.5 * dt).exp();
        self.zoom += (target_zoom - self.zoom) * smoothing;

        let t = (self.zoom - 1.0) / (MAX_ZOOM - 1.0);
        let view_w = WORLD_WIDTH / self.zoom;
        let view_h = view_w * screen_height() / screen_width();
        let left = (self.lander.position.x - view_w / 2.0).clamp(0.0, WORLD_WIDTH - view_w);
        let bottom = (ground - view_h * 0.2).max(0.0);
        self.view_left += (left * t - self.view_left) * smoothing.max(t * 0.5);
        self.view_bottom += (bottom * t - self.view_bottom) * smoothing;
    }

    // -------------------------------- draw ----------------------------------

    /// World metres → screen pixels.
    fn to_screen(&self, p: Vec2) -> macroquad::math::Vec2 {
        let scale = self.scale();
        vec2(
            (p.x - self.view_left) * scale,
            screen_height() - (p.y - self.view_bottom) * scale,
        )
    }

    fn scale(&self) -> f32 {
        screen_width() / (WORLD_WIDTH / self.zoom)
    }

    fn draw(&mut self) {
        clear_background(BG);
        if let Phase::Broken { message } = &self.phase {
            draw_construction_screen(message);
            return;
        }

        self.draw_stars();
        self.draw_terrain();
        self.draw_particles();
        if !matches!(self.phase, Phase::Title) {
            self.draw_lander();
            self.draw_hud();
        }

        let message = self.status_message();
        let reason = self.crash_reason_text();
        match &self.phase {
            Phase::Title => self.draw_title(),
            Phase::Landed { timer } => {
                draw_centered(&message, screen_height() * 0.35, 40.0, OK);
                if *timer > 1.0 {
                    draw_centered(
                        "Press SPACE for the next landing",
                        screen_height() * 0.35 + 40.0,
                        24.0,
                        OUTLINE,
                    );
                }
            }
            Phase::Crashed { timer } => {
                draw_centered(&message, screen_height() * 0.35, 40.0, BAD);
                draw_centered(&reason, screen_height() * 0.35 + 38.0, 26.0, OUTLINE);
                if *timer > 1.5 {
                    draw_centered("Press SPACE", screen_height() * 0.35 + 76.0, 24.0, OUTLINE);
                }
            }
            Phase::EnterInitials { name } => {
                draw_centered("NEW HIGH SCORE!", screen_height() * 0.3, 44.0, ACCENT);
                let blink = if (self.time * 3.0) as i32 % 2 == 0 {
                    "_"
                } else {
                    " "
                };
                draw_centered(
                    &format!("{name}{blink}"),
                    screen_height() * 0.3 + 60.0,
                    48.0,
                    OUTLINE,
                );
                draw_centered(
                    "Type your initials, then press ENTER",
                    screen_height() * 0.3 + 110.0,
                    22.0,
                    OUTLINE,
                );
            }
            Phase::Playing | Phase::Broken { .. } => {}
        }

        self.draw_workshop_banner();
    }

    fn status_message(&mut self) -> String {
        let lander = &self.lander;
        self.workshop
            .run(3, || lander.status_message())
            .flatten()
            .unwrap_or_else(|| format!("{:?}", lander.state))
    }

    fn crash_reason_text(&mut self) -> String {
        match self.lander.state {
            LanderState::Crashed { reason } => self
                .workshop
                .run(5, || reason.to_string())
                .unwrap_or_else(|| format!("{reason:?}")),
            _ => String::new(),
        }
    }

    fn draw_stars(&self) {
        for &(x, y, brightness) in &self.stars {
            let p = self.to_screen(Vec2 { x, y });
            let twinkle = 0.75 + 0.25 * (self.time * 2.0 + x).sin();
            draw_circle(
                p.x,
                p.y,
                1.0,
                Color::new(1.0, 1.0, 1.0, brightness * twinkle),
            );
        }
    }

    fn draw_terrain(&self) {
        let bottom = screen_height();
        for pair in self.terrain.points.windows(2) {
            let (a, b) = (self.to_screen(pair[0]), self.to_screen(pair[1]));
            draw_triangle(a, b, vec2(b.x, bottom), GROUND);
            draw_triangle(a, vec2(b.x, bottom), vec2(a.x, bottom), GROUND);
            draw_line(a.x, a.y, b.x, b.y, 2.0, OUTLINE);
        }
        for pad in &self.terrain.pads {
            let a = self.to_screen(Vec2 {
                x: pad.x_start,
                y: pad.y,
            });
            let b = self.to_screen(Vec2 {
                x: pad.x_end,
                y: pad.y,
            });
            draw_line(a.x, a.y, b.x, b.y, 5.0, ACCENT);
            let label = format!("x{}", pad.multiplier);
            let size = 20.0;
            let width = measure_text(&label, None, size as u16, 1.0).width;
            draw_text(
                &label,
                (a.x + b.x) / 2.0 - width / 2.0,
                a.y + 22.0,
                size,
                ACCENT,
            );
        }
    }

    fn draw_particles(&self) {
        let scale = self.scale();
        for p in &self.particles.particles {
            let s = self.to_screen(p.position);
            let b = p.brightness();
            let color = match p.kind {
                ParticleKind::Exhaust => Color::new(1.0, 0.5 + 0.5 * b, 0.2 * b, b),
                ParticleKind::Debris => Color::new(0.9, 0.9 * b, 0.6 * b, b),
            };
            draw_circle(s.x, s.y, (0.25 * scale).max(1.5), color);
        }
    }

    fn draw_lander(&self) {
        let l = &self.lander;
        let crashed = matches!(l.state, LanderState::Crashed { .. });
        let color = if crashed {
            Color::new(1.0, 0.3, 0.3, 0.35)
        } else {
            OUTLINE
        };
        // Our own rotation maths, so drawing works even before Lesson 1.
        let (sin, cos) = l.angle.sin_cos();
        let point = |x: f32, y: f32| {
            self.to_screen(Vec2 {
                x: l.position.x + x * cos + y * sin,
                y: l.position.y - x * sin + y * cos,
            })
        };
        let polyline = |pts: &[(f32, f32)], thickness: f32, color: Color| {
            for w in pts.windows(2) {
                let (a, b) = (point(w[0].0, w[0].1), point(w[1].0, w[1].1));
                draw_line(a.x, a.y, b.x, b.y, thickness, color);
            }
        };

        // Cabin, descent stage, legs, feet and nozzle.
        polyline(
            &[
                (-1.2, -0.2),
                (-1.2, 1.0),
                (-0.6, 1.8),
                (0.6, 1.8),
                (1.2, 1.0),
                (1.2, -0.2),
                (-1.2, -0.2),
            ],
            2.0,
            color,
        );
        polyline(
            &[
                (-1.6, -0.2),
                (1.6, -0.2),
                (1.6, -1.2),
                (-1.6, -1.2),
                (-1.6, -0.2),
            ],
            2.0,
            color,
        );
        polyline(&[(-1.3, -1.2), (-2.0, -2.0)], 2.0, color);
        polyline(&[(1.3, -1.2), (2.0, -2.0)], 2.0, color);
        polyline(&[(-2.5, -2.0), (-1.5, -2.0)], 2.0, color);
        polyline(&[(1.5, -2.0), (2.5, -2.0)], 2.0, color);
        polyline(
            &[(-0.4, -1.2), (-0.7, -1.8), (0.7, -1.8), (0.4, -1.2)],
            2.0,
            color,
        );
        polyline(
            &[
                (-0.35, 0.6),
                (0.35, 0.6),
                (0.35, 1.2),
                (-0.35, 1.2),
                (-0.35, 0.6),
            ],
            1.5,
            color,
        );

        if l.thrusting {
            let flicker = 2.0 + 1.2 * ((self.time * 40.0).sin() * 0.5 + 0.5);
            let (a, b, c) = (
                point(-0.55, -1.9),
                point(0.55, -1.9),
                point(0.0, -1.9 - flicker),
            );
            draw_triangle(a, b, c, Color::new(1.0, 0.6, 0.1, 0.9));
            let (a, b, c) = (
                point(-0.25, -1.9),
                point(0.25, -1.9),
                point(0.0, -1.9 - flicker * 0.6),
            );
            draw_triangle(a, b, c, Color::new(1.0, 1.0, 0.7, 0.9));
        }
    }

    fn draw_hud(&mut self) {
        let l = self.lander.clone();
        let size = 22.0;
        draw_text(&format!("SCORE  {}", self.score), 20.0, 32.0, size, OUTLINE);
        draw_text(&format!("LEVEL  {}", self.level), 20.0, 56.0, size, OUTLINE);
        draw_text("FUEL", 20.0, 80.0, size, OUTLINE);
        let ratio = (l.fuel / START_FUEL).clamp(0.0, 1.0);
        let fuel_color = if ratio > 0.25 { ACCENT } else { BAD };
        draw_rectangle_lines(80.0, 64.0, 160.0, 18.0, 2.0, OUTLINE);
        draw_rectangle(83.0, 67.0, 154.0 * ratio, 12.0, fuel_color);

        let altitude = l.position.y - self.ground_below();
        let descent = -l.velocity.y;
        let drift = l.velocity.x;
        let tilt = l.angle.to_degrees();
        let x = screen_width() - 290.0;
        let rows = [
            (format!("ALTITUDE    {altitude:6.1} m"), OUTLINE),
            (
                format!("DESCENT     {descent:6.1} m/s"),
                if descent <= MAX_VERTICAL_SPEED {
                    OK
                } else {
                    BAD
                },
            ),
            (
                format!("DRIFT       {drift:6.1} m/s"),
                if drift.abs() <= MAX_HORIZONTAL_SPEED {
                    OK
                } else {
                    BAD
                },
            ),
            (
                format!("TILT        {tilt:6.0} deg"),
                if l.angle.abs() <= MAX_LANDING_ANGLE {
                    OK
                } else {
                    BAD
                },
            ),
        ];
        for (i, (text, color)) in rows.iter().enumerate() {
            draw_text(text, x, 32.0 + 24.0 * i as f32, size, *color);
        }
    }

    fn draw_title(&self) {
        let h = screen_height();
        draw_centered("RUST LANDER", h * 0.2, 80.0, ACCENT);
        draw_centered(
            "Land gently on a yellow pad. Narrow pads score more.",
            h * 0.2 + 46.0,
            22.0,
            OUTLINE,
        );
        draw_centered(
            "UP / W: thrust     LEFT / RIGHT or A / D: rotate     ESC: quit",
            h * 0.2 + 74.0,
            20.0,
            OUTLINE,
        );

        // Systems check: which lessons are done?
        let x = screen_width() / 2.0 - 300.0;
        let mut y = h * 0.2 + 130.0;
        draw_text("SYSTEMS CHECK", x, y, 24.0, ACCENT);
        for (i, name) in LESSON_NAMES.iter().enumerate() {
            y += 26.0;
            let lesson = i as u8 + 1;
            let (status, color) = if self.workshop.issues.contains_key(&lesson) {
                ("UNDER CONSTRUCTION", WARN)
            } else {
                ("ONLINE", OK)
            };
            draw_text(&format!("Lesson {lesson}  {name:<12}"), x, y, 22.0, OUTLINE);
            draw_text(status, x + 220.0, y, 22.0, color);
        }

        // High scores.
        let x = screen_width() / 2.0 + 120.0;
        let mut y = h * 0.2 + 130.0;
        draw_text("HALL OF FAME", x, y, 24.0, ACCENT);
        match &self.highscores {
            None => {
                draw_text("(Lesson 7)", x, y + 26.0, 22.0, WARN);
            }
            Some(table) if table.entries.is_empty() => {
                draw_text("no scores yet", x, y + 26.0, 22.0, OUTLINE);
            }
            Some(table) => {
                for (rank, entry) in table.entries.iter().enumerate() {
                    y += 26.0;
                    draw_text(
                        &format!("{}. {:<3} {:>6}", rank + 1, entry.name, entry.score),
                        x,
                        y,
                        22.0,
                        OUTLINE,
                    );
                }
            }
        }
        if let Some(note) = &self.highscore_note {
            draw_centered(note, h * 0.2 + 340.0, 18.0, BAD);
        }

        if (self.time * 2.0) as i32 % 2 == 0 {
            draw_centered("Press SPACE to launch", h * 0.82, 30.0, OUTLINE);
        }
    }

    fn draw_workshop_banner(&self) {
        if self.workshop.issues.is_empty() || matches!(self.phase, Phase::Title) {
            return;
        }
        let mut y = screen_height() - 12.0 - 20.0 * (self.workshop.issues.len() as f32 - 1.0);
        for (lesson, message) in &self.workshop.issues {
            let text = format!("UNDER CONSTRUCTION  Lesson {lesson}: {message}");
            draw_text(&text, 12.0, y, 18.0, WARN);
            y += 20.0;
        }
    }
}

fn draw_construction_screen(message: &str) {
    let (w, h) = (screen_width(), screen_height());
    for i in 0..((w / 40.0) as i32 + 4) {
        let x = i as f32 * 40.0 - 40.0;
        draw_line(x, 0.0, x + 20.0, 20.0, 12.0, WARN);
        draw_line(x, h, x + 20.0, h - 20.0, 12.0, WARN);
    }
    draw_centered("UNDER CONSTRUCTION", h * 0.35, 56.0, WARN);
    draw_centered(
        "The lander can't fly until Lessons 1 and 2 are done.",
        h * 0.35 + 50.0,
        24.0,
        OUTLINE,
    );
    draw_centered(message, h * 0.35 + 100.0, 20.0, ACCENT);
    draw_centered(
        "Finish the lesson, then run `cargo run` again.",
        h * 0.35 + 150.0,
        22.0,
        OUTLINE,
    );
}

fn draw_centered(text: &str, y: f32, size: f32, color: Color) {
    let width = measure_text(text, None, size as u16, 1.0).width;
    draw_text(text, (screen_width() - width) / 2.0, y, size, color);
}

fn read_controls() -> Controls {
    let left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
    let right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
    let rotate = match (left, right) {
        (true, false) => Rotate::Left,
        (false, true) => Rotate::Right,
        _ => Rotate::None,
    };
    let thrust = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
    Controls { thrust, rotate }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Lander".to_string(),
        window_width: 1200,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    guard::install();
    let mut game = Game::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            match game.phase {
                Phase::Title | Phase::Broken { .. } => break,
                _ => game.phase = Phase::Title,
            }
        }
        // Cap the time step so a hiccup doesn't teleport the lander.
        let dt = get_frame_time().min(1.0 / 30.0);
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
