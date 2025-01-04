use ordered_float::FloatCore;

use crate::base::*;

impl<T: FloatCore> std::ops::Mul for RValue<T>
where
    ordered_float::OrderedFloat<T>: std::ops::Mul<Output = ordered_float::OrderedFloat<T>>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

/* === RPowers === */

/* Mul */
impl<T: FloatCore> std::ops::Mul for RPowers<T>
where
    T: Copy,
    ordered_float::OrderedFloat<T>: std::ops::Mul<Output = ordered_float::OrderedFloat<T>>,
{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.powers
            .iter_mut()
            .zip(rhs.powers.into_iter())
            .for_each(|(value, rhs)| {
                *value = *value * rhs;
            });
        self
    }
}

/* Div */
impl<T: FloatCore> std::ops::Div for RPowers<T>
where
    T: Copy,
    ordered_float::OrderedFloat<T>: std::ops::Div<Output = ordered_float::OrderedFloat<T>>,
{
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.powers
            .iter_mut()
            .zip(rhs.powers.into_iter())
            .for_each(|(value, rhs)| {
                *value = *value / rhs;
            });
        self
    }
}

/* Add */
impl<T: FloatCore> std::ops::Add for RPowers<T>
where
    T: Copy,
    ordered_float::OrderedFloat<T>: std::ops::Add<Output = ordered_float::OrderedFloat<T>>,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.powers
            .iter_mut()
            .zip(rhs.powers.into_iter())
            .for_each(|(value, rhs)| {
                *value = *value + rhs;
            });
        self
    }
}

/* Sub */
impl<T: FloatCore> std::ops::Sub for RPowers<T>
where
    T: Copy,
    ordered_float::OrderedFloat<T>: std::ops::Sub<Output = ordered_float::OrderedFloat<T>>,
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.powers
            .iter_mut()
            .zip(rhs.powers.into_iter())
            .for_each(|(value, rhs)| {
                *value = *value - rhs;
            });
        self
    }
}
