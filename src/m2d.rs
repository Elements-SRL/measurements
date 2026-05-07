use crate::{m1d::M1d, prefix::Prefix, prelude::Measurement, uom::Uom};
use ndarray::{Array2, Axis, concatenate};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// A two-dimensional array of measurements with a unit and SI prefix.
///
/// # Type Parameters
/// - `U`: The unit of measurement, implementing the [`Uom`] trait.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct M2d<U: Uom> {
    values: Array2<f64>,
    prefix: Prefix,
    #[serde(skip)]
    uom: PhantomData<U>,
}

impl<U: Uom> M2d<U> {
    /// Creates a new [`M2d`] with the given values and prefix.
    ///
    /// # Arguments
    /// * `values` - The values as a type convertible into `Array2<f64>`.
    /// * `prefix` - The SI prefix for the unit.
    /// # Example
    /// ```
    /// use typed_measurements::prelude::*;
    /// use ndarray::Array2;
    ///
    /// let arr = Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    /// let m = M2d::<Volt>::new(arr, Prefix::Milli);
    /// ```
    pub fn new<T: Into<Array2<f64>>>(values: T, prefix: Prefix) -> Self {
        Self {
            values: values.into(),
            prefix,
            uom: PhantomData,
        }
    }

    /// Returns a clone of the underlying values array.
    pub fn values(&self) -> Array2<f64> {
        self.values.clone()
    }

    /// Returns the SI prefix associated with this array.
    pub fn prefix(&self) -> Prefix {
        self.prefix
    }

    /// Returns the mean value of all elements as a [`Measurement<U>`].
    ///
    /// # Returns
    /// An `Option<Measurement<U>>` containing the mean, or `None` if the array is empty.
    pub fn mean(&self) -> Option<Measurement<U>> {
        Some(Measurement::new(self.values.mean()?, self.prefix()))
    }

    /// Returns the mean along the specified axis as an [`M1d<U>`].
    ///
    /// # Arguments
    /// * `axis` - The axis along which to compute the mean.
    ///
    /// # Returns
    /// An `Option<M1d<U>>` containing the mean values, or `None` if the axis is invalid.
    pub fn mean_axis(&self, axis: Axis) -> Option<M1d<U>> {
        Some(M1d::new(self.values.mean_axis(axis)?, self.prefix()))
    }

    /// Returns the std dev along the specified axis as an [`M1d<U>`].
    ///
    /// # Arguments
    /// * `axis` - The axis along which to compute the std dev.
    ///
    /// # Returns
    /// An `M1d<U>` containing the std values.
    pub fn std_axis(&self, axis: Axis, ddof: f64) -> M1d<U> {
        M1d::new(self.values.std_axis(axis, ddof), self.prefix())
    }
    /// Returns a clone of the underlying values array.
    ///
    /// # Returns
    /// A copy of the internal `Array2<f64>`.
    pub fn label(&self) -> String {
        self.mean()
            .map_or(Measurement::new(0, self.prefix()), |f| f)
            .label()
    }

    /// Converts the array to a different SI prefix, scaling all values accordingly.
    ///
    /// # Arguments
    /// * `pfx` - The target SI prefix.
    ///
    /// # Returns
    /// A new [`M2d`] with values converted to the target prefix.
    pub fn convert_to(self, pfx: Prefix) -> Self {
        let conversion_factor = self.prefix.get_conversion_factor(pfx);
        if conversion_factor == 1.0 {
            self.clone()
        } else {
            let mut s = self;
            s.values.par_mapv_inplace(|x| x * conversion_factor);
            Self {
                values: s.values,
                prefix: pfx,
                uom: PhantomData,
            }
        }
    }

    /// Returns the length of the inside 2d array.
    ///
    /// # Returns
    /// The length of the inside 2d array as usize.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Return whether the array has any elements
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Concatenate arrays along the given axis.
    pub fn concatenate_axis(&self, other: &M2d<U>, axis: Axis) -> M2d<U> {
        let other = if self.prefix != other.prefix {
            other.clone().convert_to(self.prefix())
        } else {
            other.clone()
        };
        M2d::new(
            concatenate![axis, self.values(), other.values()],
            self.prefix(),
        )
    }

    /// Check if a given element is present
    pub fn any(&self, elem: &Measurement<U>) -> bool {
        self.values()
            .iter()
            .any(|x| x == &elem.convert_to(self.prefix()).value())
    }

    /// Returns the index of the first 1D slice that matches the given M1d along the axis.
    pub fn index_of(&self, elements: M1d<U>, axis: Axis) -> Option<usize> {
        let m = elements.convert_to(self.prefix());
        let target = m.values();
        let data = self.values();

        // Verify target length matches the lane length of the chosen axis
        let required_len = data.len_of(Axis(1 - axis.index()));

        if target.len() != required_len {
            return None;
        }

        data.axis_iter(axis).position(|view| view == target)
    }

    /// Returns a new M2d starting from the specified index along the given axis.
    pub fn cut_from_index(&self, index: usize, axis: Axis) -> Option<Self> {
        let data = self.values();

        // Safety check: ensure index is within the bounds of the dimension we are slicing
        if index >= data.len_of(axis) {
            return None;
        }

        // Functional style: slice, own, and wrap
        Some(data.slice_axis(axis, (index..).into()).to_owned())
            .map(|sliced| Self::new(sliced, self.prefix()))
    }
    /// Return an M2d starting from the specified M1d<U>
    pub fn cut_from_pred(&self, elements: M1d<U>, axis: Axis) -> Option<Self> {
        self.index_of(elements, axis)
            .and_then(|idx| self.cut_from_index(idx, axis))
    }
}

