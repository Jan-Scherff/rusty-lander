//! # Rust Lander — the library half of the game
//!
//! Everything that makes the game *work* lives in this library, split into
//! one module per lesson. The binary (`main.rs`) only draws pictures and reads
//! the keyboard; it calls into these modules every frame.
//!
//! | Lesson | Module        | Topic                                   |
//! |--------|---------------|-----------------------------------------|
//! | 1      | [`vec2`]      | structs, methods, operator traits       |
//! | 2      | [`lander`]    | `&mut self`, constants, physics         |
//! | 3      | [`lander`]    | enums & `match`                         |
//! | 4      | [`terrain`]   | `Vec`, slices, iterators, `Option`      |
//! | 5      | [`collision`] | `Result`, `?`, the `Display` trait      |
//! | 6      | [`particles`] | closures & generics                     |
//! | 7      | [`highscore`] | `String`s, parsing, errors, file IO     |

// While lessons are unfinished, many parameters are not used yet (the body is
// just `todo!()`). These lines silence those warnings so the real compiler
// errors stand out. Once you finish the course, try deleting them!
#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unreachable_code
)]

pub mod collision;
pub mod highscore;
pub mod lander;
pub mod particles;
pub mod rng;
pub mod terrain;
pub mod vec2;
