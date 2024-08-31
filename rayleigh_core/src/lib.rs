//! Core unit system

#![allow(dead_code)]

use std::marker::PhantomData;

use base::*;
use prefix::Prefix;

pub mod base;
mod experiment;
pub mod prefix;

pub trait Unit {
    fn as_general_unit<T>(&self) -> GeneralUnit<T>;
}

pub struct SIUnit<Base: Into<BaseUnit>> {
    prefix: Prefix,
    _phantom: PhantomData<Base>,
}

pub struct ExpUnit<Base: Into<BaseUnit>, T> {
    base: SIUnit<Base>,
    exponent: T,
}

/// arbitrary combination of [`SIUnits`](SIUnit) multiplied together
pub struct GeneralUnit<T> {
    distance: Option<ExpUnit<Meter, T>>,
    mass: Option<ExpUnit<Kilogram, T>>,
    time: Option<ExpUnit<Second, T>>,
    current: Option<ExpUnit<Ampere, T>>,
    temperature: Option<ExpUnit<Kelvin, T>>,
    amount: Option<ExpUnit<Mole, T>>,
    luminous_intensity: Option<ExpUnit<Candela, T>>,
}

pub struct UnitWithValue<T> {
    value: T,
    unit: GeneralUnit<T>,
}

pub struct MetersPerSecond<T>(pub T);
