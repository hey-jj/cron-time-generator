//! Error type for the three failing paths.
//!
//! Three operations can fail: an empty or wrong-shaped day list, an unknown
//! day name, and a start day that comes after the end day. Each maps to a
//! [`CronError`] variant whose [`Display`] reproduces the exact message text.

use std::fmt;

/// An error from a fallible builder call.
///
/// The [`Display`] text is fixed and matches the messages the builder reports
/// for each failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CronError {
    /// A day list was empty. `method` names the call that rejected it.
    EmptyDays {
        /// The method name used in the message, such as `onSpecificDays`.
        method: &'static str,
    },
    /// A day name was not recognized. `day` is the trimmed, lowercased input.
    InvalidDay {
        /// The normalized name that failed lookup.
        day: String,
    },
    /// A weekday range had its start after its end.
    StartAfterEnd,
}

impl fmt::Display for CronError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CronError::EmptyDays { method } => {
                write!(f, "{method} expects days to be an array of days string.")
            }
            CronError::InvalidDay { day } => {
                write!(f, "Day: \"{day}\" is not a valid day.")
            }
            CronError::StartAfterEnd => {
                f.write_str("startDay must come before endDay following normal calendar sequence.")
            }
        }
    }
}

impl std::error::Error for CronError {}
