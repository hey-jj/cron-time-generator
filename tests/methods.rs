//! Public method cases (M1-M40).
//!
//! Each case asserts the exact cron string and confirms the result is a valid
//! 5-field cron expression. The `M#` comments number each case so failures
//! point straight at the method under test.

mod common;

use common::is_valid_cron;
use cron_time_generator::CronTime as C;

/// Assert the produced string equals `expected` and is valid cron.
fn check(expected: &str, got: &str) {
    assert_eq!(expected, got, "string mismatch");
    assert!(is_valid_cron(got), "produced invalid cron: {got}");
}

#[test]
fn methods_match_expected() {
    check("* * * * *", &C::every_minute()); // M1
    check("0 * * * *", &C::every_hour()); // M2
    check("15 * * * *", &C::every_hour_at(15)); // M3
    check("10,20,30 * * * *", &C::every_hour_at(vec![10, 20, 30])); // M4
    check("0 0 * * *", &C::every_day()); // M5
    check("30 10 * * *", &C::every_day_at(10, 30)); // M6
    check(
        "10,30 6,9,12 * * *",
        &C::every_day_at(vec![6, 9, 12], vec![10, 30]),
    ); // M7

    check("0 0 * * 0", &C::every_sunday()); // M8
    check("30 9 * * 0", &C::every_sunday_at(9, 30)); // M9
    check(
        "30,45 9,10,11 * * 0",
        &C::every_sunday_at(vec![9, 10, 11], vec![30, 45]),
    ); // M10

    check("0 0 * * 1", &C::every_monday()); // M11
    check("30 9 * * 1", &C::every_monday_at(9, 30)); // M12
    check(
        "30,45 9,10,11 * * 1",
        &C::every_monday_at(vec![9, 10, 11], vec![30, 45]),
    ); // M13

    check("0 0 * * 2", &C::every_tuesday()); // M14
    check("30 9 * * 2", &C::every_tuesday_at(9, 30)); // M15
    check(
        "30,45 9,10,11 * * 2",
        &C::every_tuesday_at(vec![9, 10, 11], vec![30, 45]),
    ); // M16

    check("0 0 * * 3", &C::every_wednesday()); // M17
    check("30 9 * * 3", &C::every_wednesday_at(9, 30)); // M18
    check(
        "30,45 9,10,11 * * 3",
        &C::every_wednesday_at(vec![9, 10, 11], vec![30, 45]),
    ); // M19

    check("0 0 * * 4", &C::every_thursday()); // M20
    check("30 9 * * 4", &C::every_thursday_at(9, 30)); // M21
    check(
        "30,45 9,10,11 * * 4",
        &C::every_thursday_at(vec![9, 10, 11], vec![30, 45]),
    ); // M22

    check("0 0 * * 5", &C::every_friday()); // M23
    check("30 9 * * 5", &C::every_friday_at(9, 30)); // M24
    check(
        "30,45 9,10,11 * * 5",
        &C::every_friday_at(vec![9, 10, 11], vec![30, 45]),
    ); // M25

    check("0 0 * * 6", &C::every_saturday()); // M26
    check("30 9 * * 6", &C::every_saturday_at(9, 30)); // M27
    check(
        "30,45 9,10,11 * * 6",
        &C::every_saturday_at(vec![9, 10, 11], vec![30, 45]),
    ); // M28

    check(
        "0 0 * * 1,3,5",
        &C::on_specific_days(&["monday", "wednesday", "friday"]).unwrap(),
    ); // M29

    check("0 0 * * 0", &C::every_week()); // M30
    check(
        "30 9 * * 1,3",
        &C::every_week_at(&["monday", "wednesday"], 9, 30).unwrap(),
    ); // M31

    check("0 0 * * 1-5", &C::every_week_day()); // M32
    check("30 9 * * 1-5", &C::every_week_day_at(9, 30)); // M33

    check("0 0 * * 6,0", &C::every_weekend()); // M34
    check("15 10 * * 6,0", &C::every_weekend_at(10, 15)); // M35

    check("0 0 1 * *", &C::every_month()); // M36
    check("30 9 15 * *", &C::every_month_on(15, 9, 30)); // M37

    check("0 0 1 1 *", &C::every_year()); // M38
    check("30 9 15 6 *", &C::every_year_in(6, 15, 9, 30)); // M39

    check("0 0 10-20 * *", &C::between(10, 20).days()); // M40
}
