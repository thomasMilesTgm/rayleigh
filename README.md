# Rayleigh
## Extensible Dimensional Analysis Type System

# Dream API

These crates are WIP, eventually the goal is to provide the following API:

## Provide a system capable of doing complex dimensional analysis using rusts type system

The SI unit system is the obvious choice for this, this crate will provide derivable traits
for defining units in terms of conversions & combinations of the [SI base units](https://en.wikipedia.org/wiki/SI_base_unit).

### Simple scalar conversion units

```rust
#[derive(SILengthUnit)]
#[base_unit(Meter), scale(0.0254)]
pub struct Inch(pub f32);
```

This will enable `Inch` to be converted to `Meter` when multiplied or divided by any other
`SI[Base]Unit` for the purpose of dimensional analysis.

### Composite units

Many units of interest are multiplicative combinations of base units raised to different powers.

You will be able to define these types like this:

```rust
/// Acceleration = distance / time^2
/// a = x¹t⁻²
#[CompositeUnit]
#[def(LengthUnit / powi(TimeUnit, 2))]
pub struct Acceleration(pub f32);

/// Force = Mass * Acceleration
/// F = m¹a¹
#[CompositeUnit]
#[unit(ForceUnit)]
#[unit_def(MassUnit * AccelerationUnit)]
#[orderi(MassUnit, 1)] // m¹
#[orderi(AccelerationUnit, 1)] // a¹
pub struct Newton(pub f32);

/// Spring constant = Newtons / Meter
/// K = F¹x⁻¹
#[CompositeUnit]
#[orderi(ForceUnit, 1)] // N¹
#[orderi(LengthUnit, -1.)] // m⁻¹
pub struct SpringConstant(pub f32);

/// Angular frequency of oscillation of a spring = sqrt(SpringConstant / Mass)
/// ω = K⁰⋅⁵m⁻⁰⋅⁵
#[CompositeUnit]
#[orderf(SpringContantUnit, 0.5)]
#[orderf(MassUnit, -0.5)]
pub struct SpringOscillatorFreq(pub f32)
```


### Dimensional Analysis

When `[Named|Base|Composite]Unit` types are multiplied or divided, `rayleigh` will perform 
dimensional analysis (at compile time) and produce a generic type:

```
/// The `const *_ORD` generics are the orders of each `BaseUnit` that make up the composite.
struct CompositeSIUnit<const L_ORD: f64, const M_ORD: f64, const T_ORD: f64, /*.. etc.*/>(f64);
```

By defining a struct that is a specific `CompositeUnit`, that type will automatically be 
`From<CompositeSIUnit<[ORDERS]>>` where `ORDERS` are the orders defined in the `#[order]` 
attributes. Manual dimensional the SI type of analysis angular frequency of oscillation 
shows that ω = t⁻¹:

```
ω = K⁰⋅⁵ m⁻⁰⋅⁵
K = F¹x⁻¹
=> ω = (F¹x⁻¹)⁰⋅⁵ m⁻⁰⋅⁵

F = m¹a¹
=> ω = (m¹a¹x⁻¹)⁰⋅⁵ m⁻⁰⋅⁵

a = x¹t⁻²
=> ω = (m¹(x¹t⁻²)x⁻¹)⁰⋅⁵ m⁻⁰⋅⁵
=> ω = (m¹t⁻²)⁰⋅⁵ m⁻⁰⋅⁵
=> ω = t⁻¹
```

`rayleigh` will identify this fact & generate code that looks something like this:

```rust
```





## Define custom systems of units

Suppose you'd like to define a unit system where the base unit for length is Kilometer,
and the base unit of time is Hour.

```rust
#[derive(BaseUnit)]
#[unit(LengthUnit)]
pub struct Kilometer(pub f32);

#[derive(BaseUnit)]
#[unit(TimeUnit)]
pub struct Hour(pub f32);
```
```rust
#[derive(BaseUnit)]
#[unit(LengthUnit)]
pub struct Meter(pub f32);
```


