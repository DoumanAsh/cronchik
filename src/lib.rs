//! Simple cron expression parser.
//!
//!## Syntax
//!
//!`<minutes> <hours> <days of month> <months> <days of week>`
//!
//!- `minute` is integer in range `0..=59`;
//!- `hour` is integer in range `0..=23`;
//!- `day of month` is integer in range `1..=31`;
//!- `month` is integer in range `1..=12` or textual representation like `JAN` or `DEC`;
//!- `day of week` is integer in range `0..=6` or textual representation like `SUN` or `SAT`;
//!
//!## Features
//!
//!- `std` - Enables use of `std` library types and traits.
//!- `serde` - Enables serialization/deserialization.
//!- `time` - Enables schedule calculation using `time03` crate.

#![no_std]
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

#[cfg(feature = "std")]
extern crate std;

mod utils;
mod types;
pub use types::*;

#[allow(unused)]
#[doc(hidden)]
#[cfg(not(debug_assertions))]
macro_rules! unreach {
    () => ({
        unsafe {
            core::hint::unreachable_unchecked();
        }
    })
}

#[allow(unused)]
#[doc(hidden)]
#[cfg(debug_assertions)]
macro_rules! unreach {
    () => ({
        unreachable!()
    })
}

use core::fmt;

///Cron expression to run once a year at midnight of January 1st.
pub const YEARLY: &'static str = "0 0 1 1 *";
///Cron expression to run once a month at midnight of first day.
pub const MONTHLY: &'static str = "0 0 1 * *";
///Cron expression to run once a week at midnight of the Sunday.
pub const WEEKLY: &'static str = "0 0 * * 0";
///Cron expression to run once a day at midnight.
pub const DAILY: &'static str = "0 0 * * *";
///Cron expression to run once a hour.
pub const HOURLY: &'static str = "0 * * * *";

