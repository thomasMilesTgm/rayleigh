//! Core unit system

#![allow(dead_code)]

use base::*;
use ordered_float::OrderedFloat;
use prefix::Prefix;

pub mod base;
pub mod errors;
mod experiment;
mod math;
pub mod prefix;
pub mod traits;

/// Type alias for [`OrderedFloat<f32>`]
#[allow(non_camel_case_types)]
pub type o32 = OrderedFloat<f32>;

/// Type alias for [`OrderedFloat<f64>`]
#[allow(non_camel_case_types)]
pub type o64 = OrderedFloat<f64>;

pub trait Field: From<o64> + Into<o64> + Copy {}

impl<T: From<o64> + Into<o64> + Copy> Field for T {}

pub trait Unit<T: Field>
where
    o64: From<T>,
{
    fn as_real_value(&self) -> RealValue<T>;
}

pub trait Value {
    fn idendity() -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct SIUnit<Base: Into<BaseUnit>> {
    prefix: Prefix,
    base: Base,
}

#[derive(Debug, Clone, Copy)]
pub struct ExpUnit<Base, T>
where
    Base: Into<BaseUnit>,
    T: Field,
    o64: From<T>,
{
    base: Base,
    exponent: T,
}

/// arbitrary combination of [`SIUnits`](SIUnit) multiplied together
#[derive(Debug, Clone, Copy)]
pub struct GeneralUnit<T: Field>
where
    o64: From<T>,
{
    distance: Option<ExpUnit<Meter, T>>,
    mass: Option<ExpUnit<Kilogram, T>>,
    time: Option<ExpUnit<Second, T>>,
    current: Option<ExpUnit<Ampere, T>>,
    temperature: Option<ExpUnit<Kelvin, T>>,
    amount: Option<ExpUnit<Mole, T>>,
    luminous_intensity: Option<ExpUnit<Candela, T>>,
}

impl<T: Field> Default for GeneralUnit<T>
where
    o64: From<T>,
{
    fn default() -> Self {
        GeneralUnit {
            distance: None,
            mass: None,
            time: None,
            current: None,
            temperature: None,
            amount: None,
            luminous_intensity: None,
        }
    }
}

pub struct RealValue<T: Field>
where
    o64: From<T>,
{
    value: T,
    unit: GeneralUnit<T>,
}

#[derive(Debug, Clone, Copy)]
pub struct MetersPerSecond(pub f64);

impl Unit<f64> for MetersPerSecond {
    fn as_real_value(&self) -> RealValue<f64> {
        RealValue {
            value: self.0,
            unit: GeneralUnit {
                distance: Some(ExpUnit {
                    base: Meter,
                    exponent: 1.,
                }),
                time: Some(ExpUnit {
                    base: Second,
                    exponent: -1.,
                }),
                ..Default::default()
            },
        }
    }
}
