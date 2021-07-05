//! Simple cron notation parser
//!
//!## Features
//!
//!- `serde_on` - Enables serialization/deserialization.

#![no_std]
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
///216 bytes.
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
///assert_eq!(core::mem::size_of::<CronSchedule>(), 216);
///```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde_on", derive(Serialize, Deserialize))]
pub struct CronSchedule {
    second: statiki::Array<Second, {(Second::MAX - Second::MIN) as usize + 1}>,
    minute: statiki::Array<Minute, {(Minute::MAX - Minute::MIN) as usize + 1}>,
    hour: statiki::Array<Hour, {(Hour::MAX - Hour::MIN) as usize + 1}>,
    day: statiki::Array<Day, {(Day::MAX - Day::MIN) as usize + 1}>,
    month: statiki::Array<Month, {(Month::MAX - Month::MIN) as usize + 1}>,
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

        let second = parse_next!(Second);
        let minute = parse_next!(Minute);
        let hour = parse_next!(Hour);
        let day = parse_next!(Day);
        let month = parse_next!(Month);

        if let Some(_) = text.next() {
            return Err(ParseError::Unsupported);
        }

        Ok(Self {
            second,
            minute,
            hour,
            day,
            month
        })
    }

    #[inline(always)]
    ///Returns ordered list of scheduled seconds to run at.
    pub fn seconds(&self) -> &[Second] {
        &self.second
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
    ///Returns ordered list of scheduled days to run at.
    pub fn days(&self) -> &[Day] {
        &self.day
    }

    #[inline(always)]
    ///Returns ordered list of scheduled months to run at.
    pub fn months(&self) -> &[Month] {
        &self.month
    }
}
