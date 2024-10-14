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
    Offset,
    Footprint,
}

#[derive(Eq, PartialEq, Hash, EnumIter)]
pub enum InfiniteStats {
    PPLifetime,
    PPOffset,
    PPFootprint,
    Periodicity,
    LoopDelta,
}
