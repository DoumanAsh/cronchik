//! Simple cron notation parser
//!
//!## Features
//!
//!- `serde_on` - Enables serialization/deserialization.
//!- `time` - Enables schedule calculation using `time` crate.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

#[cfg(feature = "serde_on")]
use serde::{Serialize, Deserialize};

mod utils;
mod types;
pub use types::*;

#[derive(Debug, Copy, Clone)]
///Cron expression parser error
pub enum ParseError {
    ///Indicates invalid character error.
    ///
    ///### Params:
    ///
    ///- `u8` character;
    ///- `usize` position in provided input.
    InvalidCharAt(u8, usize),
    ///Indicates invalid expression within input
    ///
    ///### Params:
    ///
    ///- `str` - Name of field;
    ///- `InvalidExpr` - describes error.
    InvalidExpr(&'static str, InvalidExpr),
    ///Cron expression is incomplete.
    Incomplete,
    ///Cron expression includes year field, which is unsupported
    Unsupported,
}

///Cron schedule.
///
///## Size
///
///184 bytes.
///
///This is relatively big struct, which might be better suited to be allocated on heap.
///So if you expect to move it a lot, prefer heap.
///Alternatively you could store cron expression as `String` and parse it each time.
///
///## Usage
///
///```
///use cronchik::CronSchedule;
///
///let schedule = CronSchedule::parse_str("5 * * * *").unwrap();
///assert_eq!(core::mem::size_of::<CronSchedule>(), 184);
///```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde_on", derive(Serialize, Deserialize))]
pub struct CronSchedule {
    minute: statiki::Array<Minute, {(Minute::MAX - Minute::MIN) as usize + 1}>,
    hour: statiki::Array<Hour, {(Hour::MAX - Hour::MIN) as usize + 1}>,
    day_m: statiki::Array<DayOfMonth, {(DayOfMonth::MAX - DayOfMonth::MIN) as usize + 1}>,
    month: statiki::Array<Month, {(Month::MAX - Month::MIN) as usize + 1}>,
    day_w: statiki::Array<Day, {(Day::MAX - Day::MIN) as usize + 1}>,
}

impl CronSchedule {
    ///Parses cron expression from string.
    pub fn parse_str(text: &str) -> Result<Self, ParseError> {
        let mut text = text.trim().split_whitespace();

        macro_rules! parse_next {
            ($ty:ty) => {
                match text.next() {
                    Some(text) => match <$ty>::from_expr(text) {
                        Ok(result) => result,
                        Err(error) => return Err(ParseError::InvalidExpr(<$ty>::NAME, error)),
                    },
                    None => return Err(ParseError::Incomplete),
                };
            }
        }

        //let second = parse_next!(Second);
        let minute = parse_next!(Minute);
        let hour = parse_next!(Hour);
        let day_m = parse_next!(DayOfMonth);
        let month = parse_next!(Month);
        let day_w = parse_next!(Day);

        if let Some(_) = text.next() {
            return Err(ParseError::Unsupported);
        }

        Ok(Self {
            minute,
            hour,
            day_m,
            month,
            day_w,
        })
    }

    #[inline(always)]
    ///Returns ordered list of scheduled days in month to run at.
    pub fn days_of_month(&self) -> &[DayOfMonth] {
        &self.day_m
    }

    #[inline(always)]
    ///Returns ordered list of scheduled minutes to run at.
    pub fn minutes(&self) -> &[Minute] {
        &self.minute
    }

    #[inline(always)]
    ///Returns ordered list of scheduled hours to run at.
    pub fn hours(&self) -> &[Hour] {
        &self.hour
    }

    #[inline(always)]
    ///Returns ordered list of scheduled days in week to run at.
    pub fn days_of_week(&self) -> &[Day] {
        &self.day_w
    }

    #[inline(always)]
    ///Returns ordered list of scheduled months to run at.
    pub fn months(&self) -> &[Month] {
        &self.month
    }