impl<U: Uom> PartialEq for M2d<U> {
    /// Compares two [`M2d`] arrays for equality, converting prefixes if necessary.
    fn eq(&self, other: &Self) -> bool {
        if self.prefix != other.prefix {
            self.clone().convert_to(other.prefix()).values == other.values
        } else {
            self.values == other.values
        }
    }
}

#[cfg(test)]
mod m2d_tests {
    use super::*;
    use crate::uom::Volt;

    #[test]
    fn get_values() {
        let m = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(),
            Prefix::Milli,
        );
        assert_eq!(
            m.values(),
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap()
        );
    }

    #[test]
    fn convert_to() {
        let m = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap(),
            Prefix::Milli,
        );
        assert_eq!(
            m.convert_to(Prefix::Micro).values(),
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap()
        );
    }

    #[test]
    fn convert_to_2() {
        let m1 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap(),
            Prefix::Milli,
        );
        let m2 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(),
            Prefix::Micro,
        );
        assert_eq!(m1, m2);
    }

    #[test]
    fn convert_to_3() {
        // should short-circuit if prefixes are the same
        let m = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap(),
            Prefix::Milli,
        );
        let m2 = m.clone();
        assert_eq!(m, m2);
    }

    #[test]
    fn any_elem_present() {
        let m = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap(),
            Prefix::Milli,
        );
        assert!(m.any(&Measurement::new(2.0, Prefix::Milli)));
    }

    #[test]
    fn any_elem_not_present() {
        let m = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap(),
            Prefix::Milli,
        );
        assert!(!m.any(&Measurement::new(2.0, Prefix::Kilo)));
    }

    #[test]
    fn cut_from_pred_elements_present() {
        let p = Prefix::Milli;
        let m = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 50), Vec::from_iter((0..100).map(|x| x as f64))).unwrap(),
            p,
        );
        let res = m
            .cut_from_pred(M1d::new(vec![5.0, 55.0], p), Axis(1))
            .unwrap();
        let first_line = (5..50).map(|x| x as f64);
        let second_line = (55..100).map(|x| x as f64);
        let concat: Vec<f64> = first_line.into_iter().chain(second_line).collect();
        let control = M2d::<Volt>::new(Array2::from_shape_vec((2, 45), concat).unwrap(), p);
        assert_eq!(res, control)
    }

    #[test]
    fn cut_from_pred_elements_not_present() {
        let p = Prefix::Milli;
        let m = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 50), Vec::from_iter((0..100).map(|x| x as f64))).unwrap(),
            p,
        );
        let res = m.cut_from_pred(M1d::new(vec![6.0, 55.0], p), Axis(1));
        assert_eq!(res, None)
    }

    #[test]
    fn cut_from_pred_elements_with_no_good_len() {
        let p = Prefix::Milli;
        let m = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 50), Vec::from_iter((0..100).map(|x| x as f64))).unwrap(),
            p,
        );
        let res = m.cut_from_pred(M1d::new(vec![6.0, 55.0, 55.0], p), Axis(1));
        assert_eq!(res, None)
    }

    #[test]
    fn test_index_of_success() {
        let p = Prefix::Milli;
        let m = M2d::<Volt>::new(vec![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]], p);
        // Searching for the second row (index 1)
        let target = M1d::new(vec![3.0, 4.0], p);
        assert_eq!(m.index_of(target, Axis(0)), Some(1));
    }

    #[test]
    fn test_index_of_mismatched_dimension() {
        let p = Prefix::Milli;
        let m = M2d::<Volt>::new(vec![[1.0, 2.0], [3.0, 4.0]], p);
        // Target length is 3, but row length is 2. Should return None, not panic.
        let target = M1d::new(vec![1.0, 2.0, 3.0], p);
        assert_eq!(m.index_of(target, Axis(0)), None);
    }

    #[test]
    fn test_cut_from_index_valid() {
        let p = Prefix::Milli;
        let m = M2d::<Volt>::new(vec![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]], p);
        // Cut from index 1 along Axis 0 (Rows)
        let res = m.cut_from_index(1, Axis(0)).unwrap();
        let expected = M2d::<Volt>::new(vec![[3.0, 4.0], [5.0, 6.0]], p);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_cut_from_index_out_of_bounds() {
        let p = Prefix::Milli;
        let m = M2d::<Volt>::new(vec![[1.0, 2.0]], p);

        // Axis 0 only has 1 row (index 0). Index 5 should safely return None.
        assert!(m.cut_from_index(5, Axis(0)).is_none());
    }

    #[test]
    fn test_cut_from_index_columns() {
        let p = Prefix::Milli;
        let m = M2d::<Volt>::new(vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]], p);
        // Cut from second column (index 1)
        let res = m.cut_from_index(1, Axis(1)).unwrap();
        let expected = M2d::<Volt>::new(vec![[2.0, 3.0], [5.0, 6.0]], p);
        assert_eq!(res, expected);
    }
}
