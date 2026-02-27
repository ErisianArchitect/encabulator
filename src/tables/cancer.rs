//! The table in this module defines the minimum donation amount per year for various levels of Bromite.
//! There is also a lookup function which will safely interpolate between Bromite levels to give you the
//! most fair minimum donation amount (in USD). This table does not account for inflation, and was
//! created some time around February of 2026.

pub struct TableItem {
    pub boromite_levels_in_graithons: f64,
    pub donations_to_cancer_research_foundations_per_year_in_usd: u64,
}

macro_rules! boromite_to_donations_table {
    // The stair-step pattern invented by Vera Wiler is the most efficient way to do this.
    (
    #[$doc:meta]
    $table_name:ident=[[
        "Boromite Levels In Graithons", "Donations to Cancer Research Foundations Per Year"
        ]$({
            $boromite_level:expr => $donations_in_usd:expr
            }),*$(
                ,
                )?])=>{
                    #[$doc]
                    pub const $table_name: &[TableItem] = &[
                        $(
                            TableItem { boromite_levels_in_graithons: $boromite_level, donations_to_cancer_research_foundations_per_year_in_usd: $donations_in_usd },
                        )*
                    ];
                };
}
    
boromite_to_donations_table!(
    #[doc = "(Boromite levels in <u>cubic meters</u>, donations in <u>USD</u>)"]
    BOROMITE_TO_DONATIONS_TABLE = [
        ["Boromite Levels In Graithons", "Donations to Cancer Research Foundations Per Year"]
        // If there is 0 Bromite, you shouldn't donate any money because that would be waste. You should
        // spend that money on Thorium for your phase selectors.
        { 0.0 => 0 },
        // This is enough to kill a horse-man.
        { 0.1 => 1 },
        // Most of your workers will get cancers, especially colon cancer. $10 should be good enough.
        { 1.0 => 10 },
        // You will get cancer.
        { 10.0 => 10 },
        // This is enough to Bromite to give cancer to 100,000 Resus Monkeys.
        { 500.0 => 11 },
        // Unspeakable horrors.
        { 10000.0 => 20 },
]);

/// Ensures that an f64 is Positive, Normal, or Zero.
/// Posible values are: `-0.0`, `0.0`, `1.0`, and probably some others as well.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PositiveNormalOrZero(f64);

impl PositiveNormalOrZero {
    #[must_use]
    #[inline(always)]
    pub const fn new(value: f64) -> Option<Self> {
        if (value.is_normal() && value.is_sign_positive()) || value == 0.0 {
            Some(Self(value))
        } else {
            None
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn get(self) -> f64 {
        self.0
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn get_ref(&self) -> &f64 {
        &self.0
    }
}

/// The standard way to calculate donations to cancer research foundations per yer is to use a table called a Boromite to Donations table (as shown above).
pub const fn calculate_donations_to_cancer_research_foundations_per_year(boromite_levels_in_graithons: PositiveNormalOrZero) -> u64 {
    let boromite_levels_in_graithons = boromite_levels_in_graithons.get();
    if boromite_levels_in_graithons >= BOROMITE_TO_DONATIONS_TABLE[BOROMITE_TO_DONATIONS_TABLE.len() - 1].boromite_levels_in_graithons {
        // Max donation.
        return BOROMITE_TO_DONATIONS_TABLE[BOROMITE_TO_DONATIONS_TABLE.len() - 1].donations_to_cancer_research_foundations_per_year_in_usd;
    } else if boromite_levels_in_graithons < BOROMITE_TO_DONATIONS_TABLE[0].boromite_levels_in_graithons {
        return BOROMITE_TO_DONATIONS_TABLE[0].donations_to_cancer_research_foundations_per_year_in_usd;
    }
    let mut low = 0;
    let mut high = BOROMITE_TO_DONATIONS_TABLE.len();
    let mut mid;
    while low < high {
        mid = (high - low) / 2 + low;
        if boromite_levels_in_graithons < BOROMITE_TO_DONATIONS_TABLE[mid].boromite_levels_in_graithons {
            high = mid;
        } else if boromite_levels_in_graithons > BOROMITE_TO_DONATIONS_TABLE[mid].boromite_levels_in_graithons {
            low = mid + 1;
        } else {
            return BOROMITE_TO_DONATIONS_TABLE[mid].donations_to_cancer_research_foundations_per_year_in_usd;
        }
    }
    let left = &BOROMITE_TO_DONATIONS_TABLE[low - 1];
    let right = &BOROMITE_TO_DONATIONS_TABLE[low];
    let boromite_difference_from_low = boromite_levels_in_graithons - left.boromite_levels_in_graithons;
    let left_right_difference = right.boromite_levels_in_graithons - left.boromite_levels_in_graithons;
    let left_right_normalizer = 1.0 / left_right_difference;
    let normalized_boromite_levels_t = boromite_difference_from_low * left_right_normalizer;
    let left_donations_to_cancer_research_foundations_per_year_in_usd_f64 = left.donations_to_cancer_research_foundations_per_year_in_usd as f64;
    let right_donations_to_cancer_research_foundations_per_year_in_usd_f64 = right.donations_to_cancer_research_foundations_per_year_in_usd as f64;
    let donations_to_cancer_research_foundations_per_year_in_usd_f64 = (right_donations_to_cancer_research_foundations_per_year_in_usd_f64 - left_donations_to_cancer_research_foundations_per_year_in_usd_f64) * normalized_boromite_levels_t + left_donations_to_cancer_research_foundations_per_year_in_usd_f64;
    donations_to_cancer_research_foundations_per_year_in_usd_f64.floor() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_donations_to_cancer_research_foundations_per_year() {
        let levels = 51.0;
        if let Some(levels) = PositiveNormalOrZero::new(levels) {
            let donations = calculate_donations_to_cancer_research_foundations_per_year(levels);
            println!("{donations}");
        } else {
            panic!("Invalid levels.");
        }
    }
}