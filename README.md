# Rust Lander 🚀🌕

Learn Rust by building a moon-landing arcade game, one lesson at a time,
with Claude as your tutor.

## How it works

- The game logic lives in `game/src/`, one module per lesson. Your job is to
  replace the `todo!("Lesson N")` markers (look for ✏️) with real code.
- Every lesson has a test suite in `game/tests/lesson_0N.rs`. Don't edit the
  tests; they check your work.
- The game always runs, even when it's unfinished. Missing parts show up as
  **UNDER CONSTRUCTION** on screen, and the title screen has a *Systems Check*.

| # | Lesson              | File            | Rust concepts                                  |
|---|---------------------|-----------------|------------------------------------------------|
| 1 | Vectors in Space    | `vec2.rs`       | structs, `impl`, methods, `derive`, operators  |
| 2 | Newton on the Moon  | `lander.rs`     | `&mut self`, constants, `f32` maths            |
| 3 | Taking Control      | `lander.rs`     | enums, `match`, `matches!`, `Option`           |
| 4 | Lunar Terrain       | `terrain.rs`    | `Vec`, slices, iterators, closures, borrowing  |
| 5 | Touchdown or Crash  | `collision.rs`  | `Result`, `?`, match guards, `Display`         |
| 6 | Fire & Debris       | `particles.rs`  | closures, generics, `FnMut`, `retain`          |
| 7 | Hall of Fame        | `highscore.rs`  | `String`/`&str`, parsing, errors, file IO      |

## Commands

```sh
cargo test --test lesson_01   # check one lesson
cargo test                    # check everything
cargo run                     # play!
```

Controls: **↑/W** thrust, **←/→ or A/D** rotate, **Esc** back or quit.

`.tutor/solutions/` has reference solutions. Try not to peek. Ask Claude for a hint instead.
