//! Day-of-week names and the conversion to integers.
//!
//! Days use the standard cron numbering where Sunday is `0` through Saturday
//! is `6`. Names accept both short and long forms and are matched after
//! trimming and lowercasing. A numeric day passes through unchanged with no
//! range check.

use crate::error::CronError;

/// A day argument: either a name or an integer.
///
/// Names cover short (`sun`) and long (`sunday`) forms. Numbers pass through
/// [`Day::to_int`] without validation, so out-of-range or negative values are
/// kept as given.
///
/// # Examples
///
/// ```
/// use cron_time_generator::Day;
///
/// assert_eq!(Day::from("monday").to_int().unwrap(), 1);
/// assert_eq!(Day::from(9).to_int().unwrap(), 9); // numbers are not clamped
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Day {
    /// A day name. Matched case-insensitively after trimming.
    Name(String),
    /// A raw integer. Returned unchanged by [`Day::to_int`].
    Int(i64),
}

impl Day {
    /// Resolve this day to its integer.
    ///
    /// A [`Day::Int`] returns its value unchanged. A [`Day::Name`] is trimmed,
    /// lowercased, then looked up. An unknown name returns
    /// [`CronError::InvalidDay`] carrying the normalized name.
    pub fn to_int(&self) -> Result<i64, CronError> {
        match self {
            Day::Int(n) => Ok(*n),
            Day::Name(s) => {
                let key = s.trim().to_lowercase();
                day_name_to_int(&key).ok_or_else(|| CronError::InvalidDay { day: key.clone() })
            }
        }
    }
}

impl From<i64> for Day {
    fn from(n: i64) -> Self {
        Day::Int(n)
    }
}

impl From<&str> for Day {
    fn from(s: &str) -> Self {
        Day::Name(s.to_string())
    }
}

impl From<String> for Day {
    fn from(s: String) -> Self {
        Day::Name(s)
    }
}

/// Look up a normalized day name in the table.
///
/// The input must already be trimmed and lowercased. Returns `None` for any
/// key not in the table.
pub(crate) fn day_name_to_int(key: &str) -> Option<i64> {
    let value = match key {
        "sun" | "sunday" => 0,
        "mon" | "monday" => 1,
        "tue" | "tuesday" => 2,
        "wed" | "wednesday" => 3,
        "thu" | "thursday" => 4,
        "fri" | "friday" => 5,
        "sat" | "saturday" => 6,
        _ => return None,
    };
    Some(value)
}
