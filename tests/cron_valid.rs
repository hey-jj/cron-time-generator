//! Shape invariants over a swept input domain.
//!
//! Each case asserts the exact string and confirms it is valid cron. Together
//! they hold field shapes across the natural input ranges. The sweep is
//! exhaustive over small ranges, which keeps the suite free of a property
//! framework while covering the whole domain.

mod common;

use common::is_valid_cron;
use cron_time_generator::CronTime as C;

// every_hour_at over every single minute: field 0 equals the input, rest fixed.
#[test]
fn every_hour_at_single_minute_shape() {
    for m in 0..=59i64 {
        let s = C::every_hour_at(m);
        assert_eq!(s, format!("{m} * * * *"));
        assert!(is_valid_cron(&s), "invalid cron: {s}");
    }
}

// every_hour_at over a multi-minute list joins with commas.
#[test]
fn every_hour_at_list_shape() {
    for a in (0..=59i64).step_by(7) {
        for b in (0..=59i64).step_by(11) {
            let s = C::every_hour_at(vec![a, b]);
            assert_eq!(s, format!("{a},{b} * * * *"));
            assert!(is_valid_cron(&s), "invalid cron: {s}");
        }
    }
}

// every_day_at over the full hour and minute grid: "m h * * *".
#[test]
fn every_day_at_shape() {
    for h in 0..=23i64 {
        for m in 0..=59i64 {
            let s = C::every_day_at(h, m);
            assert_eq!(s, format!("{m} {h} * * *"));
            assert!(is_valid_cron(&s), "invalid cron: {s}");
        }
    }
}

// every(n).minutes() for n>1 yields "*/n * * * *".
#[test]
fn every_n_minutes_shape() {
    for n in 2..=59i64 {
        let s = C::every(n).minutes();
        assert_eq!(s, format!("*/{n} * * * *"));
        assert!(is_valid_cron(&s), "invalid cron: {s}");
    }
}

// every(n).hours() for n>1 keeps the leading 0: "0 */n * * *".
#[test]
fn every_n_hours_shape() {
    for n in 2..=23i64 {
        let s = C::every(n).hours();
        assert_eq!(s, format!("0 */{n} * * *"));
        assert!(is_valid_cron(&s), "invalid cron: {s}");
    }
}

// between(a,b).days() yields "0 0 a-b * *" for forward day ranges.
#[test]
fn between_days_shape() {
    for a in 1..=31i64 {
        for b in a..=31i64 {
            let s = C::between(a, b).days();
            assert_eq!(s, format!("0 0 {a}-{b} * *"));
            assert!(is_valid_cron(&s), "invalid cron: {s}");
        }
    }
}

// The validator itself rejects malformed inputs.
#[test]
fn validator_rejects_bad_inputs() {
    assert!(!is_valid_cron("* * * *")); // four fields
    assert!(!is_valid_cron("* * * * * *")); // six fields
    assert!(!is_valid_cron("60 * * * *")); // minute out of range
    assert!(!is_valid_cron("* 24 * * *")); // hour out of range
    assert!(!is_valid_cron("* * 0 * *")); // day of month below 1
    assert!(!is_valid_cron("* * * 13 *")); // month above 12
    assert!(!is_valid_cron(" * * * *")); // empty first field
}
