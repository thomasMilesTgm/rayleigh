//! Messing around with the way I want this to work with macros using a couple of types and traits.
#![allow(unused)]

/// The SI system of units.
pub mod systeme_international {
    /// Marker trait for units of length.
    trait BaseUnit<T> {
        fn from_equivilant(equivilant: T) -> Self;
    }

    use std::ops::{Div, Mul};

    pub use length::*;
    pub use time::*;

    /// Defines a system of units that can be used to define any other unit
    ///
    /// This will be used to do dimensional analysis when doing multiplication and division
    /// between arbitrary units that are able to be converted into `BaseUnit` (or are themselves
    /// defined by some chain of products/divisions of `BaseUnit`s)
    pub struct SystemSI<T = f32> {
        length: Meter<T>,
        // .. time, mass, temperature, current, amount, luminous_intensity
    }

    mod length {
        use super::BaseUnit;
        /*
        Ideally this looks like:
        ```
            #[derive(BaseUnit)]
            #[unit(LengthUnit)]
            pub struct Meter(pub f32);
        ```
        */
        pub struct Meter<T = f32>(pub T);

        // from #[unit(LengthUnit)]
        pub trait LengthUnit {
            fn into_base(self) -> Meter;
        }

        // from #[derive(BaseUnit)]
        impl<T> BaseUnit<T> for Meter
        where
            T: LengthUnit, // from #[unit(LenghtUnit)]
        {
            fn from_equivilant(equivilant: T) -> Self {
                equivilant.into_base()
            }
        }

        /*
        Ideally this looks like:
        ```
            #[derive(LengthUnit)]
            #[base_unit(Meter), scale(0.0254)]
            pub struct Inch(pub f32);
        ```
        */
        pub struct Inch(pub f32);
        // from #[derive(LengthUnit)] + #[base_unit(Meter), ..]
        impl LengthUnit for Inch {
            fn into_base(self) -> Meter {
                Meter(self.0 * 0.0254) // from #[.., scale(0.0254)]
            }
        }
    }
    mod time {
        use super::BaseUnit;
        /*
        Ideally this looks like:
        ```
            #[derive(BaseUnit)]
            #[unit(TimeUnit)]
            pub struct Second(pub f32);
        ```
        */
        pub struct Second<T = f32>(pub T);

        // from #[unit(TimeUnit)]
        pub trait TimeUnit {
            fn into_base(self) -> Second;
        }

        // from #[derive(BaseUnit)]
        impl<T> BaseUnit<T> for Second
        where
            T: TimeUnit, // from #[unit(TimeUnit)]
        {
            fn from_equivilant(equivilant: T) -> Self {
                equivilant.into_base()
            }
        }

        /*
        Ideally this looks like:
        ```
            #[derive(TimeUnit)]
            #[base_unit(Second), scale(60.)]
            pub struct Minute(pub f32);
        ```
        */
        pub struct Minute(pub f32);
        // from #[derive(TimeUnit)] + #[base_unit(Second), ..]
        impl TimeUnit for Minute {
            fn into_base(self) -> Second {
                Second(self.0 * 60.) // from #[.., scale(60.)]
            }
        }
    }
    // .. `mod` mass, temperature, current, amount, luminous_intensity

    pub struct CompositeSIUnit {
        pub value: f32,
        ord_length: f32,
        ord_time: f32,
        //..
    }

    // impl<T: LengthUnit> Mul<T> for CompositeSIUnit<

    #[cfg(test)]
    mod si_tests {
        use super::*;
        #[test]
        fn convert_inch() {
            let result = Meter::from_equivilant(Inch(2.0));
            assert_eq!(result.0, 0.0508);
        }
        #[test]
        fn convert_minute() {
            let result = Second::from_equivilant(Minute(2.0));
            assert_eq!(result.0, 120.0);
        }
    }
}
