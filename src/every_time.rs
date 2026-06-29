//! The fluent interval builder returned by `every` and `between`.
//!
//! [`EveryTime`] holds an interval and a one-shot `between` flag. The terminal
//! methods [`EveryTime::minutes`], [`EveryTime::hours`], and [`EveryTime::days`]
//! turn that state into a cron string by replacing one field of the matching
//! base.

use crate::helpers::{day, hour, minute, splice_into_position};
use crate::nums::Nums;

/// The interval kind for [`EveryTime`].
///
/// `even` is folded into `Num(2)` at construction, so only `Num`, `Uneven`,
/// and `Pair` survive here. `Pair` carries the two `between` endpoints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Interval {
    /// A step count. Values greater than 1 produce `*/n`.
    Num(i64),
    /// The odd-step interval. Produces `1-59/2`, `1-23/2`, or `1-31/2`.
    Uneven,
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
    /// becomes [`Interval::Uneven`]. Any other string also becomes
    /// [`Interval::Uneven`], matching how the source treats a non-`"even"`
    /// string in the terminal methods.
    fn from(s: &str) -> Self {
        if s == "even" {
            Interval::Num(2)
        } else {
            Interval::Uneven
        }
    }
}

/// A fluent builder for stepped or ranged cron fields.
///
/// Build one with [`crate::CronTime::every`] or [`crate::CronTime::between`],
/// then call a terminal method. The `between` form is one-shot: the first
/// terminal call consumes the range flag.
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
    /// The interval kind driving the terminal methods.
    pub interval: Interval,
    /// Whether the next terminal call should render a `between` range.
    pub between: bool,
}

impl EveryTime {
    /// Build a plain interval builder. `between` starts off.
    pub(crate) fn new(interval: Interval) -> Self {
        EveryTime {
            interval,
            between: false,
        }
    }

    /// Build a `between` builder over a start-end pair.
    pub(crate) fn between(start: i64, end: i64) -> Self {
        EveryTime {
            interval: Interval::Pair(start, end),
            between: true,
        }
    }

    /// Every nth minute, or a minute range.
    ///
    /// A `between` pair gives `start-end`. A step above 1 gives `*/n`. The
    /// uneven interval gives `1-59/2`. Anything else returns the plain minute
    /// base `"* * * * *"`.
    pub fn minutes(&mut self) -> String {
        if self.between {
            if let Interval::Pair(a, b) = self.interval {
                self.between = false;
                return splice_into_position(0, &format!("{a}-{b}"), Some(&minute()));
            }
        }

        match &self.interval {
            Interval::Num(n) if *n > 1 => splice_into_position(0, &format!("*/{n}"), None),
            Interval::Uneven => splice_into_position(0, "1-59/2", None),
            _ => minute(),
        }
    }

    /// Every nth hour, or an hour range.
    ///
    /// The hour base `"0 * * * *"` keeps its leading `0`. A `between` pair
    /// gives `start-end`. A step above 1 gives `*/n`. The uneven interval
    /// gives `1-23/2`.
    pub fn hours(&mut self) -> String {
        let base = hour();

        if self.between {
            if let Interval::Pair(a, b) = self.interval {
                self.between = false;
                return splice_into_position(1, &format!("{a}-{b}"), Some(&base));
            }
        }

        match &self.interval {
            Interval::Num(n) if *n > 1 => splice_into_position(1, &format!("*/{n}"), Some(&base)),
            Interval::Uneven => splice_into_position(1, "1-23/2", Some(&base)),
            _ => base,
        }
    }

    /// Every nth day of the month at midnight.
    ///
    /// See [`EveryTime::days_at`] to set the hour and minute. This is the
    /// no-time form, equal to `days_at(0, 0)`.
    pub fn days(&mut self) -> String {
        self.days_at(0, 0)
    }

    /// Every nth day of the month at a set hour and minute.
    ///
    /// `hours_of_day` and `minutes_of_day` fill the day base via
    /// [`crate::helpers::day`], so minutes land first in the output. A
    /// `between` pair gives `start-end`. A step above 1 gives `*/n`. The
    /// uneven interval gives `1-31/2`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cron_time_generator::CronTime;
    ///
    /// assert_eq!(CronTime::every(7).days_at(9, 5), "5 9 */7 * *");
    /// ```
    pub fn days_at(&mut self, hours_of_day: i64, minutes_of_day: i64) -> String {
        let base = day(Nums::from(hours_of_day), Nums::from(minutes_of_day));

        if self.between {
            if let Interval::Pair(a, b) = self.interval {
                self.between = false;
                return splice_into_position(2, &format!("{a}-{b}"), Some(&base));
            }
        }

        match &self.interval {
            Interval::Num(n) if *n > 1 => splice_into_position(2, &format!("*/{n}"), Some(&base)),
            Interval::Uneven => splice_into_position(2, "1-31/2", Some(&base)),
            _ => base,
        }
    }
}
