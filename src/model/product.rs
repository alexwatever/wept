use parse_display::FromStr;
use serde::{Deserialize, Serialize};
use std::fmt::{Display as FmtDisplay, Formatter, Result as FmtResult};

/// # Product
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct Product {
    pub(crate) id: u32,
    pub(crate) title: String,
    pub(crate) price: f32,
    pub(crate) description: String,
    pub(crate) category: String,
    pub(crate) image: String,
    pub(crate) rating: ProductRating,
}

/// # Product Rating
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct ProductRating {
    pub(crate) rate: f32,
    pub(crate) count: u32,
}

impl FmtDisplay for ProductRating {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let rounded = self.rate.round() as usize;
        for _ in 0..rounded {
            "★".fmt(f)?;
        }
        for _ in 0..(5 - rounded) {
            "☆".fmt(f)?;
        }

        write!(f, " ({:01}) ({} ratings)", self.rate, self.count)?;

        Ok(())
    }
}

/// # Product Size
#[derive(Default, FromStr, Debug)]
#[display(style = "snake_case")]
pub(crate) enum ProductSize {
    Small,
    #[default]
    Medium,
    Large,
}
