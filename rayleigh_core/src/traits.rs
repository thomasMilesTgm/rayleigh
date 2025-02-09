//! Traits

use ordered_float::FloatCore;

use crate::{base::*, errors::RayleighError};

pub trait Number: FloatCore + Default + std::fmt::Debug {}
impl<T: FloatCore + Default + std::fmt::Debug> Number for T {}

pub struct Unknown;
pub struct Unitless;

impl<N: Number> RayleighUnit<N> for Unitless {
    fn units() -> RPowers<N> {
        RPowers::default()
    }
}

pub trait RayleighUnit<N: Number> {
    fn units() -> RPowers<N>;
}

pub trait WithValue<N: Number> {
    fn value(&self) -> N;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyUnit<T, N: Number> {
    pub powers: RPowers<N>,
    pub value: ordered_float::OrderedFloat<N>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T, N: Number> AnyUnit<T, N> {
    pub fn new(value: N, powers: RPowers<N>) -> Self {
        AnyUnit {
            powers,
            value: ordered_float::OrderedFloat(value),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn new_of(value: ordered_float::OrderedFloat<N>, powers: RPowers<N>) -> Self {
        AnyUnit {
            powers,
            value,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: RayleighUnit<N> + WithValue<N>, N: Number> From<T> for AnyUnit<T, N> {
    fn from(unit: T) -> Self {
        AnyUnit::new(unit.value(), T::units())
    }
}

impl<L, R, N: Number> std::ops::Mul<AnyUnit<R, N>> for AnyUnit<L, N> {
    type Output = AnyUnit<Unknown, N>;

    fn mul(self, rhs: AnyUnit<R, N>) -> Self::Output {
        let powers = self.powers * rhs.powers;
        let value = self.value * rhs.value;
        AnyUnit::new(*value, powers)
    }
}

impl<L, R, N: Number> std::ops::Div<AnyUnit<R, N>> for AnyUnit<L, N> {
    type Output = AnyUnit<Unknown, N>;

    fn div(self, rhs: AnyUnit<R, N>) -> Self::Output {
        let powers = self.powers / rhs.powers;
        let value = self.value / rhs.value;
        AnyUnit::new(*value, powers)
    }
}

impl<T, N: Number> std::ops::Add for AnyUnit<T, N> {
    type Output = AnyUnit<T, N>;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.value = self.value + rhs.value;
        self
    }
}

impl<T: Clone, N: Number> std::ops::AddAssign for AnyUnit<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl<T, N: Number> std::ops::Sub for AnyUnit<T, N> {
    type Output = AnyUnit<T, N>;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.value = self.value - rhs.value;
        self
    }
}

impl<T: Clone, N: Number> std::ops::SubAssign for AnyUnit<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl<T, N: Number> std::ops::Neg for AnyUnit<T, N> {
    type Output = AnyUnit<T, N>;

    fn neg(mut self) -> Self::Output {
        self.value = -self.value;
        self
    }
}

impl<N: Number> AnyUnit<Unknown, N> {
    pub fn try_cast<T: RayleighUnit<N>>(self) -> Result<AnyUnit<T, N>, RayleighError> {
        let target = T::units();
        (target == self.powers)
            .then(|| AnyUnit::new_of(self.value, self.powers))
            .ok_or_else(|| {
                let from_ty = std::any::type_name::<Unknown>();
                let to_ty = std::any::type_name::<T>();
                RayleighError::CastFailed {
                    from: format!("{from_ty}({:?})", self.powers),
                    to: format!("{to_ty}({:?})", target),
                }
            })
    }

    pub fn cast<T: RayleighUnit<N>>(self) -> AnyUnit<T, N> {
        self.try_cast::<T>().unwrap()
    }

    pub fn try_cast_into<T: RayleighUnit<N> + From<N>>(self) -> Result<T, RayleighError> {
        self.try_cast::<T>().map(|c| T::from(*c.value))
    }

    pub fn cast_into<T: RayleighUnit<N> + From<N>>(self) -> T {
        self.try_cast_into::<T>().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct Distance(f32);

    pub type RDistance = AnyUnit<Distance, f32>;

    impl RayleighUnit<f32> for Distance {
        fn units() -> RPowers<f32> {
            RPowers::new(vec![UnitPower::meter(1.)])
        }
    }

    impl WithValue<f32> for Distance {
        fn value(&self) -> f32 {
            self.0
        }
    }

    impl<T: RayleighUnit<f32> + WithValue<f32>> std::ops::Div<T> for Distance {
        type Output = AnyUnit<Unknown, f32>;

        fn div(self, rhs: T) -> Self::Output {
            let any = AnyUnit::from(self);
            let rhs = AnyUnit::from(rhs);
            any / rhs
        }
    }

    impl<T: RayleighUnit<f32> + WithValue<f32>> std::ops::Mul<T> for Distance {
        type Output = AnyUnit<Unknown, f32>;

        fn mul(self, rhs: T) -> Self::Output {
            let any = AnyUnit::from(self);
            let rhs = AnyUnit::from(rhs);
            any * rhs
        }
    }

    pub struct Time(f32);

    pub type RTime = AnyUnit<Time, f32>;

    impl RayleighUnit<f32> for Time {
        fn units() -> RPowers<f32> {
            RPowers::new(vec![UnitPower::second(1.)])
        }
    }
    impl WithValue<f32> for Time {
        fn value(&self) -> f32 {
            self.0
        }
    }
    impl<T: RayleighUnit<f32> + WithValue<f32>> std::ops::Div<T> for Time {
        type Output = AnyUnit<Unknown, f32>;

        fn div(self, rhs: T) -> Self::Output {
            let any = AnyUnit::from(self);
            let rhs = AnyUnit::from(rhs);
            any / rhs
        }
    }

    impl<T: RayleighUnit<f32> + WithValue<f32>> std::ops::Mul<T> for Time {
        type Output = AnyUnit<Unknown, f32>;

        fn mul(self, rhs: T) -> Self::Output {
            let any = AnyUnit::from(self);
            let rhs = AnyUnit::from(rhs);
            any * rhs
        }
    }

    pub struct Velocity(f32);

    pub type RVelocity = AnyUnit<Velocity, f32>;

    impl RayleighUnit<f32> for Velocity {
        fn units() -> RPowers<f32> {
            RPowers::new(vec![UnitPower::meter(1.), UnitPower::second(-1.)])
        }
    }

    impl WithValue<f32> for Velocity {
        fn value(&self) -> f32 {
            self.0
        }
    }

    impl<T: RayleighUnit<f32> + WithValue<f32>> std::ops::Div<T> for Velocity {
        type Output = AnyUnit<Unknown, f32>;

        fn div(self, rhs: T) -> Self::Output {
            let any = AnyUnit::from(self);
            let rhs = AnyUnit::from(rhs);
            any / rhs
        }
    }

    impl<T: RayleighUnit<f32> + WithValue<f32>> std::ops::Mul<T> for Velocity {
        type Output = AnyUnit<Unknown, f32>;

        fn mul(self, rhs: T) -> Self::Output {
            let any = AnyUnit::from(self);
            let rhs = AnyUnit::from(rhs);
            any * rhs
        }
    }
    pub struct Acceleration(f32);

    pub type RAcceleration = AnyUnit<Acceleration, f32>;

    impl RayleighUnit<f32> for Acceleration {
        fn units() -> RPowers<f32> {
            RPowers::new(vec![UnitPower::meter(1.), UnitPower::second(-2.)])
        }
    }

    impl WithValue<f32> for Acceleration {
        fn value(&self) -> f32 {
            self.0
        }
    }

    impl<T: RayleighUnit<f32> + WithValue<f32>> std::ops::Div<T> for Acceleration {
        type Output = AnyUnit<Unknown, f32>;

        fn div(self, rhs: T) -> Self::Output {
            let any = AnyUnit::from(self);
            let rhs = AnyUnit::from(rhs);
            any / rhs
        }
    }

    impl<T: RayleighUnit<f32> + WithValue<f32>> std::ops::Mul<T> for Acceleration {
        type Output = AnyUnit<Unknown, f32>;

        fn mul(self, rhs: T) -> Self::Output {
            let any = AnyUnit::from(self);
            let rhs = AnyUnit::from(rhs);
            any * rhs
        }
    }

    #[test]
    pub fn gubbins() {
        let distance = Distance(100.);
        let time = Time(9.6);
        let velocity: RVelocity = (distance / time).cast();
    }
}
