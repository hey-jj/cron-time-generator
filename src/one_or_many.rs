//! One-or-many integer values for a single cron field.
//!
//! A cron field can hold a single number or a comma-separated list. [`Nums`]
//! models both. Its [`Display`] renders a single value plainly and a list with
//! commas and no spaces, which matches how a cron field lists values.

use std::fmt;

/// A single integer or a list of integers for one cron field.
///
/// `One(30)` renders as `30`. `Many(vec![10, 20, 30])` renders as `10,20,30`.
/// An empty `Many` renders as the empty string, which leaves the field blank.
///
/// # Examples
///
/// ```
/// use cron_time_generator::Nums;
///
/// assert_eq!(Nums::from(30).to_string(), "30");
/// assert_eq!(Nums::from(vec![10, 20, 30]).to_string(), "10,20,30");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Nums {
    /// A single value.
    One(i64),
    /// A list of values rendered comma-separated.
    Many(Vec<i64>),
}

impl fmt::Display for Nums {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Nums::One(n) => write!(f, "{n}"),
            Nums::Many(v) => {
                let joined = v
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                f.write_str(&joined)
            }
        }
    }
}

impl From<i64> for Nums {
    fn from(n: i64) -> Self {
        Nums::One(n)
    }
}

impl From<Vec<i64>> for Nums {
    fn from(v: Vec<i64>) -> Self {
        Nums::Many(v)
    }
}

impl From<&[i64]> for Nums {
    fn from(v: &[i64]) -> Self {
        Nums::Many(v.to_vec())
    }
}

impl<const N: usize> From<[i64; N]> for Nums {
    fn from(v: [i64; N]) -> Self {
        Nums::Many(v.to_vec())
    }
}
