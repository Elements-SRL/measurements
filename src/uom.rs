use uom_derive::Uom;

pub trait Uom {
    fn uom() -> String;
}

#[derive(Uom, PartialEq, Debug, Clone, Copy)]
#[uom(label = V)]
pub struct Volt;

#[derive(Uom, PartialEq, Debug, Clone, Copy)]
#[uom(label = A)]
pub struct Ampere;

#[derive(Uom, PartialEq, Debug, Clone, Copy)]
#[uom(label = W)]
pub struct Watt;

#[derive(Uom, PartialEq, Debug, Clone, Copy)]
#[uom(label = s)]
pub struct Second;

#[derive(Uom, PartialEq, Debug, Clone, Copy)]
#[uom(label = Hz)]
pub struct Hertz;