#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "time")]
pub extern crate time;

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

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharAt(ch, pos) => fmt.write_fmt(format_args!("Invalid character '{ch:x}' at position {pos}")),
            Self::InvalidExpr(name, error) => fmt.write_fmt(format_args!("{name}: {:?}", error)),
            Self::Incomplete => fmt.write_str("Incomplete cron expression"),
            Self::Unsupported => fmt.write_str("Cron expression includes unsupported field (year)"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

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
///let display = format!("{}", schedule);
///assert_eq!(display, "5 * * * *");
///```
#[derive(Clone, PartialEq, Eq)]
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
                }
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
    ///
    ///Available with `time` feature
    pub fn next_time_from(&self, time: time::OffsetDateTime) -> time::OffsetDateTime {
        let offset = time.offset();
        let mut next = time + time::Duration::minutes(1);
        next = match next.replace_second(0) {
            Ok(next) => next,
            Err(_) => unreach!(),
        };
        next = match next.replace_nanosecond(0) {
            Ok(next) => next,
            Err(_) => unreach!(),
        };

        let result = loop {
            debug_assert_ne!(next.year() - time.year(), 5, "Unable to find  schedule within 4 years");

            let month = next.month() as u8;
            let day = next.day();

            if let Err(idx) = self.month.binary_search(&Month::from_num_asserted(month)) {
                let date = match self.month.get(idx) {
                    Some(month) => time::Date::from_calendar_date(next.year(), (*month).into(), 1).expect("Get next month date"),
                    None => time::Date::from_calendar_date(next.year() + 1, time::Month::January, 1).expect("Get next year date"),
                };

                let date_time = time::PrimitiveDateTime::new(date, time::Time::MIDNIGHT);
                next = date_time.assume_offset(offset);

                continue;
            }

            if let Err(idx) = self.day_m.binary_search(&DayOfMonth::from_num_asserted(day)) {
                //If not today, check next available day in schedule, if any.
                let date = match self.day_m.get(idx).and_then(|day| time::Date::from_calendar_date(next.year(), Month::from_num_asserted(month).into(), (*day).into()).ok()) {
                    Some(date) => date,
                    //If next allowed day doesn't fit the current month, then just switch to next month, unless it is last month
                    None if month < Month::MAX => time::Date::from_calendar_date(next.year(), Month::from_num_asserted(month + 1).into(), 1).expect("Get next month date"),
                    //If it is last month, then switch to next year.
                    None => time::Date::from_calendar_date(next.year() + 1, time::Month::January, 1).expect("Get next year date"),
                };

                let date_time = time::PrimitiveDateTime::new(date, time::Time::MIDNIGHT);
                next = date_time.assume_offset(offset);

                continue;
            }

            let weekday = next.weekday();
            let weekday_s = weekday.number_days_from_sunday();
            if let Err(idx) = self.day_w.binary_search(&Day::from_num_asserted(weekday_s)) {
                let date = match self.day_w.get(idx) {
                    Some(day_w) => match time::Date::from_calendar_date(next.year(), Month::from_num_asserted(month).into(), day + *day_w as u8 - weekday_s) {
                        //Day is on current week.
                        Ok(date) => date,
                        //Day is in next month so iterate onto next month (note weekday enum is in range 0..6)
                        Err(_) if month < Month::MAX => time::Date::from_calendar_date(next.year(), Month::from_num_asserted(month + 1).into(), *day_w as u8 - weekday_s).expect("Get next month date"),
                        //Day is in next year so iterate onto next month (note weekday enum is in range 0..6)
                        Err(_) => time::Date::from_calendar_date(next.year() + 1, time::Month::January, *day_w as u8 - weekday_s).expect("Get next year date"),
                    },
                    //This week doesn't work, iterate onto next week by number of days until Sunday
                    None => next.date() + time::Duration::days(time::Weekday::Sunday as i64 - weekday as i64),
                };

                let date_time = time::PrimitiveDateTime::new(date, time::Time::MIDNIGHT);
                next = date_time.assume_offset(offset);

                continue;
            }

            let hour = next.hour();
            if let Err(idx) = self.hour.binary_search(&Hour::from_num_asserted(hour)) {
                let (date, time) = match self.hour.get(idx) {
                    Some(hour) => (next.date(), time::Time::from_hms((*hour).into(), 0, 0).expect("Get next hour")),
                    //Try next day
                    None => (next.date() + time::Duration::days(1), time::Time::MIDNIGHT),
                };

                next = time::PrimitiveDateTime::new(date, time).assume_offset(offset);
                continue;
            }

            let minute = next.minute();
            if let Err(idx) = self.minute.binary_search(&Minute::from_num_asserted(minute)) {
                match self.minute.get(idx) {
                    Some(minute) => {
                        let time = time::Time::from_hms(hour, (*minute).into(), 0).expect("Get next minute");
                        next = time::PrimitiveDateTime::new(next.date(), time).assume_offset(offset);
                    },
                    //Next hour
                    None => {
                        let time = time::Time::from_hms(hour, 0, 0).expect("Get current hour");
                        next = time::PrimitiveDateTime::new(next.date(), time).assume_offset(offset) + time::Duration::hours(1);
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
    ///
    ///Available with `time` feature
    pub fn next_time_from_now(&self) -> time::OffsetDateTime {
        self.next_time_from(time::OffsetDateTime::now_utc())
    }
}

impl core::fmt::Debug for CronSchedule {
    #[inline(always)]
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, fmt)
    }
}

impl core::fmt::Display for CronSchedule {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        macro_rules! write_expr {
            ($name:ident) => {
                if self.$name.len() == self.$name.capacity() {
                    fmt.write_str("*")?;
                } else {
                    let elems = self.$name.as_slice();
                    debug_assert_ne!(elems.len(), 0);

                    let mut is_first = true;
                    let mut start = elems[0];
                    let mut end = start;
                    let mut prev: u8 = start.into();

                    let mut elems = elems.iter().skip(1);
                    while let Some(elem) = elems.next() {
                        let elem_repr: u8 = (*elem).into();

                        if (prev + 1) == elem_repr {
                            end = *elem;
                        } else {
                            if !is_first {
                                fmt.write_str(",")?;
                            }

                            is_first = false;
                            if start == end {
                                fmt.write_fmt(format_args!("{}", start))?;
                            } else {
                                fmt.write_fmt(format_args!("{}-{}", start, end))?;
                            }

                            start = *elem;
                            end = *elem;
                        }

                        prev = end.into();
                    }

                    if !is_first {
                        fmt.write_str(",")?;
                    }

                    if start == end {
                        fmt.write_fmt(format_args!("{}", start))?;
                    } else {
                        fmt.write_fmt(format_args!("{}-{}", start, end))?;
                    }
                }
            }
        }

        write_expr!(minute);
        fmt.write_str(" ")?;
        write_expr!(hour);
        fmt.write_str(" ")?;
        write_expr!(day_m);
        fmt.write_str(" ")?;
        write_expr!(month);
        fmt.write_str(" ")?;
        write_expr!(day_w);
        Ok(())
    }
}

#[inline]
#[cfg(feature = "time")]
///Gets schedule after `time`.
///
///Returns `Err` if `cron` is invalid;
pub fn parse_cron_from_time(cron: &str, time: time::OffsetDateTime) -> Result<time::OffsetDateTime, ParseError> {
    let schedule = CronSchedule::parse_str(cron)?;
    Ok(schedule.next_time_from(time))
}

#[inline]
#[cfg(feature = "time")]
///Gets schedule after current time in UTC.
///
///Returns `Err` if `cron` is invalid;
pub fn parse_cron_from_time_now(cron: &str) -> Result<time::OffsetDateTime, ParseError> {
    parse_cron_from_time(cron, time::OffsetDateTime::now_utc())
}
