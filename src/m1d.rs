use std::marker::PhantomData;
use ndarray::{parallel::prelude::*, Array1};

use crate::{prefix::Prefix, uom::Uom};

#[derive(Clone, Debug)]
pub struct M1d<U: Uom> {
    values: Array1<f64>,
    prefix: Prefix,
    uom: PhantomData<U>,
}

impl<U: Uom> M1d<U> {
    pub fn new<T: Into<Array1<f64>>>(values: T, prefix: Prefix) -> Self {
        M1d {
            values: values.into(),
            prefix,
            uom: PhantomData,
        }
    }

    pub fn values(&self) -> Array1<f64> {
        self.values.clone()
    }

    pub fn prefix(&self) -> Prefix {
        self.prefix
    }
    
    fn convert_to(self, pfx: Prefix) -> Self {        
        let conversion_factor = self.prefix.get_conversion_factor(pfx);
        if conversion_factor == 1.0 {
            self.clone()
        } else {
            let mut s = self;
            s.values.par_mapv_inplace(|x| x * conversion_factor);
            M1d {
                values: s.values,
                prefix: pfx,
                uom: PhantomData,
            }
        }
    }
}

impl<U: Uom> PartialEq for M1d<U> {
    fn eq(&self, other: &Self) -> bool {
        if self.prefix != other.prefix {
            self.clone().convert_to(other.prefix()).values == other.values
        } else {
            self.values == other.values
        }
    }
}

#[cfg(test)]
mod m1d_tests {
    use crate::uom::Volt;
    use super::*;

    #[test]
    fn get_values() {
        let m1d = M1d::<Volt>::new(vec![1.0, 2.0, 3.0], Prefix::Milli);
        assert_eq!(m1d.values(), Array1::from(vec![1.0, 2.0, 3.0]));
    }

    #[test]
    fn convert_to() {
        let m1d = M1d::<Volt>::new(vec![1.0, 2.0, 3.0], Prefix::Milli);
        assert_eq!(m1d.convert_to(Prefix::Micro).values(), Array1::from(vec![1000.0, 2000.0, 3000.0]));
    }

    #[test]
    fn convert_to_2() {
        let m1d1 = M1d::<Volt>::new(vec![1.0, 2.0, 3.0], Prefix::Milli);
        let m1d2 = M1d::<Volt>::new(vec![1000.0, 2000.0, 3000.0], Prefix::Micro);
        assert_eq!(m1d1, m1d2);
    }

    #[test]
    fn convert_to_3() {
        // should short-circuit if prefixes are the same
        let m1d1 = M1d::<Volt>::new(vec![1.0, 2.0, 3.0], Prefix::Milli);
        let m2 = m1d1.clone();
        assert_eq!(m1d1, m2);
    }
}
