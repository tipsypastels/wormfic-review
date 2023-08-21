use serde::{Deserialize, Deserializer, Serialize};
use std::{fmt, str::FromStr};
use thousands::Separable;

#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Count(usize);

impl FromStr for Count {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.replace(',', "").parse()?))
    }
}

impl<'de> Deserialize<'de> for Count {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        use serde::de::{Error, Visitor};

        struct CountVisitor;

        impl<'de> Visitor<'de> for CountVisitor {
            type Value = Count;

            fn expecting(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
                write!(f, "a `usize`, with optional thousands separators.")
            }

            fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Count(v as _))
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Count(v.replace(',', "").parse().map_err(Error::custom)?))
            }

            fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(Count(v.replace(',', "").parse().map_err(Error::custom)?))
            }
        }

        de.deserialize_u64(CountVisitor)
    }
}

impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.separate_with_commas())
    }
}
