use std::fmt::Debug;
use uom_derive::Uom;

/// Trait for units of measurement (UOM).
///
/// Implement this trait for each unit type to provide a string label for the unit.
pub trait Uom: Clone + Copy + Debug {
    /// Returns the string label for the unit (e.g., "V" for Volt).
    fn uom() -> String;
}

/// Represents the unit Volt (V).
#[derive(Uom, PartialEq, Debug, Clone, Copy)]
#[uom(label = V)]
pub struct Volt;

/// Represents the unit Ampere (A).
#[derive(Uom, PartialEq, Debug, Clone, Copy)]
#[uom(label = A)]
pub struct Ampere;

/// Represents the unit Watt (W).
#[derive(Uom, PartialEq, Debug, Clone, Copy)]
#[uom(label = W)]
pub struct Watt;

/// Represents the unit Second (s).
#[derive(Uom, PartialEq, Debug, Clone, Copy)]
#[uom(label = s)]
pub struct Second;

/// Represents the unit Hertz (Hz).
#[derive(Uom, PartialEq, Debug, Clone, Copy)]
#[uom(label = Hz)]
pub struct Hertz;
