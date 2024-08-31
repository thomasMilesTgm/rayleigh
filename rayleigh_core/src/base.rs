//! Base unit system (SI)

/// SI unit of distance
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Meter;

/// SI unit of mass
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kilogram;

/// SI unit of time
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Second;

/// SI unit of electric current
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ampere;

/// SI unit of temperature
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kelvin;

/// SI unit of amount of substance
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mole;

/// SI unit of Luminous intensity
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Candela;

/// The base units of the SI system
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BaseUnit {
    Distance(Meter),
    Mass(Kilogram),
    Time(Second),
    Current(Ampere),
    Temperature(Kelvin),
    Amount(Mole),
    LuminousIntensity(Candela),
}

impl From<Meter> for BaseUnit {
    fn from(_: Meter) -> Self {
        BaseUnit::Distance(Meter)
    }
}
impl From<Kilogram> for BaseUnit {
    fn from(_: Kilogram) -> Self {
        BaseUnit::Mass(Kilogram)
    }
}

impl From<Second> for BaseUnit {
    fn from(_: Second) -> Self {
        BaseUnit::Time(Second)
    }
}

impl From<Ampere> for BaseUnit {
    fn from(_: Ampere) -> Self {
        BaseUnit::Current(Ampere)
    }
}
impl From<Kelvin> for BaseUnit {
    fn from(_: Kelvin) -> Self {
        BaseUnit::Temperature(Kelvin)
    }
}

impl From<Mole> for BaseUnit {
    fn from(_: Mole) -> Self {
        BaseUnit::Amount(Mole)
    }
}
impl From<Candela> for BaseUnit {
    fn from(_: Candela) -> Self {
        BaseUnit::LuminousIntensity(Candela)
    }
}
