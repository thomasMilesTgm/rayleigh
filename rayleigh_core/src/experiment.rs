//!

use crate::base::*;
use ordered_float::{FloatCore, OrderedFloat};

/// Kg m s^-2
///
/// SI unit of force
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Newton;

impl<T: FloatCore + Default> UnitDef<T> for Newton {
    fn units() -> RPowers<T> {
        let powers = vec![
            UnitPower::kilogram(T::from(1.).unwrap()),
            UnitPower::meter(T::from(1.).unwrap()),
            UnitPower::second(T::from(-2.).unwrap()),
        ];

        RPowers::new(powers)
    }
}
