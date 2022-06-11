use crate::utils::IteratorExt;

const ZERO_CHAR_BYTE: u8 = b'0';
const EXPR_SPLIT: char = ',';

#[derive(Debug, Copy, Clone)]
///Describes potential error within expression
pub enum InvalidExpr {
    ///Wildcard is used with other values.
    InvalidWildCard,
    ///Indicates invalid integer used as step value.
    InvalidStepRange,
    ///Indicates that step is not integer.
    InvalidStepValue,
    ///Indicates that specified value is outside of allowed range.
    InvalidEntryRange,
    ///Indicates that specified value is not of valid type.
    InvalidEntryValue,
    ///Indicates that specified range is not valid.
    InvalidRange,
    ///Indicates that specified range contains reversed values.
    InvalidRangeRev,
    ///Indicates that too many values are parsed. Indicates Internal Error of library.
    ParserOverflow
}

#[cold]
#[inline(never)]
const fn parser_overflow() -> InvalidExpr {
    InvalidExpr::ParserOverflow
}

macro_rules! impl_into_inner {
    ($($ty:ident as $as:ty;)+) => {
        $(
            impl Into<$as> for $ty {
                #[inline(always)]
                fn into(self) -> $as {
                    self as $as
                }
            }
        )+
    };
    ($($ty:ident unpack $as:ty;)+) => {
        $(
            impl Into<$as> for $ty {
                #[inline(always)]
                fn into(self) -> $as {
                    self.0 as _
                }
            }
        )+
    }
}

impl_into_inner!(
    DayOfMonth unpack u8;
    DayOfMonth unpack usize;
    Minute unpack u8;
    Minute unpack usize;
    Hour unpack u8;
    Hour unpack usize;
);
impl_into_inner!(
    Month as u8;
    Month as usize;
    Day as u8;
    Day as usize;
);

