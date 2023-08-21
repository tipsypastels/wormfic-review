use anyhow::anyhow;
use enum_assoc::Assoc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

#[derive(Debug, Assoc, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[func(pub const fn name(self) -> &'static str)]
#[func(pub const fn about(self) -> &'static str)]
#[repr(u8)]
pub enum Rating {
    #[assoc(name = "Unread")]
    #[assoc(about = "Unread.")]
    Unread = 0,

    #[assoc(name = "Terrible")]
    #[assoc(about = "Terrible. Do not read.")]
    Terrible = 1,

    #[assoc(name = "Boring")]
    #[assoc(about = "Boring. Not worth it.")]
    Boring = 2,

    #[assoc(name = "GoodMoments")]
    #[assoc(about = "Has good moments.")]
    GoodMoments = 3,

    #[assoc(name = "Okay")]
    #[assoc(about = "Okay.")]
    Okay = 4,

    #[assoc(name = "Good")]
    #[assoc(about = "Good. Recommended.")]
    Good = 5,

    #[assoc(name = "Great")]
    #[assoc(about = "Great! Read this.")]
    Great = 6,

    #[assoc(name = "Favourite")]
    #[assoc(about = "One of my favourites.")]
    Favourite = 7,
}

impl TryFrom<u8> for Rating {
    type Error = anyhow::Error;

    fn try_from(n: u8) -> anyhow::Result<Self> {
        match n {
            0 => Ok(Self::Unread),
            1 => Ok(Self::Terrible),
            2 => Ok(Self::Boring),
            3 => Ok(Self::GoodMoments),
            4 => Ok(Self::Okay),
            5 => Ok(Self::Good),
            6 => Ok(Self::Great),
            7 => Ok(Self::Favourite),
            n => Err(anyhow!("unknown `Rating` from byte: `{n}`")),
        }
    }
}

impl FromStr for Rating {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.parse::<u8>() {
            Ok(n) => n.try_into(),
            Err(_) => match s {
                "Unread" => Ok(Self::Unread),
                "Terrible" => Ok(Self::Terrible),
                "Boring" => Ok(Self::Boring),
                "GoodMoments" => Ok(Self::GoodMoments),
                "Okay" => Ok(Self::Okay),
                "Good" => Ok(Self::Good),
                "Great" => Ok(Self::Great),
                "Favourite" => Ok(Self::Favourite),
                s => Err(anyhow!("could not parse `Rating` from string: `{s}`")),
            },
        }
    }
}

impl<'de> Deserialize<'de> for Rating {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        u8::deserialize(de)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for Rating {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_u8(*self as _)
    }
}

impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
