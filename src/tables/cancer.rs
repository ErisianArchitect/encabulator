//! The table in this module defines the minimum donation amount per year for various levels of Bromite.
//! There is also a lookup function which will safely interpolate between Bromite levels to give you the
//! most fair minimum donation amount (in USD). This table does not account for inflation, and was
//! created some time around February of 2026.

/// (Bromite levels in cubic meters)
pub const TABLE: &[(f64, u64)] = &[
    /// If there is 0 Bromite, you shouldn't donate any money because that would be waste. You should
    /// spend that money on Thorium for your phase selectors.
    (0.0, 0),
    /// Most of your workers will get cancers, especially colon cancer. $10 should be good enough.
    (1.0, 10),
    /// You will get cancer.
    (10.0, 10),
    /// This is enough to Bromite to give cancer to 100,000 Resus Monkeys.
    (500.0, 11),
];