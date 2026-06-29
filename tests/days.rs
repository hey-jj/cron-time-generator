//! Day-name resolution through the public `Day` type.
//!
//! These exercise the name table, normalization, and the unchecked numeric
//! passthrough that the day-list methods rely on.

use cron_time_generator::Day;

// Numeric and named Sunday resolve to the same integer.
#[test]
fn day_to_int_number_matches_name() {
    assert_eq!(
        Day::from(0).to_int().unwrap(),
        Day::from("sunday").to_int().unwrap()
    );
}

// Long names map to 0 through 6 in calendar order.
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

// Short names map to the same integers as the long forms.
#[test]
fn day_to_int_short_forms() {
    let names = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
    let ints: Vec<i64> = names
        .iter()
        .map(|n| Day::from(*n).to_int().unwrap())
        .collect();
    assert_eq!(ints, vec![0, 1, 2, 3, 4, 5, 6]);
}

// Names are trimmed and lowercased before lookup.
#[test]
fn day_to_int_trims_and_lowercases() {
    assert_eq!(Day::from(" SUNDAY ").to_int().unwrap(), 0);
}

// Numbers pass through with no clamp, including out-of-range and negative.
#[test]
fn day_to_int_number_passthrough() {
    assert_eq!(Day::from(9).to_int().unwrap(), 9);
    assert_eq!(Day::from(-3).to_int().unwrap(), -3);
}

// A String builds a Day name just like a &str.
#[test]
fn day_from_owned_string() {
    assert_eq!(Day::from("monday".to_string()).to_int().unwrap(), 1);
}
