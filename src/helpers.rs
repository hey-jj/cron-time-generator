//! Building blocks shared by the public methods.
//!
//! These functions produce the canonical field strings and the single-field
//! replacement that the fluent builder uses. They are public so tests and
//! callers can reach the same primitives the higher-level methods build on.

use crate::days::Day;
use crate::error::CronError;
use crate::nums::Nums;

/// Replace one field of a 5-field cron string.
///
/// `base` is split on single spaces, the element at `position` is replaced
/// with `char`, and the parts are rejoined with single spaces. When `base` is
/// `None` the minute base `"* * * * *"` is used.
///
/// `position` is assumed in range `0..=4`, which is all the builder ever uses.
///
/// # Examples
///
/// ```
/// use cron_time_generator::helpers::splice_into_position;
///
/// assert_eq!(splice_into_position(0, "2", None), "2 * * * *");
/// assert_eq!(splice_into_position(4, "5", None), "* * * * 5");
/// ```
pub fn splice_into_position(position: usize, char: &str, base: Option<&str>) -> String {
    let owned;
    let source = match base {
        Some(s) => s,
        None => {
            owned = minute();
            &owned
        }
    };

    let mut parts: Vec<String> = source.split(' ').map(|s| s.to_string()).collect();
    if position < parts.len() {
        parts[position] = char.to_string();
    } else {
        parts.push(char.to_string());
    }
    parts.join(" ")
}

/// The every-minute base: `"* * * * *"`.
pub fn minute() -> String {
    "* * * * *".to_string()
}

/// The top-of-the-hour base: `"0 * * * *"`.
pub fn hour() -> String {
    "0 * * * *".to_string()
}

/// Build a daily cron string with set hour and minute fields.
///
/// The output puts minutes first, then hours: `"{minutes} {hours} * * *"`.
/// Note the argument order is hours then minutes, the reverse of the field
/// order. Both default to `0`.
///
/// # Examples
///
/// ```
/// use cron_time_generator::helpers::day;
/// use cron_time_generator::Nums;
///
/// assert_eq!(day(Nums::from(10), Nums::from(30)), "30 10 * * *");
/// ```
pub fn day(hours_of_the_day: Nums, minutes_of_the_hour: Nums) -> String {
    format!("{minutes_of_the_hour} {hours_of_the_day} * * *")
}

/// Resolve a list of day arguments to their integers.
///
/// Each element runs through [`Day::to_int`]. A bad name stops the whole call
/// with [`CronError::InvalidDay`].
pub fn days_to_integers(days: &[Day]) -> Result<Vec<i64>, CronError> {
    days.iter().map(Day::to_int).collect()
}

/// Check that a weekday range runs forward.
///
/// Returns [`CronError::StartAfterEnd`] when `start_day > end_day`.
pub fn validate_start_to_end_day(start_day: i64, end_day: i64) -> Result<(), CronError> {
    if start_day > end_day {
        return Err(CronError::StartAfterEnd);
    }
    Ok(())
}

/// Render a slice of integers comma-joined, matching a list cron field.
pub(crate) fn join_ints(values: &[i64]) -> String {
    values
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