macro_rules! impl_from_expr {
    ($text:expr) => {
        let text = $text;
        let mut result = statiki::Array::new();

        let mut fields = text.split(EXPR_SPLIT);
        while let Some(field) = fields.next() {
            if field == "*" {
                if !result.is_empty() || fields.next().is_some() {
                    return Err(InvalidExpr::InvalidWildCard);
                }

                for num in Self::MIN..=Self::MAX {
                    if result.push(Self::from_num_asserted(num)).is_some() {
                        return Err(parser_overflow());
                    }
                }

            } else if let Some([init, step]) = field.split("/").collect_exact() {
                let init: u8 = match init {
                    "*" => Self::MIN,
                    init => Self::from_str(init, InvalidExpr::InvalidStepValue, InvalidExpr::InvalidStepRange)?.into(),
                };
                let step: usize = Self::from_str(step, InvalidExpr::InvalidStepValue, InvalidExpr::InvalidStepRange)?.into();

                if step == 0 {
                    return Err(InvalidExpr::InvalidStepRange);
                }

                for num in (init..=Self::MAX).step_by(step) {
                    let num = Self::from_num_asserted(num);
                    if !result.contains(&num) {
                        if result.push(num).is_some() {
                            return Err(parser_overflow());
                        }
                    }
                }

                result.sort_unstable();
            } else if let Some([from, to]) = field.split("-").collect_exact() {
                let from = Self::from_str(from, InvalidExpr::InvalidRange, InvalidExpr::InvalidRange)?;
                let to = Self::from_str(to, InvalidExpr::InvalidRange, InvalidExpr::InvalidRange)?;

                if from > to {
                    return Err(InvalidExpr::InvalidRangeRev);
                }

                for num in from.into()..=to.into() {
                    let num = Self::from_num_asserted(num);
                    if !result.contains(&num) {
                        if result.push(num).is_some() {
                            return Err(parser_overflow());
                        }
                    }
                }

                result.sort_unstable();
            } else {
                let num = Self::from_str(field, InvalidExpr::InvalidEntryValue, InvalidExpr::InvalidEntryRange)?;
                if !result.contains(&num) {
                    if result.push(num).is_some() {
                        return Err(parser_overflow());
                    }
                    result.sort_unstable();
                }
            }
        }

        return Ok(result);
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
///Second of the minute.
///
///# Allowed values:
///
///- `1..=31`
pub struct DayOfMonth(u8);

impl DayOfMonth {
    ///Min possible value.
    pub const MIN: u8 = 1;
    ///Max possible value.
    pub const MAX: u8 = 31;
    ///Expression name.
    pub const NAME: &'static str = "Day of Month";

    ///Creates instance from numeric
    pub(crate) const fn from_num_asserted(num: u8) -> Self {
        Self(num)
    }

    ///Creates instance from numeric
    pub const fn from_num(num: u8) -> Option<Self> {
        if num <= Self::MAX {
            Some(Self(num))
        } else {
            None
        }
    }

    #[inline(always)]
    fn from_str(text: &str, invalid_val: InvalidExpr, invalid_range: InvalidExpr) -> Result<Self, InvalidExpr> {
        match text.parse() {
            Ok(num) if num <= Self::MAX && num >= Self::MIN => Ok(Self(num)),
            Ok(_) => return Err(invalid_range),
            Err(_) => return Err(invalid_val),
        }
    }

    ///Creates instance from cron expression
    pub fn from_expr(text: &str) -> Result<statiki::Array<Self, 31>, InvalidExpr> {
        impl_from_expr!(text);
    }
}

impl core::fmt::Display for DayOfMonth {
    #[inline(always)]
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.write_fmt(format_args!("{}", self.0))
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
///Minute of the hour.
///
///# Allowed values:
///
///- `0..=59`
pub struct Minute(u8);

impl Minute {
    ///Min possible value.
    pub const MIN: u8 = 0;
    ///Max possible value.
    pub const MAX: u8 = 59;
    ///Expression name.
    pub const NAME: &'static str = "Minute";

    ///Creates instance from numeric
    pub(crate) const fn from_num_asserted(num: u8) -> Self {
        Self(num)
    }

    ///Creates instance from numeric
    pub const fn from_num(num: u8) -> Option<Self> {
        if num < 60 {
            Some(Self(num))
        } else {
            None
        }
    }

    #[inline(always)]
    fn from_str(text: &str, invalid_val: InvalidExpr, invalid_range: InvalidExpr) -> Result<Self, InvalidExpr> {
        match text.parse() {
            Ok(num) if num <= Self::MAX => Ok(Self(num)),
            Ok(_) => return Err(invalid_range),
            Err(_) => return Err(invalid_val),
        }
    }

    ///Creates instance from cron expression
    pub fn from_expr(text: &str) -> Result<statiki::Array<Self, 60>, InvalidExpr> {
        impl_from_expr!(text);
    }
}

impl core::fmt::Display for Minute {
    #[inline(always)]
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.write_fmt(format_args!("{}", self.0))
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
///Hour of the day.
///
///# Allowed values:
///
///- `0..=23`
pub struct Hour(u8);

impl Hour {
    ///Min possible value.
    pub const MIN: u8 = 0;
    ///Max possible value.
    pub const MAX: u8 = 23;
    ///Expression name.
    pub const NAME: &'static str = "Hour";

    ///Creates instance from numeric
    pub(crate) const fn from_num_asserted(num: u8) -> Self {
        Self(num)
    }

    ///Creates instance from numeric
    pub const fn from_num(num: u8) -> Option<Self> {
        if num <= Self::MAX {
            Some(Self(num))
        } else {
            None
        }
    }

    #[inline(always)]
    fn from_str(text: &str, invalid_val: InvalidExpr, invalid_range: InvalidExpr) -> Result<Self, InvalidExpr> {
        match text.parse() {
            Ok(num) if num <= Self::MAX => Ok(Self(num)),
            Ok(_) => return Err(invalid_range),
            Err(_) => return Err(invalid_val),
        }
    }

    ///Creates instance from cron expression
    pub fn from_expr(text: &str) -> Result<statiki::Array<Self, 24>, InvalidExpr> {
        impl_from_expr!(text);
    }
}

impl core::fmt::Display for Hour {
    #[inline(always)]
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.write_fmt(format_args!("{}", self.0))
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
///Day of the week.
///
///# Allowed values:
///
///- `0..=6`
///- `SUN..=SAT` - case is ignored
pub enum Day {
    ///Sunday
    Sunday = 0,
    ///Monday
    Monday = 1,
    ///Tuesday
    Tuesday = 2,
    ///Wednesday
    Wednesday = 3,
    ///Thursday
    Thursday = 4,
    ///Friday
    Friday = 5,
    ///Friday
    Saturday = 6,
}

impl Day {
    ///Min possible value.
    pub const MIN: u8 = 0;
    ///Max possible value.
    pub const MAX: u8 = 6;
    ///Expression name.
    pub const NAME: &'static str = "Day of Week";

    ///Creates instance from numeric
    pub(crate) fn from_num_asserted(num: u8) -> Self {
        unsafe {
            core::mem::transmute(num)
        }
    }

    ///Creates instance from numeric
    pub const fn from_num(num: u8) -> Option<Self> {
        //transmute once it is const fn
        match num {
            0 => Some(Self::Sunday),
            1 => Some(Self::Monday),
            2 => Some(Self::Tuesday),
            3 => Some(Self::Wednesday),
            4 => Some(Self::Thursday),
            5 => Some(Self::Friday),
            6 => Some(Self::Saturday),
            _ => None
        }
    }

    ///Returns textual representation of cron expression
    #[inline(always)]
    pub const fn to_textual_repr(self) -> &'static str {
        match self {
            Self::Sunday => "SUN",
            Self::Monday => "MON",
            Self::Tuesday => "TUE",
            Self::Wednesday => "WED",
            Self::Thursday => "THU",
            Self::Friday => "FRI",
            Self::Saturday => "SAT",
        }
    }

    const fn from_textual_repr(text: &[u8]) -> Option<Self> {
        let text = [
            text[0].to_ascii_uppercase(),
            text[1].to_ascii_uppercase(),
            text[2].to_ascii_uppercase(),
        ];

        return match &text {
            b"SUN" => Some(Self::Sunday),
            b"MON" => Some(Self::Monday),
            b"TUE" => Some(Self::Tuesday),
            b"WED" => Some(Self::Wednesday),
            b"THU" => Some(Self::Thursday),
            b"FRI" => Some(Self::Friday),
            b"SAT" => Some(Self::Saturday),
            _ => None
        };
    }

    ///Parses day from the string accordingly to allowed values.
    pub const fn from_bytes(text: &[u8]) -> Option<Self> {
        if text.len() == 1 {
            let num = text[0];
            if num >= ZERO_CHAR_BYTE {
                return Self::from_num(num - ZERO_CHAR_BYTE);
            } else {
                return None
            }
        } else if text.len() == 3 {
            return Self::from_textual_repr(text)
        }

        None
    }

    #[inline(always)]
    fn from_str(text: &str, invalid_val: InvalidExpr, invalid_range: InvalidExpr) -> Result<Self, InvalidExpr> {
        match text.parse() {
            Ok(num) if num <= Self::MAX => Ok(Self::from_num_asserted(num)),
            Ok(_) => Err(invalid_range),
            Err(_) if text.len() == 3 => match Self::from_textual_repr(text.as_bytes()) {
                Some(num) => Ok(num),
                None => Err(invalid_val)
            },
            Err(_) => Err(invalid_val),
        }
    }

    ///Creates instance from cron expression
    pub fn from_expr(text: &str) -> Result<statiki::Array<Self, 7>, InvalidExpr> {
        impl_from_expr!(text);
    }
}

impl core::fmt::Display for Day {
    #[inline(always)]
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.write_str(self.to_textual_repr())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
///Month of the year.
///
///# Allowed values:
///
///- `1..=12`
///- `JAN..=DEC` - case is ignored
pub enum Month {
    ///January
    January = 1,
    ///February
    February = 2,
    ///March
    March = 3,
    ///April
    April = 4,
    ///May
    May = 5,
    ///June
    June = 6,
    ///July
    July = 7,
    ///August
    August = 8,
    ///September
    September = 9,
    ///October
    October = 10,
    ///November
    November = 11,
    ///December
    December = 12,
}

#[cfg(feature = "time")]
impl Into<time::Month> for Month {
    #[inline]
    fn into(self) -> time::Month {
        unsafe {
            core::mem::transmute(self)
        }
    }
}

impl Month {
    ///Min possible value.
    pub const MIN: u8 = 1;
    ///Max possible value.
    pub const MAX: u8 = 12;
    ///Expression name.
    pub const NAME: &'static str = "Month";

    ///Creates instance from numeric
    pub(crate) fn from_num_asserted(num: u8) -> Self {
        unsafe {
            core::mem::transmute(num)
        }
    }

    ///Creates instance from numeric
    pub const fn from_num(num: u8) -> Option<Self> {
        //transmute once it is const fn
        match num {
            1 => Some(Self::January),
            2 => Some(Self::February),
            3 => Some(Self::March),
            4 => Some(Self::April),
            5 => Some(Self::May),
            6 => Some(Self::June),
            7 => Some(Self::July),
            8 => Some(Self::August),
            9 => Some(Self::September),
            10 => Some(Self::October),
            11 => Some(Self::November),
            12 => Some(Self::December),
            _ => None
        }

    }

    ///Returns textual representation of cron expression
    #[inline(always)]
    pub const fn to_textual_repr(self) -> &'static str {
        match self {
            Self::January => "JAN",
            Self::February => "FEB",
            Self::March => "MAR",
            Self::April => "APR",
            Self::May => "MAY",
            Self::June => "JUN",
            Self::July => "JUL",
            Self::August => "AUG",
            Self::September => "SEP",
            Self::October => "OCT",
            Self::November => "NOV",
            Self::December => "DEC",
        }
    }

    const fn from_textual_repr(text: &[u8]) -> Option<Self> {
        let text = [
            text[0].to_ascii_uppercase(),
            text[1].to_ascii_uppercase(),
            text[2].to_ascii_uppercase(),
        ];

        return match &text {
            b"JAN" => Some(Self::January),
            b"FEB" => Some(Self::February),
            b"MAR" => Some(Self::March),
            b"APR" => Some(Self::April),
            b"MAY" => Some(Self::May),
            b"JUN" => Some(Self::June),
            b"JUL" => Some(Self::July),
            b"AUG" => Some(Self::August),
            b"SEP" => Some(Self::September),
            b"OCT" => Some(Self::October),
            b"NOV" => Some(Self::November),
            b"DEC" => Some(Self::December),
            _ => None
        };
    }

    ///Parses day from the string accordingly to allowed values.
    pub const fn from_bytes(text: &[u8]) -> Option<Self> {
        if text.len() < 3 {
            return match text {
                b"1" => Some(Self::January),
                b"2" => Some(Self::February),
                b"3" => Some(Self::March),
                b"4" => Some(Self::April),
                b"5" => Some(Self::May),
                b"6" => Some(Self::June),
                b"7" => Some(Self::July),
                b"8" => Some(Self::August),
                b"9" => Some(Self::September),
                b"10" => Some(Self::October),
                b"11" => Some(Self::November),
                b"12" => Some(Self::December),
                _ => None,
            };
        } else if text.len() == 3 {
            return Self::from_textual_repr(text);
        }

        None
    }

    #[inline(always)]
    fn from_str(text: &str, invalid_val: InvalidExpr, invalid_range: InvalidExpr) -> Result<Self, InvalidExpr> {
        match text.parse() {
            Ok(num) if num <= Self::MAX && num >= Self::MIN => Ok(Self::from_num_asserted(num)),
            Ok(_) => Err(invalid_range),
            Err(_) if text.len() == 3 => match Self::from_textual_repr(text.as_bytes()) {
                Some(num) => Ok(num),
                None => Err(invalid_val)
            },
            Err(_) => Err(invalid_val),
        }
    }

    ///Creates instance from cron expression
    pub fn from_expr(text: &str) -> Result<statiki::Array<Self, 12>, InvalidExpr> {
        impl_from_expr!(text);
    }
}

impl core::fmt::Display for Month {
    #[inline(always)]
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.write_str(self.to_textual_repr())
    }
}
