//! Base unit system (SI)

use std::marker::PhantomData;

use ordered_float::OrderedFloat;

use crate::{
    errors::RayleighError,
    traits::{Number, RayleighUnit},
};

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

#[allow(non_camel_case_types)]
pub type of32 = OrderedFloat<f32>;

#[allow(non_camel_case_types)]
pub type of64 = OrderedFloat<f32>;

#[allow(non_camel_case_types)]
pub type rf32 = RValue<f32>;

#[allow(non_camel_case_types)]
pub type rf64 = RValue<f32>;

impl<T: RayleighUnit<N>, N: Number> TryInto<TypedValue<T, N>> for RValue<N> {
    type Error = RayleighError;

    fn try_into(self) -> Result<TypedValue<T, N>, Self::Error> {
        if self.units == T::units() {
            Ok(TypedValue {
                rval: self,
                ty: PhantomData,
            })
        } else {
            Err(RayleighError::UnitMismatch)
        }
    }
}

/// A strongly typed [`RValue`].
pub struct TypedValue<T, N: Number> {
    pub rval: RValue<N>,
    ty: PhantomData<T>,
}

impl<T: RayleighUnit<N>, N: Number> TypedValue<T, N> {
    pub fn new(value: N) -> Self {
        TypedValue {
            rval: RValue::new::<T>(value),
            ty: PhantomData,
        }
    }
}

pub struct RValue<N: Number> {
    /// The value of the
    pub(crate) value: OrderedFloat<N>,
    pub(crate) units: RPowers<N>,
}

impl<N: Number> RValue<N> {
    pub fn new<U: RayleighUnit<N>>(value: N) -> Self {
        RValue {
            value: OrderedFloat(value),
            units: U::units(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RPowers<N: Number> {
    ///  Powers of each base unit
    /// [m^x, kg^x, s^x, a^x, k^x, mol^x, cd^x]
    pub powers: [OrderedFloat<N>; 7],
}

impl<N: Number> Default for RPowers<N> {
    fn default() -> Self {
        RPowers {
            powers: [OrderedFloat(N::default()); 7],
        }
    }
}

impl<N: Number> RPowers<N> {
    pub fn new(unit_powers: Vec<UnitPower<N>>) -> Self {
        let mut new = Self::default();
        unit_powers.into_iter().for_each(|up| {
            new.powers[up.unit.index()] = up.power;
        });

        new
    }
}

pub struct UnitPower<T> {
    pub unit: BaseUnit,
    pub power: OrderedFloat<T>,
}

impl<T> UnitPower<T> {
    pub fn new<U: Into<BaseUnit> + Default>(power: T) -> Self {
        UnitPower {
            unit: U::default().into(),
            power: OrderedFloat(power),
        }
    }

    pub fn kilogram(power: T) -> Self {
        UnitPower {
            unit: Kilogram.into(),
            power: OrderedFloat(power),
        }
    }
    pub fn meter(power: T) -> Self {
        UnitPower {
            unit: Meter.into(),
            power: OrderedFloat(power),
        }
    }
    pub fn second(power: T) -> Self {
        UnitPower {
            unit: Second.into(),
            power: OrderedFloat(power),
        }
    }
    pub fn ampere(power: T) -> Self {
        UnitPower {
            unit: Ampere.into(),
            power: OrderedFloat(power),
        }
    }
    pub fn kelvin(power: T) -> Self {
        UnitPower {
            unit: Kelvin.into(),
            power: OrderedFloat(power),
        }
    }
    pub fn mole(power: T) -> Self {
        UnitPower {
            unit: Mole.into(),
            power: OrderedFloat(power),
        }
    }
    pub fn candela(power: T) -> Self {
        UnitPower {
            unit: Candela.into(),
            power: OrderedFloat(power),
        }
    }
}

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

impl BaseUnit {
    pub fn index(self) -> usize {
        self.into()
    }
}

impl Into<usize> for BaseUnit {
    fn into(self) -> usize {
        match self {
            BaseUnit::Distance(_) => 0,
            BaseUnit::Mass(_) => 1,
            BaseUnit::Time(_) => 2,
            BaseUnit::Current(_) => 3,
            BaseUnit::Temperature(_) => 4,
            BaseUnit::Amount(_) => 5,
            BaseUnit::LuminousIntensity(_) => 6,
        }
    }
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
