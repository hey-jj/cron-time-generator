//! Building blocks shared by the public methods.
//!
//! These functions produce the canonical field strings and the single-field
//! replacement that the fluent builder uses. They are crate-internal.

use crate::days::Day;
use crate::error::CronError;
use crate::one_or_many::OneOrMany;

/// Replace one field of a 5-field cron string.
///
/// `base` is split on single spaces, the element at `position` is replaced
/// with `value`, and the parts are rejoined with single spaces. When `base` is
/// `None` the minute base `"* * * * *"` is used.
///
/// `position` is assumed in range `0..=4`, which is all the builder ever uses.
pub(crate) fn replace_field(position: usize, value: &str, base: Option<&str>) -> String {
    let owned;
    let base_str = match base {
        Some(s) => s,
        None => {
            owned = minute();
            &owned
        }
    };

    let mut parts: Vec<String> = base_str.split(' ').map(|s| s.to_string()).collect();
    if position < parts.len() {
        parts[position] = value.to_string();
    } else {
        parts.push(value.to_string());
    }
    parts.join(" ")
}

/// The every-minute base: `"* * * * *"`.
pub(crate) fn minute() -> String {
    "* * * * *".to_string()
}

/// The top-of-the-hour base: `"0 * * * *"`.
pub(crate) fn hour() -> String {
    "0 * * * *".to_string()
}

/// Build a daily cron string with set hour and minute fields.
///
/// The output puts minutes first, then hours: `"{minutes} {hours} * * *"`.
/// Note the argument order is hours then minutes, the reverse of the field
/// order. Both default to `0`.
pub(crate) fn day(hours_of_the_day: OneOrMany, minutes_of_the_hour: OneOrMany) -> String {
    format!("{minutes_of_the_hour} {hours_of_the_day} * * *")
}

/// Resolve a sequence of day arguments to a [`OneOrMany`] list of integers.
///
/// Each item is converted to a [`Day`] and run through [`Day::to_int`]. A bad
/// name stops the whole call with [`CronError::InvalidDay`].
pub(crate) fn days_to_field<I, D>(days: I) -> Result<OneOrMany, CronError>
where
    I: IntoIterator<Item = D>,
    D: Into<Day>,
{
    let ints = days
        .into_iter()
        .map(|d| d.into().to_int())
        .collect::<Result<Vec<i64>, _>>()?;
    Ok(OneOrMany::Many(ints))
}

/// Check that a weekday range runs forward.
///
/// Returns [`CronError::StartAfterEnd`] when `start_day > end_day`.
pub(crate) fn validate_start_to_end_day(start_day: i64, end_day: i64) -> Result<(), CronError> {
    if start_day > end_day {
        return Err(CronError::StartAfterEnd);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_field_field0() {
        assert_eq!(replace_field(0, "2", None), "2 * * * *");
    }

    #[test]
    fn replace_field_field4() {
        assert_eq!(replace_field(4, "5", None), "* * * * 5");
    }

    #[test]
    fn replace_field_explicit_base() {
        assert_eq!(replace_field(2, "X", Some("a b c d e")), "a b X d e");
    }

    #[test]
    fn minute_base() {
        assert_eq!(minute(), "* * * * *");
    }

    #[test]
    fn hour_base() {
        assert_eq!(hour(), "0 * * * *");
    }

    #[test]
    fn day_with_hour_and_minute() {
        assert_eq!(day(OneOrMany::from(10), OneOrMany::from(30)), "30 10 * * *");
    }

    #[test]
    fn day_with_defaults() {
        assert_eq!(day(OneOrMany::from(0), OneOrMany::from(0)), "0 0 * * *");
    }

    #[test]
    fn days_to_field_all_long_names() {
        let names = [
            "sunday",
            "monday",
            "tuesday",
            "wednesday",
            "thursday",
            "friday",
            "saturday",
        ];
        assert_eq!(days_to_field(names).unwrap().to_string(), "0,1,2,3,4,5,6");
    }

    #[test]
    fn days_to_field_single_name() {
        assert_eq!(days_to_field(["monday"]).unwrap().to_string(), "1");
    }

    #[test]
    fn days_to_field_numeric_passthrough() {
        assert_eq!(days_to_field([9i64]).unwrap().to_string(), "9");
    }

    #[test]
    fn days_to_field_invalid_name() {
        assert_eq!(
            days_to_field(["garbage"]),
            Err(CronError::InvalidDay {
                day: "garbage".to_string()
            })
        );
    }

    #[test]
    fn validate_start_to_end_day_ok() {
        assert!(validate_start_to_end_day(1, 5).is_ok());
        assert!(validate_start_to_end_day(3, 3).is_ok());
    }

    #[test]
    fn validate_start_to_end_day_rejects_backward() {
        assert_eq!(
            validate_start_to_end_day(5, 1),
            Err(CronError::StartAfterEnd)
        );
    }
}
