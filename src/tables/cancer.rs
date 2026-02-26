//! The table in this module defines the minimum donation amount per year for various levels of Bromite.
//! There is also a lookup function which will safely interpolate between Bromite levels to give you the
//! most fair minimum donation amount.

/// (Bromite levels in cubic meters)
pub const TABLE: &[(f64, u64)] = &[
    /// If there is 0 Bromite, you shouldn't donate any money because that would be waste. You should
    /// spend that money on Thorium for your phase selectors.
    (0.0f64, 0),
    
];