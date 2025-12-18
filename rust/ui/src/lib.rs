#![deny(missing_docs)]

//! Watch-style UI using iced. Supports displaying 1, 2, or all 3 tier displays simultaneously.
//!
//! ## Interactions
//! - **Tap** (mouse click): select/acknowledge
//! - **Gestures** (arrow keys): navigate screens
//! - **Voice** (TBD keybind): push-to-talk for LLM
//! - **Zoom** (spacebar/+/-): zoom charts

use core_types::Tier;

pub mod display_mode;
pub use display_mode::DisplayMode;

/// Simple placeholder for a watch-style screen render.
pub fn render_placeholder(tier: Tier) {
    println!("[UI stub] Rendering for tier {:?}", tier);
}
