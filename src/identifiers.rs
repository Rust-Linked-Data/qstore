use std::hash::{Hash};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct InternalUriID(pub ThirtyTwoBitID, pub ThirtyTwoBitID);

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct InternalID(pub SixtyFourBitID);

impl InternalID {
    pub const fn min_value() -> Self {
        InternalID(SixtyFourBitID::min_value())
    }
    pub const fn max_value() -> Self {
        InternalID(SixtyFourBitID::max_value())
    }
    pub const MIN: Self = Self::min_value();
    pub const MAX: Self = Self::max_value();
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Hash, Debug)]
pub struct ThirtyTwoBitID(u32);

impl ThirtyTwoBitID {
    pub const fn min_value() -> Self {
        ThirtyTwoBitID(0)
    }
    pub const fn max_value() -> Self {
        ThirtyTwoBitID(u32::max_value())
    }
    pub const MIN: Self = Self::min_value();
    pub const MAX: Self = Self::max_value();
}

impl From<ThirtyTwoBitID> for usize {
    fn from(id: ThirtyTwoBitID) -> usize {
        id.0 as usize
    }
}
impl From<usize> for ThirtyTwoBitID {
    fn from(us: usize) -> ThirtyTwoBitID {
        ThirtyTwoBitID(us as u32)
    }
}
impl From<ThirtyTwoBitID> for u32 {
    fn from(id: ThirtyTwoBitID) -> u32 {
        id.0
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Hash, Debug)]
pub struct SixtyFourBitID(u64);

impl SixtyFourBitID {
    pub const fn min_value() -> Self {
        SixtyFourBitID(0)
    }
    pub const fn max_value() -> Self {
        SixtyFourBitID(u64::max_value())
    }
    pub const MIN: Self = Self::min_value();
    pub const MAX: Self = Self::max_value();
}

impl From<SixtyFourBitID> for usize {
    fn from(id: SixtyFourBitID) -> usize {
        id.0 as usize
    }
}
impl From<usize> for SixtyFourBitID {
    fn from(us: usize) -> SixtyFourBitID {
        SixtyFourBitID(us as u64)
    }
}
impl From<InternalID> for SixtyFourBitID {
    fn from(iid: InternalID) -> SixtyFourBitID { iid.0 }
}
impl From<SixtyFourBitID> for u64 {
    fn from(id: SixtyFourBitID) -> u64 {
        id.0
    }
}

pub trait IndexedID: Clone+Hash+PartialEq+PartialOrd+Eq+Ord+Into<usize>+From<usize> {
    const MIN: usize;
    const MAX: usize;
}

impl IndexedID for ThirtyTwoBitID {
    const MIN: usize = Self::MIN.0 as usize;
    const MAX: usize = Self::MAX.0 as usize;
}
impl IndexedID for SixtyFourBitID {
    const MIN: usize = Self::MIN.0 as usize;
    const MAX: usize = Self::MAX.0 as usize;
}