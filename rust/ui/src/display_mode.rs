//! Display mode types for multi-tier viewing.

/// Display mode: which tier(s) to show.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    /// Show only Mini 8GB tier.
    Mini,
    /// Show only Regular 16GB tier.
    Regular,
    /// Show only Pro 32GB tier.
    Pro,
    /// Show Mini and Regular side-by-side.
    MiniRegular,
    /// Show Regular and Pro side-by-side.
    RegularPro,
    /// Show all three tiers.
    Triple,
}
