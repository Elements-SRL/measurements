# measurements

**A Rust library for type-safe physical measurements, units, SI prefixes, and ranges.**

## Features

- **Type-safe measurements** with units and SI prefixes
- **One- and two-dimensional arrays** of measurements (`M1d`, `M2d`)
- **Range types** for measurements with min, max, and step (`RangedMeasurement`)
- **Percentage type** for expressing proportions
- **Prefix arithmetic** and conversions
- **Custom derive macro** for units of measurement (`Uom`)
- **Serde support** for serialization/deserialization
- **Parallel operations** on arrays (with `ndarray` + `rayon`)

## Example

```rust
use typed_measurements::prelude::*;

let voltage = Measurement::<Volt>::new(5.0, Prefix::Milli); // 5 mV
let current = Measurement::<Ampere>::new(2.0, Prefix::None); // 2 A

let arr = M1d::<Volt>::new(vec![1.0, 2.0, 3.0], Prefix::Milli);
println!("{:?}", arr.values());
```

## Units

- Volt (`Volt`)
- Ampere (`Ampere`)
- Watt (`Watt`)
- Second (`Second`)
- Hertz (`Hertz`)
- *(Easily extensible via the `Uom` trait and derive macro)*

## Prefixes

Supports SI prefixes: Tera, Giga, Mega, Kilo, None, Milli, Micro, Nano, Femto.

## Ranges

```rust
let range = RangedMeasurement::<Volt>::new(0.0, 10.0, 0.5, Prefix::None);
assert!(range.is_in_range(Measurement::new(5.0, Prefix::None), None));
```

## Percentage

```rust
let p = percentage!(0.25);
assert_eq!(p.get_value(), 0.25);
```

## Crate Structure

- `measurement.rs` — Scalar measurements
- `m1d.rs`, `m2d.rs` — 1D and 2D arrays of measurements
- `ranged_measurement.rs` — Ranges for measurements
- `percentage.rs` — Percentage type and macro
- `prefix.rs` — SI prefix enum and arithmetic
- `uom.rs` — Units of measurement trait and types
- `prelude.rs` — Convenient re-exports

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
measurements = { path = "." }
ndarray = { version = "0.15", features = ["serde", "rayon"] }
serde = { version = "1.0", features = ["derive"] }
```

## License

MIT

---

*See the documentation for more details and examples.*