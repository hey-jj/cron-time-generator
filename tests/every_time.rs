//! Fluent builder cases (E1-E15) plus the `between` one-shot behavior.
//!
//! These cover the `every` and `between` terminal methods across stepped,
//! even, uneven, and range intervals.

mod common;

use common::is_valid_cron;
use cron_time_generator::CronTime as C;

/// Assert the produced string equals `expected` and is valid cron.
fn check(expected: &str, got: &str) {
    assert_eq!(expected, got, "string mismatch");
    assert!(is_valid_cron(got), "produced invalid cron: {got}");
}

#[test]
fn every_n_steps() {
    check("*/5 * * * *", &C::every(5).minutes()); // E1
    check("0 */2 * * *", &C::every(2).hours()); // E2
    check("0 0 */7 * *", &C::every(7).days()); // E3
    check("5 9 */7 * *", &C::every(7).days_at(9, 5)); // E4
}

#[test]
fn every_one_returns_base() {
    check("* * * * *", &C::every(1).minutes()); // E5
    check("0 * * * *", &C::every(1).hours()); // E6
    check("0 0 * * *", &C::every(1).days()); // E7
}

#[test]
fn every_even() {
    check("0 */2 * * *", &C::every("even").hours()); // E8
    check("*/2 * * * *", &C::every("even").minutes()); // E9
}

#[test]
fn every_uneven() {
    check("1-59/2 * * * *", &C::every("uneven").minutes()); // E10
    check("0 1-23/2 * * *", &C::every("uneven").hours()); // E11
    check("0 0 1-31/2 * *", &C::every("uneven").days()); // E12
}

#[test]
fn between_ranges() {
    check("1-4 * * * *", &C::between(1, 4).minutes()); // E13
    check("0 1-4 * * *", &C::between(1, 4).hours()); // E14
    check("0 0 1-4 * *", &C::between(1, 4).days()); // E15
}

// every(0) falls through to the base, same as every(1)
#[test]
fn every_zero_returns_base() {
    check("* * * * *", &C::every(0).minutes());
    check("0 * * * *", &C::every(0).hours());
    check("0 0 * * *", &C::every(0).days());
}

// between is one-shot: a second terminal call no longer ranges
#[test]
fn between_is_one_shot() {
    let mut builder = C::between(10, 20);
    assert_eq!(builder.days(), "0 0 10-20 * *");
    // The pair is not a step above 1 and not uneven, so the second call
    // returns the plain base.
    assert_eq!(builder.minutes(), "* * * * *");
}

// between does not validate endpoint order
#[test]
fn between_keeps_reversed_endpoints() {
    assert_eq!(C::between(20, 10).days(), "0 0 20-10 * *");
}
