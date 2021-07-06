extern crate alloc;

use crate::CronSchedule;

use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer};

impl Serialize for CronSchedule {
    #[inline]
    fn serialize<SER: Serializer>(&self, ser: SER) -> Result<SER::Ok, SER::Error> {
        ser.collect_str(self)
    }
}

struct StrVisitor;

impl<'de> serde::de::Visitor<'de> for StrVisitor {
    type Value = CronSchedule;

    #[inline(always)]
    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a cron expression")
    }

    #[inline]
    fn visit_str<E: serde::de::Error>(self, input: &str) -> Result<Self::Value, E> {
        CronSchedule::parse_str(input).map_err(|err| serde::de::Error::custom(format_args!("Not a valid cron: {:?}", err)))
    }

    #[inline]
    fn visit_bytes<E: serde::de::Error>(self, input: &[u8]) -> Result<Self::Value, E> {
        match core::str::from_utf8(input) {
            Ok(text) => CronSchedule::parse_str(text).map_err(|err| serde::de::Error::custom(format_args!("Not a valid cron: {:?}", err))),
            Err(error) => Err(serde::de::Error::custom(error)),
        }
    }
}

impl<'de> Deserialize<'de> for CronSchedule {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(des: D) -> Result<Self, D::Error> {
        des.deserialize_str(StrVisitor)
    }
}
