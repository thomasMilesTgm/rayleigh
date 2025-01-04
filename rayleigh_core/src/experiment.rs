//!

use crate::{base::*, o32};
use ordered_float::FloatCore;

/// Kg m s^-2
///
/// SI unit of force
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Newton(pub o32);

impl<T: FloatCore + Default> RayleighUnit<T> for Newton {
    fn units() -> RPowers<T> {
        let powers = vec![
            UnitPower::kilogram(T::from(1_f32).unwrap()),
            UnitPower::meter(T::from(1.).unwrap()),
            UnitPower::second(T::from(-2.).unwrap()),
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
