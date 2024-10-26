use strum_macros::EnumIter;


#[derive(Eq, PartialEq, Hash, EnumIter)]
pub enum BeaverType {
    Undetermined,
    Halting,
    Infinite,
}

#[derive(Eq, PartialEq, Hash, EnumIter)]
pub enum HaltingStats {
    Lifetime,
    Coverage,
    Footprint,
}

#[derive(Eq, PartialEq, Hash, EnumIter)]
pub enum InfiniteStats {
    Periodicity,
    LoopDelta,
    PCLifetime,
    PCCoverage,
    PCFootprint,
}
