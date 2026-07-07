//! Build standard 5-field cron strings from a fluent, named API.
//!
//! This crate writes cron expressions. It does not parse, schedule, or
//! validate ranges. Every method formats numbers and day names into a
//! 5-field, space-separated string of the form
//! `minute hour day-of-month month day-of-week`.
//!
//! Day-of-week uses standard cron numbering: Sunday is `0` through Saturday
//! is `6`. A field can hold one value or a comma-separated list. The builder
//! emits whatever you ask for, including out-of-range numbers, so pair it with
//! a validator if you need to reject bad input.
//!
//! # Examples
//!
//! ```
//! use cron_time_generator::CronTime;
//!
//! assert_eq!(CronTime::every_minute(), "* * * * *");
//! assert_eq!(CronTime::every_day_at(9, 30), "30 9 * * *");
//! assert_eq!(CronTime::every_week_day(), "0 0 * * 1-5");
//! assert_eq!(CronTime::every(5).minutes(), "*/5 * * * *");
//! ```
//!
//! ## Fallible calls
//!
//! Seven associated functions return [`Result`]:
//! [`CronTime::on_specific_days`], [`CronTime::on_specific_days_at`],
//! [`CronTime::every_week_at`], [`CronTime::every_week_day_range`],
//! [`CronTime::every_week_day_at_range`], [`CronTime::every_weekend_range`],
//! and [`CronTime::every_weekend_at_range`]. They can reject an empty day list,
//! unknown day names, or a weekday range where the start comes after the end.
//! Everything else is total.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod days;
mod error;
mod every_time;
mod helpers;
mod one_or_many;

pub use days::Day;
pub use error::CronError;
pub use every_time::{EveryTime, Interval};
pub use one_or_many::OneOrMany;

use helpers::{day, days_to_field, hour, minute, validate_start_to_end_day};

/// The entry point. A stateless namespace for the builder functions.
///
/// `CronTime` carries no state. Users normally call associated functions such
/// as `CronTime::every_minute()`. The unit struct groups the API under one
/// name. Each method returns a cron string, or a [`Result`] for the fallible
/// calls. See the crate docs for the field layout and numbering.
pub struct CronTime;

impl CronTime {
    /// Start a stepped interval builder. Call a terminal method to finish.
    ///
    /// Pass an integer for `*/n`, `"even"` for a step of 2, or `"uneven"` for
    /// the odd-step form. The interval is an integer step, so a fractional
    /// value cannot be expressed. Only `"even"` and `"uneven"` are keywords.
    /// Any other string renders the plain field base, the same as `every(1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cron_time_generator::CronTime;
    ///
    /// assert_eq!(CronTime::every(2).hours(), "0 */2 * * *");
    /// assert_eq!(CronTime::every("uneven").minutes(), "1-59/2 * * * *");
    /// assert_eq!(CronTime::every("noop").minutes(), "* * * * *");
    /// ```
    pub fn every(interval: impl Into<Interval>) -> EveryTime {
        EveryTime::new(interval.into())
    }

    /// Start a range builder over `start` to `end`.
    ///
    /// The first terminal call renders `start-end` in its field and clears the
    /// range flag. The endpoints are not checked for order.
    pub fn between(start: i64, end: i64) -> EveryTime {
        EveryTime::between(start, end)
    }

    /// Every minute: `"* * * * *"`.
    pub fn every_minute() -> String {
        minute()
    }

    /// Every hour on the hour: `"0 * * * *"`.
    pub fn every_hour() -> String {
        hour()
    }

    /// Every hour at the given minute or minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use cron_time_generator::CronTime;
    ///
    /// assert_eq!(CronTime::every_hour_at(15), "15 * * * *");
    /// assert_eq!(CronTime::every_hour_at(vec![10, 20, 30]), "10,20,30 * * * *");
    /// ```
    pub fn every_hour_at(minutes_of_the_hour: impl Into<OneOrMany>) -> String {
        format!("{} * * * *", minutes_of_the_hour.into())
    }

    /// Every day at midnight: `"0 0 * * *"`.
    pub fn every_day() -> String {
        day(OneOrMany::One(0), OneOrMany::One(0))
    }

