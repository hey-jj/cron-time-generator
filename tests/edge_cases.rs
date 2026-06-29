//! Edge-case strings that the builder emits verbatim.
//!
//! The builder never validates ranges or field emptiness. An empty list leaves
//! a blank field. Numeric days pass through with no clamp. The weekend path
//! keeps source order with no calendar check. These cases lock the exact
//! strings so a refactor cannot drift them.

use cron_time_generator::{CronTime as C, Day};

// Empty lists pass through as blank fields with no space collapsing. These are
// malformed cron on purpose, so assert the raw string only.
#[test]
fn empty_minute_list_leaves_blank_field() {
    assert_eq!(C::every_hour_at(Vec::<i64>::new()), " * * * *");
}

#[test]
fn empty_hour_and_minute_lists_leave_two_blanks() {
    assert_eq!(
        C::every_day_at(Vec::<i64>::new(), Vec::<i64>::new()),
        "  * * *"
    );
}

#[test]
fn empty_week_day_list_leaves_blank_dow() {
    assert_eq!(C::every_week_at::<&str>(&[], 0, 0).unwrap(), "0 0 * * ");
}

#[test]
fn empty_month_list_leaves_blank_month() {
    assert_eq!(C::every_year_in(Vec::<i64>::new(), 1, 0, 0), "0 0 1  *");
}

// Numeric day inputs reach the output through Into<Day> unchanged.
#[test]
fn on_specific_days_numeric() {
    assert_eq!(C::on_specific_days(&[1i64, 3, 5]).unwrap(), "0 0 * * 1,3,5");
}

#[test]
fn every_week_at_numeric() {
    assert_eq!(C::every_week_at(&[1i64, 3], 9, 30).unwrap(), "30 9 * * 1,3");
}

#[test]
fn on_specific_days_at_numeric() {
    assert_eq!(
        C::on_specific_days_at(&[0i64, 2, 4], 3, 30).unwrap(),
        "30 3 * * 0,2,4"
    );
}

// A numeric day is emitted verbatim with no range check.
#[test]
fn on_specific_days_numeric_out_of_range() {
    assert_eq!(C::on_specific_days(&[9i64]).unwrap(), "0 0 * * 9");
}

// every_weekend_at_range never runs the order check, so a reversed pair is
// accepted and kept as a comma list in source order.
#[test]
fn every_weekend_at_range_reversed_pair() {
    assert_eq!(
        C::every_weekend_at_range(1, 30, Day::from("sunday"), Day::from("friday")).unwrap(),
        "30 1 * * 0,5"
    );
}

#[test]
fn every_weekend_at_range_default_pair_with_time() {
    assert_eq!(
        C::every_weekend_at_range(10, 15, Day::from("saturday"), Day::from("sunday")).unwrap(),
        "15 10 * * 6,0"
    );
}