    #[cfg(feature = "time")]
    ///Returns next point if time, after `time`, accordingly to the schedule.
    pub fn next_time_from(&self, time: time::OffsetDateTime) -> time::OffsetDateTime {
        let mut next = time + time::Duration::minute();

        let result = loop {
            debug_assert_ne!(next.year() - time.year(), 5, "Unable to find  schedule within 4 years");

            let (month, day) = next.month_day();

            if let Err(idx) = self.month.binary_search(&Month::from_num_asserted(month)) {
                let date = match self.month.get(idx) {
                    Some(month) => time::Date::try_from_ymd(next.year(), *month as _, 1).expect("Get next month date"),
                    None => time::Date::try_from_ymd(next.year() + 1, 1, 1).expect("Get next year date"),
                };

                let date_time = time::PrimitiveDateTime::new(date, time::Time::midnight());
                next = date_time.assume_utc();

                continue;
            }

            if let Err(idx) = self.day_m.binary_search(&DayOfMonth::from_num_asserted(day)) {
                //If not today, check next available day in schedule, if any.
                let date = match self.day_m.get(idx).and_then(|day| time::Date::try_from_ymd(next.year(), month, (*day).into()).ok()) {
                    Some(date) => date,
                    //If next allowed day doesn't fit the current month, then just switch to next month, unless it is last month
                    None if month < Month::MAX => time::Date::try_from_ymd(next.year(), month + 1, 1).expect("Get next month date"),
                    //If it is last month, then switch to next year.
                    None => time::Date::try_from_ymd(next.year() + 1, 1, 1).expect("Get next year date"),
                };

                let date_time = time::PrimitiveDateTime::new(date, time::Time::midnight());
                next = date_time.assume_utc();

                continue;
            }

            let weekday = next.weekday();
            let weekday_s = weekday.number_days_from_sunday();
            if let Err(idx) = self.day_w.binary_search(&Day::from_num_asserted(weekday_s)) {
                let date = match self.day_w.get(idx) {
                    Some(day_w) => match time::Date::try_from_ymd(next.year(), month, day + *day_w as u8 - weekday_s) {
                        //Day is on current week.
                        Ok(date) => date,
                        //Day is in next month so iterate onto next month (note weekday enum is in range 0..6)
                        Err(_) if month < Month::MAX => time::Date::try_from_ymd(next.year(), month + 1, *day_w as u8 - weekday_s).expect("Get next month date"),
                        //Day is in next year so iterate onto next month (note weekday enum is in range 0..6)
                        Err(_) => time::Date::try_from_ymd(next.year() + 1, 1, *day_w as u8 - weekday_s).expect("Get next year date"),
                    },
                    //This week doesn't work, iterate onto next week by number of days until Sunday
                    None => next.date() + time::Duration::days(time::Weekday::Sunday as i64 - weekday as i64),
                };

                let date_time = time::PrimitiveDateTime::new(date, time::Time::midnight());
                next = date_time.assume_utc();

                continue;
            }

            let hour = next.hour();
            if let Err(idx) = self.hour.binary_search(&Hour::from_num_asserted(hour)) {
                let (date, time) = match self.hour.get(idx) {
                    Some(hour) => (next.date(), time::Time::try_from_hms((*hour).into(), 0, 0).expect("Get next hour")),
                    //Try next day
                    None => (next.date() + time::Duration::day(), time::Time::midnight()),
                };

                next = time::PrimitiveDateTime::new(date, time).assume_utc();
                continue;
            }

            let minute = next.minute();
            if let Err(idx) = self.minute.binary_search(&Minute::from_num_asserted(minute)) {
                match self.minute.get(idx) {
                    Some(minute) => {
                        let time = time::Time::try_from_hms(hour, (*minute).into(), 0).expect("Get next minute");
                        next = time::PrimitiveDateTime::new(next.date(), time).assume_utc();
                    },
                    //Next hour
                    None => {
                        let time = time::Time::try_from_hms(hour, 0, 0).expect("Get current hour");
                        next = time::PrimitiveDateTime::new(next.date(), time).assume_utc() + time::Duration::hour();
                    }
                }
                continue;
            }

            break next;
        };

        result
    }

    #[cfg(feature = "time")]
    #[inline(always)]
    ///Returns next point if time, after current time in UTC timezone.
    pub fn next_time_from_now(&self) -> time::OffsetDateTime {
        self.next_time_from(time::OffsetDateTime::now_utc())
    }
}
