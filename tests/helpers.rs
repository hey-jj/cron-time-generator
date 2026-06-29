//! Helper-level cases (H1-H8) plus the helper edge cases.
//!
//! These exercise the field primitives directly: splice, the canonical bases,
//! the day-name table, and the list conversion.

use cron_time_generator::helpers::{
    day, days_to_integers, hour, minute, splice_into_position, validate_start_to_end_day,
};
use cron_time_generator::{CronError, Day, Nums};

// H1
#[test]
fn splice_into_position_field0() {
    assert_eq!(splice_into_position(0, "2", None), "2 * * * *");
}

// H2
#[test]
fn splice_into_position_field4() {
    assert_eq!(splice_into_position(4, "5", None), "* * * * 5");
}

// H3
#[test]
fn minute_base() {
    assert_eq!(minute(), "* * * * *");
}

// H4
#[test]
fn hour_base() {
    assert_eq!(hour(), "0 * * * *");
}

// H5
#[test]
fn day_with_hour_and_minute() {
    assert_eq!(day(Nums::from(10), Nums::from(30)), "30 10 * * *");
}

// H6
#[test]
fn day_to_int_number_matches_name() {
    assert_eq!(
        Day::from(0).to_int().unwrap(),
        Day::from("sunday").to_int().unwrap()
    );
}

// H7
#[test]
fn day_to_int_over_all_long_names() {
    let names = [
        "sunday",
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
    ];
    let ints: Vec<i64> = names
        .iter()
        .map(|n| Day::from(*n).to_int().unwrap())
        .collect();
    assert_eq!(ints, vec![0, 1, 2, 3, 4, 5, 6]);
}

// H8
#[test]
fn days_to_integers_all_long_names() {
    let days: Vec<Day> = [
        "sunday",
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
    ]
    .iter()
    .map(|n| Day::from(*n))
    .collect();
    assert_eq!(days_to_integers(&days).unwrap(), vec![0, 1, 2, 3, 4, 5, 6]);
}

// HE1
#[test]
fn day_with_defaults() {
    assert_eq!(day(Nums::from(0), Nums::from(0)), "0 0 * * *");
}

// HE2
#[test]
fn splice_into_position_explicit_base() {
    assert_eq!(splice_into_position(2, "X", Some("a b c d e")), "a b X d e");
}

// N1
#[test]
fn day_to_int_trims_and_lowercases() {
    assert_eq!(Day::from(" SUNDAY ").to_int().unwrap(), 0);
}

// N2
#[test]
fn day_to_int_short_forms() {
    let names = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
    let ints: Vec<i64> = names
        .iter()
        .map(|n| Day::from(*n).to_int().unwrap())
        .collect();
    assert_eq!(ints, vec![0, 1, 2, 3, 4, 5, 6]);
}

// N3
#[test]
fn days_to_integers_single_name() {
    assert_eq!(days_to_integers(&[Day::from("monday")]).unwrap(), vec![1]);
}

// dayToInt passes numbers through with no clamp
#[test]
fn day_to_int_number_passthrough() {
    assert_eq!(Day::from(9).to_int().unwrap(), 9);
    assert_eq!(Day::from(-3).to_int().unwrap(), -3);
}

// validateStartToEndDay accepts forward and equal ranges
#[test]
fn validate_start_to_end_day_ok() {
    assert!(validate_start_to_end_day(1, 5).is_ok());
    assert!(validate_start_to_end_day(3, 3).is_ok());
}

// validateStartToEndDay rejects a backward range
#[test]
fn validate_start_to_end_day_rejects_backward() {
    assert_eq!(
        validate_start_to_end_day(5, 1),
        Err(CronError::StartAfterEnd)
    );
}
