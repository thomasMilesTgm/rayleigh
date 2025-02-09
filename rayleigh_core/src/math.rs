use ordered_float::FloatCore;

use crate::{base::*, traits::Number};

impl<N: Number> std::ops::Mul for RValue<N>
where
    ordered_float::OrderedFloat<N>: std::ops::Mul<Output = ordered_float::OrderedFloat<N>>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

/* === RPowers === */

/* Mul */
impl<N: Number> std::ops::Mul for RPowers<N>
where
    N: Copy,
    ordered_float::OrderedFloat<N>: std::ops::Mul<Output = ordered_float::OrderedFloat<N>>,
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
impl<N: Number> std::ops::Div for RPowers<N>
where
    N: Copy,
    ordered_float::OrderedFloat<N>: std::ops::Div<Output = ordered_float::OrderedFloat<N>>,
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
impl<N: Number> std::ops::Add for RPowers<N>
where
    N: Copy,
    ordered_float::OrderedFloat<N>: std::ops::Add<Output = ordered_float::OrderedFloat<N>>,
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
impl<N: Number> std::ops::Sub for RPowers<N>
where
    N: Copy,
    ordered_float::OrderedFloat<N>: std::ops::Sub<Output = ordered_float::OrderedFloat<N>>,
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
