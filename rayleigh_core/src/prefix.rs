//! Prefixes scale the value of a unit by a power of 10.

#[derive(Debug, Clone, Copy, Default)]
pub enum Prefix {
    /// 10e30
    Quetta,
    /// 10e27
    Ronna,
    /// 10e24
    Yotta,
    /// 10e21
    Zetta,
    /// 10e18
    Exa,
    /// 10e15
    Peta,
    /// 10e12
    Tera,
    /// 10e9
    Giga,
    /// 10e6
    Mega,
    /// 10e3
    Kilo,
    /// 10e2
    Hecto,
    /// 10e1
    Deca,
    /// 10e0
    #[default]
    None,
    /// 10e-1
    Deci,
    /// 10e-2
    Centi,
    /// 10e-3
    Milli,
    /// 10e-6
    Micro,
    /// 10e-9
    Nano,
    /// 10e-12
    Pico,
    /// 10e-15
    Femto,
    /// 10e-18
    Atto,
    /// 10e−21
    Zepto,
    /// 10e-24
    Yocto,
    /// 10e-27
    Ronto,
    /// 10e-30
    Quecto,
}

impl Prefix {}
