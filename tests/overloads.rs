//! Default-argument and custom-day cases (D1-D10) plus short-name lists.
//!
//! These cover the branches the source test file skips: default minutes,
//! custom weekday and weekend ranges, array day-of-month and month inputs,
//! and short day names.

mod common;

use common::is_valid_cron;
use cron_time_generator::{CronTime as C, Day};

/// Assert the produced string equals `expected` and is valid cron.
fn check(expected: &str, got: &str) {
    assert_eq!(expected, got, "string mismatch");
    assert!(is_valid_cron(got), "produced invalid cron: {got}");
}

// D1
#[test]
fn every_day_at_default_minute() {
    check("0 6 * * *", &C::every_day_at(6, 0));
}

// D2
#[test]
fn every_sunday_at_default_minute() {
    check("0 4 * * 0", &C::every_sunday_at(4, 0));
}

// D3
#[test]
fn every_week_day_custom_range() {
    check(
        "0 0 * * 0-4",
        &C::every_week_day_range(Day::from("sunday"), Day::from("thursday")).unwrap(),
    );
}

// D4
#[test]
fn every_week_day_at_custom_range() {
    check(
        "30 1 * * 0-4",
        &C::every_week_day_at_range(1, 30, Day::from("sunday"), Day::from("thursday")).unwrap(),
    );
}

// D5
#[test]
fn every_weekend_custom_range() {
    check(
        "0 0 * * 5,6",
        &C::every_weekend_range(Day::from("friday"), Day::from("saturday")).unwrap(),
    );
}

// D6
#[test]
fn every_weekend_at_custom_range() {
    check(
        "30 1 * * 5,6",
        &C::every_weekend_at_range(1, 30, Day::from("friday"), Day::from("saturday")).unwrap(),
    );
}

// D7
#[test]
fn every_week_at_default_time() {
    check(
        "0 0 * * 1,3",
        &C::every_week_at(&["monday", "wednesday"], 0, 0).unwrap(),
    );
}

// D8
#[test]
fn every_month_on_array_default_time() {
    check("0 0 1,15 * *", &C::every_month_on(vec![1, 15], 0, 0));
}

// D9
#[test]
fn every_year_in_array_default_time() {
    check("0 0 1 6,12 *", &C::every_year_in(vec![6, 12], 1, 0, 0));
}

// D10
#[test]
fn on_specific_days_at_with_time() {
    check(
        "30 3 * * 0,2,4",
        &C::on_specific_days_at(&["sunday", "tuesday", "thursday"], 3, 30).unwrap(),
    );
}

// N4
#[test]
fn on_specific_days_short_names() {
    check(
        "0 0 * * 0,3,5",
        &C::on_specific_days(&["sun", "wed", "fri"]).unwrap(),
    );
}

// every_week_at accepts a scalar day via a single-element slice
#[test]
fn every_week_at_scalar() {
    check("0 0 * * 1", &C::every_week_at(&["monday"], 0, 0).unwrap());
}

// every_weekend keeps the reversed pair without ordering it
#[test]
fn every_weekend_reversed_pair() {
    check(
        "0 0 * * 0,6",
        &C::every_weekend_range(Day::from("sunday"), Day::from("saturday")).unwrap(),
    );
}
