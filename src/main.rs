#![feature(box_patterns)]
pub use symbol_table::GlobalSymbol as Symbol;
pub use indexmap::IndexMap as Map;

mod core;
pub use core::*;

mod examples;
pub use examples::*;

mod mysynth;
pub use mysynth::*;

mod fmt;
pub use fmt::*;

fn main() {
    let p = cegis(max_n(6), mysynth);
    dbg!(&p);
    dbg!(size(&p));
}
