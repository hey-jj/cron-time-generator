//! Error type for the three failing paths.
//!
//! Three operations can fail: an empty day list, an unknown day name, and a
//! start day that comes after the end day. Each maps to a [`CronError`]
//! variant.

use std::fmt;

/// An error from a fallible builder call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CronError {
    /// A day list was empty where at least one day is required.
    EmptyDays,
    /// A day name was not recognized. `day` is the trimmed, lowercased input.
    InvalidDay {
        /// The normalized name that failed lookup.
        day: String,
    },
    /// A weekday range had its start day after its end day.
    StartAfterEnd,
}

impl fmt::Display for CronError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CronError::EmptyDays => f.write_str("day list must not be empty"),
            CronError::InvalidDay { day } => {
                write!(f, "\"{day}\" is not a valid day name")
            }
            CronError::StartAfterEnd => f.write_str("start day must not come after end day"),
        }
    }
}

impl std::error::Error for CronError {}
