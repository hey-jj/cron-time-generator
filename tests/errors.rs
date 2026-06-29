//! Error-path cases (X1-X5).
//!
//! The three fallible operations return a typed error. Each case checks the
//! variant and the message text.

use cron_time_generator::{CronError, CronTime as C, Day};

// X1
#[test]
fn on_specific_days_empty() {
    let empty: [&str; 0] = [];
    let err = C::on_specific_days(&empty).unwrap_err();
    assert_eq!(
        err,
        CronError::EmptyDays {
            method: "onSpecificDays"
        }
    );
    assert_eq!(
        err.to_string(),
        "onSpecificDays expects days to be an array of days string."
    );
}

// X2
#[test]
fn on_specific_days_at_empty() {
    let empty: [&str; 0] = [];
    let err = C::on_specific_days_at(&empty, 3, 0).unwrap_err();
    assert_eq!(
        err,
        CronError::EmptyDays {
            method: "onSpecificDaysAt"
        }
    );
    assert_eq!(
        err.to_string(),
        "onSpecificDaysAt expects days to be an array of days string."
    );
}

// X3
#[test]
fn day_to_int_invalid_name() {
    let err = Day::from("garbage").to_int().unwrap_err();
    assert_eq!(
        err,
        CronError::InvalidDay {
            day: "garbage".to_string()
        }
    );
    assert_eq!(err.to_string(), "Day: \"garbage\" is not a valid day.");
}

// X3 variant: the message uses the trimmed, lowercased form
#[test]
fn day_to_int_invalid_name_normalized() {
    let err = Day::from("  GARBAGE  ").to_int().unwrap_err();
    assert_eq!(err.to_string(), "Day: \"garbage\" is not a valid day.");
}

// X4
#[test]
fn every_week_day_start_after_end() {
    let err = C::every_week_day_range(Day::from("friday"), Day::from("monday")).unwrap_err();
    assert_eq!(err, CronError::StartAfterEnd);
    assert_eq!(
        err.to_string(),
        "startDay must come before endDay following normal calendar sequence."
    );
}

// X5
#[test]
fn every_week_day_at_start_after_end() {
    let err =
        C::every_week_day_at_range(9, 30, Day::from("saturday"), Day::from("monday")).unwrap_err();
    assert_eq!(err, CronError::StartAfterEnd);
}

// An invalid day name inside a weekday range surfaces as InvalidDay
#[test]
fn every_week_day_range_invalid_name() {
    let err = C::every_week_day_range(Day::from("noday"), Day::from("friday")).unwrap_err();
    assert_eq!(
        err,
        CronError::InvalidDay {
            day: "noday".to_string()
        }
    );
}
