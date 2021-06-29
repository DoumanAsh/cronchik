//! Simple cron notation parser

#![no_std]
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

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
pub struct CronSchedule {
    second: statiki::Array<Second, {Second::MAX as usize +1}>,
    minute: statiki::Array<Minute, {Minute::MAX as usize +1}>,
    hour: statiki::Array<Hour, {Hour::MAX as usize +1}>,
    day: statiki::Array<Day, {Day::MAX as usize +1}>,
    month: statiki::Array<Month, {Month::MAX as usize}>,
}

impl CronSchedule {
    ///Parses cron expression from ascii bytes.
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
