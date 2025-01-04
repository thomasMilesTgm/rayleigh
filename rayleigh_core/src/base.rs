//! Base unit system (SI)

use std::marker::PhantomData;

use ordered_float::{FloatCore, OrderedFloat};

use crate::errors::RayleighError;

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
pub type rf32 = RValue<f32>;

#[allow(non_camel_case_types)]
pub type rf64 = RValue<f32>;

pub trait RayleighUnit<Number: FloatCore> {
    fn units() -> RPowers<Number>;
}

impl<T: RayleighUnit<Number>, Number: FloatCore> TryInto<TypedValue<T, Number>> for RValue<Number> {
    type Error = RayleighError;

    fn try_into(self) -> Result<TypedValue<T, Number>, Self::Error> {
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
pub struct TypedValue<T, Number: FloatCore> {
    pub rval: RValue<Number>,
    ty: PhantomData<T>,
}

impl<T: RayleighUnit<Number>, Number: FloatCore> TypedValue<T, Number> {
    pub fn new(value: Number) -> Self {
        TypedValue {
            rval: RValue::new::<T>(value),
            ty: PhantomData,
        }
    }
}

pub struct RValue<Number: FloatCore> {
    /// The value of the
    pub(crate) value: OrderedFloat<Number>,
    pub(crate) units: RPowers<Number>,
}

impl<Number: FloatCore> RValue<Number> {
    pub fn new<U: RayleighUnit<Number>>(value: Number) -> Self {
        RValue {
            value: OrderedFloat(value),
            units: U::units(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct RPowers<Number: FloatCore> {
    ///  Powers of each base unit
    /// [m^x, kg^x, s^x, a^x, k^x, mol^x, cd^x]
    pub powers: [OrderedFloat<Number>; 7],
}

impl<Number: FloatCore + Default> RPowers<Number> {
    pub fn new(unit_powers: Vec<UnitPower<Number>>) -> Self {
        let mut powers = [OrderedFloat(Number::default()); 7];
        unit_powers.into_iter().for_each(|up| {
            powers[up.unit.index()] = up.power;
        });

        Self { powers }
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
