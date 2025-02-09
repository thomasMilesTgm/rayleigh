//!

use crate::{
    base::*,
    o32,
    traits::{Number, RayleighUnit},
};

/// Kg m s^-2
///
/// SI unit of force
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Newton(pub o32);

impl<N: Number> RayleighUnit<N> for Newton {
    fn units() -> RPowers<N> {
        let powers = vec![
            UnitPower::kilogram(N::from(1_f32).unwrap()),
            UnitPower::meter(N::from(1.).unwrap()),
            UnitPower::second(N::from(-2.).unwrap()),
        ];

        RPowers::new(powers)
    }
}

impl Into<TypedValue<Newton, f32>> for Newton {
    fn into(self) -> TypedValue<Newton, f32> {
        TypedValue::new(*self.0)
    }
}

impl From<TypedValue<Newton, f32>> for Newton {
    fn from(value: TypedValue<Newton, f32>) -> Newton {
        Newton(value.rval.value)
    }
}
