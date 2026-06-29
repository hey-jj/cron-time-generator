//! The fluent interval builder returned by `every` and `between`.
//!
//! [`EveryTime`] holds an interval kind and turns it into a cron string through
//! one of the terminal methods [`EveryTime::minutes`], [`EveryTime::hours`],
//! [`EveryTime::days`], or [`EveryTime::days_at`]. Each terminal replaces one
//! field of the matching base string.

use crate::helpers::{day, hour, minute, replace_field};
use crate::one_or_many::OneOrMany;

/// The interval kind for [`EveryTime`].
///
/// `"even"` folds into `Num(2)` at construction. `"uneven"` becomes
/// [`Interval::Uneven`]. Any other string becomes [`Interval::Base`], which
/// renders the plain field base. `between` builds a [`Interval::Pair`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Interval {
    /// A step count. Values greater than 1 produce `*/n`. Anything else
    /// renders the plain base.
    Num(i64),
    /// The odd-step interval. Produces `1-59/2`, `1-23/2`, or `1-31/2`.
    Uneven,
    /// The plain base with no step. Produced by an unrecognized keyword.
    Base,
    /// A start-end pair used by `between`. Renders as a `start-end` range.
    Pair(i64, i64),
}

impl From<i64> for Interval {
    fn from(n: i64) -> Self {
        Interval::Num(n)
    }
}

impl From<&str> for Interval {
    /// Map the two interval keywords. `"even"` becomes `Num(2)`. `"uneven"`
    /// becomes [`Interval::Uneven`]. Every other string becomes
    /// [`Interval::Base`], so an unrecognized keyword renders the plain field
    /// base rather than a step.
    fn from(s: &str) -> Self {
        match s {
            "even" => Interval::Num(2),
            "uneven" => Interval::Uneven,
            _ => Interval::Base,
        }
    }
}

impl From<String> for Interval {
    fn from(s: String) -> Self {
        Interval::from(s.as_str())
    }
}

/// A fluent builder for stepped or ranged cron fields.
///
/// Build one with [`crate::CronTime::every`] or [`crate::CronTime::between`],
/// then call a terminal method. The terminals consume the builder, so each
/// builder yields one string.
///
/// # Examples
///
/// ```
/// use cron_time_generator::CronTime;
///
/// assert_eq!(CronTime::every(5).minutes(), "*/5 * * * *");
/// assert_eq!(CronTime::between(1, 4).hours(), "0 1-4 * * *");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EveryTime {
    interval: Interval,
}

impl EveryTime {
    /// Build a plain interval builder.
    pub(crate) fn new(interval: Interval) -> Self {
        EveryTime { interval }
    }

    /// Build a `between` builder over a start-end pair.
    pub(crate) fn between(start: i64, end: i64) -> Self {
        EveryTime {
            interval: Interval::Pair(start, end),
        }
    }

    /// Every nth minute, or a minute range.
    ///
    /// A `between` pair gives `start-end`. A step above 1 gives `*/n`. The
    /// uneven interval gives `1-59/2`. Anything else returns the plain minute
    /// base `"* * * * *"`.
    pub fn minutes(self) -> String {
        match self.interval {
            Interval::Pair(a, b) => replace_field(0, &format!("{a}-{b}"), Some(&minute())),
            Interval::Num(n) if n > 1 => replace_field(0, &format!("*/{n}"), None),
            Interval::Uneven => replace_field(0, "1-59/2", None),
            _ => minute(),
        }
    }

    /// Every nth hour, or an hour range.
    ///
    /// The hour base `"0 * * * *"` keeps its leading `0`. A `between` pair
    /// gives `start-end`. A step above 1 gives `*/n`. The uneven interval
    /// gives `1-23/2`.
    pub fn hours(self) -> String {
        let base = hour();
        match self.interval {
            Interval::Pair(a, b) => replace_field(1, &format!("{a}-{b}"), Some(&base)),
            Interval::Num(n) if n > 1 => replace_field(1, &format!("*/{n}"), Some(&base)),
            Interval::Uneven => replace_field(1, "1-23/2", Some(&base)),
            _ => base,
        }
    }

    /// Every nth day of the month at midnight.
    ///
    /// See [`EveryTime::days_at`] to set the hour and minute. This is the
    /// no-time form, equal to `days_at(0, 0)`.
    pub fn days(self) -> String {
        self.days_at(0, 0)
    }

    /// Every nth day of the month at a set hour and minute.
    ///
    /// `hours_of_day` and `minutes_of_day` fill the day base, so minutes land
    /// first in the output. A `between` pair gives `start-end`. A step above 1
    /// gives `*/n`. The uneven interval gives `1-31/2`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cron_time_generator::CronTime;
    ///
    /// assert_eq!(CronTime::every(7).days_at(9, 5), "5 9 */7 * *");
    /// ```
    pub fn days_at(self, hours_of_day: i64, minutes_of_day: i64) -> String {
        let base = day(
            OneOrMany::from(hours_of_day),
            OneOrMany::from(minutes_of_day),
        );
        match self.interval {
            Interval::Pair(a, b) => replace_field(2, &format!("{a}-{b}"), Some(&base)),
            Interval::Num(n) if n > 1 => replace_field(2, &format!("*/{n}"), Some(&base)),
            Interval::Uneven => replace_field(2, "1-31/2", Some(&base)),
            _ => base,
        }
    }
}
