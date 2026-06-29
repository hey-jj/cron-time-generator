# cron-time-generator

Build standard 5-field cron strings from a named, fluent API.

This crate writes cron expressions. It does not parse, schedule, or validate
ranges. Every call formats numbers and day names into a 5-field, space-separated
string of the form `minute hour day-of-month month day-of-week`.

Day-of-week uses standard cron numbering: Sunday is `0` through Saturday is `6`.

## Installation

```toml
[dependencies]
cron-time-generator = "0.1"
```

## Usage

```rust
use cron_time_generator::CronTime;

// Fixed frequencies
assert_eq!(CronTime::every_minute(), "* * * * *");
assert_eq!(CronTime::every_hour(), "0 * * * *");
assert_eq!(CronTime::every_day_at(9, 30), "30 9 * * *");

// A field can take a list. Lists join with commas.
assert_eq!(CronTime::every_hour_at(vec![10, 20, 30]), "10,20,30 * * * *");

// Named weekdays
assert_eq!(CronTime::every_monday(), "0 0 * * 1");
assert_eq!(CronTime::every_week_day(), "0 0 * * 1-5");
assert_eq!(CronTime::every_weekend(), "0 0 * * 6,0");

// Stepped intervals via the fluent builder
assert_eq!(CronTime::every(5).minutes(), "*/5 * * * *");
assert_eq!(CronTime::every(2).hours(), "0 */2 * * *");
assert_eq!(CronTime::every("uneven").hours(), "0 1-23/2 * * *");

// Ranges via between
assert_eq!(CronTime::between(1, 4).days(), "0 0 1-4 * *");
```

## Day names

Day arguments accept names or integers. Names cover short and long forms and
match after trimming and lowercasing.

```rust
use cron_time_generator::CronTime;

let cron = CronTime::on_specific_days(&["sun", "wed", "fri"]).unwrap();
assert_eq!(cron, "0 0 * * 0,3,5");
```

## Fallible calls

Three calls return `Result`:

- `on_specific_days` and `on_specific_days_at` reject an empty day list.
- Any call that resolves a day name rejects an unknown name.
- The weekday range rejects a start day that comes after the end day.

```rust
use cron_time_generator::{CronTime, Day};

let err = CronTime::every_week_day_range(Day::from("friday"), Day::from("monday"));
assert!(err.is_err());
```

The minute or minute-list time arguments come second in the `*_at` calls but
print first in the cron string, matching the field order.

## License

Licensed under the [MIT license](LICENSE).
