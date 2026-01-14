use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uom_derive::Uom;

/// Trait for units of measurement (UOM).
///
/// Implement this trait for each unit type to provide a string label for the unit.
pub trait Uom: Clone + Copy + Debug + Serialize + Send +  PartialEq {
    /// Returns the string label for the unit (e.g., "V" for Volt).
    fn uom() -> String;
}

/// Represents the unit Adimensional (F).
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Adimensional;
impl Uom for Adimensional {
    fn uom() -> String {
        "".to_string()
    }
}

/// Represents the unit Volt (V).
#[derive(Uom, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
#[uom(label = V)]
pub struct Volt;

/// Represents the unit Ampere (A).
#[derive(Uom, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
#[uom(label = A)]
pub struct Ampere;

/// Represents the unit Watt (W).
#[derive(Uom, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
#[uom(label = W)]
pub struct Watt;

/// Represents the unit Second (s).
#[derive(Uom, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
#[uom(label = s)]
pub struct Second;

/// Represents the unit Hertz (Hz).
#[derive(Uom, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
#[uom(label = Hz)]
pub struct Hertz;

/// Represents the unit Ohm (Ω).
#[derive(Uom, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
#[uom(label = Ω)]
pub struct Ohm;

/// Represents the unit Siemens (S).
#[derive(Uom, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
#[uom(label = S)]
pub struct Siemens;

/// Represents the unit Coulomb (C).
#[derive(Uom, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
#[uom(label = C)]
pub struct Coulomb;

/// Represents the unit Farad (F).
#[derive(Uom, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
#[uom(label = F)]
pub struct Farad;



#[cfg(test)]
mod uom {
    use super::*;

    #[test]
    fn equality_check() {
        assert_eq!(Volt, Volt);
    }
}