use std::{cmp::Ordering, str::FromStr};
use strum::ParseError;
use strum_macros::{EnumIter, EnumString, Display};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct StatesAndSymbols {
    pub n_states: u8,
    pub n_symbols: u8,
}

#[derive(Eq, PartialEq, Hash, EnumIter, EnumString, Display, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum BeaverType {
    Undetermined,
    Halting,
    Infinite,
}

#[derive(Eq, PartialEq, Hash, EnumIter, EnumString, Display, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum HaltingStat {
    Lifetime,
    Coverage,
    Footprint,
}

#[derive(Eq, PartialEq, Hash, EnumIter, EnumString, Display, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum InfiniteStat {
    Periodicity,
    LoopDelta,
    PCLifetime,
    PCCoverage,
    PCFootprint,
}

pub enum Stat {
    Halting(HaltingStat),
    Infinite(InfiniteStat),
}

impl ToString for Stat {
    fn to_string(&self) -> String {
        match self {
            Stat::Halting(halting_stat) => halting_stat.to_string(),
            Stat::Infinite(infinite_stat) => infinite_stat.to_string(),
        }
    }
}


impl FromStr for Stat {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(halting_stat) = HaltingStat::from_str(s) {
            Ok(Stat::Halting(halting_stat))
        } else if let Ok(infinite_stat) = InfiniteStat::from_str(s) {
            Ok(Stat::Infinite(infinite_stat))
        } else {
            Err(ParseError::VariantNotFound)
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Default, Debug)]
pub struct BeaverScore {
    pub beaver: usize,
    pub score: usize,
}

impl Ord for BeaverScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl PartialOrd for BeaverScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}