    /// Every day at the given hour and minute.
    ///
    /// The first argument is the hour, the second the minute, even though the
    /// minute prints first in the cron string.
    ///
    /// # Examples
    ///
    /// ```
    /// use cron_time_generator::CronTime;
    ///
    /// assert_eq!(CronTime::every_day_at(10, 30), "30 10 * * *");
    /// ```
    pub fn every_day_at(
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> String {
        day(hours_of_the_day.into(), minutes_of_the_hour.into())
    }

    /// Every Sunday at the given hour and minute. Day-of-week field is `0`.
    pub fn every_sunday_at(
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> String {
        weekday_at(hours_of_the_day.into(), minutes_of_the_hour.into(), 0)
    }

    /// Every Sunday at midnight: `"0 0 * * 0"`.
    pub fn every_sunday() -> String {
        Self::every_sunday_at(0, 0)
    }

    /// Every Monday at the given hour and minute. Day-of-week field is `1`.
    pub fn every_monday_at(
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> String {
        weekday_at(hours_of_the_day.into(), minutes_of_the_hour.into(), 1)
    }

    /// Every Monday at midnight: `"0 0 * * 1"`.
    pub fn every_monday() -> String {
        Self::every_monday_at(0, 0)
    }

    /// Every Tuesday at the given hour and minute. Day-of-week field is `2`.
    pub fn every_tuesday_at(
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> String {
        weekday_at(hours_of_the_day.into(), minutes_of_the_hour.into(), 2)
    }

    /// Every Tuesday at midnight: `"0 0 * * 2"`.
    pub fn every_tuesday() -> String {
        Self::every_tuesday_at(0, 0)
    }

    /// Every Wednesday at the given hour and minute. Day-of-week field is `3`.
    pub fn every_wednesday_at(
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> String {
        weekday_at(hours_of_the_day.into(), minutes_of_the_hour.into(), 3)
    }

    /// Every Wednesday at midnight: `"0 0 * * 3"`.
    pub fn every_wednesday() -> String {
        Self::every_wednesday_at(0, 0)
    }

    /// Every Thursday at the given hour and minute. Day-of-week field is `4`.
    pub fn every_thursday_at(
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> String {
        weekday_at(hours_of_the_day.into(), minutes_of_the_hour.into(), 4)
    }

    /// Every Thursday at midnight: `"0 0 * * 4"`.
    pub fn every_thursday() -> String {
        Self::every_thursday_at(0, 0)
    }

    /// Every Friday at the given hour and minute. Day-of-week field is `5`.
    pub fn every_friday_at(
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> String {
        weekday_at(hours_of_the_day.into(), minutes_of_the_hour.into(), 5)
    }

    /// Every Friday at midnight: `"0 0 * * 5"`.
    pub fn every_friday() -> String {
        Self::every_friday_at(0, 0)
    }

    /// Every Saturday at the given hour and minute. Day-of-week field is `6`.
    pub fn every_saturday_at(
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> String {
        weekday_at(hours_of_the_day.into(), minutes_of_the_hour.into(), 6)
    }

    /// Every Saturday at midnight: `"0 0 * * 6"`.
    pub fn every_saturday() -> String {
        Self::every_saturday_at(0, 0)
    }

    /// On the given days of the week at midnight.
    ///
    /// Returns [`CronError::EmptyDays`] when `days` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use cron_time_generator::CronTime;
    ///
    /// let cron = CronTime::on_specific_days(&["monday", "wednesday", "friday"]).unwrap();
    /// assert_eq!(cron, "0 0 * * 1,3,5");
    /// ```
    pub fn on_specific_days<D>(days: &[D]) -> Result<String, CronError>
    where
        D: Clone + Into<Day>,
    {
        if days.is_empty() {
            return Err(CronError::EmptyDays);
        }
        let days = days_to_field(days.iter().cloned())?;
        Ok(format!("0 0 * * {days}"))
    }

    /// On the given days of the week at the given hour and minute.
    ///
    /// Returns [`CronError::EmptyDays`] when `days` is empty.
    pub fn on_specific_days_at<D>(
        days: &[D],
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> Result<String, CronError>
    where
        D: Clone + Into<Day>,
    {
        if days.is_empty() {
            return Err(CronError::EmptyDays);
        }
        let days = days_to_field(days.iter().cloned())?;
        Ok(format!(
            "{} {} * * {days}",
            minutes_of_the_hour.into(),
            hours_of_the_day.into(),
        ))
    }

    /// Every week on Sunday at midnight: `"0 0 * * 0"`.
    pub fn every_week() -> String {
        Self::every_week_at(&[0i64], 0, 0).expect("numeric day cannot fail")
    }

    /// On the given days of the week at the given hour and minute.
    ///
    /// Unlike [`CronTime::on_specific_days`] this does not reject an empty
    /// list. An empty list leaves the day field blank.
    ///
    /// # Examples
    ///
    /// ```
    /// use cron_time_generator::CronTime;
    ///
    /// let cron = CronTime::every_week_at(&["monday", "wednesday"], 9, 30).unwrap();
    /// assert_eq!(cron, "30 9 * * 1,3");
    /// ```
    pub fn every_week_at<D>(
        days_of_the_week: &[D],
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> Result<String, CronError>
    where
        D: Clone + Into<Day>,
    {
        let days = days_to_field(days_of_the_week.iter().cloned())?;
        Ok(format!(
            "{} {} * * {days}",
            minutes_of_the_hour.into(),
            hours_of_the_day.into(),
        ))
    }

    /// Every weekday, Monday through Friday: `"0 0 * * 1-5"`.
    ///
    /// Returns a [`String`], not a [`Result`], because the fixed Monday to
    /// Friday range always runs forward. Use
    /// [`CronTime::every_week_day_range`] for a custom pair that can fail the
    /// order check.
    pub fn every_week_day() -> String {
        Self::every_week_day_range(Day::from("monday"), Day::from("friday"))
            .expect("monday precedes friday")
    }

    /// Every weekday over a custom range at midnight.
    ///
    /// Returns [`CronError::StartAfterEnd`] when the start day comes after the
    /// end day.
    ///
    /// # Examples
    ///
    /// ```
    /// use cron_time_generator::{CronTime, Day};
    ///
    /// let cron = CronTime::every_week_day_range(Day::from("sunday"), Day::from("thursday")).unwrap();
    /// assert_eq!(cron, "0 0 * * 0-4");
    /// ```
    pub fn every_week_day_range(
        start_day: impl Into<Day>,
        end_day: impl Into<Day>,
    ) -> Result<String, CronError> {
        Self::every_week_day_at_range(0, 0, start_day, end_day)
    }

    /// Every weekday Monday through Friday at the given hour and minute.
    ///
    /// Returns a [`String`], not a [`Result`], because the fixed Monday to
    /// Friday range always runs forward. Use
    /// [`CronTime::every_week_day_at_range`] for a custom pair that can fail
    /// the order check.
    pub fn every_week_day_at(
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> String {
        Self::every_week_day_at_range(
            hours_of_the_day,
            minutes_of_the_hour,
            Day::from("monday"),
            Day::from("friday"),
        )
        .expect("monday precedes friday")
    }

    /// Every weekday over a custom range at the given hour and minute.
    ///
    /// Returns [`CronError::StartAfterEnd`] when the start day comes after the
    /// end day.
    pub fn every_week_day_at_range(
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
        start_day: impl Into<Day>,
        end_day: impl Into<Day>,
    ) -> Result<String, CronError> {
        let start = start_day.into().to_int()?;
        let end = end_day.into().to_int()?;
        validate_start_to_end_day(start, end)?;
        Ok(format!(
            "{} {} * * {start}-{end}",
            minutes_of_the_hour.into(),
            hours_of_the_day.into()
        ))
    }

    /// Every weekend, Saturday and Sunday: `"0 0 * * 6,0"`.
    ///
    /// The day field keeps the order given, so Saturday `6` comes before Sunday
    /// `0`. Returns a [`String`], not a [`Result`], because the weekend path
    /// builds a comma list and never runs the order check. Use
    /// [`CronTime::every_weekend_range`] for a custom pair.
    pub fn every_weekend() -> String {
        Self::every_weekend_range(Day::from("saturday"), Day::from("sunday"))
            .expect("numeric day cannot fail")
    }

    /// Every weekend over a custom day pair at midnight.
    ///
    /// The two days form a comma list in the order given. Order is not
    /// validated.
    ///
    /// # Examples
    ///
    /// ```
    /// use cron_time_generator::{CronTime, Day};
    ///
    /// let cron = CronTime::every_weekend_range(Day::from("friday"), Day::from("saturday")).unwrap();
    /// assert_eq!(cron, "0 0 * * 5,6");
    /// ```
    pub fn every_weekend_range(
        start_day: impl Into<Day>,
        end_day: impl Into<Day>,
    ) -> Result<String, CronError> {
        Self::every_weekend_at_range(0, 0, start_day, end_day)
    }

    /// Every weekend, Saturday and Sunday, at the given hour and minute.
    ///
    /// Returns a [`String`], not a [`Result`], because the weekend path builds
    /// a comma list and never runs the order check. Use
    /// [`CronTime::every_weekend_at_range`] for a custom pair.
    pub fn every_weekend_at(
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> String {
        Self::every_weekend_at_range(
            hours_of_the_day,
            minutes_of_the_hour,
            Day::from("saturday"),
            Day::from("sunday"),
        )
        .expect("numeric day cannot fail")
    }

    /// Every weekend over a custom day pair at the given hour and minute.
    ///
    /// The two days form a comma list in the order given. Order is not
    /// validated.
    pub fn every_weekend_at_range(
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
        start_day: impl Into<Day>,
        end_day: impl Into<Day>,
    ) -> Result<String, CronError> {
        let days = days_to_field([start_day.into(), end_day.into()])?;
        Ok(format!(
            "{} {} * * {days}",
            minutes_of_the_hour.into(),
            hours_of_the_day.into(),
        ))
    }

    /// Every month on the first at midnight: `"0 0 1 * *"`.
    pub fn every_month() -> String {
        Self::every_month_on(1, 0, 0)
    }

    /// Every month on the given day or days at the given hour and minute.
    ///
    /// # Examples
    ///
    /// ```
    /// use cron_time_generator::CronTime;
    ///
    /// assert_eq!(CronTime::every_month_on(15, 9, 30), "30 9 15 * *");
    /// assert_eq!(CronTime::every_month_on(vec![1, 15], 0, 0), "0 0 1,15 * *");
    /// ```
    pub fn every_month_on(
        days_of_the_month: impl Into<OneOrMany>,
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> String {
        format!(
            "{} {} {} * *",
            minutes_of_the_hour.into(),
            hours_of_the_day.into(),
            days_of_the_month.into()
        )
    }

    /// Every year on January 1 at midnight: `"0 0 1 1 *"`.
    pub fn every_year() -> String {
        Self::every_year_in(1, 1, 0, 0)
    }

    /// Every year in the given month or months on a set day, hour, and minute.
    ///
    /// # Examples
    ///
    /// ```
    /// use cron_time_generator::CronTime;
    ///
    /// assert_eq!(CronTime::every_year_in(6, 15, 9, 30), "30 9 15 6 *");
    /// assert_eq!(CronTime::every_year_in(vec![6, 12], 1, 0, 0), "0 0 1 6,12 *");
    /// ```
    pub fn every_year_in(
        months_of_the_year: impl Into<OneOrMany>,
        days_of_the_month: impl Into<OneOrMany>,
        hours_of_the_day: impl Into<OneOrMany>,
        minutes_of_the_hour: impl Into<OneOrMany>,
    ) -> String {
        format!(
            "{} {} {} {} *",
            minutes_of_the_hour.into(),
            hours_of_the_day.into(),
            days_of_the_month.into(),
            months_of_the_year.into()
        )
    }
}

/// Format a weekday-at string for a fixed day-of-week integer.
fn weekday_at(hours: OneOrMany, minutes: OneOrMany, dow: i64) -> String {
    format!("{minutes} {hours} * * {dow}")
}
