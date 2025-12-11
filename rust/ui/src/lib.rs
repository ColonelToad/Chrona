#![deny(missing_docs)]

//! Placeholder UI crate. Actual UI toolkit (egui/iced) to be chosen later.

use core_types::Tier;
use logic::Engine;

/// Simple placeholder for a watch-style screen render.
pub fn render_placeholder(tier: Tier) {
    println!("[UI stub] Rendering for tier {:?}", tier);
}
